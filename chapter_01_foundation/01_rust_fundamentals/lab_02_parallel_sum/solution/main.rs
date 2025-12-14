//! Lab 2 Reference Answer

use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Instant;

/// Single-threaded version
fn sum_sequential(n: u64) -> u64 {
    (1..=n).sum()
}

/// Arc + Mutex version
fn sum_with_mutex(n: u64, num_threads: usize) -> u64 {
    let sum = Arc::new(Mutex::new(0u64));
    let chunk_size = n / num_threads as u64;
    let mut handles = vec![];

    for i in 0..num_threads {
        let sum = Arc::clone(&sum);
        let start = i as u64 * chunk_size + 1;
        let end = if i == num_threads - 1 {
            n
        } else {
            (i + 1) as u64 * chunk_size
        };

        let handle = thread::spawn(move || {
            // Compute locally first to reduce lock contention
            let partial_sum: u64 = (start..=end).sum();

            // Only add to shared variable at the end
            let mut total = sum.lock().unwrap();
            *total += partial_sum;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    *sum.lock().unwrap()
}

/// Channel version
fn sum_with_channel(n: u64, num_threads: usize) -> u64 {
    let (tx, rx) = mpsc::channel();
    let chunk_size = n / num_threads as u64;

    for i in 0..num_threads {
        let tx = tx.clone();
        let start = i as u64 * chunk_size + 1;
        let end = if i == num_threads - 1 {
            n
        } else {
            (i + 1) as u64 * chunk_size
        };

        thread::spawn(move || {
            let partial_sum: u64 = (start..=end).sum();
            tx.send(partial_sum).unwrap();
        });
    }

    // Drop the original tx, otherwise rx.iter() will deadlock
    drop(tx);

    // Collect all results
    rx.iter().sum()
}

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

    benchmark("Sequential", || sum_sequential(n));

    println!("{}", "-".repeat(70));

    for &threads in &[1, 2, 4, 8] {
        let name = format!("Mutex ({} threads)", threads);
        benchmark(&name, || sum_with_mutex(n, threads));
    }

    println!("{}", "-".repeat(70));

    for &threads in &[1, 2, 4, 8] {
        let name = format!("Channel ({} threads)", threads);
        benchmark(&name, || sum_with_channel(n, threads));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_N: u64 = 1000;
    const EXPECTED: u64 = 500_500;

    #[test]
    fn test_sequential() {
        assert_eq!(sum_sequential(TEST_N), EXPECTED);
    }

    #[test]
    fn test_mutex() {
        assert_eq!(sum_with_mutex(TEST_N, 4), EXPECTED);
    }

    #[test]
    fn test_channel() {
        assert_eq!(sum_with_channel(TEST_N, 4), EXPECTED);
    }
}
