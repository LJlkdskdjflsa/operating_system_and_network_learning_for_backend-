//! Lab 1: Axum CRUD API
//!
//! ## Goal
//! Build a complete REST API for managing items using Axum with in-memory storage.
//!
//! ## Requirements
//! 1. POST /items - Create a new item (returns 201 Created)
//! 2. GET /items/:id - Get an item by ID (returns 404 if not found)
//! 3. GET /items - List all items with optional pagination (?page=1&limit=10)
//! 4. PUT /items/:id - Update an item (returns 404 if not found)
//! 5. DELETE /items/:id - Delete an item (returns 204 No Content)
//!
//! ## Data Model
//! ```rust
//! struct Item {
//!     id: Uuid,
//!     name: String,
//!     description: Option<String>,
//!     price: f64,
//!     created_at: String,
//! }
//! ```
//!
//! ## Hints
//! - Use `Arc<RwLock<HashMap<Uuid, Item>>>` for shared state
//! - Use `axum::extract::{State, Path, Query, Json}`
//! - Implement proper error responses with status codes
//! - Use `uuid::Uuid::new_v4()` to generate IDs
//!
//! ## Verification
//! ```bash
//! # Terminal 1: Run server
//! cargo run
//!
//! # Terminal 2: Test endpoints
//! curl -X POST http://localhost:3000/items \
//!   -H "Content-Type: application/json" \
//!   -d '{"name": "Widget", "price": 9.99}'
//!
//! curl http://localhost:3000/items
//! curl http://localhost:3000/items/<id>
//! ```
//!
//! ## Acceptance Criteria
//! - [ ] All CRUD operations work correctly
//! - [ ] Proper HTTP status codes returned
//! - [ ] 404 returned for non-existent items
//! - [ ] Pagination works with page and limit params
//! - [ ] JSON serialization/deserialization works
//!
//! Check solution/main.rs after completing

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
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

// Shared state type
type AppState = Arc<RwLock<HashMap<Uuid, Item>>>;

// Error type for the API
enum AppError {
    NotFound(String),
    BadRequest(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        // TODO: Implement proper error response
        // Return JSON with error message and appropriate status code
        todo!()
    }
}

// Handler: Create item
async fn create_item(
    State(state): State<AppState>,
    Json(payload): Json<CreateItem>,
) -> Result<impl IntoResponse, AppError> {
    // TODO: Implement create item
    //
    // Steps:
    // 1. Generate a new UUID for the item
    // 2. Get current timestamp (use chrono or simple string)
    // 3. Create Item struct
    // 4. Insert into state HashMap
    // 5. Return (StatusCode::CREATED, Json(item))
    todo!()
}

// Handler: Get item by ID
async fn get_item(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Item>, AppError> {
    // TODO: Implement get item
    //
    // Steps:
    // 1. Read from state
    // 2. Look up item by ID
    // 3. Return item or NotFound error
    todo!()
}

// Handler: List all items with pagination
async fn list_items(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> Json<Vec<Item>> {
    // TODO: Implement list items with pagination
    //
    // Steps:
    // 1. Read from state
    // 2. Collect items into a Vec
    // 3. Apply pagination (skip and take)
    // 4. Return items
    //
    // Default: page=1, limit=10
    todo!()
}

// Handler: Update item
async fn update_item(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateItem>,
) -> Result<Json<Item>, AppError> {
    // TODO: Implement update item
    //
    // Steps:
    // 1. Write lock on state
    // 2. Find item by ID
    // 3. Update fields that are Some in payload
    // 4. Return updated item or NotFound error
    todo!()
}

// Handler: Delete item
async fn delete_item(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    // TODO: Implement delete item
    //
    // Steps:
    // 1. Write lock on state
    // 2. Remove item by ID
    // 3. Return NO_CONTENT or NotFound error
    todo!()
}

#[tokio::main]
async fn main() {
    // Initialize shared state
    let state: AppState = Arc::new(RwLock::new(HashMap::new()));

    // Build router
    // TODO: Set up routes
    //
    // Routes needed:
    // - GET  /items      -> list_items
    // - POST /items      -> create_item
    // - GET  /items/:id  -> get_item
    // - PUT  /items/:id  -> update_item
    // - DELETE /items/:id -> delete_item
    let app = Router::new()
        // Add routes here
        .with_state(state);

    println!("Server running on http://localhost:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
