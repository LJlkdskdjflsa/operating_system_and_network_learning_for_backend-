//! Lab 2: Database Integration - Solution
//!
//! CRUD API with SQLite persistence using SQLx.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use sqlx::Row;
use std::time::Duration;
use uuid::Uuid;

// Item model - matches database schema
#[derive(Clone, Serialize, Deserialize, sqlx::FromRow)]
struct Item {
    id: String,
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
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS items (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT,
            price REAL NOT NULL,
            created_at TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    println!("Database initialized");
    Ok(())
}

// Simple timestamp
fn now_timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    format!("{}", duration.as_secs())
}

// Handler: Create item
async fn create_item(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateItem>,
) -> Result<impl IntoResponse, AppError> {
    let id = Uuid::new_v4().to_string();
    let created_at = now_timestamp();

    sqlx::query(
        "INSERT INTO items (id, name, description, price, created_at) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(&payload.name)
    .bind(&payload.description)
    .bind(payload.price)
    .bind(&created_at)
    .execute(&pool)
    .await?;

    let item = Item {
        id,
        name: payload.name,
        description: payload.description,
        price: payload.price,
        created_at,
    };

    Ok((StatusCode::CREATED, Json(item)))
}

// Handler: Get item by ID
async fn get_item(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<Json<Item>, AppError> {
    let item = sqlx::query_as::<_, Item>("SELECT * FROM items WHERE id = ?")
        .bind(&id)
        .fetch_optional(&pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Item {} not found", id)))?;

    Ok(Json(item))
}

// Handler: List items with pagination
async fn list_items(
    State(pool): State<SqlitePool>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<PaginatedResponse>, AppError> {
    let page = pagination.page.unwrap_or(1).max(1);
    let limit = pagination.limit.unwrap_or(10).min(100);
    let offset = (page - 1) * limit;

    // Get total count
    let total: i64 = sqlx::query("SELECT COUNT(*) as count FROM items")
        .fetch_one(&pool)
        .await?
        .get("count");

    // Get items for current page
    let items = sqlx::query_as::<_, Item>("SELECT * FROM items ORDER BY created_at DESC LIMIT ? OFFSET ?")
        .bind(limit)
        .bind(offset)
        .fetch_all(&pool)
        .await?;

    Ok(Json(PaginatedResponse {
        items,
        page,
        limit,
        total,
    }))
}

// Handler: Update item
async fn update_item(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateItem>,
) -> Result<Json<Item>, AppError> {
    // First check if item exists
    let existing = sqlx::query_as::<_, Item>("SELECT * FROM items WHERE id = ?")
        .bind(&id)
        .fetch_optional(&pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Item {} not found", id)))?;

    // Apply updates
    let name = payload.name.unwrap_or(existing.name);
    let description = payload.description.or(existing.description);
    let price = payload.price.unwrap_or(existing.price);

    sqlx::query("UPDATE items SET name = ?, description = ?, price = ? WHERE id = ?")
        .bind(&name)
        .bind(&description)
        .bind(price)
        .bind(&id)
        .execute(&pool)
        .await?;

    let updated = Item {
        id: existing.id,
        name,
        description,
        price,
        created_at: existing.created_at,
    };

    Ok(Json(updated))
}

// Handler: Delete item
async fn delete_item(
    State(pool): State<SqlitePool>,
    Path(id): Path<String>,
) -> Result<StatusCode, AppError> {
    let result = sqlx::query("DELETE FROM items WHERE id = ?")
        .bind(&id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Item {} not found", id)));
    }

    Ok(StatusCode::NO_CONTENT)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create connection pool with configuration
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect("sqlite::memory:")
        .await?;

    // Initialize database schema
    init_db(&pool).await?;

    // Build router
    let app = Router::new()
        .route("/items", get(list_items).post(create_item))
        .route(
            "/items/:id",
            get(get_item).put(update_item).delete(delete_item),
        )
        .with_state(pool);

    println!("Server running on http://localhost:3000");
    println!();
    println!("Using in-memory SQLite database");
    println!("Data persists within session but resets on restart");
    println!();
    println!("Try:");
    println!("  curl -X POST http://localhost:3000/items \\");
    println!("    -H \"Content-Type: application/json\" \\");
    println!("    -d '{{\"name\": \"Widget\", \"price\": 9.99}}'");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
