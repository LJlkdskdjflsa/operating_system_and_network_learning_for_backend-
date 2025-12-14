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

use std::time::Instant;

// ============================================================
// TODO: Implement these three functions
// ============================================================

/// Single-threaded version
fn sum_sequential(n: u64) -> u64 {
    // TODO: Use (1..=n).sum() or a loop to implement
    todo!()
}

/// Arc + Mutex version
fn sum_with_mutex(n: u64, num_threads: usize) -> u64 {
    // TODO:
    // 1. Create Arc<Mutex<u64>> to store the result
    // 2. Split the range into num_threads chunks
    // 3. Each thread computes its chunk, then adds to shared variable
    // 4. Wait for all threads to complete, return result
    todo!()
}

/// Channel version
fn sum_with_channel(n: u64, num_threads: usize) -> u64 {
    // TODO:
    // 1. Create a channel
    // 2. Split the range into num_threads chunks
    // 3. Each thread computes its chunk, sends result to channel
    // 4. Main thread collects all results and sums them
    todo!()
}

// ============================================================
// Performance testing (no modification needed)
// ============================================================

fn benchmark<F>(name: &str, f: F)
where
    F: FnOnce() -> u64,
{
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();
    println!("{:25} | Result: {:20} | Time: {:?}", name, result, duration);
}

fn main() {
    let n: u64 = 100_000_000;
    let expected = n * (n + 1) / 2;

    println!("N = {}", n);
    println!("Expected: {}", expected);
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
}
