//! Lab 2: Parallel Computation
//!
//! ## Goal
//! Calculate 1 + 2 + 3 + ... + N using multiple threads, and compare different implementations
//!
//! ## Requirements
//! Implement three versions:
//! 1. `sum_sequential(n)` - Single-threaded version (baseline)
//! 2. `sum_with_mutex(n, num_threads)` - Use Arc<Mutex<T>> to share result
//! 3. `sum_with_channel(n, num_threads)` - Use mpsc::channel to pass results
//!
//! ## Expected Output
//! ```
//! N = 100,000,000
//! Expected: 5000000050000000
//!
//! Sequential:          xxx ms
//! Mutex (4 threads):   xxx ms
//! Channel (4 threads): xxx ms
//! ```
//!
//! ## Hints
//! - Split the range into num_threads chunks, let each thread compute one chunk
//! - Mutex version: After each thread calculates partial_sum, add it to shared sum
//! - Channel version: Each thread sends partial_sum to channel, main thread collects and sums
//! - Remember to use `Arc::clone()` to clone the Arc
//! - Channel version: Remember to `drop(tx)` the original sender
//!
//! ## Verification
//! ```bash
//! cargo test              # Run automated tests
//! cargo run --release     # Run performance comparison
//! ```
//!
//! ## Acceptance Criteria
//! - [ ] `cargo test` all pass (or `cargo run` results correct)
//! - [ ] All three versions compute correct results
//! - [ ] Performance comparison done
//! - [ ] Can explain the purpose of Arc, Mutex, and Channel
//!
//! Check solution/main.rs after completing
use std::hint::black_box;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::time::Instant;

mod thread_pool;
use thread_pool::ThreadPool;

use rayon::prelude::*;
// ============================================================
// TODO: Implement these three functions
// ============================================================

/// Single-threaded version
fn sum_sequential(n: u64) -> u64 {
    sum_range(1, n, observe_mode())
}

fn sum_range(start: u64, end: u64, slow: bool) -> u64 {
    if start > end {
        return 0;
    }

    if slow {
        let mut acc = 0_u64;
        for i in start..=end {
            acc = acc.wrapping_add(black_box(i));
        }
        acc
    } else {
        (start..=end).sum()
    }
}

fn read_env_u64(name: &str) -> Option<u64> {
    std::env::var(name).ok()?.trim().parse::<u64>().ok()
}

fn read_env_usize(name: &str) -> Option<usize> {
    std::env::var(name).ok()?.trim().parse::<usize>().ok()
}

fn observe_mode() -> bool {
    matches!(
        std::env::var("OBSERVE").as_deref(),
        Ok("1") | Ok("true") | Ok("TRUE") | Ok("yes") | Ok("YES")
    )
}

fn maybe_sleep() {
    let sleep_ms = read_env_u64("SLEEP_MS").unwrap_or(0);
    if sleep_ms > 0 {
        thread::sleep(Duration::from_millis(sleep_ms));
    }
}

/// Arc + Mutex version
fn sum_with_mutex(n: u64, num_threads: usize) -> u64 {
    // Handle edge cases
    if n == 0 || num_threads == 0 {
        return 0;
    }

    let result = Arc::new(Mutex::new(0_u64));

    // Compute chunk size (ceiling division)
    let chunk_size = (n + num_threads as u64 - 1) / num_threads as u64;

    let mut handles = Vec::with_capacity(num_threads);

    for thread_id in 0..num_threads {
        let start = thread_id as u64 * chunk_size + 1;
        let mut end = (thread_id as u64 + 1) * chunk_size;
        if end > n {
            end = n;
        }

        // If this thread's range is empty, skip creating the thread, happen when thread count is big while n is small, like num_thread=10, n=3
        if start > end {
            continue;
        }

        let result_clone = Arc::clone(&result);

        let handle = thread::spawn(move || {
            maybe_sleep();
            // 1) Sum this chunk locally
            let local_sum = sum_range(start, end, observe_mode());

            // 2) Add to shared result under mutex
            let mut guard = result_clone.lock().unwrap();
            *guard += local_sum;
        });

        handles.push(handle);
    }

    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }

    // Extract final result
    let final_result = *result.lock().unwrap();
    final_result
}
/// Channel version
fn sum_with_channel(n: u64, num_threads: usize) -> u64 {
    if n == 0 || num_threads == 0 {
        return 0;
    }

    let (tx, rx) = mpsc::channel();

    let chunk_size = (n + num_threads as u64 - 1) / num_threads as u64;

    for thread_id in 0..num_threads {
        let start = thread_id as u64 * chunk_size + 1;
        let mut end = (thread_id as u64 + 1) * chunk_size;
        if end > n {
            end = n;
        }

        if start > end {
            continue;
        }

        let tx_clone = tx.clone();
        thread::spawn(move || {
            maybe_sleep();
            let partial_sum = sum_range(start, end, observe_mode());
            tx_clone.send(partial_sum).unwrap();
        });
    }

    // Close the original sender so the receiver iterator terminates when workers finish
    drop(tx);

    rx.iter().sum()
}

