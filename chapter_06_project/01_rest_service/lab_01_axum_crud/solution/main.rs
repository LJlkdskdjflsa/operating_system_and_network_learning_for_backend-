//! Lab 1: Axum CRUD API - Solution
//!
//! A complete REST API for managing items using Axum.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

// Item model
#[derive(Clone, Serialize, Deserialize)]
struct Item {
    id: Uuid,
    name: String,
    description: Option<String>,
    price: f64,
    created_at: String,
}

// Request bodies
#[derive(Deserialize)]
struct CreateItem {
    name: String,
    description: Option<String>,
    price: f64,
}

#[derive(Deserialize)]
struct UpdateItem {
    name: Option<String>,
    description: Option<String>,
    price: Option<f64>,
}

// Pagination query params
#[derive(Deserialize)]
struct Pagination {
    page: Option<usize>,
    limit: Option<usize>,
}

// Paginated response
#[derive(Serialize)]
struct PaginatedResponse<T> {
    items: Vec<T>,
    page: usize,
    limit: usize,
    total: usize,
}

// Shared state type
type AppState = Arc<RwLock<HashMap<Uuid, Item>>>;

// Error type for the API
enum AppError {
    NotFound(String),
    #[allow(dead_code)]
    BadRequest(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
        };

        let body = Json(json!({
            "error": message
        }));

        (status, body).into_response()
    }
}

// Handler: Create item
async fn create_item(
    State(state): State<AppState>,
    Json(payload): Json<CreateItem>,
) -> Result<impl IntoResponse, AppError> {
    let id = Uuid::new_v4();
    let now = chrono_lite_now();

    let item = Item {
        id,
        name: payload.name,
        description: payload.description,
        price: payload.price,
        created_at: now,
    };

    let mut items = state.write().await;
    items.insert(id, item.clone());

    Ok((StatusCode::CREATED, Json(item)))
}

// Handler: Get item by ID
async fn get_item(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Item>, AppError> {
    let items = state.read().await;

    items
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or_else(|| AppError::NotFound(format!("Item {} not found", id)))
}

// Handler: List all items with pagination
async fn list_items(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> Json<PaginatedResponse<Item>> {
    let page = pagination.page.unwrap_or(1).max(1);
    let limit = pagination.limit.unwrap_or(10).min(100);

    let items = state.read().await;
    let total = items.len();

    let skip = (page - 1) * limit;
    let items: Vec<Item> = items.values().skip(skip).take(limit).cloned().collect();

    Json(PaginatedResponse {
        items,
        page,
        limit,
        total,
    })
}

// Handler: Update item
async fn update_item(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateItem>,
) -> Result<Json<Item>, AppError> {
    let mut items = state.write().await;

    let item = items
        .get_mut(&id)
        .ok_or_else(|| AppError::NotFound(format!("Item {} not found", id)))?;

    // Update fields if provided
    if let Some(name) = payload.name {
        item.name = name;
    }
    if let Some(description) = payload.description {
        item.description = Some(description);
    }
    if let Some(price) = payload.price {
        item.price = price;
    }

    Ok(Json(item.clone()))
}

// Handler: Delete item
async fn delete_item(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let mut items = state.write().await;

    items
        .remove(&id)
        .map(|_| StatusCode::NO_CONTENT)
        .ok_or_else(|| AppError::NotFound(format!("Item {} not found", id)))
}

// Simple timestamp function (avoids chrono dependency)
fn chrono_lite_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap();
    format!("{}", duration.as_secs())
}

#[tokio::main]
async fn main() {
    // Initialize shared state
    let state: AppState = Arc::new(RwLock::new(HashMap::new()));

    // Build router with all routes
    let app = Router::new()
        .route("/items", get(list_items).post(create_item))
        .route(
            "/items/:id",
            get(get_item).put(update_item).delete(delete_item),
        )
        .with_state(state);

    println!("Server running on http://localhost:3000");
    println!();
    println!("Try these commands:");
    println!("  # Create an item");
    println!("  curl -X POST http://localhost:3000/items \\");
    println!("    -H \"Content-Type: application/json\" \\");
    println!("    -d '{{\"name\": \"Widget\", \"price\": 9.99}}'");
    println!();
    println!("  # List items");
    println!("  curl http://localhost:3000/items");
    println!();
    println!("  # Get item (replace <id> with actual UUID)");
    println!("  curl http://localhost:3000/items/<id>");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_item() {
        let state: AppState = Arc::new(RwLock::new(HashMap::new()));
        let payload = CreateItem {
            name: "Test".to_string(),
            description: None,
            price: 10.0,
        };

        let result = create_item(State(state.clone()), Json(payload)).await;
        assert!(result.is_ok());

        let items = state.read().await;
        assert_eq!(items.len(), 1);
    }

    #[tokio::test]
    async fn test_get_item_not_found() {
        let state: AppState = Arc::new(RwLock::new(HashMap::new()));
        let id = Uuid::new_v4();

        let result = get_item(State(state), Path(id)).await;
        assert!(result.is_err());
    }
}
