# Messaging Patterns

## Overview

Messaging enables asynchronous communication between components. Understanding messaging patterns is essential for building scalable, decoupled systems.

## Why Messaging?

### Direct Communication (Tight Coupling)
```
Service A --HTTP--> Service B
           ↓
    If B is slow or down, A blocks or fails
```

### Message-Based (Loose Coupling)
```
Service A --> Queue --> Service B
     ↓
  A doesn't wait
  B processes when ready
  If B is down, messages wait in queue
```

## Channel Types in Rust

### oneshot - Single Value

```rust
use tokio::sync::oneshot;

// Create channel
let (tx, rx) = oneshot::channel();

// Send single value
tx.send("done").unwrap();

// Receive
let result = rx.await.unwrap();
```

Use case: Request-response, task completion notification

### mpsc - Multi-Producer, Single Consumer

```rust
use tokio::sync::mpsc;

// Create bounded channel
let (tx, mut rx) = mpsc::channel(100);

// Multiple producers
let tx2 = tx.clone();
tokio::spawn(async move { tx.send("from 1").await });
tokio::spawn(async move { tx2.send("from 2").await });

// Single consumer
while let Some(msg) = rx.recv().await {
    println!("{}", msg);
}
```

Use case: Job queues, logging, fan-in pattern

### broadcast - Multi-Producer, Multi-Consumer

```rust
use tokio::sync::broadcast;

// Create channel
let (tx, _rx) = broadcast::channel(100);

// Multiple subscribers
let mut rx1 = tx.subscribe();
let mut rx2 = tx.subscribe();

// Send (all subscribers receive)
tx.send("event").unwrap();

// Each receives independently
let msg1 = rx1.recv().await.unwrap();
let msg2 = rx2.recv().await.unwrap();
```

Use case: Event broadcasting, pub/sub

### watch - Single Value, Multiple Observers

```rust
use tokio::sync::watch;

// Create with initial value
let (tx, rx) = watch::channel("initial");

// Multiple observers
let mut rx2 = rx.clone();

// Update value
tx.send("updated").unwrap();

// Observers see latest value
println!("{}", *rx.borrow());
```

Use case: Configuration updates, state broadcasting

## Messaging Patterns

### Fan-Out (One to Many)

```
Producer --> Broadcast Channel --> Consumer 1
                              --> Consumer 2
                              --> Consumer 3
```

```rust
let (tx, _) = broadcast::channel(100);

// Spawn consumers
for i in 0..3 {
    let mut rx = tx.subscribe();
    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            println!("Consumer {}: {}", i, msg);
        }
    });
}

// Producer sends to all
tx.send("message").unwrap();
```

### Fan-In (Many to One)

```
Producer 1 --> mpsc Channel --> Consumer
Producer 2 -->
Producer 3 -->
```

```rust
let (tx, mut rx) = mpsc::channel(100);

// Spawn producers
for i in 0..3 {
    let tx = tx.clone();
    tokio::spawn(async move {
        tx.send(format!("from producer {}", i)).await.unwrap();
    });
}
drop(tx); // Drop original sender

// Single consumer
while let Some(msg) = rx.recv().await {
    println!("{}", msg);
}
```

### Worker Pool

```
         ┌─> Worker 1 ─┐
Jobs --> │   Worker 2  │ --> Results
         └─> Worker 3 ─┘
```

```rust
let (job_tx, job_rx) = async_channel::bounded(100);
let (result_tx, mut result_rx) = mpsc::channel(100);

// Spawn workers
for id in 0..4 {
    let rx = job_rx.clone();
    let tx = result_tx.clone();
    tokio::spawn(async move {
        while let Ok(job) = rx.recv().await {
            let result = process(job);
            tx.send((id, result)).await.unwrap();
        }
    });
}

// Submit jobs
for job in jobs {
    job_tx.send(job).await.unwrap();
}

// Collect results
while let Some((worker_id, result)) = result_rx.recv().await {
    println!("Worker {} completed: {:?}", worker_id, result);
}
```

## Backpressure

