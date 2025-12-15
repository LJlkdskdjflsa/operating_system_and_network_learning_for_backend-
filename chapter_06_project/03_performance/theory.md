# Performance Testing and Analysis

## Section Goals

> Learn to measure, analyze, and improve service performance under load.

After completing this section, you will be able to:

- Use load testing tools to stress your service
- Measure throughput, latency, and error rates
- Identify performance bottlenecks
- Apply optimization strategies

---

## 1. Why Performance Testing?

### Production Reality

Your service will face:
- Traffic spikes (product launches, viral content)
- Gradual growth
- Slow clients
- Network issues

Without testing, you discover problems when users complain.

### Key Questions

1. **Capacity**: How many requests/second can we handle?
2. **Latency**: How fast are responses under load?
3. **Reliability**: When do errors start occurring?
4. **Resources**: How does CPU/memory scale with load?

---

## 2. Performance Metrics

### Throughput

Requests per second (RPS) the service can handle:

```
Throughput = Successful Requests / Time
```

### Latency Percentiles

Average latency hides problems. Use percentiles:

| Percentile | Meaning |
|------------|---------|
| p50 (median) | Half of requests are faster |
| p95 | 95% of requests are faster |
| p99 | 99% of requests are faster |
| p99.9 | 99.9% of requests are faster |

Example:
```
p50 = 10ms   (typical request)
p95 = 50ms   (slower requests)
p99 = 200ms  (tail latency)
```

If p99 is 20x higher than p50, you have tail latency issues.

### Error Rate

```
Error Rate = Failed Requests / Total Requests * 100%
```

Typically aim for < 0.1% errors.

---

## 3. Load Testing Tools

### wrk - HTTP Benchmarking Tool

```bash
# Basic usage: 10 threads, 100 connections, 30 seconds
wrk -t10 -c100 -d30s http://localhost:3000/items

# Output:
# Running 30s test @ http://localhost:3000/items
#   10 threads and 100 connections
#   Thread Stats   Avg      Stdev     Max   +/- Stdev
#     Latency    10.23ms    5.67ms  89.12ms   74.32%
#     Req/Sec     1.02k   123.45     1.56k    68.00%
#   305421 requests in 30.10s, 52.34MB read
# Requests/sec:  10146.54
# Transfer/sec:      1.74MB
```

### hey - HTTP Load Generator

```bash
# 10000 requests, 100 concurrent
hey -n 10000 -c 100 http://localhost:3000/items

# Output includes latency distribution:
# Latency distribution:
#   10% in 0.0045 secs
#   25% in 0.0062 secs
#   50% in 0.0089 secs
#   75% in 0.0123 secs
#   90% in 0.0178 secs
#   95% in 0.0234 secs
#   99% in 0.0456 secs
```

### Apache Bench (ab)

```bash
# 10000 requests, 100 concurrent
ab -n 10000 -c 100 http://localhost:3000/items

# Shows percentile breakdown
```

### Custom Load Generators

For complex scenarios, write your own:

```rust
use tokio::time::{Duration, Instant};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

struct LoadTestResult {
    total_requests: u64,
    successful: u64,
    failed: u64,
    duration: Duration,
    latencies: Vec<Duration>,
}

async fn run_load_test(
    url: &str,
    concurrent: usize,
    duration: Duration,
) -> LoadTestResult {
    let client = reqwest::Client::new();
    let success = Arc::new(AtomicU64::new(0));
    let failed = Arc::new(AtomicU64::new(0));
    let latencies = Arc::new(tokio::sync::Mutex::new(Vec::new()));

    let start = Instant::now();
    let mut handles = vec![];

    for _ in 0..concurrent {
        let client = client.clone();
        let url = url.to_string();
        let success = success.clone();
        let failed = failed.clone();
        let latencies = latencies.clone();

        let handle = tokio::spawn(async move {
            while start.elapsed() < duration {
                let req_start = Instant::now();
                let result = client.get(&url).send().await;
                let latency = req_start.elapsed();

                match result {
                    Ok(resp) if resp.status().is_success() => {
                        success.fetch_add(1, Ordering::Relaxed);
                    }
                    _ => {
                        failed.fetch_add(1, Ordering::Relaxed);
                    }
                }

                latencies.lock().await.push(latency);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }

    let elapsed = start.elapsed();
    let latencies = latencies.lock().await.clone();

    LoadTestResult {
        total_requests: success.load(Ordering::Relaxed) + failed.load(Ordering::Relaxed),
        successful: success.load(Ordering::Relaxed),
        failed: failed.load(Ordering::Relaxed),
        duration: elapsed,
        latencies,
    }
}
```

---

## 4. Analyzing Results

### Calculating Percentiles

```rust
fn percentile(sorted_latencies: &[Duration], p: f64) -> Duration {
    let index = ((sorted_latencies.len() as f64) * p / 100.0) as usize;
    sorted_latencies[index.min(sorted_latencies.len() - 1)]
}

fn analyze_latencies(mut latencies: Vec<Duration>) {
    latencies.sort();

    println!("Latency Statistics:");
    println!("  Min:  {:?}", latencies.first().unwrap());
    println!("  p50:  {:?}", percentile(&latencies, 50.0));
    println!("  p95:  {:?}", percentile(&latencies, 95.0));
    println!("  p99:  {:?}", percentile(&latencies, 99.0));
    println!("  Max:  {:?}", latencies.last().unwrap());

    let sum: Duration = latencies.iter().sum();
    let avg = sum / latencies.len() as u32;
    println!("  Avg:  {:?}", avg);
}
```

