//! Lab 2: Database Integration
//!
//! ## Goal
//! Extend the CRUD API to persist data in SQLite using SQLx.
//!
//! ## Requirements
//! 1. Initialize SQLite database with schema on startup
//! 2. Store items in database instead of in-memory HashMap
//! 3. All CRUD operations should use SQLx queries
//! 4. Use connection pool for database access
//! 5. Handle database errors gracefully
//!
//! ## Database Schema
//! ```sql
//! CREATE TABLE IF NOT EXISTS items (
//!     id TEXT PRIMARY KEY,
//!     name TEXT NOT NULL,
//!     description TEXT,
//!     price REAL NOT NULL,
//!     created_at TEXT NOT NULL
//! );
//! ```
//!
//! ## Hints
//! - Use `SqlitePool::connect(":memory:")` for in-memory database
//! - Use `sqlx::query!` or `sqlx::query_as!` for type-safe queries
//! - Store UUID as TEXT in SQLite
//! - Share pool via Axum State
//!
//! ## Verification
//! ```bash
//! cargo run
//! # Test CRUD operations - data persists within session
//! ```
//!
//! ## Acceptance Criteria
//! - [ ] Database initialized on startup
//! - [ ] Items persist in SQLite
//! - [ ] All CRUD operations work with database
//! - [ ] Proper error handling for database failures
//! - [ ] Connection pool properly configured
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
use serde_json::json;
use sqlx::sqlite::SqlitePool;
use uuid::Uuid;

// Item model - matches database schema
#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
struct Item {
    id: String,  // UUID stored as TEXT
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

// Pagination
#[derive(Deserialize)]
struct Pagination {
    page: Option<i64>,
    limit: Option<i64>,
}

#[derive(Serialize)]
struct PaginatedResponse {
    items: Vec<Item>,
    page: i64,
    limit: i64,
    total: i64,
}

// Error type
enum AppError {
    NotFound(String),
    Database(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Database(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound("Resource not found".to_string()),
            _ => AppError::Database(err.to_string()),
        }
    }
}

// Initialize database schema
async fn init_db(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // TODO: Create the items table if it doesn't exist
    //
    // SQL:
    // CREATE TABLE IF NOT EXISTS items (
    //     id TEXT PRIMARY KEY,
    //     name TEXT NOT NULL,
    //     description TEXT,
    //     price REAL NOT NULL,
    //     created_at TEXT NOT NULL
    // )
    todo!()
}

// Handler: Create item
async fn create_item(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateItem>,
) -> Result<impl IntoResponse, AppError> {
    // TODO: Insert item into database
    //
    // Steps:
    // 1. Generate UUID and timestamp
    // 2. INSERT INTO items VALUES (...)
    // 3. Return the created item with 201 status
    todo!()
}

// Handler: Get item by ID
async fn get_item(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<Json<Item>, AppError> {
    // TODO: Query item from database
    //
    // SQL: SELECT * FROM items WHERE id = ?
    todo!()
}

// Handler: List items with pagination
async fn list_items(
    State(pool): State<SqlitePool>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<PaginatedResponse>, AppError> {
    // TODO: Query items with pagination
    //
    // Steps:
    // 1. Get total count: SELECT COUNT(*) FROM items
    // 2. Get page of items: SELECT * FROM items LIMIT ? OFFSET ?
    // 3. Return paginated response
    todo!()
}

// Handler: Update item
async fn update_item(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateItem>,
) -> Result<Json<Item>, AppError> {
    // TODO: Update item in database
    //
    // Steps:
    // 1. First check if item exists
    // 2. Build UPDATE query for provided fields
    // 3. Return updated item
    todo!()
}

// Handler: Delete item
async fn delete_item(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<StatusCode, AppError> {
    // TODO: Delete item from database
    //
    // Steps:
    // 1. DELETE FROM items WHERE id = ?
    // 2. Check rows_affected() to verify deletion
    // 3. Return 204 or 404
    todo!()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Set up database connection pool
    //
    // Steps:
    // 1. Create SqlitePool with ":memory:" or file path
    // 2. Call init_db to create schema
    // 3. Build router with pool as state
    // 4. Start server

    println!("Server running on http://localhost:3000");

    todo!()
}
