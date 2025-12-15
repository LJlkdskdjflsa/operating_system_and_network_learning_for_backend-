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

        todo!("Implement Queue::new")
    }

    /// Add message to queue
    fn enqueue(&mut self, payload: String) -> String {
        // TODO: Create message with UUID, add to pending

        todo!("Implement Queue::enqueue")
    }

    /// Get next message (makes it invisible)
    fn dequeue(&mut self) -> Option<Message> {
        // TODO: Move message from pending to processing
        // Set dequeued_at and increment attempts

        todo!("Implement Queue::dequeue")
    }

    /// Acknowledge message (remove from processing)
    fn acknowledge(&mut self, id: &str) -> bool {
        // TODO: Remove message from processing

        todo!("Implement Queue::acknowledge")
    }

    /// Check for timed out messages and redeliver
    fn check_timeouts(&mut self) {
        // TODO: Move timed-out messages back to pending

        todo!("Implement Queue::check_timeouts")
    }

    /// Get queue statistics
    fn stats(&self) -> (usize, usize) {
        // TODO: Return (pending_count, processing_count)

        todo!("Implement Queue::stats")
    }
}

#[tokio::main]
async fn main() {
    // TODO: Implement demo
    // 1. Create queue
    // 2. Enqueue messages
    // 3. Process some with ack
    // 4. Process some without ack
    // 5. Show redelivery after timeout

    todo!("Implement main")
}
