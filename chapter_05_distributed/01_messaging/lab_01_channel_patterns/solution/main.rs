//! Lab 1 Reference Answer

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{broadcast, mpsc, Mutex};

/// Fan-in: Multiple producers send to single consumer
async fn demo_fan_in() {
    let num_producers = 3;
    let messages_per_producer = 3;

    // Create mpsc channel
    let (tx, mut rx) = mpsc::channel::<String>(100);

    // Spawn producers
    let mut handles = vec![];
    for id in 0..num_producers {
        let tx = tx.clone();
        let handle = tokio::spawn(async move {
            for i in 0..messages_per_producer {
                let msg = format!("Producer {} message {}", id, i);
                println!("  [Producer {}] Sending: {}", id, msg);
                tx.send(msg).await.unwrap();
                tokio::time::sleep(Duration::from_millis(50)).await;
            }
            println!("  [Producer {}] Done", id);
        });
        handles.push(handle);
    }

    // Drop original sender so channel closes when producers finish
    drop(tx);

    // Consumer task
    let consumer = tokio::spawn(async move {
        let mut count = 0;
        while let Some(msg) = rx.recv().await {
            println!("  [Consumer] Received: {}", msg);
            count += 1;
        }
        println!("  [Consumer] Total received: {}", count);
    });

    // Wait for all producers
    for handle in handles {
        handle.await.unwrap();
    }

    // Wait for consumer
    consumer.await.unwrap();
}

/// Fan-out: Single producer sends to multiple consumers
async fn demo_fan_out() {
    let num_subscribers = 3;
    let num_messages = 3;

    // Create broadcast channel
    let (tx, _rx) = broadcast::channel::<String>(100);

    // Spawn subscribers
    let mut handles = vec![];
    for id in 0..num_subscribers {
        let mut rx = tx.subscribe();
        let handle = tokio::spawn(async move {
            let mut count = 0;
            while let Ok(msg) = rx.recv().await {
                println!("  [Subscriber {}] Received: {}", id, msg);
                count += 1;
            }
            println!("  [Subscriber {}] Total: {}", id, count);
        });
        handles.push(handle);
    }

    // Give subscribers time to start
    tokio::time::sleep(Duration::from_millis(50)).await;

    // Send messages
    for i in 0..num_messages {
        let msg = format!("Event {}", i);
        println!("  [Producer] Broadcasting: {}", msg);
        let _ = tx.send(msg);
        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    // Close channel
    drop(tx);

    // Wait for subscribers
    for handle in handles {
        handle.await.unwrap();
    }
}

/// Worker pool: Distribute jobs across workers
async fn demo_worker_pool() {
    let num_workers = 3;
    let num_jobs = 10;

    // Create job channel
    let (tx, rx) = mpsc::channel::<usize>(100);
    let rx = Arc::new(Mutex::new(rx));

    // Create result channel
    let (result_tx, mut result_rx) = mpsc::channel::<(usize, usize)>(100);

    // Spawn workers
    let mut handles = vec![];
    for worker_id in 0..num_workers {
        let rx = rx.clone();
        let result_tx = result_tx.clone();

        let handle = tokio::spawn(async move {
            loop {
                // Try to get a job
                let job = {
                    let mut rx = rx.lock().await;
                    rx.recv().await
                };

                match job {
                    Some(job_id) => {
                        println!("  [Worker {}] Processing job {}", worker_id, job_id);
                        // Simulate work
                        tokio::time::sleep(Duration::from_millis(100)).await;
                        result_tx.send((worker_id, job_id)).await.unwrap();
                    }
                    None => {
                        println!("  [Worker {}] Shutting down", worker_id);
                        break;
                    }
                }
            }
        });
        handles.push(handle);
    }

    // Drop original result sender
    drop(result_tx);

    // Submit jobs
    println!("  Submitting {} jobs to {} workers", num_jobs, num_workers);
    for job_id in 0..num_jobs {
        tx.send(job_id).await.unwrap();
    }

    // Close job channel
    drop(tx);

    // Collect results
    let results_handle = tokio::spawn(async move {
        let mut results = vec![];
        while let Some((worker_id, job_id)) = result_rx.recv().await {
            results.push((worker_id, job_id));
        }
        results
    });

    // Wait for workers
    for handle in handles {
        handle.await.unwrap();
    }

    // Get results
    let results = results_handle.await.unwrap();
    println!("\n  Results:");
    let mut jobs_per_worker = vec![0; num_workers];
    for (worker_id, _) in &results {
        jobs_per_worker[*worker_id] += 1;
    }
    for (id, count) in jobs_per_worker.iter().enumerate() {
        println!("    Worker {}: {} jobs", id, count);
    }
}

/// Demonstrate backpressure with bounded channel
async fn demo_backpressure() {
    println!("\n=== Backpressure Demo ===");

    // Small bounded channel
    let (tx, mut rx) = mpsc::channel::<usize>(3);

    // Fast producer
    let producer = tokio::spawn(async move {
        for i in 0..10 {
            println!("  [Producer] Sending {}...", i);
            tx.send(i).await.unwrap();
            println!("  [Producer] Sent {}", i);
        }
    });

    // Slow consumer
    let consumer = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            println!("  [Consumer] Processing {}...", msg);
            tokio::time::sleep(Duration::from_millis(200)).await;
            println!("  [Consumer] Done with {}", msg);
        }
    });

    producer.await.unwrap();
    consumer.await.unwrap();
}

