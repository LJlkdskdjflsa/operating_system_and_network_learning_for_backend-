//! Lab 4: Prometheus Metrics - Solution
//!
//! Export HTTP metrics in Prometheus format.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    middleware::{self, Next},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use prometheus::{
    Encoder, HistogramOpts, HistogramVec, Opts, Registry, TextEncoder,
    CounterVec,
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

        // Request counter with method, path, status labels
        let requests_total = CounterVec::new(
            Opts::new("http_requests_total", "Total HTTP requests"),
            &["method", "path", "status"],
        )
        .unwrap();
        registry
            .register(Box::new(requests_total.clone()))
            .unwrap();

        // Request duration histogram with method, path labels
        let request_duration = HistogramVec::new(
            HistogramOpts::new(
                "http_request_duration_seconds",
                "HTTP request latency in seconds",
            )
            .buckets(vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5]),
            &["method", "path"],
        )
        .unwrap();
        registry
            .register(Box::new(request_duration.clone()))
            .unwrap();

        Metrics {
            requests_total,
            request_duration,
            registry,
        }
    }
}

// Shared state type
type AppState = Arc<Metrics>;

// Metrics middleware - records metrics for every request
async fn metrics_middleware(
    State(metrics): State<AppState>,
    request: axum::extract::Request,
    next: Next,
) -> impl IntoResponse {
    let start = Instant::now();
    let method = request.method().to_string();
    let path = normalize_path(request.uri().path());

    let response = next.run(request).await;

    let duration = start.elapsed().as_secs_f64();
    let status = response.status().as_u16().to_string();

    // Record latency
    metrics
        .request_duration
        .with_label_values(&[&method, &path])
        .observe(duration);

    // Increment request counter
    metrics
        .requests_total
        .with_label_values(&[&method, &path, &status])
        .inc();

    response
}

// Normalize path to avoid high cardinality from path parameters
fn normalize_path(path: &str) -> String {
    // Replace numeric IDs with :id placeholder
    let parts: Vec<&str> = path.split('/').collect();
    let normalized: Vec<String> = parts
        .iter()
        .map(|part| {
            if part.parse::<u64>().is_ok() {
                ":id".to_string()
            } else {
                part.to_string()
            }
        })
        .collect();
    normalized.join("/")
}

// Metrics endpoint - returns Prometheus text format
async fn metrics_handler(State(metrics): State<AppState>) -> impl IntoResponse {
    let encoder = TextEncoder::new();
    let metric_families = metrics.registry.gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    (
        [(
            axum::http::header::CONTENT_TYPE,
            "text/plain; version=0.0.4; charset=utf-8",
        )],
        String::from_utf8(buffer).unwrap(),
    )
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
        Item {
            id: 1,
            name: "Widget".to_string(),
        },
        Item {
            id: 2,
            name: "Gadget".to_string(),
        },
        Item {
            id: 3,
            name: "Doohickey".to_string(),
        },
    ])
}

async fn get_item(Path(id): Path<u32>) -> impl IntoResponse {
    // Simulate varying latency based on ID
    let delay = if id % 2 == 0 { 50 } else { 5 };
    tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;

    if id > 100 {
        return (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Not found"})),
        )
            .into_response();
    }

    Json(Item {
        id,
        name: format!("Item {}", id),
    })
    .into_response()
}

async fn health() -> &'static str {
    "OK"
}

#[tokio::main]
async fn main() {
    let metrics = Arc::new(Metrics::new());

    // Note: /metrics route is added before the middleware layer
    // so it doesn't record its own metrics (avoiding recursion)
    let app = Router::new()
        .route("/metrics", get(metrics_handler))
        .route("/health", get(health))
        .route("/items", get(list_items))
        .route("/items/:id", get(get_item))
        .layer(middleware::from_fn_with_state(
            metrics.clone(),
            metrics_middleware,
        ))
        .with_state(metrics);

    println!("Server running on http://localhost:3000");
    println!();
    println!("Try these commands:");
    println!("  curl http://localhost:3000/items");
    println!("  curl http://localhost:3000/items/1");
    println!("  curl http://localhost:3000/items/999  # 404");
    println!();
    println!("Then check metrics:");
    println!("  curl http://localhost:3000/metrics");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_path() {
        assert_eq!(normalize_path("/items"), "/items");
        assert_eq!(normalize_path("/items/123"), "/items/:id");
        assert_eq!(normalize_path("/users/456/orders/789"), "/users/:id/orders/:id");
    }

    #[test]
    fn test_metrics_creation() {
        let metrics = Metrics::new();

        // Should be able to increment counters
        metrics
            .requests_total
            .with_label_values(&["GET", "/items", "200"])
            .inc();

        // Should be able to observe histogram
        metrics
            .request_duration
            .with_label_values(&["GET", "/items"])
            .observe(0.042);
    }
}
