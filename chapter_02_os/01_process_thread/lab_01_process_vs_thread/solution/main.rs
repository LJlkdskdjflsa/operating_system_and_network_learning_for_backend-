//! Lab 1 Reference Answer

use nix::sys::wait::waitpid;
use nix::unistd::{fork, ForkResult};
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::sync::mpsc;
use std::thread;
use std::time::Instant;

/// Multi-process version using fork()
fn sum_with_processes(n: u64, num_workers: usize) -> u64 {
    let chunk_size = n / num_workers as u64;
    let mut streams = Vec::new();
    let mut child_pids = Vec::new();

    for i in 0..num_workers {
        // Create a socket pair for communication
        let (parent_stream, child_stream) = UnixStream::pair().expect("Failed to create socket pair");
        streams.push(parent_stream);

        // Calculate range for this worker
        let start = i as u64 * chunk_size + 1;
        let end = if i == num_workers - 1 {
            n // Last worker takes remainder
        } else {
            (i + 1) as u64 * chunk_size
        };

        // Fork child process
        match unsafe { fork() } {
            Ok(ForkResult::Child) => {
                // Child process: compute and send result
                drop(streams); // Close parent's copies

                let partial_sum: u64 = (start..=end).sum();

                // Send result to parent
                let bytes = partial_sum.to_le_bytes();
                let mut stream = child_stream;
                stream.write_all(&bytes).expect("Failed to write");

                std::process::exit(0);
            }
            Ok(ForkResult::Parent { child }) => {
                // Parent process: continue
                child_pids.push(child);
                drop(child_stream); // Close child's copy
            }
            Err(e) => panic!("Fork failed: {}", e),
        }
    }

    // Parent: collect results from all children
    let mut total: u64 = 0;
    for mut stream in streams {
        let mut buf = [0u8; 8];
        stream.read_exact(&mut buf).expect("Failed to read");
        let partial_sum = u64::from_le_bytes(buf);
        total += partial_sum;
    }

    // Wait for all children to finish
    for pid in child_pids {
        waitpid(pid, None).expect("Failed to wait");
    }

    total
}

/// Multi-thread version using std::thread
fn sum_with_threads(n: u64, num_workers: usize) -> u64 {
    let chunk_size = n / num_workers as u64;
    let (tx, rx) = mpsc::channel();

    for i in 0..num_workers {
        let tx = tx.clone();
        let start = i as u64 * chunk_size + 1;
        let end = if i == num_workers - 1 {
            n
        } else {
            (i + 1) as u64 * chunk_size
        };

        thread::spawn(move || {
            let partial_sum: u64 = (start..=end).sum();
            tx.send(partial_sum).expect("Failed to send");
        });
    }

    // Drop the original sender so rx.iter() can complete
    drop(tx);

    // Collect all results
    rx.iter().sum()
}

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
    let result = benchmark("Multi-Thread version:", || sum_with_threads(n, num_workers));
    assert_eq!(result, expected, "Thread version result mismatch!");

    println!("{}", "=".repeat(60));
    println!("Both versions produced correct results!");

    println!("\nObservations to make:");
    println!("1. Run 'htop' while this runs - see process/thread creation");
    println!("2. Run 'strace -f' to see fork() vs clone() syscalls");
    println!("3. Thread version is usually faster (no IPC overhead)");
    println!("4. Process version has better isolation (crash safety)");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_N: u64 = 10000;
    const EXPECTED: u64 = 50005000; // 10000 * 10001 / 2

    #[test]
    #[cfg(target_os = "linux")]
    fn test_process_version() {
        assert_eq!(sum_with_processes(TEST_N, 4), EXPECTED);
    }

    #[test]
    fn test_thread_version() {
        assert_eq!(sum_with_threads(TEST_N, 4), EXPECTED);
    }

    #[test]
    fn test_thread_version_single_worker() {
        assert_eq!(sum_with_threads(TEST_N, 1), EXPECTED);
    }

    #[test]
    fn test_thread_version_many_workers() {
        assert_eq!(sum_with_threads(TEST_N, 8), EXPECTED);
    }
}
