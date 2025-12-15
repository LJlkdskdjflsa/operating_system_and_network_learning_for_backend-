//! Lab 1: Channel Patterns
//!
//! ## Goal
//! Learn common channel patterns: fan-in, fan-out, and worker pool
//!
//! ## Requirements
//! 1. Implement fan-in (multiple producers, single consumer)
//! 2. Implement fan-out (single producer, multiple consumers)
//! 3. Implement worker pool (distribute work across workers)
//! 4. Handle graceful shutdown
//!
//! ## Expected Behavior
//! ```
//! $ cargo run
//! === Fan-In Pattern ===
//! Producer 0 sending...
//! Producer 1 sending...
//! Consumer received: from producer 0
//! Consumer received: from producer 1
//!
//! === Fan-Out Pattern ===
//! Broadcasting event...
//! Subscriber 0 received: event
//! Subscriber 1 received: event
//!
//! === Worker Pool ===
//! Submitting 10 jobs to 3 workers...
//! Worker 0 processing job 0
//! Worker 1 processing job 1
//! Worker 2 processing job 2
//! ...
//! ```
//!
//! ## Hints
//! - Use `mpsc` for fan-in
//! - Use `broadcast` for fan-out
//! - Use `async_channel` or shared `mpsc` receiver for worker pool
//! - Clone sender for multiple producers
//!
//! ## Acceptance Criteria
//! - [ ] Fan-in collects from multiple producers
//! - [ ] Fan-out delivers to all subscribers
//! - [ ] Worker pool distributes work evenly
//! - [ ] Clean shutdown (no hanging)

use tokio::sync::{mpsc, broadcast};
use std::time::Duration;

// ============================================================
// TODO: Implement channel patterns
// ============================================================

/// Fan-in: Multiple producers send to single consumer
async fn demo_fan_in() {
    // TODO: Implement
    // 1. Create mpsc channel
    // 2. Spawn multiple producer tasks
    // 3. Single consumer loop
    // 4. Wait for all producers to finish

    todo!("Implement demo_fan_in")
}

/// Fan-out: Single producer sends to multiple consumers
async fn demo_fan_out() {
    // TODO: Implement
    // 1. Create broadcast channel
    // 2. Spawn multiple subscriber tasks
    // 3. Send messages from main task
    // 4. Wait for subscribers to process

    todo!("Implement demo_fan_out")
}

/// Worker pool: Distribute jobs across workers
async fn demo_worker_pool() {
    // TODO: Implement
    // 1. Create job channel
    // 2. Spawn worker tasks (each receives from same channel)
    // 3. Submit jobs
    // 4. Wait for completion

    todo!("Implement demo_worker_pool")
}

#[tokio::main]
async fn main() {
    println!("Channel Patterns Demo\n");

    println!("=== Fan-In Pattern ===");
    demo_fan_in().await;

    println!("\n=== Fan-Out Pattern ===");
    demo_fan_out().await;

    println!("\n=== Worker Pool Pattern ===");
    demo_worker_pool().await;

    println!("\nDone!");
}
