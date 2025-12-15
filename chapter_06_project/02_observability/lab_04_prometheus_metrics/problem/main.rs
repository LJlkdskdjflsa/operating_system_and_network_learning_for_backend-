//! Lab 4: Prometheus Metrics
//!
//! ## Goal
//! Export HTTP metrics in Prometheus format from a REST API.
//!
//! ## Requirements
//! 1. Create metrics: request counter and latency histogram
//! 2. Add labels for method, path, and status
//! 3. Record metrics in middleware for every request
//! 4. Expose /metrics endpoint in Prometheus text format
//!
//! ## Metrics to Implement
//! - `http_requests_total` (Counter): Total requests with labels
//! - `http_request_duration_seconds` (Histogram): Request latency
//!
//! ## Hints
//! - Use `prometheus::{Counter, CounterVec, Histogram, HistogramVec}`
//! - Use `lazy_static!` to create global metrics
//! - Use `prometheus::TextEncoder` to format output
//! - Labels: method, path, status
//!
//! ## Verification
//! ```bash
//! cargo run
//! # Make some requests
//! curl http://localhost:3000/items
//! curl http://localhost:3000/items/1
//!
//! # Check metrics
//! curl http://localhost:3000/metrics
//! ```
//!
//! ## Expected Output
//! ```
//! # HELP http_requests_total Total HTTP requests
//! # TYPE http_requests_total counter
//! http_requests_total{method="GET",path="/items",status="200"} 5
//!
//! # HELP http_request_duration_seconds HTTP request latency
//! # TYPE http_request_duration_seconds histogram
//! http_request_duration_seconds_bucket{method="GET",path="/items",le="0.001"} 3
//! ...
//! ```
//!
//! ## Acceptance Criteria
//! - [ ] /metrics endpoint returns Prometheus format
//! - [ ] Request counter increments correctly
//! - [ ] Histogram records latency distribution
//! - [ ] Labels are correctly applied
//!
//! Check solution/main.rs after completing

use axum::{
    extract::State,
    http::StatusCode,
    middleware::{self, Next},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use prometheus::{
    Counter, CounterVec, Encoder, Histogram, HistogramOpts, HistogramVec,
    Opts, Registry, TextEncoder,
};
use serde::Serialize;
use std::sync::Arc;
use std::time::Instant;

// Metrics struct to hold all our metrics
struct Metrics {
    requests_total: CounterVec,
    request_duration: HistogramVec,
    registry: Registry,
}

impl Metrics {
    fn new() -> Self {
        let registry = Registry::new();

        // TODO: Create request counter with labels [method, path, status]
        //
        // let requests_total = CounterVec::new(
        //     Opts::new("http_requests_total", "Total HTTP requests"),
        //     &["method", "path", "status"]
        // ).unwrap();
        // registry.register(Box::new(requests_total.clone())).unwrap();

        // TODO: Create latency histogram with labels [method, path]
        //
        // let request_duration = HistogramVec::new(
        //     HistogramOpts::new("http_request_duration_seconds", "HTTP request latency")
        //         .buckets(vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0]),
        //     &["method", "path"]
        // ).unwrap();
        // registry.register(Box::new(request_duration.clone())).unwrap();

        todo!()
    }
}

// Shared state type
type AppState = Arc<Metrics>;

// TODO: Implement metrics middleware
//
// This middleware should:
// 1. Record the start time
// 2. Extract method and path from request
// 3. Call next.run(request)
// 4. Record duration in histogram
// 5. Increment request counter with labels
async fn metrics_middleware(
    State(metrics): State<AppState>,
    request: axum::extract::Request,
    next: Next,
) -> impl IntoResponse {
    // TODO: Implement
    //
    // let start = Instant::now();
    // let method = request.method().to_string();
    // let path = request.uri().path().to_string();
    //
    // let response = next.run(request).await;
    //
    // let duration = start.elapsed().as_secs_f64();
    // let status = response.status().as_u16().to_string();
    //
    // metrics.request_duration
    //     .with_label_values(&[&method, &path])
    //     .observe(duration);
    //
    // metrics.requests_total
    //     .with_label_values(&[&method, &path, &status])
    //     .inc();
    //
    // response

    todo!()
}

// TODO: Implement metrics endpoint
//
// Should return Prometheus text format
async fn metrics_handler(State(metrics): State<AppState>) -> impl IntoResponse {
    // TODO: Implement
    //
    // let encoder = TextEncoder::new();
    // let metric_families = metrics.registry.gather();
    // let mut buffer = Vec::new();
    // encoder.encode(&metric_families, &mut buffer).unwrap();
    //
    // (
    //     [(axum::http::header::CONTENT_TYPE, "text/plain; charset=utf-8")],
    //     String::from_utf8(buffer).unwrap()
    // )

    todo!()
}

// Sample API endpoints
#[derive(Serialize)]
struct Item {
    id: u32,
    name: String,
}

async fn list_items() -> Json<Vec<Item>> {
    // Simulate some work
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

    Json(vec![
        Item { id: 1, name: "Widget".to_string() },
        Item { id: 2, name: "Gadget".to_string() },
    ])
}

async fn get_item(axum::extract::Path(id): axum::extract::Path<u32>) -> impl IntoResponse {
    // Simulate varying latency
    let delay = if id % 2 == 0 { 50 } else { 5 };
    tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;

    if id > 100 {
        return (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Not found"}))).into_response();
    }

    Json(Item { id, name: format!("Item {}", id) }).into_response()
}

async fn health() -> &'static str {
    "OK"
}

#[tokio::main]
async fn main() {
    // TODO: Create metrics and set up router
    //
    // let metrics = Arc::new(Metrics::new());
    //
    // let app = Router::new()
    //     .route("/health", get(health))
    //     .route("/items", get(list_items))
    //     .route("/items/:id", get(get_item))
    //     .route("/metrics", get(metrics_handler))
    //     .layer(middleware::from_fn_with_state(metrics.clone(), metrics_middleware))
    //     .with_state(metrics);

    println!("Server running on http://localhost:3000");
    println!("Metrics available at http://localhost:3000/metrics");

    todo!()
}
