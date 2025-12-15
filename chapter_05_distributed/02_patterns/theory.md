# Resilience Patterns

## Overview

Resilience patterns help systems handle failures gracefully. They prevent cascade failures, protect resources, and maintain availability under adverse conditions.

## Why Resilience Matters

```
Without resilience:
User -> Service A -> Service B (slow/failing)
        ↓
   A waits forever
        ↓
   A's resources exhausted
        ↓
   A fails
        ↓
   Cascade failure

With resilience:
User -> Service A -> Service B (slow/failing)
        ↓
   Circuit breaker opens
        ↓
   A returns fallback immediately
        ↓
   System remains available
```

## Rate Limiting

### Why Rate Limit?

- **Protect resources**: Prevent overload
- **Ensure fairness**: Share resources among users
- **Prevent abuse**: Stop malicious traffic
- **Enforce quotas**: Billing, API limits

### Token Bucket Algorithm

```
┌─────────────────────┐
│   Token Bucket      │
│   [● ● ● ● ○ ○]     │  ← Bucket (capacity: 6)
│                     │
│   Refill: 1 token/s │
│   Current: 4 tokens │
└─────────────────────┘

Request arrives:
- If tokens > 0: consume token, allow request
- If tokens = 0: reject request

Allows bursts up to bucket capacity!
```

```rust
struct TokenBucket {
    tokens: f64,
    capacity: f64,
    refill_rate: f64,  // tokens per second
    last_refill: Instant,
}

impl TokenBucket {
    fn allow(&mut self) -> bool {
        self.refill();

        if self.tokens >= 1.0 {
            self.tokens -= 1.0;
            true
        } else {
            false
        }
    }

    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill).as_secs_f64();
        self.tokens = (self.tokens + elapsed * self.refill_rate).min(self.capacity);
        self.last_refill = now;
    }
}
```

### Leaky Bucket Algorithm

```
┌─────────────────────┐
│   Leaky Bucket      │
│   [● ● ● ● ● ●]     │  ← Requests queue
│         ↓           │
│        ━━━          │  ← Constant outflow rate
│         ● ●         │
└─────────────────────┘

- Requests enter bucket
- Processed at fixed rate
- Overflow rejected

Smooths out bursts!
```

### Sliding Window

```
Time: |-----|-----|-----|-----|-----|
      t-5   t-4   t-3   t-2   t-1   t

Window: [        last 60 seconds        ]
Requests in window: 45
Limit: 100/minute

Current request: 45 < 100 → Allow
```

```rust
struct SlidingWindow {
    requests: VecDeque<Instant>,
    window: Duration,
    limit: usize,
}

impl SlidingWindow {
    fn allow(&mut self) -> bool {
        let now = Instant::now();
        let cutoff = now - self.window;

        // Remove old requests
        while let Some(front) = self.requests.front() {
            if *front < cutoff {
                self.requests.pop_front();
            } else {
                break;
            }
        }

        if self.requests.len() < self.limit {
            self.requests.push_back(now);
            true
        } else {
            false
        }
    }
}
```

### Rate Limiter Comparison

| Algorithm | Bursts | Memory | Precision |
|-----------|--------|--------|-----------|
| Token Bucket | Allows | O(1) | Good |
| Leaky Bucket | Smooths | O(n) | Good |
| Fixed Window | Edge burst | O(1) | Poor |
| Sliding Window | No | O(n) | Best |

## Circuit Breaker

### Why Circuit Breaker?

- **Fail fast**: Don't wait for timeouts
- **Prevent cascade**: Don't overwhelm failing service
- **Allow recovery**: Give service time to recover
- **Provide fallback**: Return default or cached data

### States

```
        ┌──────────────────────────────────────┐
        │                                      │
        ↓                                      │
   ┌─────────┐    failures >= threshold   ┌────────┐
   │ CLOSED  │ ────────────────────────→  │  OPEN  │
   │ (normal)│                            │ (fail) │
   └─────────┘                            └────────┘
        ↑                                      │
        │                                      │
        │         timeout expires              ↓
        │                                ┌───────────┐
        │                                │ HALF_OPEN │
        │                                │  (test)   │
        └────────────────────────────────┴───────────┘
              success                         │
              ←──────────────────────────────┘
                    failure → back to OPEN
```

### Implementation

