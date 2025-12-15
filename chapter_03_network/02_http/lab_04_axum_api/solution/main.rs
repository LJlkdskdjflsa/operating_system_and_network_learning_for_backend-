//! Lab 4 Reference Answer

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

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

/// GET /items - List all items
async fn list_items(State(state): State<Arc<AppState>>) -> Json<Vec<Item>> {
    let items = state.items.lock().unwrap();
    let items_vec: Vec<Item> = items.values().cloned().collect();
    Json(items_vec)
}

/// POST /items - Create new item
async fn create_item(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateItem>,
) -> (StatusCode, Json<Item>) {
    // Get next ID
    let mut next_id = state.next_id.lock().unwrap();
    let id = *next_id;
    *next_id += 1;

    // Create item
    let item = Item {
        id,
        name: payload.name,
        price: payload.price,
    };

    // Store item
    let mut items = state.items.lock().unwrap();
    items.insert(id, item.clone());

    println!("Created item: {:?}", serde_json::to_string(&item).unwrap());

    (StatusCode::CREATED, Json(item))
}

/// GET /items/:id - Get single item
async fn get_item(
    State(state): State<Arc<AppState>>,
    Path(id): Path<u64>,
) -> Result<Json<Item>, StatusCode> {
    let items = state.items.lock().unwrap();

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
    let mut items = state.items.lock().unwrap();

    if !items.contains_key(&id) {
        return Err(StatusCode::NOT_FOUND);
    }

    let item = Item {
        id,
        name: payload.name,
        price: payload.price,
    };

    items.insert(id, item.clone());

    println!("Updated item {}: {:?}", id, serde_json::to_string(&item).unwrap());

    Ok(Json(item))
}

/// DELETE /items/:id - Delete item
async fn delete_item(
    State(state): State<Arc<AppState>>,
    Path(id): Path<u64>,
) -> StatusCode {
    let mut items = state.items.lock().unwrap();

    if items.remove(&id).is_some() {
        println!("Deleted item {}", id);
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

/// GET / - API info
async fn root() -> &'static str {
    "Items API - Use /items endpoint"
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";

    // Create shared state
    let state = Arc::new(AppState {
        items: Mutex::new(HashMap::new()),
        next_id: Mutex::new(1),
    });

    // Build router
    let app = Router::new()
        .route("/", get(root))
        .route("/items", get(list_items).post(create_item))
        .route(
            "/items/:id",
            get(get_item).put(update_item).delete(delete_item),
        )
        .with_state(state);

    println!("Axum REST API Server");
    println!("Listening on http://{}", addr);
    println!("\nEndpoints:");
    println!("  GET    /items       - List all items");
    println!("  POST   /items       - Create new item");
    println!("  GET    /items/:id   - Get single item");
    println!("  PUT    /items/:id   - Update item");
    println!("  DELETE /items/:id   - Delete item");
    println!("\nExample:");
    println!("  curl -X POST -H 'Content-Type: application/json' \\");
    println!("    -d '{{\"name\":\"Widget\",\"price\":9.99}}' \\");
    println!("    http://localhost:8080/items\n");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Key concepts demonstrated:
//
// 1. AXUM ROUTING:
//    - Router::new() creates router
//    - .route() adds paths and handlers
//    - HTTP methods: get(), post(), put(), delete()
//    - Can chain methods: get(handler).post(handler)
//
// 2. EXTRACTORS:
//    - State<T>: Shared application state
//    - Path<T>: URL path parameters
//    - Json<T>: JSON request body
//
// 3. RESPONSES:
//    - Return Json<T> for JSON response
//    - Return StatusCode for status only
//    - Return (StatusCode, Json<T>) for both
//    - Return Result<T, StatusCode> for fallible handlers
//
// 4. STATE MANAGEMENT:
//    - Arc<T> for shared ownership
//    - Mutex<T> for interior mutability
//    - .with_state() adds state to router
//
// Compare with raw HTTP:
// - Axum handles parsing, routing, serialization
// - Type-safe extractors
// - Automatic Content-Type headers
// - Less boilerplate code

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    fn create_app() -> Router {
        let state = Arc::new(AppState {
            items: Mutex::new(HashMap::new()),
            next_id: Mutex::new(1),
        });

        Router::new()
            .route("/items", get(list_items).post(create_item))
            .route(
                "/items/:id",
                get(get_item).put(update_item).delete(delete_item),
            )
            .with_state(state)
    }

    #[tokio::test]
    async fn test_list_empty() {
        let app = create_app();

        let response = app
            .oneshot(Request::builder().uri("/items").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_create_item() {
        let app = create_app();

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/items")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"name":"Test","price":1.99}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_get_not_found() {
        let app = create_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/items/999")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
