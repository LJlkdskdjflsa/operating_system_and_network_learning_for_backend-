# Observability: Logging, Tracing, and Metrics

## Section Goals

> Implement production-grade observability to understand what your service is doing.

After completing this section, you will be able to:

- Add structured logging with the `tracing` crate
- Create request spans for distributed tracing
- Export Prometheus metrics from your service
- Understand the three pillars of observability

---

## 1. The Three Pillars of Observability

### Why Observability?

In production, you can't attach a debugger. You need other ways to understand:
- What is the service doing right now?
- Why did that request fail?
- Why is the service slow?
- Where is time being spent?

The three pillars of observability address these questions:

| Pillar | Purpose | Example |
|--------|---------|---------|
| **Logs** | Record discrete events | "User 123 logged in" |
| **Metrics** | Aggregate numerical data | "99th percentile latency is 50ms" |
| **Traces** | Follow request flow | "Request spent 10ms in DB, 5ms in cache" |

### How They Work Together

```
Request arrives
    │
    ├─→ Log: "Received GET /items/123"
    │
    ├─→ Trace: Start span "handle_request"
    │       │
    │       ├─→ Child span: "database_query"
    │       │       └─→ Log: "Query took 15ms"
    │       │
    │       └─→ Child span: "serialize_response"
    │
    ├─→ Metrics: Increment request_count
    │           Record latency histogram
    │
    └─→ Log: "Completed 200 OK in 20ms"
```

---

## 2. Structured Logging with Tracing

### Why Structured Logging?

Traditional logging:
```
println!("User {} logged in from {}", user_id, ip);
// Output: "User 123 logged in from 192.168.1.1"
```

Problems:
- Hard to parse programmatically
- Can't filter by user_id
- Can't aggregate by IP

Structured logging:
```rust
tracing::info!(user_id = 123, ip = "192.168.1.1", "User logged in");
// Output: {"level":"INFO","user_id":123,"ip":"192.168.1.1","message":"User logged in"}
```

Benefits:
- Machine parseable (JSON)
- Filterable and searchable
- Aggregatable

### The Tracing Crate

```rust
use tracing::{info, warn, error, debug, trace, instrument, span, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Initialize tracing subscriber
fn init_tracing() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();
}

// Basic logging with fields
fn log_examples() {
    // Different log levels
    trace!("Very detailed info");
    debug!("Debug information");
    info!("Normal operation");
    warn!("Something unexpected");
    error!("Something failed");

    // Structured fields
    info!(
        user_id = 123,
        action = "login",
        success = true,
        "User action completed"
    );

    // Dynamic fields
    let user = "alice";
    let duration_ms = 42;
    info!(user, duration_ms, "Request processed");
}
```

### Spans for Request Tracing

```rust
use tracing::{instrument, span, Level, Instrument};

// Automatic span via attribute
#[instrument(skip(pool), fields(user_id))]
async fn get_user(pool: &Pool, user_id: i64) -> Result<User, Error> {
    // This function is automatically wrapped in a span
    // named "get_user" with user_id field

    let user = query_user(pool, user_id).await?;
    tracing::Span::current().record("user_id", user_id);
    Ok(user)
}

// Manual span creation
async fn process_request(req: Request) -> Response {
    let span = span!(Level::INFO, "process_request",
        method = %req.method(),
        path = %req.uri().path(),
    );

    async {
        // All logs here are associated with this span
        info!("Starting request processing");

        let result = handle(req).await;

        info!(status = %result.status(), "Request completed");
        result
    }
    .instrument(span)
    .await
}
```

### Request ID Propagation

```rust
use uuid::Uuid;
use tracing::info_span;

async fn request_middleware(req: Request, next: Next) -> Response {
    let request_id = Uuid::new_v4().to_string();

    let span = info_span!(
        "request",
        request_id = %request_id,
        method = %req.method(),
        path = %req.uri().path(),
    );

    async move {
        let start = std::time::Instant::now();
        let response = next.run(req).await;
        let duration = start.elapsed();

        info!(
            status = %response.status(),
            duration_ms = %duration.as_millis(),
            "Request completed"
        );

        response
    }
    .instrument(span)
    .await
}
```

### JSON Output

```rust
use tracing_subscriber::fmt::format::FmtSpan;

fn init_json_logging() {
    tracing_subscriber::fmt()
        .json()
        .with_span_events(FmtSpan::CLOSE)
        .with_current_span(true)
        .init();
}

// Output example:
// {"timestamp":"2024-01-15T10:30:00Z","level":"INFO","fields":{"message":"Request completed","status":200,"duration_ms":42},"target":"my_app","span":{"request_id":"abc-123","method":"GET","path":"/items"}}
```

---

## 3. Prometheus Metrics

### Why Prometheus?

Prometheus is the standard for metrics in cloud-native applications:
- Pull-based (Prometheus scrapes your `/metrics` endpoint)
- Powerful query language (PromQL)
- Integrates with Grafana for dashboards
- Supports alerting

### Metric Types

