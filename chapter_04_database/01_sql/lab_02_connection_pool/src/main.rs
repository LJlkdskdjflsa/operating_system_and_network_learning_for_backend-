//! Lab 2: Connection Pool Behavior
//!
//! ## Goal
//! Understand connection pool behavior under concurrent load
//!
//! ## Requirements
//! 1. Create a pool with configurable size
//! 2. Simulate concurrent database queries
//! 3. Observe pool behavior (waiting, timeouts)
//! 4. Measure query latency with different pool sizes
//!
//! ## Expected Behavior
//! ```
//! $ cargo run
//! === Pool Size: 2, Concurrent Requests: 10 ===
//! Pool stats - Size: 2, Idle: 2
//! Starting 10 concurrent queries...
//! Query 1 completed in 102ms
//! Query 2 completed in 103ms
//! Query 3 completed in 205ms (waited for connection)
//! ...
//! Average latency: 250ms
//! ```
//!
//! ## Hints
//! - Use `tokio::spawn` for concurrent queries
//! - Use `tokio::time::Instant` for timing
//! - Pool connections are acquired implicitly on query
//! - Small pool + many requests = waiting
//!
//! ## Acceptance Criteria
//! - [ ] Pool respects max_connections limit
//! - [ ] Queries wait when pool is exhausted
//! - [ ] Can measure and report latencies
//! - [ ] Demonstrates pool sizing impact

use sqlx::postgres::{PgPool, PgPoolOptions};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Query execution time (simulated slow query)
const QUERY_DURATION_MS: u64 = 100;

/// Number of concurrent queries to run
const NUM_QUERIES: usize = 20;

const DIVIDER: &str =
    "============================================================";

/// Simulate a slow database query
async fn slow_query(pool: &PgPool, query_id: usize) -> (usize, Duration) {
    let start = Instant::now();

    let sleep_seconds = QUERY_DURATION_MS as f64 / 1000.0;
    sqlx::query("SELECT pg_sleep($1)")
        .bind(sleep_seconds)
        .execute(pool)
        .await
        .unwrap();

    (query_id, start.elapsed())
}

/// Run multiple concurrent queries and collect results
async fn run_concurrent_queries(
    pool: Arc<PgPool>,
    num_queries: usize,
) -> Vec<(usize, Duration)> {
    let mut handles = Vec::with_capacity(num_queries);

    for i in 0..num_queries {
        let pool = pool.clone();
        let handle = tokio::spawn(async move { slow_query(&pool, i).await });
        handles.push(handle);
    }

    let mut results = Vec::with_capacity(num_queries);
    for handle in handles {
        if let Ok(result) = handle.await {
            results.push(result);
        }
    }

    results.sort_by_key(|(id, _)| *id);
    results
}

/// Calculate and print statistics
fn print_stats(durations: &[(usize, Duration)]) {
    if durations.is_empty() {
        println!("No results");
        return;
    }

    let mut times: Vec<u128> = durations.iter().map(|(_, d)| d.as_millis()).collect();
    times.sort_unstable();

    let min = times.first().unwrap();
    let max = times.last().unwrap();
    let sum: u128 = times.iter().sum();
    let avg = sum / times.len() as u128;

    let p95_idx = (times.len() as f64 * 0.95).floor() as usize;
    let p95 = times.get(p95_idx.min(times.len() - 1)).unwrap();

    println!("\nStatistics:");
    println!("  Min latency:  {}ms", min);
    println!("  Max latency:  {}ms", max);
    println!("  Avg latency:  {}ms", avg);
    println!("  P95 latency:  {}ms", p95);
}

/// Test with a specific pool size
async fn test_pool_size(
    database_url: &str,
    pool_size: u32,
    num_queries: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{DIVIDER}");
    println!(
        "=== Pool Size: {}, Concurrent Queries: {} ===",
        pool_size, num_queries
    );
    println!("{DIVIDER}");

    let pool = PgPoolOptions::new()
        .max_connections(pool_size)
        .min_connections(pool_size)
        .acquire_timeout(Duration::from_secs(30))
        .connect(database_url)
        .await?;

    let pool = Arc::new(pool);

    sqlx::migrate!("./migrations").run(pool.as_ref()).await?;

    println!(
        "Pool stats - Max: {}, Current: {}, Idle: {}",
        pool_size,
        pool.size(),
        pool.num_idle()
    );
    println!(
        "Starting {} concurrent queries (each takes ~{}ms)...\n",
        num_queries, QUERY_DURATION_MS
    );

    let start = Instant::now();
    let results = run_concurrent_queries(pool.clone(), num_queries).await;
    let total_time = start.elapsed();

    for (id, duration) in &results {
        let waited = duration.as_millis() > QUERY_DURATION_MS as u128 + 50;
        if waited {
            println!(
                "Query {:2} completed in {:4}ms (waited for connection)",
                id,
                duration.as_millis()
            );
        } else {
            println!(
                "Query {:2} completed in {:4}ms",
                id,
                duration.as_millis()
            );
        }
    }

    print_stats(&results);

    let batches = (num_queries as f64 / pool_size as f64).ceil() as u64;
    let theoretical_min = QUERY_DURATION_MS;
    let theoretical_max = batches * QUERY_DURATION_MS;

    println!("\nTheoretical analysis:");
    println!("  Pool can run {} queries in parallel", pool_size);
    println!("  {} queries require ~{} batches", num_queries, batches);
    println!(
        "  Expected time range: {}ms - {}ms",
        theoretical_min, theoretical_max
    );
    println!("  Actual total time: {}ms", total_time.as_millis());

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    println!("Connection Pool Behavior Demo (Postgres)");
    println!("Each query takes ~{}ms", QUERY_DURATION_MS);

    for pool_size in [2_u32, 5, 10, 20] {
        test_pool_size(&database_url, pool_size, NUM_QUERIES).await?;
    }

    println!("\n{DIVIDER}");
    println!("Summary");
    println!("{DIVIDER}");
    println!(
        "\nKey observations:
1. With pool_size < num_queries, some queries must wait
2. Larger pool = lower latency (up to a point)
3. Pool size > num_queries provides no benefit
4. Optimal pool size depends on:
   - Database connection limits
   - Query patterns
   - Available resources
   - Acceptable latency
"
    );

    Ok(())
}