When producer is faster than consumer:

### Unbounded Channel (Dangerous)

```rust
// Memory can grow without limit!
let (tx, rx) = mpsc::unbounded_channel();
```

### Bounded Channel (Safe)

```rust
// Blocks when full
let (tx, rx) = mpsc::channel(100);

// Or use try_send to avoid blocking
match tx.try_send(msg) {
    Ok(()) => { /* sent */ }
    Err(TrySendError::Full(_)) => { /* handle backpressure */ }
    Err(TrySendError::Closed(_)) => { /* channel closed */ }
}
```

### Backpressure Strategies

1. **Block**: Wait until space available (default)
2. **Drop**: Discard message on full
3. **Replace**: Overwrite oldest message
4. **Error**: Return error to caller

## Message Delivery Guarantees

### At-Most-Once

```rust
// Send and forget
let _ = tx.send(msg);  // Ignoring result
```

- May lose messages
- No duplicates
- Lowest latency

### At-Least-Once

```rust
// Retry until acknowledged
loop {
    tx.send(msg.clone()).await?;
    match rx.recv_timeout(Duration::from_secs(5)).await {
        Ok(ack) => break,
        Err(_) => continue,  // Retry
    }
}
```

- No message loss
- May have duplicates
- Requires idempotent processing

### Exactly-Once

Requires:
- Deduplication (track message IDs)
- Idempotent operations
- Transactional processing

## Simple Message Queue Design

```rust
struct Message {
    id: Uuid,
    payload: String,
    created_at: Instant,
    attempts: u32,
}

struct Queue {
    pending: VecDeque<Message>,
    processing: HashMap<Uuid, Message>,
    visibility_timeout: Duration,
}

impl Queue {
    fn enqueue(&mut self, payload: String) -> Uuid {
        let msg = Message {
            id: Uuid::new_v4(),
            payload,
            created_at: Instant::now(),
            attempts: 0,
        };
        let id = msg.id;
        self.pending.push_back(msg);
        id
    }

    fn dequeue(&mut self) -> Option<Message> {
        let mut msg = self.pending.pop_front()?;
        msg.attempts += 1;
        self.processing.insert(msg.id, msg.clone());
        Some(msg)
    }

    fn acknowledge(&mut self, id: Uuid) -> bool {
        self.processing.remove(&id).is_some()
    }

    fn check_timeouts(&mut self) {
        let now = Instant::now();
        let expired: Vec<_> = self.processing
            .iter()
            .filter(|(_, m)| now.duration_since(m.created_at) > self.visibility_timeout)
            .map(|(id, _)| *id)
            .collect();

        for id in expired {
            if let Some(msg) = self.processing.remove(&id) {
                self.pending.push_back(msg);
            }
        }
    }
}
```

## Graceful Shutdown

```rust
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

async fn worker(
    mut rx: mpsc::Receiver<Job>,
    cancel: CancellationToken,
) {
    loop {
        tokio::select! {
            Some(job) = rx.recv() => {
                process(job).await;
            }
            _ = cancel.cancelled() => {
                // Finish current work
                while let Ok(job) = rx.try_recv() {
                    process(job).await;
                }
                break;
            }
        }
    }
}

// Shutdown
cancel.cancel();
worker_handle.await.unwrap();
```

## Real-World Message Queues

### Redis (Simple)
- LPUSH/RPOP for basic queues
- BLPOP for blocking
- Pub/Sub for broadcast

### RabbitMQ
- AMQP protocol
- Acknowledgments
- Dead letter queues
- Routing

### Kafka
- Distributed log
- Partitioning
- Consumer groups
- High throughput

## Summary

- **Channels** enable async communication
- **mpsc** for fan-in, **broadcast** for fan-out
- **Bounded channels** prevent memory exhaustion
- **At-least-once** delivery needs idempotency
- **Graceful shutdown** handles in-flight messages

## Labs

1. **Lab 1: Channel Patterns** - Fan-in, fan-out, worker pool
2. **Lab 2: Simple Queue** - Message queue with acknowledgment
