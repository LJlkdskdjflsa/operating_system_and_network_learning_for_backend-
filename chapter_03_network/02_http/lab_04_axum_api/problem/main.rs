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
    routing::{get, post},
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
async fn list_items(
    State(state): State<Arc<AppState>>,
) -> Json<Vec<Item>> {
    todo!("Implement list_items")
}

/// POST /items - Create new item
async fn create_item(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateItem>,
) -> (StatusCode, Json<Item>) {
    todo!("Implement create_item")
}

/// GET /items/:id - Get single item
async fn get_item(
    State(state): State<Arc<AppState>>,
    Path(id): Path<u64>,
) -> Result<Json<Item>, StatusCode> {
    todo!("Implement get_item")
}

/// PUT /items/:id - Update item
async fn update_item(
    State(state): State<Arc<AppState>>,
    Path(id): Path<u64>,
    Json(payload): Json<CreateItem>,
) -> Result<Json<Item>, StatusCode> {
    todo!("Implement update_item")
}

/// DELETE /items/:id - Delete item
async fn delete_item(
    State(state): State<Arc<AppState>>,
    Path(id): Path<u64>,
) -> StatusCode {
    todo!("Implement delete_item")
}

#[tokio::main]
async fn main() {
    // TODO: Implement
    // 1. Create AppState
    // 2. Build router with routes
    // 3. Run server

    todo!("Implement main")
}