```rust
enum State {
    Closed,
    Open { until: Instant },
    HalfOpen,
}

struct CircuitBreaker {
    state: State,
    failure_count: u32,
    failure_threshold: u32,
    reset_timeout: Duration,
    success_threshold: u32,  // for half-open
    half_open_successes: u32,
}

impl CircuitBreaker {
    fn call<F, T, E>(&mut self, f: F) -> Result<T, CircuitError<E>>
    where
        F: FnOnce() -> Result<T, E>,
    {
        match &self.state {
            State::Open { until } if Instant::now() < *until => {
                return Err(CircuitError::Open);
            }
            State::Open { .. } => {
                self.state = State::HalfOpen;
                self.half_open_successes = 0;
            }
            _ => {}
        }

        match f() {
            Ok(result) => {
                self.on_success();
                Ok(result)
            }
            Err(e) => {
                self.on_failure();
                Err(CircuitError::Failed(e))
            }
        }
    }

    fn on_success(&mut self) {
        match self.state {
            State::HalfOpen => {
                self.half_open_successes += 1;
                if self.half_open_successes >= self.success_threshold {
                    self.state = State::Closed;
                    self.failure_count = 0;
                }
            }
            State::Closed => {
                self.failure_count = 0;
            }
            _ => {}
        }
    }

    fn on_failure(&mut self) {
        match self.state {
            State::HalfOpen => {
                self.state = State::Open {
                    until: Instant::now() + self.reset_timeout,
                };
            }
            State::Closed => {
                self.failure_count += 1;
                if self.failure_count >= self.failure_threshold {
                    self.state = State::Open {
                        until: Instant::now() + self.reset_timeout,
                    };
                }
            }
            _ => {}
        }
    }
}
```

### Configuration

```rust
let breaker = CircuitBreaker {
    failure_threshold: 5,     // Open after 5 failures
    reset_timeout: Duration::from_secs(30),  // Try again after 30s
    success_threshold: 3,     // Close after 3 successes in half-open
};
```

## Bulkhead

Isolate components to prevent cascade failures.

```
┌────────────────────────────────────────┐
│             Application                │
│  ┌─────────────┐  ┌─────────────┐     │
│  │ Bulkhead A  │  │ Bulkhead B  │     │
│  │ (10 threads)│  │ (10 threads)│     │
│  │   [A calls] │  │   [B calls] │     │
│  └─────────────┘  └─────────────┘     │
│                                        │
│  If B is slow, only B's threads block │
│  A continues to work normally          │
└────────────────────────────────────────┘
```

```rust
use tokio::sync::Semaphore;

struct Bulkhead {
    semaphore: Semaphore,
}

impl Bulkhead {
    fn new(permits: usize) -> Self {
        Bulkhead {
            semaphore: Semaphore::new(permits),
        }
    }

    async fn execute<F, T>(&self, f: F) -> Result<T, BulkheadError>
    where
        F: Future<Output = T>,
    {
        let _permit = self.semaphore
            .acquire()
            .await
            .map_err(|_| BulkheadError::Closed)?;

        Ok(f.await)
    }
}
```

## Timeout

Never wait forever.

```rust
use tokio::time::timeout;

async fn call_service() -> Result<Response, Error> {
    match timeout(Duration::from_secs(5), service.call()).await {
        Ok(Ok(response)) => Ok(response),
        Ok(Err(e)) => Err(e),
        Err(_) => Err(Error::Timeout),
    }
}
```

## Retry with Backoff

```rust
async fn retry_with_backoff<F, T, E>(
    mut f: F,
    max_retries: u32,
) -> Result<T, E>
where
    F: FnMut() -> Future<Output = Result<T, E>>,
{
    let mut attempts = 0;
    let mut delay = Duration::from_millis(100);

    loop {
        match f().await {
            Ok(result) => return Ok(result),
            Err(e) if attempts < max_retries => {
                attempts += 1;
                // Exponential backoff with jitter
                let jitter = rand::random::<f64>() * 0.3;
                let sleep_time = delay.mul_f64(1.0 + jitter);
                tokio::time::sleep(sleep_time).await;
                delay *= 2;  // Double delay each retry
            }
            Err(e) => return Err(e),
        }
    }
}
```

## Combining Patterns

```rust
async fn resilient_call<T>(
    rate_limiter: &mut RateLimiter,
    circuit_breaker: &mut CircuitBreaker,
    bulkhead: &Bulkhead,
) -> Result<T, Error> {
    // 1. Rate limiting
    if !rate_limiter.allow() {
        return Err(Error::RateLimited);
    }

    // 2. Circuit breaker
    if circuit_breaker.is_open() {
        return Err(Error::CircuitOpen);
    }

    // 3. Bulkhead (limit concurrency)
    bulkhead.execute(async {
        // 4. Timeout
        let result = timeout(Duration::from_secs(5), async {
            // 5. Retry with backoff
            retry_with_backoff(|| service.call(), 3).await
        }).await;

        // Update circuit breaker based on result
        match &result {
            Ok(_) => circuit_breaker.record_success(),
            Err(_) => circuit_breaker.record_failure(),
        }

        result
    }).await
}
```

## Summary

| Pattern | Purpose | When to Use |
|---------|---------|-------------|
| Rate Limiting | Control request rate | API protection |
| Circuit Breaker | Fail fast | External dependencies |
| Bulkhead | Isolate failures | Resource isolation |
| Timeout | Bound wait time | All external calls |
| Retry | Handle transient failures | Idempotent operations |

## Labs

1. **Lab 3: Rate Limiter** - Token bucket implementation
2. **Lab 4: Circuit Breaker** - Full state machine implementation
