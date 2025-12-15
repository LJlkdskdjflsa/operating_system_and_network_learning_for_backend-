# Chapter 5 Checkpoint

## Self-Assessment Questions

### Messaging (Lab 1-2)

1. **What is the difference between mpsc and broadcast channels?**
   - mpsc: Multiple producers, single consumer
   - broadcast: Multiple producers, multiple consumers (each gets copy)

2. **What is backpressure and how do you handle it?**
   - When producer is faster than consumer
   - Solutions: bounded channels, dropping messages, blocking

3. **Why use message queues instead of direct calls?**
   - Decoupling (services don't need to know each other)
   - Load leveling (handle spikes)
   - Retry on failure
   - Async processing

4. **What is the difference between at-most-once and at-least-once delivery?**
   - At-most-once: May lose messages, no duplicates
   - At-least-once: May have duplicates, no loss

### Resilience Patterns (Lab 3-4)

5. **What is a token bucket rate limiter?**
   - Bucket holds tokens
   - Tokens added at fixed rate
   - Request consumes token
   - No token = rejected

6. **What are the three states of a circuit breaker?**
   - Closed: Normal operation
   - Open: Failing fast (no calls to service)
   - Half-Open: Testing if service recovered

7. **When should you use rate limiting?**
   - Protect against DDoS
   - Enforce API quotas
   - Prevent resource exhaustion
   - Fair resource sharing

8. **What triggers a circuit to open?**
   - Consecutive failures exceed threshold
   - Error rate exceeds threshold
   - Response time exceeds threshold

## Concept Quiz

### Question 1: Channel Selection
Which channel type should you use for logging where multiple services send logs to a single aggregator?
- A) oneshot
- B) mpsc
- C) broadcast
- D) watch

<details>
<summary>Answer</summary>
B) mpsc (Multi-producer, single-consumer)

Multiple services (producers) send logs to one aggregator (consumer).
mpsc is perfect for this fan-in pattern.
</details>

### Question 2: Backpressure
Your producer sends 1000 msg/sec but consumer handles 100 msg/sec. What happens with a bounded channel (capacity 100)?
- A) Messages are dropped
- B) Producer blocks when channel is full
- C) Channel grows unbounded
- D) Consumer speeds up

<details>
<summary>Answer</summary>
B) Producer blocks when channel is full

Bounded channels apply backpressure by blocking send() when full.
This prevents memory exhaustion but may slow down the producer.
Options: increase capacity, drop messages, or add more consumers.
</details>

### Question 3: Rate Limiting Algorithm
You want to allow 10 requests per second with occasional bursts of 20. Which algorithm is best?
- A) Fixed window counter
- B) Token bucket
- C) Leaky bucket
- D) Simple counter

<details>
<summary>Answer</summary>
B) Token bucket

Token bucket allows bursts (bucket capacity = 20) while maintaining
average rate (refill 10 tokens/sec).
Leaky bucket doesn't allow bursts.
Fixed window has boundary issues.
</details>

### Question 4: Circuit Breaker Transition
A circuit breaker is in OPEN state. After the timeout period, what state does it transition to?
- A) CLOSED
- B) HALF_OPEN
- C) Stays OPEN
- D) ERROR

<details>
<summary>Answer</summary>
B) HALF_OPEN

After timeout, circuit moves to HALF_OPEN to test if service recovered.
- If test succeeds: transition to CLOSED
- If test fails: transition back to OPEN
</details>

### Question 5: Message Acknowledgment
In a job queue, what happens if a worker crashes before acknowledging a message?
- A) Message is lost forever
- B) Message is redelivered to another worker
- C) Queue stops processing
- D) Message is automatically deleted

<details>
<summary>Answer</summary>
B) Message is redelivered to another worker

With at-least-once delivery and visibility timeout:
1. Message becomes invisible when claimed
2. If not acknowledged before timeout, becomes visible again
3. Another worker can claim it

This ensures no message loss but requires idempotent processing.
</details>

## Practical Verification

### Channel Patterns
```bash
cd lab_01_channel_patterns
cargo run

# Verify:
# - Messages flow from producer to consumer
# - Backpressure works (bounded channel)
# - Clean shutdown
```

### Simple Queue
```bash
cd lab_02_simple_queue
cargo run

# Verify:
# - Messages enqueue and dequeue
# - Acknowledgment works
# - Unacked messages redeliver
```

### Rate Limiter
```bash
cd lab_03_rate_limiter
cargo run

# Verify:
# - Requests within limit succeed
# - Requests over limit are rejected
# - Bucket refills over time
```

### Circuit Breaker
```bash
cd lab_04_circuit_breaker
cargo run

# Verify:
# - Normal calls succeed (CLOSED)
# - Failures trigger OPEN state
# - After timeout, moves to HALF_OPEN
# - Success in HALF_OPEN closes circuit
```

## Key Takeaways

1. **Channels decouple producers and consumers** - enables async processing
2. **Backpressure prevents memory exhaustion** - use bounded channels
3. **Rate limiting protects resources** - token bucket allows bursts
4. **Circuit breakers prevent cascade failures** - fail fast, recover slow
5. **At-least-once delivery requires idempotency** - design for duplicates
6. **Timeouts are essential** - never wait forever
