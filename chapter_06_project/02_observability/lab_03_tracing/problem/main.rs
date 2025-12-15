//! Lab 3: Structured Logging with Tracing
//!
//! ## Goal
//! Add production-grade structured logging to a REST API using the tracing crate.
//!
//! ## Requirements
//! 1. Initialize tracing subscriber with JSON output
//! 2. Generate unique request ID for each request
//! 3. Create request span with method, path, and request_id
//! 4. Log request start and completion with duration
//! 5. Use appropriate log levels (info, warn, error)
//!
//! ## Output Format
//! Each log line should be JSON with these fields:
//! - timestamp
//! - level (INFO, WARN, ERROR)
//! - request_id
//! - method
//! - path
//! - message
//! - additional context fields
//!
//! ## Hints
//! - Use `tracing_subscriber::fmt().json()` for JSON output
//! - Use `tracing::info_span!` to create spans
//! - Use `.instrument(span)` to attach spans to futures
//! - Use `Instant::now()` for timing
//!
//! ## Verification
//! ```bash
//! cargo run
//! curl http://localhost:3000/items
//! # Check server output for structured JSON logs
//! ```
//!
//! ## Acceptance Criteria
//! - [ ] Logs are JSON formatted
//! - [ ] Each request has a unique ID
//! - [ ] Request start and end are logged
//! - [ ] Duration is recorded in milliseconds
//! - [ ] Errors are logged at appropriate level
//!
//! Check solution/main.rs after completing

use axum::{
    extract::{Path, State},
    http::StatusCode,
    middleware::{self, Next},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;
use tracing::{info, warn, error, instrument, Instrument};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
struct Item {
    id: String,
    name: String,
}

type AppState = Arc<RwLock<HashMap<String, Item>>>;

enum AppError {
    NotFound(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::NotFound(msg) => {
                // TODO: Log the error at warn level
                (StatusCode::NOT_FOUND, Json(json!({ "error": msg }))).into_response()
            }
        }
    }
}

// TODO: Implement logging middleware
//
// This middleware should:
// 1. Generate a unique request ID (UUID)
// 2. Create a span with request_id, method, and path
// 3. Log "Request started" at info level
// 4. Record the start time
// 5. Call next.run(request) within the span
// 6. Log "Request completed" with status and duration_ms
async fn logging_middleware(
    request: axum::extract::Request,
    next: Next,
) -> impl IntoResponse {
    // TODO: Implement
    //
    // Steps:
    // 1. let request_id = Uuid::new_v4().to_string();
    // 2. let method = request.method().to_string();
    // 3. let path = request.uri().path().to_string();
    // 4. Create span with tracing::info_span!
    // 5. Use async {}.instrument(span).await pattern

    todo!()
}

// Handler with automatic span via #[instrument]
#[instrument(skip(state), fields(item_id = %id))]
async fn get_item(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Item>, AppError> {
    // TODO: Add logging for database lookup
    info!("Looking up item");

    let items = state.read().await;
    items
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or_else(|| {
            warn!(item_id = %id, "Item not found");
            AppError::NotFound(format!("Item {} not found", id))
        })
}

async fn list_items(State(state): State<AppState>) -> Json<Vec<Item>> {
    // TODO: Add span and logging
    let items = state.read().await;
    let items: Vec<Item> = items.values().cloned().collect();
    info!(count = items.len(), "Listing items");
    Json(items)
}

// TODO: Initialize tracing subscriber
//
// Should configure:
// 1. JSON format output
// 2. Include timestamps
// 3. Include span information
// 4. Filter by RUST_LOG env var (default to "info")
fn init_tracing() {
    // TODO: Implement
    //
    // Use tracing_subscriber::fmt()
    //     .json()
    //     .with_...()
    //     .init()

    todo!()
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    init_tracing();

    // Create state with some test data
    let state: AppState = Arc::new(RwLock::new(HashMap::new()));
    {
        let mut items = state.write().await;
        items.insert(
            "1".to_string(),
            Item { id: "1".to_string(), name: "Widget".to_string() },
        );
        items.insert(
            "2".to_string(),
            Item { id: "2".to_string(), name: "Gadget".to_string() },
        );
    }

    // Build router with logging middleware
    let app = Router::new()
        .route("/items", get(list_items))
        .route("/items/:id", get(get_item))
        .layer(middleware::from_fn(logging_middleware))
        .with_state(state);

    info!("Server starting on http://localhost:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
