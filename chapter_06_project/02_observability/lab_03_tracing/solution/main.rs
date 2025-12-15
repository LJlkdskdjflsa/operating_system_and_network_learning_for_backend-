//! Lab 3: Structured Logging with Tracing - Solution
//!
//! Production-grade structured logging using the tracing crate.

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
use tracing::{info, info_span, warn, Instrument, Level};
use tracing_subscriber::{fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt};
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
                warn!(error = %msg, "Returning 404 response");
                (StatusCode::NOT_FOUND, Json(json!({ "error": msg }))).into_response()
            }
        }
    }
}

// Logging middleware that wraps each request in a span
async fn logging_middleware(
    request: axum::extract::Request,
    next: Next,
) -> impl IntoResponse {
    let request_id = Uuid::new_v4().to_string();
    let method = request.method().to_string();
    let path = request.uri().path().to_string();

    // Create a span for this request
    let span = info_span!(
        "http_request",
        %request_id,
        %method,
        %path,
    );

    async move {
        let start = Instant::now();

        info!("Request started");

        // Process the request
        let response = next.run(request).await;

        let duration = start.elapsed();
        let status = response.status().as_u16();

        info!(
            status,
            duration_ms = %duration.as_millis(),
            "Request completed"
        );

        response
    }
    .instrument(span)
    .await
}

// Handler with automatic span via #[instrument]
#[tracing::instrument(skip(state), fields(item_id = %id))]
async fn get_item(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Item>, AppError> {
    info!("Looking up item in store");

    let items = state.read().await;

    match items.get(&id) {
        Some(item) => {
            info!(item_name = %item.name, "Item found");
            Ok(Json(item.clone()))
        }
        None => {
            warn!("Item not found in store");
            Err(AppError::NotFound(format!("Item {} not found", id)))
        }
    }
}

#[tracing::instrument(skip(state))]
async fn list_items(State(state): State<AppState>) -> Json<Vec<Item>> {
    let items = state.read().await;
    let items: Vec<Item> = items.values().cloned().collect();

    info!(count = items.len(), "Listing all items");

    Json(items)
}

// Health check endpoint
async fn health() -> &'static str {
    "OK"
}

// Initialize tracing subscriber with JSON output
fn init_tracing() {
    // Create a filter from RUST_LOG env var, defaulting to "info"
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));

    // Create the subscriber
    tracing_subscriber::registry()
        .with(filter)
        .with(
            tracing_subscriber::fmt::layer()
                .json()
                .with_span_events(FmtSpan::CLOSE)
                .with_current_span(true)
                .with_target(true)
                .flatten_event(true),
        )
        .init();
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
            Item {
                id: "1".to_string(),
                name: "Widget".to_string(),
            },
        );
        items.insert(
            "2".to_string(),
            Item {
                id: "2".to_string(),
                name: "Gadget".to_string(),
            },
        );
        items.insert(
            "3".to_string(),
            Item {
                id: "3".to_string(),
                name: "Doohickey".to_string(),
            },
        );
    }

    info!(
        items_count = 3,
        "Initialized application state"
    );

    // Build router with logging middleware
    let app = Router::new()
        .route("/health", get(health))
        .route("/items", get(list_items))
        .route("/items/:id", get(get_item))
        .layer(middleware::from_fn(logging_middleware))
        .with_state(state);

    let addr = "0.0.0.0:3000";
    info!(addr, "Server starting");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_items() {
        let state: AppState = Arc::new(RwLock::new(HashMap::new()));
        {
            let mut items = state.write().await;
            items.insert(
                "1".to_string(),
                Item {
                    id: "1".to_string(),
                    name: "Test".to_string(),
                },
            );
        }

        let result = list_items(State(state)).await;
        assert_eq!(result.0.len(), 1);
    }
}
