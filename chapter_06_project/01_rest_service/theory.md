# REST Service with Axum

## Section Goals

> Build production-ready REST APIs using Axum, Rust's modern async web framework.

After completing this section, you will be able to:

- Design RESTful APIs with proper resource modeling
- Use Axum's router and handler patterns
- Extract and validate request data
- Return proper HTTP responses with error handling
- Integrate with databases for persistence

---

## 1. REST API Principles

### Why Learn This?

REST (Representational State Transfer) is the dominant architectural style for web APIs. Understanding REST principles helps you design intuitive, maintainable APIs that other developers can easily use.

### Core Concepts

**Resources and URLs**

Resources are the nouns in your API - things like users, items, orders:

```
GET    /items          # List all items
POST   /items          # Create a new item
GET    /items/123      # Get item 123
PUT    /items/123      # Update item 123
DELETE /items/123      # Delete item 123
```

**HTTP Methods**

| Method | Purpose | Idempotent | Safe |
|--------|---------|------------|------|
| GET | Read resource | Yes | Yes |
| POST | Create resource | No | No |
| PUT | Update/Replace | Yes | No |
| PATCH | Partial update | No | No |
| DELETE | Remove resource | Yes | No |

**Status Codes**

```rust
// Success
200 OK           // Request succeeded
201 Created      // Resource created
204 No Content   // Success, no body (DELETE)

// Client Errors
400 Bad Request  // Invalid input
401 Unauthorized // Not authenticated
403 Forbidden    // Not authorized
404 Not Found    // Resource doesn't exist
422 Unprocessable // Validation failed

// Server Errors
500 Internal Server Error
503 Service Unavailable
```

---

## 2. Axum Framework

### Why Axum?

Axum is built on top of Tokio and Tower, providing:

- Type-safe request extraction
- Composable middleware via Tower
- Excellent async performance
- Strong community and maintenance (by Tokio team)

### Basic Structure

```rust
use axum::{
    routing::{get, post},
    Router,
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Item {
    id: u64,
    name: String,
}

async fn list_items() -> Json<Vec<Item>> {
    Json(vec![
        Item { id: 1, name: "First".to_string() },
    ])
}

async fn create_item(Json(payload): Json<CreateItem>) -> Json<Item> {
    Json(Item {
        id: 42,
        name: payload.name,
    })
}

#[derive(Deserialize)]
struct CreateItem {
    name: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/items", get(list_items).post(create_item));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

### Extractors

Extractors pull data from requests in a type-safe way:

```rust
use axum::extract::{Path, Query, State};
use std::collections::HashMap;

// Path parameters: /items/:id
async fn get_item(Path(id): Path<u64>) -> String {
    format!("Item {}", id)
}

// Query parameters: /items?page=1&limit=10
#[derive(Deserialize)]
struct Pagination {
    page: Option<u32>,
    limit: Option<u32>,
}

async fn list_items(Query(pagination): Query<Pagination>) -> String {
    let page = pagination.page.unwrap_or(1);
    let limit = pagination.limit.unwrap_or(10);
    format!("Page {} with {} items", page, limit)
}

// JSON body
#[derive(Deserialize)]
struct CreateItem {
    name: String,
    price: f64,
}

async fn create_item(Json(item): Json<CreateItem>) -> String {
    format!("Created {} at ${}", item.name, item.price)
}

// Multiple extractors
async fn update_item(
    Path(id): Path<u64>,
    Json(item): Json<CreateItem>,
) -> String {
    format!("Updated item {} to {}", id, item.name)
}
```

### Shared State

Use `State` to share data (like database pools) across handlers:

```rust
use axum::extract::State;
use std::sync::Arc;
use tokio::sync::RwLock;

type SharedState = Arc<RwLock<Vec<Item>>>;

async fn list_items(State(state): State<SharedState>) -> Json<Vec<Item>> {
    let items = state.read().await;
    Json(items.clone())
}

async fn create_item(
    State(state): State<SharedState>,
    Json(payload): Json<CreateItem>,
) -> Json<Item> {
    let mut items = state.write().await;
    let item = Item {
        id: items.len() as u64 + 1,
        name: payload.name,
    };
    items.push(item.clone());
    Json(item)
}

#[tokio::main]
async fn main() {
    let state: SharedState = Arc::new(RwLock::new(Vec::new()));

    let app = Router::new()
        .route("/items", get(list_items).post(create_item))
        .with_state(state);

    // ... serve
}
```

---

## 3. Error Handling

### Custom Error Types

```rust
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

// Define your error type
enum AppError {
    NotFound(String),
    BadRequest(String),
    Internal(String),
}

