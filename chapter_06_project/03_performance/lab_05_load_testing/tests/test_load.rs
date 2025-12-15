//! Lab 5: Load Testing Tests
//!
//! These tests verify the load tester works correctly.
//! Run with: cargo test

use std::time::Duration;

#[test]
fn test_percentile_calculation() {
    // Create sorted durations
    let latencies: Vec<Duration> = (1..=100)
        .map(|i| Duration::from_millis(i))
        .collect();

    // Test percentile function
    fn percentile(sorted: &[Duration], p: f64) -> Duration {
        if sorted.is_empty() {
            return Duration::ZERO;
        }
        let index = ((sorted.len() as f64) * p / 100.0) as usize;
        sorted[index.min(sorted.len() - 1)]
    }

    // p50 should be around 50ms
    let p50 = percentile(&latencies, 50.0);
    assert!(p50 >= Duration::from_millis(49) && p50 <= Duration::from_millis(51));

    // p99 should be around 99ms
    let p99 = percentile(&latencies, 99.0);
    assert!(p99 >= Duration::from_millis(98) && p99 <= Duration::from_millis(100));

    // p0 should be 1ms (first element)
    let p0 = percentile(&latencies, 0.0);
    assert_eq!(p0, Duration::from_millis(1));
}

#[test]
fn test_empty_percentile() {
    let latencies: Vec<Duration> = vec![];

    fn percentile(sorted: &[Duration], p: f64) -> Duration {
        if sorted.is_empty() {
            return Duration::ZERO;
        }
        let index = ((sorted.len() as f64) * p / 100.0) as usize;
        sorted[index.min(sorted.len() - 1)]
    }

    assert_eq!(percentile(&latencies, 50.0), Duration::ZERO);
}

#[test]
fn test_format_duration() {
    fn format_duration(d: Duration) -> String {
        let micros = d.as_micros();
        if micros < 1000 {
            format!("{}µs", micros)
        } else if micros < 1_000_000 {
            format!("{:.2}ms", micros as f64 / 1000.0)
        } else {
            format!("{:.2}s", d.as_secs_f64())
        }
    }

    assert_eq!(format_duration(Duration::from_micros(500)), "500µs");
    assert_eq!(format_duration(Duration::from_millis(5)), "5.00ms");
    assert_eq!(format_duration(Duration::from_millis(1500)), "1500.00ms");
    assert_eq!(format_duration(Duration::from_secs(2)), "2.00s");
}

#[tokio::test]
async fn test_stats_counting() {
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::Arc;
    use tokio::sync::Mutex;

    struct Stats {
        successful: AtomicU64,
        failed: AtomicU64,
        latencies: Mutex<Vec<Duration>>,
    }

    let stats = Arc::new(Stats {
        successful: AtomicU64::new(0),
        failed: AtomicU64::new(0),
        latencies: Mutex::new(Vec::new()),
    });

    // Simulate requests
    for i in 0..100 {
        if i % 10 == 0 {
            stats.failed.fetch_add(1, Ordering::Relaxed);
        } else {
            stats.successful.fetch_add(1, Ordering::Relaxed);
            stats.latencies.lock().await.push(Duration::from_millis(i));
        }
    }

    assert_eq!(stats.successful.load(Ordering::Relaxed), 90);
    assert_eq!(stats.failed.load(Ordering::Relaxed), 10);
    assert_eq!(stats.latencies.lock().await.len(), 90);
}

#[test]
fn test_throughput_calculation() {
    let successful = 1000u64;
    let duration = Duration::from_secs(10);

    let rps = successful as f64 / duration.as_secs_f64();
    assert_eq!(rps, 100.0);
}
