//! Lab 5: Load Testing and Analysis
//!
//! ## Goal
//! Build a load testing tool and use it to analyze service performance.
//!
//! ## Requirements
//! 1. Accept CLI arguments: URL, concurrency, duration/requests
//! 2. Generate concurrent HTTP requests
//! 3. Collect latency measurements for each request
//! 4. Calculate and display statistics:
//!    - Total requests
//!    - Successful/failed count
//!    - Requests per second (throughput)
//!    - Latency percentiles (p50, p95, p99)
//!    - Min/max/average latency
//!
//! ## Usage
//! ```bash
//! # Run the target server first (in another terminal)
//! cargo run --bin target_server
//!
//! # Run load test
//! cargo run --bin load_tester -- \
//!   --url http://localhost:3000/items \
//!   --concurrency 50 \
//!   --duration 10
//! ```
//!
//! ## Hints
//! - Use `tokio::spawn` to create concurrent workers
//! - Use `Instant::now()` and `elapsed()` for timing
//! - Use `AtomicU64` for thread-safe counters
//! - Sort latencies to calculate percentiles
//!
//! ## Acceptance Criteria
//! - [ ] CLI accepts url, concurrency, duration arguments
//! - [ ] Generates concurrent requests
//! - [ ] Measures individual request latencies
//! - [ ] Displays throughput (req/sec)
//! - [ ] Displays latency percentiles
//! - [ ] Handles errors gracefully
//!
//! Check solution/main.rs after completing

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
            latencies: Mutex::new(Vec::new()),
        }
    }

    fn record_success(&self, latency: Duration) {
        self.successful.fetch_add(1, Ordering::Relaxed);
        // TODO: Record latency (need async context for mutex)
    }

    fn record_failure(&self) {
        self.failed.fetch_add(1, Ordering::Relaxed);
    }
}

// TODO: Implement worker function
//
// Each worker should:
// 1. Loop until duration expires
// 2. Make HTTP GET request to URL
// 3. Record latency and success/failure
async fn worker(
    client: reqwest::Client,
    url: String,
    stats: Arc<Stats>,
    end_time: Instant,
) {
    // TODO: Implement
    //
    // while Instant::now() < end_time {
    //     let start = Instant::now();
    //     let result = client.get(&url).send().await;
    //     let latency = start.elapsed();
    //
    //     match result {
    //         Ok(resp) if resp.status().is_success() => {
    //             stats.record_success(latency);
    //         }
    //         _ => {
    //             stats.record_failure();
    //         }
    //     }
    // }

    todo!()
}

// TODO: Implement percentile calculation
fn percentile(sorted: &[Duration], p: f64) -> Duration {
    // TODO: Calculate the value at percentile p
    // index = (len * p / 100) as usize
    todo!()
}

// TODO: Implement results display
fn display_results(stats: &Stats, total_duration: Duration) {
    // TODO: Calculate and print:
    // - Total requests
    // - Successful / Failed
    // - Requests per second
    // - Latency: min, max, avg, p50, p95, p99
    todo!()
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    println!("Load Testing: {}", args.url);
    println!("Concurrency: {}", args.concurrency);
    println!("Duration: {}s", args.duration);
    println!();

    // TODO: Set up and run load test
    //
    // 1. Create reqwest client
    // 2. Create shared Stats
    // 3. Calculate end_time
    // 4. Spawn worker tasks
    // 5. Wait for all workers
    // 6. Display results

    todo!()
}
