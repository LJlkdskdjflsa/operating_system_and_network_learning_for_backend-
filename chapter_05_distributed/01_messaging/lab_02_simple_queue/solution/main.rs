//! Lab 2 Reference Answer

use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};
use uuid::Uuid;

/// Message in the queue
#[derive(Debug, Clone)]
struct Message {
    id: String,
    payload: String,
    attempts: u32,
    dequeued_at: Option<Instant>,
}

/// Simple message queue with visibility timeout
struct Queue {
    pending: VecDeque<Message>,
    processing: HashMap<String, Message>,
    visibility_timeout: Duration,
}

impl Queue {
    fn new(visibility_timeout: Duration) -> Self {
        Queue {
            pending: VecDeque::new(),
            processing: HashMap::new(),
            visibility_timeout,
        }
    }

    /// Add message to queue
    fn enqueue(&mut self, payload: String) -> String {
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
        let mut msg = self.pending.pop_front()?;
        msg.attempts += 1;
        msg.dequeued_at = Some(Instant::now());

        let id = msg.id.clone();
        self.processing.insert(id, msg.clone());
        Some(msg)
    }

    /// Acknowledge message (remove from processing)
    fn acknowledge(&mut self, id: &str) -> bool {
        self.processing.remove(id).is_some()
    }

    /// Negative acknowledge (return to queue immediately)
    fn nack(&mut self, id: &str) -> bool {
        if let Some(mut msg) = self.processing.remove(id) {
            msg.dequeued_at = None;
            self.pending.push_front(msg);
            true
        } else {
            false
        }
    }

    /// Check for timed out messages and redeliver
    fn check_timeouts(&mut self) -> Vec<String> {
        let now = Instant::now();
        let mut timed_out = Vec::new();

        // Find timed out messages
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

        // Move back to pending
        for id in expired_ids {
            if let Some(mut msg) = self.processing.remove(&id) {
                msg.dequeued_at = None;
                timed_out.push(msg.id.clone());
                self.pending.push_back(msg);
            }
        }

        timed_out
    }

    /// Get queue statistics
    fn stats(&self) -> (usize, usize) {
        (self.pending.len(), self.processing.len())
    }

    /// Check if queue is empty
    fn is_empty(&self) -> bool {
        self.pending.is_empty() && self.processing.is_empty()
    }
}

