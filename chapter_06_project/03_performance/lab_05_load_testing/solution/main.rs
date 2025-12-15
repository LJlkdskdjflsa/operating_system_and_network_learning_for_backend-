//! Lab 5: Load Testing and Analysis - Solution
//!
//! A command-line HTTP load testing tool.

use clap::Parser;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

#[derive(Parser, Debug)]
#[command(name = "load_tester")]
#[command(about = "HTTP load testing tool")]
struct Args {
    /// Target URL to test
    #[arg(short, long)]
    url: String,

    /// Number of concurrent workers
    #[arg(short, long, default_value = "10")]
    concurrency: usize,

    /// Test duration in seconds
    #[arg(short, long, default_value = "10")]
    duration: u64,
}

struct Stats {
    successful: AtomicU64,
    failed: AtomicU64,
    latencies: Mutex<Vec<Duration>>,
}

impl Stats {
    fn new() -> Self {
        Self {
            successful: AtomicU64::new(0),
            failed: AtomicU64::new(0),
            latencies: Mutex::new(Vec::with_capacity(100_000)),
        }
    }

    async fn record_success(&self, latency: Duration) {
        self.successful.fetch_add(1, Ordering::Relaxed);
        self.latencies.lock().await.push(latency);
    }

    fn record_failure(&self) {
        self.failed.fetch_add(1, Ordering::Relaxed);
    }

    fn get_counts(&self) -> (u64, u64) {
        (
            self.successful.load(Ordering::Relaxed),
            self.failed.load(Ordering::Relaxed),
        )
    }
}

async fn worker(
    client: reqwest::Client,
    url: String,
    stats: Arc<Stats>,
    end_time: Instant,
) {
    while Instant::now() < end_time {
        let start = Instant::now();
        let result = client.get(&url).send().await;
        let latency = start.elapsed();

        match result {
            Ok(resp) if resp.status().is_success() => {
                stats.record_success(latency).await;
            }
            Ok(resp) => {
                // Non-success status code
                eprintln!("Request failed with status: {}", resp.status());
                stats.record_failure();
            }
            Err(e) => {
                eprintln!("Request error: {}", e);
                stats.record_failure();
            }
        }
    }
}

fn percentile(sorted: &[Duration], p: f64) -> Duration {
    if sorted.is_empty() {
        return Duration::ZERO;
    }
    let index = ((sorted.len() as f64) * p / 100.0) as usize;
    sorted[index.min(sorted.len() - 1)]
}

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

async fn display_results(stats: &Stats, total_duration: Duration) {
    let (successful, failed) = stats.get_counts();
    let total = successful + failed;

    println!("\n{}", "=".repeat(50));
    println!("LOAD TEST RESULTS");
    println!("{}", "=".repeat(50));

    // Request counts
    println!("\nRequests:");
    println!("  Total:      {}", total);
    println!("  Successful: {} ({:.1}%)", successful,
        if total > 0 { successful as f64 / total as f64 * 100.0 } else { 0.0 });
    println!("  Failed:     {} ({:.1}%)", failed,
        if total > 0 { failed as f64 / total as f64 * 100.0 } else { 0.0 });

    // Throughput
    let rps = successful as f64 / total_duration.as_secs_f64();
    println!("\nThroughput:");
    println!("  {:.2} requests/sec", rps);

    // Latency statistics
    let mut latencies = stats.latencies.lock().await;
    if !latencies.is_empty() {
        latencies.sort();

        let sum: Duration = latencies.iter().sum();
        let avg = sum / latencies.len() as u32;
        let min = *latencies.first().unwrap();
        let max = *latencies.last().unwrap();

        println!("\nLatency:");
        println!("  Min:  {}", format_duration(min));
        println!("  Max:  {}", format_duration(max));
        println!("  Avg:  {}", format_duration(avg));
        println!();
        println!("  p50:  {}", format_duration(percentile(&latencies, 50.0)));
        println!("  p75:  {}", format_duration(percentile(&latencies, 75.0)));
        println!("  p90:  {}", format_duration(percentile(&latencies, 90.0)));
        println!("  p95:  {}", format_duration(percentile(&latencies, 95.0)));
        println!("  p99:  {}", format_duration(percentile(&latencies, 99.0)));

        // Latency distribution histogram
        println!("\nLatency Distribution:");
        let buckets = [
            Duration::from_micros(500),
            Duration::from_millis(1),
            Duration::from_millis(5),
            Duration::from_millis(10),
            Duration::from_millis(50),
            Duration::from_millis(100),
            Duration::from_millis(500),
            Duration::from_secs(1),
        ];

        let mut prev = Duration::ZERO;
        for bucket in buckets {
            let count = latencies.iter()
                .filter(|&&l| l > prev && l <= bucket)
                .count();
            let pct = count as f64 / latencies.len() as f64 * 100.0;
            let bar_len = (pct / 2.0) as usize;
            println!("  {:>8} | {:>5.1}% | {}",
                format_duration(bucket),
                pct,
                "#".repeat(bar_len));
            prev = bucket;
        }

        // Anything above 1s
        let count = latencies.iter().filter(|&&l| l > Duration::from_secs(1)).count();
        if count > 0 {
            let pct = count as f64 / latencies.len() as f64 * 100.0;
            let bar_len = (pct / 2.0) as usize;
            println!("  {:>8} | {:>5.1}% | {}", ">1s", pct, "#".repeat(bar_len));
        }
    }

    println!("\n{}", "=".repeat(50));
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    println!("{}", "=".repeat(50));
    println!("LOAD TEST CONFIGURATION");
    println!("{}", "=".repeat(50));
    println!("URL:         {}", args.url);
    println!("Concurrency: {} workers", args.concurrency);
    println!("Duration:    {} seconds", args.duration);
    println!("{}", "=".repeat(50));
    println!("\nRunning load test...\n");

    // Create HTTP client with connection pooling
    let client = reqwest::Client::builder()
        .pool_max_idle_per_host(args.concurrency)
        .timeout(Duration::from_secs(30))
        .build()
        .expect("Failed to create HTTP client");

    let stats = Arc::new(Stats::new());
    let duration = Duration::from_secs(args.duration);
    let end_time = Instant::now() + duration;

    // Progress indicator
    let stats_clone = stats.clone();
    let progress_handle = tokio::spawn(async move {
        let start = Instant::now();
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            if Instant::now() >= end_time {
                break;
            }
            let (success, failed) = stats_clone.get_counts();
            let elapsed = start.elapsed().as_secs();
            print!("\r[{:>3}s] Requests: {} successful, {} failed",
                elapsed, success, failed);
            use std::io::Write;
            std::io::stdout().flush().ok();
        }
        println!();
    });

    // Spawn workers
    let mut handles = Vec::with_capacity(args.concurrency);
    let test_start = Instant::now();

    for _ in 0..args.concurrency {
        let client = client.clone();
        let url = args.url.clone();
        let stats = stats.clone();

        let handle = tokio::spawn(async move {
            worker(client, url, stats, end_time).await;
        });
        handles.push(handle);
    }

    // Wait for all workers
    for handle in handles {
        let _ = handle.await;
    }

    // Wait for progress indicator to finish
    let _ = progress_handle.await;

    let total_duration = test_start.elapsed();

    // Display results
    display_results(&stats, total_duration).await;
}
