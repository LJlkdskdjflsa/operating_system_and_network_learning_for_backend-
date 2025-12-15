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
    // TODO: Implement using nix::unistd::fork()
    //
    // Hint for dividing work:
    // let chunk_size = n / num_workers as u64;
    // Worker i computes: (i * chunk_size + 1) ..= ((i + 1) * chunk_size)
    // Last worker handles remainder

    todo!("Implement multi-process version")
}

/// Multi-thread version using std::thread
///
/// Steps:
/// 1. Create a channel or shared state
/// 2. Spawn `num_workers` threads
/// 3. Each thread computes its portion
/// 4. Collect and sum all results
fn sum_with_threads(n: u64, num_workers: usize) -> u64 {
    // TODO: Implement using std::thread
    //
    // You can use either:
    // - mpsc::channel to send results
    // - Arc<Mutex<u64>> to accumulate results

    todo!("Implement multi-thread version")
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

    // Multi-process version
    #[cfg(target_os = "linux")]
    {
        let result = benchmark("Multi-Process version:", || {
            sum_with_processes(n, num_workers)
        });
        assert_eq!(result, expected, "Process version result mismatch!");
    }

    // Multi-thread version
    let result = benchmark("Multi-Thread version:", || {
        sum_with_threads(n, num_workers)
    });
    assert_eq!(result, expected, "Thread version result mismatch!");

    println!("{}", "=".repeat(60));
    println!("Both versions produced correct results!");

    println!("\nTry observing with:");
    println!("  htop    # Watch process/thread creation");
    println!("  strace -f ./target/release/process_vs_thread");
}
