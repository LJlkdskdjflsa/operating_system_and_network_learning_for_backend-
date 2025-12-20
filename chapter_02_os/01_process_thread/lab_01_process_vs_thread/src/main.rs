//! Lab 1: Process vs Thread Comparison
//!
//! ## Goal
//! Compare multi-process vs multi-thread approaches for parallel computation
//!
//! ## Requirements
//! Implement two versions of parallel sum (1 + 2 + ... + N):
//! 1. `sum_with_processes(n, num_workers)` - Fork child processes
//! 2. `sum_with_threads(n, num_workers)` - Spawn threads
//!
//! ## Expected Output
//! ```
//! N = 100,000,000, Workers = 4
//! Expected result: 5000000050000000
//!
//! Multi-Process version:  xxx ms, result: 5000000050000000
//! Multi-Thread version:   xxx ms, result: 5000000050000000
//! ```
//!
//! ## Hints
//! - For processes: Use `nix::unistd::fork()` to create child processes
//! - For IPC: Use `std::os::unix::net::UnixStream::pair()` or pipes
//! - Child processes need to send results back to parent
//! - For threads: Use `std::thread` + `Arc<Mutex<T>>` or channels
//!
//! ## Verification
//! ```bash
//! cargo test                    # Run automated tests
//! cargo run --release           # Run performance comparison
//! ```
//!
//! ## Acceptance Criteria
//! - [ ] `cargo test` all pass
//! - [ ] Both versions produce correct results
//! - [ ] Can observe process/thread creation with `htop`
//! - [ ] Can explain the performance difference
//!
//! Warning: Process version requires Linux (uses fork)
//!
//! Check solution/main.rs after completing

use nix::sys::wait::waitpid;
use nix::unistd::{fork, ForkResult};
use std::io::{Read, Write};
use std::os::fd::AsRawFd;
use std::os::unix::net::UnixStream;
use std::sync::mpsc;
use std::thread;
use std::time::Instant;
// ============================================================
// TODO: Implement these two functions
// ============================================================

/// Multi-process version using fork()
///
/// Steps:
/// 1. Create communication channels (pipes or unix sockets)
/// 2. Fork `num_workers` child processes
/// 3. Each child computes its portion and sends result to parent
/// 4. Parent collects all results and sums them
fn sum_with_processes(n: u64, num_workers: usize) -> u64 {
    if n == 0 || num_workers == 0 {
        return 0;
    }

    let workers = num_workers.min(n as usize);
    let chunk = (n + workers as u64 - 1) / workers as u64;
    let mut streams = Vec::with_capacity(workers);
    let mut child_pids = Vec::with_capacity(workers);

    for i in 0..workers {
        let (parent_stream, child_stream) =
            UnixStream::pair().expect("Failed to create socket pair");
        let parent_fd = parent_stream.as_raw_fd();
        let child_fd = child_stream.as_raw_fd();
        streams.push(parent_stream);

        let start = i as u64 * chunk + 1;
        let mut end = (i as u64 + 1) * chunk;
        if end > n {
            end = n;
        }

        match unsafe { fork() } {
            Ok(ForkResult::Child) => {
                eprintln!(
                    "[child pid={}] stream_fd={}",
                    std::process::id(),
                    child_fd
                );
                drop(streams);
                let local_sum = if start > end {
                    0
                } else {
                    (start..=end).sum::<u64>()
                };
                let mut stream = child_stream;
                stream
                    .write_all(&local_sum.to_le_bytes())
                    .expect("Failed to write");
                std::process::exit(0);
            }
            Ok(ForkResult::Parent { child }) => {
                eprintln!(
                    "[parent pid={}] child_pid={} stream_fd={}",
                    std::process::id(),
                    child.as_raw(),
                    parent_fd
                );
                child_pids.push(child);
                drop(child_stream);
            }
            Err(err) => panic!("Fork failed: {}", err),
        }
    }

    let mut total = 0u64;
    for mut stream in streams {
        let mut buf = [0u8; 8];
        stream.read_exact(&mut buf).expect("Failed to read");
        total += u64::from_le_bytes(buf);
    }

    for pid in child_pids {
        waitpid(pid, None).expect("Failed to wait");
    }

    total
}

/// Multi-thread version using std::thread
///
/// Steps:
/// 1. Create a channel or shared state
/// 2. Spawn `num_workers` threads
/// 3. Each thread computes its portion
/// 4. Collect and sum all results
fn sum_with_threads(n: u64, num_workers: usize) -> u64 {
    if n == 0 || num_workers == 0 {
        return 0;
    }
    let workers = num_workers.min(n as usize); // 避免開太多無事可做的 thread
    let chunk = (n + workers as u64 - 1) / workers as u64;
    let (tx, rx) = mpsc::channel::<u64>();
    for i in 0..workers {
        let tx = tx.clone();

        // 對第 i 個 worker，算它的區間
        let start = i as u64 * chunk + 1;
        let mut end = (i as u64 + 1) * chunk;
        if end > n {
            end = n;
        }

        thread::spawn(move || {
            // 如果因為 min/ceil 邏輯導致空區間，直接回 0
            let local_sum = if start > end {
                0
            } else {
                (start..=end).sum::<u64>()
            };

            tx.send(local_sum).expect("receiver dropped");
        });
    }

    drop(tx); // 很重要：關閉原始 sender，讓 rx 知道何時結束

    // 收集所有部分和
    rx.iter().sum()
}

// ============================================================
// Benchmarking code (no modification needed)
// ============================================================

fn benchmark<F>(name: &str, f: F) -> u64
where
    F: FnOnce() -> u64,
{
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();
    println!("{:25} {:?}, result: {}", name, duration, result);
    result
}

fn main() {
    // Check if we're on Linux (fork requires it)
    #[cfg(not(target_os = "linux"))]
    {
        eprintln!("Warning: Process version requires Linux (uses fork)");
        eprintln!("Thread version will still work on any platform");
    }

    let n: u64 = 100_000_000;
    let num_workers = 4;
    let expected = n * (n + 1) / 2;

    println!("N = {}, Workers = {}", n, num_workers);
    println!("Expected result: {}", expected);
    println!("{}", "=".repeat(60));

    // Multi-thread version
    let multithread_result =
        benchmark("Multi-Thread version:", || sum_with_threads(n, num_workers));
    assert_eq!(
        multithread_result, expected,
        "Thread version result mismatch!"
    );

    // Multi-process version
    #[cfg(target_os = "linux")]
    {
        let result = benchmark("Multi-Process version:", || {
            sum_with_processes(n, num_workers)
        });
        assert_eq!(result, expected, "Process version result mismatch!");
    }

    println!("{}", "=".repeat(60));
    println!("Both versions produced correct results!");

    println!("\nTry observing with:");
    println!("  htop    # Watch process/thread creation");
    println!("  strace -f ./target/release/process_vs_thread");
}
