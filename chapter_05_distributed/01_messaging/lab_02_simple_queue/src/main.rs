//! Lab 2: Simple Message Queue
//!
//! ## Goal
//! Build an in-memory message queue with acknowledgment
//!
//! ## Requirements
//! 1. Enqueue messages
//! 2. Dequeue messages (with visibility timeout)
//! 3. Acknowledge processed messages
//! 4. Redeliver unacknowledged messages
//!
//! ## Expected Behavior
//! ```
//! $ cargo run
//! === Simple Queue Demo ===
//!
//! Enqueuing 5 messages...
//! Enqueued: msg-1
//! ...
//!
//! Worker processing...
//! Dequeued: msg-1
//! Processed and acknowledged: msg-1
//!
//! Simulating failure (no ack)...
//! Dequeued: msg-3
//! (Worker crashed, no ack)
//!
//! After timeout, message redelivered:
//! Dequeued: msg-3 (attempt 2)
//! ```
//!
//! ## Hints
//! - Use VecDeque for pending messages
//! - Use HashMap for in-flight (processing) messages
//! - Track message attempts
//! - Use tokio::time::Instant for timeout tracking
//!
//! ## Acceptance Criteria
//! - [ ] Messages can be enqueued
//! - [ ] Dequeue returns one message at a time
//! - [ ] Acknowledged messages are removed
//! - [ ] Unacked messages are redelivered after timeout

use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};
use uuid::Uuid;

// ============================================================
// TODO: Implement simple message queue
// ============================================================

/// Message in the queue
#[derive(Debug, Clone)]
struct Message {
    id: String,
    payload: String,
    attempts: u32,
    dequeued_at: Option<Instant>,
}

/// Simple message queue
struct Queue {
    pending: VecDeque<Message>,
    processing: HashMap<String, Message>,
    visibility_timeout: Duration,
}

impl Queue {
    fn new(visibility_timeout: Duration) -> Self {
        // TODO: Initialize queue
        Queue {
            pending: VecDeque::new(),
            processing: HashMap::new(),
            visibility_timeout,
        }
    }

    /// Add message to queue
    fn enqueue(&mut self, payload: String) -> String {
        // TODO: Create message with UUID, add to pending
        let id = Uuid::new_v4().to_string()[..8].to_string();
        let msg = Message {
            id: id.clone(),
            payload,
            attempts: 0,
            dequeued_at: None,
        };
        self.pending.push_back(msg);
        id
    }

    /// Get next message (makes it invisible)
    fn dequeue(&mut self) -> Option<Message> {
        // TODO: Move message from pending to processing
        // Set dequeued_at and increment attempts

        let mut msg = self.pending.pop_front()?;
        msg.attempts += 1;
        msg.dequeued_at = Some(Instant::now());

        let id = msg.id.clone();
        self.processing.insert(id, msg.clone());
        Some(msg)
    }

    /// Acknowledge message (remove from processing)
    fn acknowledge(&mut self, id: &str) -> bool {
        // TODO: Remove message from processing
        self.processing.remove(id).is_some()
    }

    /// Check for timed out messages and redeliver
    fn check_timeouts(&mut self) {
        // TODO: Move timed-out messages back to pending
        let now = Instant::now();
        let expired_ids: Vec<String> = self
            .processing
            .iter()
            .filter(|(_, msg)| {
                if let Some(dequeued_at) = msg.dequeued_at {
                    now.duration_since(dequeued_at) > self.visibility_timeout
                } else {
                    false
                }
            })
            .map(|(id, _)| id.clone())
            .collect();

        for id in expired_ids {
            if let Some(mut msg) = self.processing.remove(&id) {
                msg.dequeued_at = None;
                self.pending.push_back(msg);
            }
        }
    }

    /// Get queue statistics
    fn stats(&self) -> (usize, usize) {
        // TODO: Return (pending_count, processing_count)

        (self.pending.len(), self.processing.len())
    }
}

#[tokio::main]
async fn main() {
    // TODO: Implement demo
    println!("=== Simple Queue Demo ===\n");

    // 1. Create queue
    let mut queue = Queue::new(Duration::from_secs(2));
    // 2. Enqueue messages
    println!("Enqueuing 5 messages...");
    for i in 1..=5 {
        let payload = format!("msg-{}", i);
        let id = queue.enqueue(payload.clone());
        println!("Enqueued: {} ({})", payload, id);
    }

    let (pending, processing) = queue.stats();
    println!("Stats: pending={}, processing={}\n", pending, processing);

    // 3. Process some with ack
    println!("Worker processing...");
    for _ in 0..2 {
        if let Some(msg) = queue.dequeue() {
            println!("Dequeued: {} (attempt {})", msg.payload, msg.attempts);
            tokio::time::sleep(Duration::from_millis(100)).await;
            queue.acknowledge(&msg.id);
            println!("Processed and acknowledged: {}", msg.payload);
        }
    }
    let (pending, processing) = queue.stats();
    println!("Stats: pending={}, processing={}\n", pending, processing);

    // 4. Process some without ack
    println!("Simulating failure (no ack)...");
    if let Some(msg) = queue.dequeue() {
        println!("Dequeued: {} (attempt {})", msg.payload, msg.attempts);
        println!("(Worker crashed, no ack)");
    }
    let (pending, processing) = queue.stats();
    println!("Stats: pending={}, processing={}\n", pending, processing);
    println!("Waiting for visibility timeout...");
    tokio::time::sleep(Duration::from_secs(3)).await;

    queue.check_timeouts();
    let (pending, processing) = queue.stats();
    println!(
        "\nstats after check timeout: pending={}, processing={}",
        pending, processing
    );
    // 5. Show redelivery after timeout
    println!("After timeout, message redelivered:");
    if let Some(msg) = queue.dequeue() {
        println!("Dequeued: {} (attempt {})", msg.payload, msg.attempts);
        queue.acknowledge(&msg.id);
    }
    let (pending, processing) = queue.stats();
    println!(
        "\nstats after redelivered: pending={}, processing={}",
        pending, processing
    );

    println!("\nDraining remaining messages...");
    while let Some(msg) = queue.dequeue() {
        println!("Dequeued: {} (attempt {})", msg.payload, msg.attempts);
        queue.acknowledge(&msg.id);
    }
    let (pending, processing) = queue.stats();
    println!(
        "\nFinal stats: pending={}, processing={}",
        pending, processing
    );
}