// Implement IntoResponse for automatic conversion
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(json!({
            "error": message
        }));

        (status, body).into_response()
    }
}

// Use in handlers
async fn get_item(Path(id): Path<u64>) -> Result<Json<Item>, AppError> {
    if id == 0 {
        return Err(AppError::BadRequest("ID must be positive".to_string()));
    }

    // Simulate not found
    if id > 100 {
        return Err(AppError::NotFound(format!("Item {} not found", id)));
    }

    Ok(Json(Item { id, name: "Found".to_string() }))
}
```

### Converting External Errors

```rust
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => {
                AppError::NotFound("Resource not found".to_string())
            }
            _ => AppError::Internal(err.to_string()),
        }
    }
}

// Now you can use ? with SQLx
async fn get_item(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<Item>, AppError> {
    let item = sqlx::query_as!(Item, "SELECT * FROM items WHERE id = ?", id)
        .fetch_one(&pool)
        .await?;  // Automatically converts to AppError
    Ok(Json(item))
}
```

---

## 4. Response Types

### Different Response Formats

```rust
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    Json,
};

// JSON response (most common)
async fn json_response() -> Json<Item> {
    Json(Item { id: 1, name: "Test".to_string() })
}

// HTML response
async fn html_response() -> Html<&'static str> {
    Html("<h1>Hello World</h1>")
}

// Status code with JSON
async fn created_response() -> (StatusCode, Json<Item>) {
    (StatusCode::CREATED, Json(Item { id: 1, name: "New".to_string() }))
}

// No content (for DELETE)
async fn delete_item(Path(id): Path<u64>) -> StatusCode {
    // ... delete logic
    StatusCode::NO_CONTENT
}

// Headers
use axum::http::header;

async fn with_headers() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "application/json")],
        Json(Item { id: 1, name: "Test".to_string() }),
    )
}
```

---

## 5. Request Validation

### Using Validator Crate

```rust
use validator::Validate;

#[derive(Deserialize, Validate)]
struct CreateItem {
    #[validate(length(min = 1, max = 100))]
    name: String,

    #[validate(range(min = 0.01))]
    price: f64,

    #[validate(email)]
    contact_email: Option<String>,
}

// Custom extractor that validates
use axum::{
    async_trait,
    extract::{FromRequest, Request},
};

struct ValidatedJson<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate,
    Json<T>: FromRequest<S>,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|e| AppError::BadRequest(e.to_string()))?;

        value.validate()
            .map_err(|e| AppError::BadRequest(e.to_string()))?;

        Ok(ValidatedJson(value))
    }
}

// Usage
async fn create_item(
    ValidatedJson(payload): ValidatedJson<CreateItem>,
) -> Result<Json<Item>, AppError> {
    // payload is guaranteed to be valid here
    Ok(Json(Item { id: 1, name: payload.name }))
}
```

---

## 6. Middleware with Tower

### Adding Middleware

```rust
use axum::{
    middleware::{self, Next},
    extract::Request,
    response::Response,
};
use std::time::Instant;

// Logging middleware
async fn logging_middleware(
    request: Request,
    next: Next,
) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let start = Instant::now();

    let response = next.run(request).await;

    let duration = start.elapsed();
    println!("{} {} - {:?} - {}ms",
        method, uri, response.status(), duration.as_millis());

    response
}

// Apply to router
let app = Router::new()
    .route("/items", get(list_items))
    .layer(middleware::from_fn(logging_middleware));
```

### Tower Services

```rust
use tower_http::{
    cors::CorsLayer,
    timeout::TimeoutLayer,
    compression::CompressionLayer,
};
use std::time::Duration;

let app = Router::new()
    .route("/items", get(list_items))
    .layer(CorsLayer::permissive())
    .layer(TimeoutLayer::new(Duration::from_secs(30)))
    .layer(CompressionLayer::new());
```

---

## Summary

Building REST APIs with Axum involves:

1. **Routing**: Map URLs to handlers
2. **Extractors**: Type-safe request parsing (Path, Query, Json, State)
3. **Responses**: Return appropriate status codes and data
4. **Error Handling**: Custom error types that implement IntoResponse
5. **Middleware**: Cross-cutting concerns via Tower layers

Key patterns:
- Use `State` for shared resources (database pools, caches)
- Implement `IntoResponse` for custom error types
- Use validation crates for input validation
- Layer middleware for logging, auth, CORS, etc.

---

## Next Steps

1. **Lab 1**: Build a complete CRUD API with in-memory storage
2. **Lab 2**: Add SQLite database integration
