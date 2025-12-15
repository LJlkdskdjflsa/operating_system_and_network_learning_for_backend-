//! Lab 2 Reference Answer

use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Query execution time (simulated slow query)
const QUERY_DURATION_MS: u64 = 100;

/// Number of concurrent queries to run
const NUM_QUERIES: usize = 20;

/// Simulate a slow database query
async fn slow_query(pool: &SqlitePool, query_id: usize) -> (usize, Duration) {
    let start = Instant::now();

    // Execute a simple query
    let _: (i32,) = sqlx::query_as("SELECT 1")
        .fetch_one(pool)
        .await
        .unwrap();

    // Simulate slow query processing
    tokio::time::sleep(Duration::from_millis(QUERY_DURATION_MS)).await;

    let elapsed = start.elapsed();
    (query_id, elapsed)
}

/// Run multiple concurrent queries and collect results
async fn run_concurrent_queries(
    pool: Arc<SqlitePool>,
    num_queries: usize,
) -> Vec<(usize, Duration)> {
    let mut handles = Vec::new();

    for i in 0..num_queries {
        let pool = pool.clone();
        let handle = tokio::spawn(async move {
            slow_query(&pool, i).await
        });
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        if let Ok(result) = handle.await {
            results.push(result);
        }
    }

    // Sort by query id for consistent output
    results.sort_by_key(|(id, _)| *id);
    results
}

/// Calculate and print statistics
fn print_stats(durations: &[(usize, Duration)]) {
    if durations.is_empty() {
        println!("No results");
        return;
    }

    let times: Vec<u128> = durations.iter().map(|(_, d)| d.as_millis()).collect();

    let min = times.iter().min().unwrap();
    let max = times.iter().max().unwrap();
    let sum: u128 = times.iter().sum();
    let avg = sum / times.len() as u128;

    // Calculate p95
    let mut sorted = times.clone();
    sorted.sort();
    let p95_idx = (sorted.len() as f64 * 0.95) as usize;
    let p95 = sorted.get(p95_idx.min(sorted.len() - 1)).unwrap();

    println!("\nStatistics:");
    println!("  Min latency:  {}ms", min);
    println!("  Max latency:  {}ms", max);
    println!("  Avg latency:  {}ms", avg);
    println!("  P95 latency:  {}ms", p95);

    // Expected time calculation
    // With pool_size connections and num_queries queries taking query_time each:
    // Total time ≈ ceil(num_queries / pool_size) * query_time
}

/// Test with a specific pool size
async fn test_pool_size(
    pool_size: u32,
    num_queries: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n{'='*60}");
    println!("=== Pool Size: {}, Concurrent Queries: {} ===", pool_size, num_queries);
    println!("{'='*60}");

    // Create pool with specified size
    let pool = SqlitePoolOptions::new()
        .max_connections(pool_size)
        .min_connections(pool_size) // Pre-create all connections
        .acquire_timeout(Duration::from_secs(30))
        .connect("sqlite::memory:")
        .await?;

    let pool = Arc::new(pool);

    // Show pool stats
    println!("Pool stats - Max: {}, Current: {}", pool_size, pool.size());

    // Initialize schema
    sqlx::query("CREATE TABLE IF NOT EXISTS test (id INTEGER)")
        .execute(pool.as_ref())
        .await?;

    println!("Starting {} concurrent queries (each takes ~{}ms)...\n",
             num_queries, QUERY_DURATION_MS);

    let start = Instant::now();

    // Run concurrent queries
    let results = run_concurrent_queries(pool.clone(), num_queries).await;

    let total_time = start.elapsed();

    // Print individual results
    for (id, duration) in &results {
        let waited = duration.as_millis() > QUERY_DURATION_MS as u128 + 50;
        if waited {
            println!("Query {:2} completed in {:4}ms (waited for connection)",
                     id, duration.as_millis());
        } else {
            println!("Query {:2} completed in {:4}ms",
                     id, duration.as_millis());
        }
    }

    // Print statistics
    print_stats(&results);

    // Calculate theoretical times
    let batches = (num_queries as f64 / pool_size as f64).ceil() as u64;
    let theoretical_min = QUERY_DURATION_MS;
    let theoretical_max = batches * QUERY_DURATION_MS;

    println!("\nTheoretical analysis:");
    println!("  Pool can run {} queries in parallel", pool_size);
    println!("  {} queries require ~{} batches", num_queries, batches);
    println!("  Expected time range: {}ms - {}ms", theoretical_min, theoretical_max);
    println!("  Actual total time: {}ms", total_time.as_millis());

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Connection Pool Behavior Demo");
    println!("Each query takes ~{}ms", QUERY_DURATION_MS);

    // Test with different pool sizes
    test_pool_size(2, NUM_QUERIES).await?;
    test_pool_size(5, NUM_QUERIES).await?;
    test_pool_size(10, NUM_QUERIES).await?;
    test_pool_size(20, NUM_QUERIES).await?;

    println!("\n{'='*60}");
    println!("Summary");
    println!("{'='*60}");
    println!("
Key observations:
1. With pool_size < num_queries, some queries must wait
2. Larger pool = lower latency (up to a point)
3. Pool size > num_queries provides no benefit
4. Optimal pool size depends on:
   - Database connection limits
   - Query patterns
   - Available resources
   - Acceptable latency

Rule of thumb for pool sizing:
  connections = (core_count * 2) + spindle_count
  For SSDs: connections ≈ cores * 2-4
");

    Ok(())
}

// Key concepts demonstrated:
//
// 1. POOL CONTENTION:
//    - When pool is full, new queries wait
//    - Wait time adds to query latency
//
// 2. THROUGHPUT vs LATENCY:
//    - Small pool = higher latency per query
//    - Large pool = more resource usage
//
// 3. BATCHING EFFECT:
//    - Queries run in batches equal to pool size
//    - ceil(queries / pool_size) batches needed
//
// 4. POOL SIZING:
//    - Too small = high latency
//    - Too large = wasted resources, database overload
//    - Sweet spot depends on workload

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_slow_query() {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();

        let (id, duration) = slow_query(&pool, 0).await;
        assert_eq!(id, 0);
        assert!(duration.as_millis() >= QUERY_DURATION_MS as u128);
    }

    #[tokio::test]
    async fn test_pool_limits_concurrency() {
        let pool = SqlitePoolOptions::new()
            .max_connections(2)
            .connect("sqlite::memory:")
            .await
            .unwrap();

        let pool = Arc::new(pool);
        let start = Instant::now();

        // Run 4 queries with pool size 2
        // Should take ~2 batches = 2 * QUERY_DURATION_MS
        let results = run_concurrent_queries(pool, 4).await;

        let elapsed = start.elapsed();

        assert_eq!(results.len(), 4);
        // Should take at least 2x query duration (2 batches)
        assert!(elapsed.as_millis() >= (QUERY_DURATION_MS * 2) as u128 - 50);
    }
}
