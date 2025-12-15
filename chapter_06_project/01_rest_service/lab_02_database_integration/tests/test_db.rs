//! Lab 2: Database Integration Tests
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
    page: i64,
    limit: i64,
    total: i64,
}

const BASE_URL: &str = "http://localhost:3000";

#[tokio::test]
#[ignore = "requires running server"]
async fn test_01_create_and_retrieve() {
    let client = reqwest::Client::new();

    // Create
    let create_resp = client
        .post(format!("{}/items", BASE_URL))
        .json(&json!({
            "name": "DB Test Item",
            "description": "Testing database persistence",
            "price": 29.99
        }))
        .send()
        .await
        .expect("Failed to create");

    assert_eq!(create_resp.status(), 201);
    let created: Item = create_resp.json().await.unwrap();

    // Retrieve
    let get_resp = client
        .get(format!("{}/items/{}", BASE_URL, created.id))
        .send()
        .await
        .expect("Failed to get");

    assert_eq!(get_resp.status(), 200);
    let retrieved: Item = get_resp.json().await.unwrap();

    assert_eq!(retrieved.id, created.id);
    assert_eq!(retrieved.name, "DB Test Item");
    assert_eq!(retrieved.price, 29.99);
}

#[tokio::test]
#[ignore = "requires running server"]
async fn test_02_pagination_with_total() {
    let client = reqwest::Client::new();

    // Create multiple items
    for i in 1..=5 {
        client
            .post(format!("{}/items", BASE_URL))
            .json(&json!({
                "name": format!("Pagination Item {}", i),
                "price": i as f64 * 5.0
            }))
            .send()
            .await
            .expect("Failed to create");
    }

    // Test pagination
    let resp = client
        .get(format!("{}/items?page=1&limit=2", BASE_URL))
        .send()
        .await
        .expect("Failed to list");

    assert_eq!(resp.status(), 200);
    let paginated: PaginatedResponse = resp.json().await.unwrap();

    assert_eq!(paginated.items.len(), 2);
    assert_eq!(paginated.page, 1);
    assert_eq!(paginated.limit, 2);
    assert!(paginated.total >= 5, "Should have at least 5 items total");
}

#[tokio::test]
#[ignore = "requires running server"]
async fn test_03_update_persists() {
    let client = reqwest::Client::new();

    // Create
    let create_resp = client
        .post(format!("{}/items", BASE_URL))
        .json(&json!({
            "name": "Update Persist Test",
            "price": 10.00
        }))
        .send()
        .await
        .unwrap();

    let created: Item = create_resp.json().await.unwrap();

    // Update
    let update_resp = client
        .put(format!("{}/items/{}", BASE_URL, created.id))
        .json(&json!({
            "name": "Updated Persist Test",
            "price": 20.00
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(update_resp.status(), 200);

    // Verify update persisted
    let get_resp = client
        .get(format!("{}/items/{}", BASE_URL, created.id))
        .send()
        .await
        .unwrap();

    let retrieved: Item = get_resp.json().await.unwrap();
    assert_eq!(retrieved.name, "Updated Persist Test");
    assert_eq!(retrieved.price, 20.00);
}

#[tokio::test]
#[ignore = "requires running server"]
async fn test_04_delete_removes_from_db() {
    let client = reqwest::Client::new();

    // Create
    let create_resp = client
        .post(format!("{}/items", BASE_URL))
        .json(&json!({
            "name": "Delete Test",
            "price": 5.00
        }))
        .send()
        .await
        .unwrap();

    let created: Item = create_resp.json().await.unwrap();

    // Delete
    let delete_resp = client
        .delete(format!("{}/items/{}", BASE_URL, created.id))
        .send()
        .await
        .unwrap();

    assert_eq!(delete_resp.status(), 204);

    // Verify deleted
    let get_resp = client
        .get(format!("{}/items/{}", BASE_URL, created.id))
        .send()
        .await
        .unwrap();

    assert_eq!(get_resp.status(), 404);
}

#[tokio::test]
#[ignore = "requires running server"]
async fn test_05_concurrent_creates() {
    let client = reqwest::Client::new();

    // Create items concurrently
    let mut handles = vec![];
    for i in 1..=10 {
        let client = client.clone();
        let handle = tokio::spawn(async move {
            client
                .post(format!("{}/items", BASE_URL))
                .json(&json!({
                    "name": format!("Concurrent Item {}", i),
                    "price": i as f64
                }))
                .send()
                .await
        });
        handles.push(handle);
    }

    // Wait for all
    let mut success_count = 0;
    for handle in handles {
        if let Ok(Ok(resp)) = handle.await {
            if resp.status() == 201 {
                success_count += 1;
            }
        }
    }

    assert_eq!(success_count, 10, "All concurrent creates should succeed");
}