/// ThreadPool version (fixed-size worker threads)
fn sum_with_thread_pool(n: u64, num_threads: usize) -> u64 {
    if n == 0 || num_threads == 0 {
        return 0;
    }

    let pool = ThreadPool::new(num_threads);
    let (result_tx, result_rx) = mpsc::channel::<u64>();

    let chunk_size = (n + num_threads as u64 - 1) / num_threads as u64;

    for thread_id in 0..num_threads {
        let start = thread_id as u64 * chunk_size + 1;
        let mut end = (thread_id as u64 + 1) * chunk_size;
        if end > n {
            end = n;
        }

        if start > end {
            continue;
        }

        let tx_clone = result_tx.clone();
        pool.execute(move || {
            maybe_sleep();
            let partial_sum = sum_range(start, end, observe_mode());
            tx_clone.send(partial_sum).unwrap();
        });
    }

    drop(result_tx);
    result_rx.iter().sum()
}

/// Rayon version (data-parallel iterator)
fn sum_with_rayon(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    if observe_mode() {
        use rayon::iter::ParallelBridge;
        (1..=n).par_bridge().map(|i| black_box(i)).sum()
    } else {
        (1..=n).into_par_iter().sum()
    }
}

// ============================================================
// Performance testing (no modification needed)
// ============================================================

fn benchmark<F>(name: &str, f: F)
where
    F: FnOnce() -> u64,
{
    let start = Instant::now();
    let result = black_box(f());
    let duration = start.elapsed();
    println!("{:25} | Result: {:20} | Time: {:?}", name, result, duration);
}

fn main() {
    let observe = observe_mode();
    if observe && std::env::var("SLEEP_MS").is_err() {
        std::env::set_var("SLEEP_MS", "200");
    }

    let n_default: u64 = if observe { 500_000_000 } else { 100_000_000 };
    let n: u64 = black_box(read_env_u64("N").unwrap_or(n_default));
    let expected = n * (n + 1) / 2;

    println!("N = {}", n);
    println!("Expected: {}", expected);
    if observe {
        let sleep_ms = read_env_u64("SLEEP_MS").unwrap_or(0);
        let rayon_threads = read_env_usize("RAYON_NUM_THREADS");
        println!(
            "OBSERVE=1 (slow_sum=true, SLEEP_MS={}, RAYON_NUM_THREADS={})",
            sleep_ms,
            rayon_threads
                .map(|n| n.to_string())
                .unwrap_or_else(|| "default".to_string())
        );
    }
    println!("{}", "=".repeat(70));

    // Single-threaded
    benchmark("Sequential", || sum_sequential(n));

    println!("{}", "-".repeat(70));

    // Mutex version
    for &threads in &[1, 2, 4, 8] {
        let name = format!("Mutex ({} threads)", threads);
        benchmark(&name, || sum_with_mutex(n, threads));
    }

    println!("{}", "-".repeat(70));

    // Channel version
    for &threads in &[1, 2, 4, 8] {
        let name = format!("Channel ({} threads)", threads);
        benchmark(&name, || sum_with_channel(n, threads));
    }

    println!("{}", "-".repeat(70));

    // ThreadPool version
    for &threads in &[1, 2, 4, 8] {
        let name = format!("ThreadPool ({} threads)", threads);
        benchmark(&name, || sum_with_thread_pool(n, threads));
    }

    println!("{}", "-".repeat(70));

    // Rayon version
    benchmark("Rayon", || sum_with_rayon(n));
}
