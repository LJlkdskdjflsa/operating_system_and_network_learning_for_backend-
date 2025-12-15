//! Lab 1: Axum CRUD API Tests
//!
//! These tests require the server to be running on localhost:3000
//! Run with: cargo test -- --ignored

use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
struct Item {
    id: String,
    name: String,
    description: Option<String>,
    price: f64,
    created_at: String,
}

#[derive(Debug, Deserialize)]
struct PaginatedResponse {
    items: Vec<Item>,
    page: usize,
    limit: usize,
    total: usize,
}

#[derive(Debug, Deserialize)]
struct ErrorResponse {
    error: String,
}

const BASE_URL: &str = "http://localhost:3000";

#[tokio::test]
#[ignore = "requires running server"]
async fn test_01_create_item() {
    let client = reqwest::Client::new();

    let response = client
        .post(format!("{}/items", BASE_URL))
        .json(&json!({
            "name": "Test Widget",
            "description": "A test item",
            "price": 19.99
        }))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 201, "Should return 201 Created");

    let item: Item = response.json().await.expect("Failed to parse response");
    assert_eq!(item.name, "Test Widget");
    assert_eq!(item.price, 19.99);
    assert!(item.description.is_some());
}

#[tokio::test]
#[ignore = "requires running server"]
async fn test_02_create_item_minimal() {
    let client = reqwest::Client::new();

    let response = client
        .post(format!("{}/items", BASE_URL))
        .json(&json!({
            "name": "Minimal Item",
            "price": 5.00
        }))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 201);

    let item: Item = response.json().await.expect("Failed to parse response");
    assert_eq!(item.name, "Minimal Item");
    assert!(item.description.is_none());
}

#[tokio::test]
#[ignore = "requires running server"]
async fn test_03_list_items() {
    let client = reqwest::Client::new();

    // Create a few items first
    for i in 1..=3 {
        client
            .post(format!("{}/items", BASE_URL))
            .json(&json!({
                "name": format!("List Item {}", i),
                "price": i as f64 * 10.0
            }))
            .send()
            .await
            .expect("Failed to create item");
    }

    let response = client
        .get(format!("{}/items", BASE_URL))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 200);

    let paginated: PaginatedResponse = response.json().await.expect("Failed to parse response");
    assert!(paginated.items.len() >= 3, "Should have at least 3 items");
    assert_eq!(paginated.page, 1);
}

#[tokio::test]
#[ignore = "requires running server"]
async fn test_04_list_items_with_pagination() {
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/items?page=1&limit=2", BASE_URL))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 200);

    let paginated: PaginatedResponse = response.json().await.expect("Failed to parse response");
    assert!(paginated.items.len() <= 2, "Should respect limit");
    assert_eq!(paginated.limit, 2);
}

#[tokio::test]
#[ignore = "requires running server"]
async fn test_05_get_item() {
    let client = reqwest::Client::new();

    // Create an item first
    let create_response = client
        .post(format!("{}/items", BASE_URL))
        .json(&json!({
            "name": "Get Test Item",
            "price": 25.00
        }))
        .send()
        .await
        .expect("Failed to create item");

    let created: Item = create_response.json().await.expect("Failed to parse");
    let id = created.id;

    // Now get it
    let response = client
        .get(format!("{}/items/{}", BASE_URL, id))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 200);

    let item: Item = response.json().await.expect("Failed to parse response");
    assert_eq!(item.id, id);
    assert_eq!(item.name, "Get Test Item");
}

#[tokio::test]
#[ignore = "requires running server"]
async fn test_06_get_item_not_found() {
    let client = reqwest::Client::new();
    let fake_id = "00000000-0000-0000-0000-000000000000";

    let response = client
        .get(format!("{}/items/{}", BASE_URL, fake_id))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 404, "Should return 404 for non-existent item");

    let error: ErrorResponse = response.json().await.expect("Failed to parse error");
    assert!(error.error.contains("not found"));
}

#[tokio::test]
#[ignore = "requires running server"]
async fn test_07_update_item() {
    let client = reqwest::Client::new();

    // Create an item first
    let create_response = client
        .post(format!("{}/items", BASE_URL))
        .json(&json!({
            "name": "Update Test",
            "price": 30.00
        }))
        .send()
        .await
        .expect("Failed to create item");

    let created: Item = create_response.json().await.expect("Failed to parse");
    let id = created.id;

    // Update it
    let response = client
        .put(format!("{}/items/{}", BASE_URL, id))
        .json(&json!({
            "name": "Updated Name",
            "price": 35.00
        }))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 200);

    let item: Item = response.json().await.expect("Failed to parse response");
    assert_eq!(item.name, "Updated Name");
    assert_eq!(item.price, 35.00);
}

#[tokio::test]
#[ignore = "requires running server"]
async fn test_08_update_item_partial() {
    let client = reqwest::Client::new();

    // Create an item first
    let create_response = client
        .post(format!("{}/items", BASE_URL))
        .json(&json!({
            "name": "Partial Update Test",
            "price": 40.00
        }))
        .send()
        .await
        .expect("Failed to create item");

    let created: Item = create_response.json().await.expect("Failed to parse");
    let id = created.id;

    // Update only the name
    let response = client
        .put(format!("{}/items/{}", BASE_URL, id))
        .json(&json!({
            "name": "Only Name Changed"
        }))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 200);

    let item: Item = response.json().await.expect("Failed to parse response");
    assert_eq!(item.name, "Only Name Changed");
    assert_eq!(item.price, 40.00, "Price should remain unchanged");
}

#[tokio::test]
#[ignore = "requires running server"]
async fn test_09_delete_item() {
    let client = reqwest::Client::new();

    // Create an item first
    let create_response = client
        .post(format!("{}/items", BASE_URL))
        .json(&json!({
            "name": "Delete Test",
            "price": 50.00
        }))
        .send()
        .await
        .expect("Failed to create item");

    let created: Item = create_response.json().await.expect("Failed to parse");
    let id = created.id;

    // Delete it
    let response = client
        .delete(format!("{}/items/{}", BASE_URL, id))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 204, "Should return 204 No Content");

    // Verify it's gone
    let get_response = client
        .get(format!("{}/items/{}", BASE_URL, id))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(get_response.status(), 404, "Item should no longer exist");
}

#[tokio::test]
#[ignore = "requires running server"]
async fn test_10_delete_item_not_found() {
    let client = reqwest::Client::new();
    let fake_id = "00000000-0000-0000-0000-000000000000";

    let response = client
        .delete(format!("{}/items/{}", BASE_URL, fake_id))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 404, "Should return 404 for non-existent item");
}
