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

use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::time::{Duration, Instant};
use tokio::task::JoinHandle;

// ============================================================
// TODO: Implement connection pool demonstration
// ============================================================

/// Simulate a slow database query
async fn slow_query(pool: &SqlitePool, query_id: usize) -> Duration {
    // TODO: Implement
    // 1. Record start time
    // 2. Execute a query (SELECT 1, or use sqlite's sleep equivalent)
    // 3. Add artificial delay to simulate slow query
    // 4. Return elapsed time

    todo!("Implement slow_query")
}

/// Run multiple concurrent queries and collect results
async fn run_concurrent_queries(
    pool: &SqlitePool,
    num_queries: usize,
) -> Vec<Duration> {
    // TODO: Implement
    // 1. Spawn `num_queries` tasks, each running slow_query
    // 2. Wait for all to complete
    // 3. Collect and return durations

    todo!("Implement run_concurrent_queries")
}

/// Calculate and print statistics
fn print_stats(durations: &[Duration]) {
    // TODO: Implement
    // Calculate min, max, average, p95 latency

    todo!("Implement print_stats")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Implement
    // 1. Create pools with different sizes (e.g., 2, 5, 10)
    // 2. For each pool size, run concurrent queries
    // 3. Compare results

    todo!("Implement main")
}