#[tokio::main]
async fn main() {
    println!("Channel Patterns Demo\n");

    println!("=== Fan-In Pattern ===");
    println!("(Multiple producers -> Single consumer)\n");
    demo_fan_in().await;

    println!("\n=== Fan-Out Pattern ===");
    println!("(Single producer -> Multiple consumers)\n");
    demo_fan_out().await;

    println!("\n=== Worker Pool Pattern ===");
    println!("(Distribute work across workers)\n");
    demo_worker_pool().await;

    demo_backpressure().await;

    println!("\n=== Summary ===");
    println!("- Fan-In: Use mpsc, clone sender for producers");
    println!("- Fan-Out: Use broadcast, subscribe for consumers");
    println!("- Worker Pool: Shared receiver with mutex");
    println!("- Backpressure: Bounded channels block when full");
}

// Key concepts demonstrated:
//
// 1. FAN-IN (mpsc):
//    - Multiple senders, one receiver
//    - Clone tx for each producer
//    - Drop tx to signal completion
//
// 2. FAN-OUT (broadcast):
//    - One sender, multiple subscribers
//    - Each subscriber gets all messages
//    - subscribe() creates new receiver
//
// 3. WORKER POOL:
//    - Shared receiver (Arc<Mutex<Receiver>>)
//    - Workers compete for jobs
//    - Automatic load balancing
//
// 4. BACKPRESSURE:
//    - Bounded channels block send when full
//    - Prevents memory exhaustion
//    - Slows down fast producers

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fan_in() {
        let (tx, mut rx) = mpsc::channel(10);

        let tx1 = tx.clone();
        let tx2 = tx.clone();
        drop(tx);

        tokio::spawn(async move { tx1.send(1).await.unwrap() });
        tokio::spawn(async move { tx2.send(2).await.unwrap() });

        let mut received = vec![];
        while let Some(v) = rx.recv().await {
            received.push(v);
        }

        assert_eq!(received.len(), 2);
    }

    #[tokio::test]
    async fn test_fan_out() {
        let (tx, _) = broadcast::channel(10);

        let mut rx1 = tx.subscribe();
        let mut rx2 = tx.subscribe();

        tx.send("test").unwrap();

        assert_eq!(rx1.recv().await.unwrap(), "test");
        assert_eq!(rx2.recv().await.unwrap(), "test");
    }
}