### Identifying Bottlenecks

| Symptom | Possible Cause |
|---------|---------------|
| Low CPU, high latency | I/O bound (DB, network) |
| High CPU, high latency | CPU bound (computation) |
| Latency increases with load | Queuing, lock contention |
| Sudden latency spikes | GC, connection pool exhaustion |
| High p99 vs p50 | Tail latency (slow queries, retries) |

---

## 5. Common Bottlenecks

### Connection Pool Exhaustion

```
Symptom: Latency spikes when connections run out
Solution: Increase pool size or reduce query time

// Check pool configuration
let pool = SqlitePoolOptions::new()
    .max_connections(20)        // Increase if needed
    .acquire_timeout(Duration::from_secs(5))
    .connect(url).await?;
```

### Lock Contention

```
Symptom: Latency increases with concurrency
Solution: Use RwLock, reduce critical sections

// Bad: Global mutex
let data = Arc::new(Mutex::new(vec![]));

// Better: Read-write lock
let data = Arc::new(RwLock::new(vec![]));

// Best: Lock-free data structures or sharding
```

### Async Task Starvation

```
Symptom: Some requests take very long
Solution: Increase worker threads, avoid blocking

// Configure Tokio runtime
#[tokio::main(worker_threads = 4)]
async fn main() { }

// Don't block async tasks
// Bad:
std::thread::sleep(Duration::from_secs(1));

// Good:
tokio::time::sleep(Duration::from_secs(1)).await;
```

### Memory Allocation

```
Symptom: Latency varies, high memory usage
Solution: Reuse allocations, use arenas

// Bad: Allocate per request
async fn handle() -> Vec<u8> {
    let mut buffer = Vec::new();  // Allocation
    // ...
}

// Better: Reuse buffers
async fn handle(buffer: &mut Vec<u8>) {
    buffer.clear();  // Reuse existing allocation
    // ...
}
```

---

## 6. Optimization Strategies

### 1. Profile First

Don't guess - measure:

```bash
# CPU profiling
cargo flamegraph --bin my_service

# Memory profiling
cargo run --release &
heaptrack -p $(pgrep my_service)
```

### 2. Optimize Hot Paths

Focus on:
- Request handling code
- Database queries
- Serialization/deserialization

### 3. Reduce Allocations

```rust
// Use &str instead of String where possible
// Use Cow<str> for flexibility
// Pre-allocate vectors with known capacity
let mut vec = Vec::with_capacity(expected_size);
```

### 4. Cache Aggressively

```rust
// In-memory cache for frequently accessed data
use std::collections::HashMap;
use tokio::sync::RwLock;

struct Cache {
    data: RwLock<HashMap<String, CachedItem>>,
}

impl Cache {
    async fn get_or_fetch<F, Fut>(&self, key: &str, fetch: F) -> Item
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Item>,
    {
        // Check cache first
        if let Some(item) = self.data.read().await.get(key) {
            if !item.is_expired() {
                return item.value.clone();
            }
        }

        // Fetch and cache
        let value = fetch().await;
        self.data.write().await.insert(
            key.to_string(),
            CachedItem::new(value.clone()),
        );
        value
    }
}
```

### 5. Batch Operations

```rust
// Bad: One query per item
for id in ids {
    let item = db.get_item(id).await?;
}

// Good: Batch query
let items = db.get_items(&ids).await?;
```

---

## 7. Load Testing Methodology

### 1. Establish Baseline

Test with minimal load to get baseline metrics:
```bash
hey -n 100 -c 1 http://localhost:3000/items
```

### 2. Find Saturation Point

Gradually increase load until errors or latency degrades:
```bash
# Start low
hey -n 1000 -c 10 http://localhost:3000/items

# Increase
hey -n 1000 -c 50 http://localhost:3000/items

# Keep increasing
hey -n 1000 -c 100 http://localhost:3000/items
hey -n 1000 -c 200 http://localhost:3000/items
```

### 3. Sustained Load Test

Run for extended period at expected load:
```bash
wrk -t4 -c50 -d300s http://localhost:3000/items
```

### 4. Spike Test

Test sudden traffic increase:
```bash
# Normal load for 60s, then spike
wrk -t4 -c50 -d60s http://localhost:3000/items
wrk -t4 -c500 -d60s http://localhost:3000/items  # 10x spike
```

---

## 8. Monitoring During Tests

While running load tests, monitor:

```bash
# Terminal 1: Load test
wrk -t4 -c100 -d60s http://localhost:3000/items

# Terminal 2: CPU/Memory
htop

# Terminal 3: Network connections
watch -n1 'ss -s'

# Terminal 4: Application metrics
watch -n5 'curl -s localhost:3000/metrics | grep http_requests'
```

---

## Summary

Performance testing workflow:

1. **Define goals**: Target RPS, latency percentiles, error rate
2. **Establish baseline**: Test at low load
3. **Load test**: Gradually increase to find limits
4. **Analyze**: Identify bottlenecks from metrics and profiles
5. **Optimize**: Fix bottlenecks, re-test
6. **Monitor**: Track performance in production

Key metrics to track:
- Throughput (RPS)
- Latency percentiles (p50, p95, p99)
- Error rate
- Resource utilization (CPU, memory, connections)

Common optimizations:
- Connection pool tuning
- Caching
- Query optimization
- Reducing allocations
- Async/await best practices

---

## Next Steps

1. **Lab 5**: Build a load tester and analyze your service