#[tokio::main]
async fn main() {
    println!("=== Simple Message Queue Demo ===\n");

    // Create queue with 2 second visibility timeout
    let mut queue = Queue::new(Duration::from_secs(2));

    // Enqueue messages
    println!("1. Enqueuing messages...");
    for i in 1..=5 {
        let payload = format!("Task {}", i);
        let id = queue.enqueue(payload.clone());
        println!("   Enqueued [{}]: {}", id, payload);
    }

    let (pending, processing) = queue.stats();
    println!("   Stats: pending={}, processing={}\n", pending, processing);

    // Process messages with acknowledgment
    println!("2. Processing with acknowledgment...");
    for _ in 0..2 {
        if let Some(msg) = queue.dequeue() {
            println!("   Dequeued [{}]: {} (attempt {})", msg.id, msg.payload, msg.attempts);

            // Simulate processing
            tokio::time::sleep(Duration::from_millis(100)).await;

            // Acknowledge
            queue.acknowledge(&msg.id);
            println!("   Acknowledged [{}]", msg.id);
        }
    }

    let (pending, processing) = queue.stats();
    println!("   Stats: pending={}, processing={}\n", pending, processing);

    // Process without acknowledgment (simulate failure)
    println!("3. Simulating worker failure (no ack)...");
    if let Some(msg) = queue.dequeue() {
        println!("   Dequeued [{}]: {} (attempt {})", msg.id, msg.payload, msg.attempts);
        println!("   Worker 'crashed' - no acknowledgment!");
        // Note: We're not calling acknowledge()
    }

    let (pending, processing) = queue.stats();
    println!("   Stats: pending={}, processing={}\n", pending, processing);

    // Wait for visibility timeout
    println!("4. Waiting for visibility timeout (2 seconds)...");
    tokio::time::sleep(Duration::from_secs(3)).await;

    // Check timeouts - message should be redelivered
    let redelivered = queue.check_timeouts();
    println!("   Redelivered messages: {:?}", redelivered);

    let (pending, processing) = queue.stats();
    println!("   Stats: pending={}, processing={}\n", pending, processing);

    // Process the redelivered message
    println!("5. Processing redelivered message...");
    if let Some(msg) = queue.dequeue() {
        println!(
            "   Dequeued [{}]: {} (attempt {})",
            msg.id, msg.payload, msg.attempts
        );
        queue.acknowledge(&msg.id);
        println!("   Acknowledged [{}]", msg.id);
    }

    // Process remaining
    println!("\n6. Processing remaining messages...");
    while let Some(msg) = queue.dequeue() {
        println!(
            "   Dequeued [{}]: {} (attempt {})",
            msg.id, msg.payload, msg.attempts
        );
        queue.acknowledge(&msg.id);
        println!("   Acknowledged [{}]", msg.id);
    }

    let (pending, processing) = queue.stats();
    println!("\nFinal stats: pending={}, processing={}", pending, processing);
    println!("Queue empty: {}", queue.is_empty());

    println!("\n=== Key Concepts ===");
    println!("- Visibility timeout prevents duplicate processing");
    println!("- Unacked messages are redelivered");
    println!("- Attempt counter tracks retries");
    println!("- At-least-once delivery (may have duplicates)");
}

// Key concepts demonstrated:
//
// 1. VISIBILITY TIMEOUT:
//    - When dequeued, message is "invisible" to other consumers
//    - If not acknowledged within timeout, becomes visible again
//    - Prevents duplicate processing while allowing retry
//
// 2. ACKNOWLEDGMENT:
//    - Explicit confirmation that message was processed
//    - Without ack, message will be redelivered
//    - Enables at-least-once delivery
//
// 3. ATTEMPT TRACKING:
//    - Count how many times message was dequeued
//    - Can implement dead-letter queue after N attempts
//    - Helps debug problematic messages
//
// 4. AT-LEAST-ONCE DELIVERY:
//    - Message may be delivered multiple times
//    - Consumer must be idempotent
//    - Prefer over at-most-once for important data

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enqueue_dequeue() {
        let mut queue = Queue::new(Duration::from_secs(30));

        queue.enqueue("test".to_string());
        let msg = queue.dequeue().unwrap();

        assert_eq!(msg.payload, "test");
        assert_eq!(msg.attempts, 1);
    }

    #[test]
    fn test_acknowledge() {
        let mut queue = Queue::new(Duration::from_secs(30));

        queue.enqueue("test".to_string());
        let msg = queue.dequeue().unwrap();

        assert!(queue.acknowledge(&msg.id));
        assert!(!queue.acknowledge(&msg.id)); // Second ack fails

        let (pending, processing) = queue.stats();
        assert_eq!(pending, 0);
        assert_eq!(processing, 0);
    }

    #[test]
    fn test_visibility_timeout() {
        let mut queue = Queue::new(Duration::from_millis(100));

        queue.enqueue("test".to_string());
        let msg = queue.dequeue().unwrap();

        // Immediately check - should not redeliver
        let redelivered = queue.check_timeouts();
        assert!(redelivered.is_empty());

        // Wait for timeout
        std::thread::sleep(Duration::from_millis(150));

        // Now should redeliver
        let redelivered = queue.check_timeouts();
        assert_eq!(redelivered.len(), 1);
        assert_eq!(redelivered[0], msg.id);

        // Message should be back in pending
        let msg2 = queue.dequeue().unwrap();
        assert_eq!(msg2.id, msg.id);
        assert_eq!(msg2.attempts, 2);
    }
}
