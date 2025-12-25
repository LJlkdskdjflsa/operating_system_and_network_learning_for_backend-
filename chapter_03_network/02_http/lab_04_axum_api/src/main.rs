//! Lab 4: Axum REST API
//!
//! ## Goal
//! Build a CRUD REST API using Axum framework
//!
//! ## Requirements
//! 1. Implement a simple "items" API with CRUD operations
//! 2. Store items in memory (HashMap with Mutex)
//! 3. Return proper JSON responses
//! 4. Use appropriate HTTP status codes
//!
//! ## API Endpoints
//! - GET    /items       - List all items
//! - POST   /items       - Create new item
//! - GET    /items/:id   - Get single item
//! - PUT    /items/:id   - Update item
//! - DELETE /items/:id   - Delete item
//!
//! ## Expected Behavior
//! ```bash
//! # Create item
//! curl -X POST -H "Content-Type: application/json" \
//!   -d '{"name":"Widget","price":9.99}' \
//!   http://localhost:8080/items
//! # Returns: {"id":1,"name":"Widget","price":9.99}
//!
//! # List items
//! curl http://localhost:8080/items
//! # Returns: [{"id":1,"name":"Widget","price":9.99}]
//!
//! # Get item
//! curl http://localhost:8080/items/1
//! # Returns: {"id":1,"name":"Widget","price":9.99}
//!
//! # Update item
//! curl -X PUT -H "Content-Type: application/json" \
//!   -d '{"name":"Super Widget","price":19.99}' \
//!   http://localhost:8080/items/1
//!
//! # Delete item
//! curl -X DELETE http://localhost:8080/items/1
//! ```
//!
//! ## Hints
//! - Use `axum::extract::State` for shared state
//! - Use `axum::extract::Path` for path parameters
//! - Use `axum::Json` for JSON request/response
//! - Use `Arc<Mutex<HashMap>>` for thread-safe storage
//!
//! ## Acceptance Criteria
//! - [ ] All CRUD operations work
//! - [ ] Returns JSON with correct Content-Type
//! - [ ] Uses proper status codes (200, 201, 404, etc.)
//! - [ ] Handles concurrent requests safely

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// ============================================================
// TODO: Implement the REST API
// ============================================================

/// Item model
#[derive(Clone, Serialize, Deserialize)]
struct Item {
    id: u64,
    name: String,
    price: f64,
}

/// Request body for creating/updating items
#[derive(Deserialize)]
struct CreateItem {
    name: String,
    price: f64,
}

/// Application state
struct AppState {
    items: Mutex<HashMap<u64, Item>>,
    next_id: Mutex<u64>,
}

// TODO: Implement handlers

/// GET /items - List all items
async fn list_items(State(state): State<Arc<AppState>>) -> Json<Vec<Item>> {
    let items = state.items.lock().expect("items mutex poisoned");
    let mut result: Vec<Item> = items.values().cloned().collect();
    result.sort_by_key(|item| item.id);
    Json(result)
}

/// POST /items - Create new item
async fn create_item(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateItem>,
) -> (StatusCode, Json<Item>) {
    let id = {
        let mut next_id = state.next_id.lock().expect("next_id mutex poisoned");
        let id = *next_id;
        *next_id += 1;
        id
    };

    let item = Item {
        id,
        name: payload.name,
        price: payload.price,
    };

    let mut items = state.items.lock().expect("items mutex poisoned");
    items.insert(id, item.clone());

    (StatusCode::CREATED, Json(item))
}

/// GET /items/:id - Get single item
async fn get_item(
    State(state): State<Arc<AppState>>,
    Path(id): Path<u64>,
) -> Result<Json<Item>, StatusCode> {
    let items = state.items.lock().expect("items mutex poisoned");
    items
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

/// PUT /items/:id - Update item
async fn update_item(
    State(state): State<Arc<AppState>>,
    Path(id): Path<u64>,
    Json(payload): Json<CreateItem>,
) -> Result<Json<Item>, StatusCode> {
    // todo!("Implement update_item")
    let mut items = state.items.lock().expect("items mutex poisoned");
    if !items.contains_key(&id) {
        return Err(StatusCode::NOT_FOUND);
    }
    let item = Item {
        id,
        name: payload.name,
        price: payload.price,
    };
    items.insert(id, item.clone());

    Ok(Json(item))
}

/// DELETE /items/:id - Delete item
async fn delete_item(State(state): State<Arc<AppState>>, Path(id): Path<u64>) -> StatusCode {
    let mut items = state.items.lock().expect("items mutex poisoned");
    match items.remove(&id) {
        Some(_) => StatusCode::NO_CONTENT,
        None => StatusCode::NOT_FOUND,
    }
}

#[tokio::main]
async fn main() {
    // TODO: Implement
    // 1. Create AppState
    let state = Arc::new(AppState {
        items: Mutex::new(HashMap::new()),
        next_id: Mutex::new(1),
    });

    // 2. Build router with routes

    let app = Router::new()
        .route("/items", get(list_items).post(create_item))
        .route(
            "/items/:id",
            get(get_item).put(update_item).delete(delete_item),
        )
        .with_state(state);
    // 3. Run server

    let addr = "0.0.0:8080";
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind address");
    println!("Listening on http://{addr}");
    axum::serve(listener, app).await.expect("server error");
}
