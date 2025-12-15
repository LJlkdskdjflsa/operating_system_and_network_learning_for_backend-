# Chapter 6 Checkpoint

## Self-Assessment Questions

### REST Service
- [ ] Can you explain the Axum router and handler pattern?
- [ ] How do you extract path parameters, query parameters, and JSON bodies?
- [ ] What is the difference between `Json<T>` and `axum::response::IntoResponse`?
- [ ] How do you handle errors gracefully in an Axum application?

### Database Integration
- [ ] How does SQLx provide compile-time query checking?
- [ ] What is the purpose of connection pooling in a web service?
- [ ] How do you handle database migrations?
- [ ] What are the tradeoffs between SQLite and PostgreSQL for production?

### Observability
- [ ] What is structured logging and why is it important?
- [ ] How do spans help in distributed tracing?
- [ ] What metrics should every HTTP service expose?
- [ ] What is the difference between a counter and a histogram?

### Performance
- [ ] How do you measure p95 and p99 latency?
- [ ] What causes high tail latency in async services?
- [ ] How does the Tokio runtime affect performance?
- [ ] What system resources should you monitor during load testing?

---

## Implementation Verification

### Lab 1: Axum CRUD API
- [ ] POST /items creates a new item
- [ ] GET /items/:id returns an item
- [ ] GET /items returns paginated list
- [ ] PUT /items/:id updates an item
- [ ] DELETE /items/:id removes an item
- [ ] Proper error responses for invalid requests

### Lab 2: Database Integration
- [ ] Items persist across server restarts
- [ ] Connection pool is properly configured
- [ ] Queries use parameterized statements
- [ ] Errors are handled gracefully

### Lab 3: Structured Logging
- [ ] Each request has a unique request ID
- [ ] Logs include method, path, status, and duration
- [ ] Log levels are used appropriately
- [ ] JSON format output works correctly

### Lab 4: Prometheus Metrics
- [ ] /metrics endpoint returns Prometheus format
- [ ] Request counter tracks total requests
- [ ] Histogram tracks request duration
- [ ] Labels include method, path, and status

### Lab 5: Load Testing
- [ ] Can generate concurrent load
- [ ] Measures throughput (requests/sec)
- [ ] Measures latency percentiles
- [ ] Identifies bottlenecks under load

---

## Concept Connection Quiz

1. **Why does Axum use extractors instead of parsing requests manually?**

   Answer: Extractors provide type-safe, composable request parsing with automatic error handling. They leverage Rust's type system to ensure data is validated before reaching handler code, reducing boilerplate and bugs.

2. **How does structured logging differ from printf-style logging?**

   Answer: Structured logging captures data as key-value pairs that can be queried and analyzed programmatically. Printf-style logs are just strings that require parsing. Structured logs enable filtering, aggregation, and correlation across distributed systems.

3. **Why are histograms preferred over averages for latency metrics?**

   Answer: Averages hide distribution problems - a few slow requests can indicate issues even with a good average. Histograms capture the full distribution, letting you see p50, p95, p99 latencies and identify tail latency problems that affect user experience.

4. **What is the relationship between connection pool size and concurrent requests?**

   Answer: The pool size limits how many concurrent database operations can occur. Too small = requests wait for connections (higher latency). Too large = database overwhelmed, context switching overhead. Optimal size depends on database and workload characteristics.

5. **How does observability help with performance tuning?**

   Answer: Observability provides data to identify bottlenecks: logs show errors and slow operations, metrics reveal trends and capacity limits, traces show where time is spent in request processing. Without observability, optimization is guesswork.

---

## Final Project Checklist

- [ ] REST API with all CRUD operations
- [ ] SQLite database with migrations
- [ ] Structured logging with tracing
- [ ] Prometheus metrics endpoint
- [ ] Load test results documented
- [ ] Performance analysis written

## Extension Ideas

1. Add authentication (JWT or API keys)
2. Implement rate limiting
3. Add caching layer (from Chapter 4)
4. Deploy with Docker
5. Set up Grafana dashboard for metrics