```rust
use prometheus::{Counter, Histogram, Gauge, IntCounter, IntGauge};

// Counter: Only goes up (requests, errors, bytes sent)
let requests_total = Counter::new("http_requests_total", "Total HTTP requests")?;
requests_total.inc();

// Gauge: Can go up or down (connections, queue size, temperature)
let active_connections = Gauge::new("active_connections", "Current connections")?;
active_connections.inc();
active_connections.dec();
active_connections.set(42.0);

// Histogram: Distribution of values (latency, request size)
let request_duration = Histogram::with_opts(
    HistogramOpts::new("http_request_duration_seconds", "Request latency")
        .buckets(vec![0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1.0])
)?;
request_duration.observe(0.042); // 42ms
```

### Labels for Dimensions

```rust
use prometheus::{CounterVec, HistogramVec, opts, histogram_opts};

// Counter with labels
let requests = CounterVec::new(
    opts!("http_requests_total", "Total HTTP requests"),
    &["method", "path", "status"]
)?;

// Increment with specific labels
requests.with_label_values(&["GET", "/items", "200"]).inc();
requests.with_label_values(&["POST", "/items", "201"]).inc();

// Histogram with labels
let duration = HistogramVec::new(
    histogram_opts!(
        "http_request_duration_seconds",
        "Request latency",
        vec![0.001, 0.01, 0.1, 1.0]
    ),
    &["method", "path"]
)?;

duration.with_label_values(&["GET", "/items"]).observe(0.05);
```

### Exposing Metrics Endpoint

```rust
use axum::{routing::get, Router};
use prometheus::{Encoder, TextEncoder, Registry};

async fn metrics_handler(
    State(registry): State<Registry>
) -> String {
    let encoder = TextEncoder::new();
    let metric_families = registry.gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}

// In your router
let registry = Registry::new();
// ... register metrics ...

let app = Router::new()
    .route("/metrics", get(metrics_handler))
    .with_state(registry);
```

### Metrics Output Format

```
# HELP http_requests_total Total HTTP requests
# TYPE http_requests_total counter
http_requests_total{method="GET",path="/items",status="200"} 142
http_requests_total{method="POST",path="/items",status="201"} 23

# HELP http_request_duration_seconds Request latency
# TYPE http_request_duration_seconds histogram
http_request_duration_seconds_bucket{method="GET",path="/items",le="0.001"} 50
http_request_duration_seconds_bucket{method="GET",path="/items",le="0.01"} 120
http_request_duration_seconds_bucket{method="GET",path="/items",le="0.1"} 140
http_request_duration_seconds_bucket{method="GET",path="/items",le="+Inf"} 142
http_request_duration_seconds_sum{method="GET",path="/items"} 1.234
http_request_duration_seconds_count{method="GET",path="/items"} 142
```

---

## 4. Middleware Pattern

### Combining Logging and Metrics

```rust
use axum::{
    middleware::{self, Next},
    extract::Request,
    response::Response,
};
use std::time::Instant;

async fn observability_middleware(
    State(metrics): State<Metrics>,
    request: Request,
    next: Next,
) -> Response {
    let method = request.method().to_string();
    let path = request.uri().path().to_string();
    let request_id = uuid::Uuid::new_v4().to_string();

    let span = tracing::info_span!(
        "http_request",
        %request_id,
        %method,
        %path,
    );

    async move {
        let start = Instant::now();

        tracing::info!("Request started");

        let response = next.run(request).await;

        let duration = start.elapsed();
        let status = response.status().as_u16().to_string();

        // Record metrics
        metrics.requests_total
            .with_label_values(&[&method, &path, &status])
            .inc();

        metrics.request_duration
            .with_label_values(&[&method, &path])
            .observe(duration.as_secs_f64());

        tracing::info!(
            %status,
            duration_ms = %duration.as_millis(),
            "Request completed"
        );

        response
    }
    .instrument(span)
    .await
}
```

---

## 5. What to Measure

### The RED Method (for services)

| Metric | What it measures |
|--------|-----------------|
| **R**ate | Requests per second |
| **E**rrors | Failed requests per second |
| **D**uration | Time per request |

### The USE Method (for resources)

| Metric | What it measures |
|--------|-----------------|
| **U**tilization | % time resource is busy |
| **S**aturation | Queue depth |
| **E**rrors | Error count |

### Essential HTTP Service Metrics

```rust
// Request rate and errors
http_requests_total{method, path, status}

// Latency distribution
http_request_duration_seconds{method, path}

// Active requests (concurrency)
http_requests_in_flight

// Response size
http_response_size_bytes{method, path}
```

### Database Metrics

```rust
// Query performance
db_query_duration_seconds{query_type}

// Connection pool
db_connections_active
db_connections_idle
db_connection_wait_seconds

// Errors
db_errors_total{error_type}
```

---

## Summary

Observability gives you visibility into your running service:

1. **Logs**: Record what happened
   - Use structured logging (tracing crate)
   - Include request IDs for correlation
   - Use appropriate log levels

2. **Metrics**: Track aggregates
   - Use Prometheus format
   - Track RED metrics (Rate, Errors, Duration)
   - Use labels for dimensions

3. **Traces**: Follow request flow
   - Use spans to mark operations
   - Propagate context across services
   - Record timing information

Key principles:
- Instrument early, not after problems occur
- Use consistent naming conventions
- Don't log sensitive data
- Balance detail vs. overhead

---

## Next Steps

1. **Lab 3**: Add structured logging to your REST API
2. **Lab 4**: Export Prometheus metrics
