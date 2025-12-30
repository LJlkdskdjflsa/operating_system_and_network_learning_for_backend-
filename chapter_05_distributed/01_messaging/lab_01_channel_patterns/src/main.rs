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

use std::time::Duration;
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc, Mutex};

// ============================================================
// TODO: Implement channel patterns
// ============================================================

/// Fan-in: Multiple producers send to single consumer
async fn demo_fan_in() {
    // TODO: Implement
    // 1. Create mpsc channel
    let (tx, mut rx) = mpsc::channel(8);
    let mut producers = Vec::new();
    // 2. Spawn multiple producer tasks
    for i in 0..100 {
        let tx = tx.clone();
        producers.push(tokio::spawn(async move {
            println!("Producer {} sending...", i);
            let _ = tx.send(format!("from producer {}", i)).await;
        }));
    }
    drop(tx);
    // 3. Single consumer loop
    let consumer = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            println!("Consumer received: {}", message);
        }
    });
    // 4. Wait for all producers to finish
    for producer in producers {
        let _ = producer.await;
    }
    let _ = consumer.await;
}

/// Fan-out: Single producer sends to multiple consumers
async fn demo_fan_out() {
    // TODO: Implement
    // 1. Create broadcast channel
    let (tx, _) = broadcast::channel(8);
    let mut subscribers = Vec::new();
    // 2. Spawn multiple subscriber tasks
    for i in 0..10 {
        let mut rx = tx.subscribe();
        subscribers.push(tokio::spawn(async move {
            match rx.recv().await {
                Ok(message) => println!("Subscriber {} received: {}", i, message),
                Err(err) => eprintln!("Subscriber {} error: {}", i, err),
            }
        }));
    }

    // 3. Send messages from main task
    println!("Broadcasting event...");
    let _ = tx.send("event");
    // 4. Wait for subscribers to process
    for subscriber in subscribers {
        let _ = subscriber.await;
    }
}

/// Worker pool: Distribute jobs across workers
async fn demo_worker_pool() {
    let worker_count = 3;
    let job_count = 10;
    let (tx, rx) = mpsc::channel(8);
    let rx = Arc::new(Mutex::new(rx));
    let mut workers = Vec::new();

    println!("Submitting {} jobs to {} workers...", job_count, worker_count);

    for worker_id in 0..worker_count {
        let rx = Arc::clone(&rx);
        workers.push(tokio::spawn(async move {
            loop {
                let job = {
                    let mut guard = rx.lock().await;
                    guard.recv().await
                };
                match job {
                    Some(job_id) => {
                        println!("Worker {} processing job {}", worker_id, job_id);
                        tokio::time::sleep(Duration::from_millis(50)).await;
                    }
                    None => break,
                }
            }
        }));
    }

    for job_id in 0..job_count {
        let _ = tx.send(job_id).await;
    }
    drop(tx);

    for worker in workers {
        let _ = worker.await;
    }
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
