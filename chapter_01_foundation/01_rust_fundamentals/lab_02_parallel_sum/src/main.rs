//! 平行計算實驗：比較不同多執行緒實作方式
//!
//! 目標：計算 1 + 2 + 3 + ... + N

use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Instant;

// ============================================================
// 單執行緒版本（基準）
// ============================================================

fn sum_sequential(n: u64) -> u64 {
    (1..=n).sum()
}

// ============================================================
// Arc + Mutex 版本
// ============================================================

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
            // 先在本地計算，減少鎖競爭
            let partial_sum: u64 = (start..=end).sum();

            // 只在最後加到共享變數
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

// ============================================================
// Channel 版本
// ============================================================

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

    // 丟掉原始的 tx，否則 rx.iter() 會 deadlock
    drop(tx);

    // 收集所有結果
    rx.iter().sum()
}

// ============================================================
// 效能測試
// ============================================================

fn benchmark<F>(name: &str, f: F)
where
    F: FnOnce() -> u64,
{
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();
    println!("{:30} | Result: {:20} | Time: {:?}", name, result, duration);
}

fn main() {
    let n: u64 = 100_000_000;

    // 數學驗證
    let expected = n * (n + 1) / 2;
    println!("N = {}", n);
    println!("Expected result: {}", expected);
    println!("{}", "=".repeat(80));
    println!(
        "{:30} | {:27} | {}",
        "Method", "Result", "Time"
    );
    println!("{}", "-".repeat(80));

    // 單執行緒基準
    benchmark("Sequential", || sum_sequential(n));

    println!("{}", "-".repeat(80));

    // Mutex 版本，不同執行緒數量
    for &threads in &[1, 2, 4, 8] {
        let name = format!("Mutex ({} threads)", threads);
        benchmark(&name, || sum_with_mutex(n, threads));
    }

    println!("{}", "-".repeat(80));

    // Channel 版本，不同執行緒數量
    for &threads in &[1, 2, 4, 8] {
        let name = format!("Channel ({} threads)", threads);
        benchmark(&name, || sum_with_channel(n, threads));
    }

    println!("{}", "=".repeat(80));
}

// ============================================================
// 測試
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_N: u64 = 1000;
    const EXPECTED: u64 = 500_500; // 1000 * 1001 / 2

    #[test]
    fn test_sequential() {
        assert_eq!(sum_sequential(TEST_N), EXPECTED);
    }

    #[test]
    fn test_mutex_single_thread() {
        assert_eq!(sum_with_mutex(TEST_N, 1), EXPECTED);
    }

    #[test]
    fn test_mutex_multi_thread() {
        assert_eq!(sum_with_mutex(TEST_N, 4), EXPECTED);
    }

    #[test]
    fn test_channel_single_thread() {
        assert_eq!(sum_with_channel(TEST_N, 1), EXPECTED);
    }

    #[test]
    fn test_channel_multi_thread() {
        assert_eq!(sum_with_channel(TEST_N, 4), EXPECTED);
    }
}
