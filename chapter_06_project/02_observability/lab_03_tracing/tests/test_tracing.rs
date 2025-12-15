//! Lab 3: Tracing Tests
//!
//! These tests require the server to be running on localhost:3000
//! Run with: cargo test -- --ignored
//!
//! Observe the server logs to verify structured logging is working.

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Item {
    id: String,
    name: String,
}

const BASE_URL: &str = "http://localhost:3000";

#[tokio::test]
#[ignore = "requires running server"]
async fn test_01_health_check() {
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health", BASE_URL))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 200);
    println!("Health check passed - check server logs for JSON output");
}

#[tokio::test]
#[ignore = "requires running server"]
async fn test_02_list_items_logging() {
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/items", BASE_URL))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 200);

    let items: Vec<Item> = response.json().await.expect("Failed to parse");
    assert!(!items.is_empty());

    println!("List items returned {} items", items.len());
    println!("Check server logs for:");
    println!("  - request_id field");
    println!("  - method: GET");
    println!("  - path: /items");
    println!("  - duration_ms field");
}

#[tokio::test]
#[ignore = "requires running server"]
async fn test_03_get_item_found() {
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/items/1", BASE_URL))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 200);

    let item: Item = response.json().await.expect("Failed to parse");
    assert_eq!(item.id, "1");

    println!("Got item: {:?}", item);
    println!("Check server logs for item_id field in span");
}

#[tokio::test]
#[ignore = "requires running server"]
async fn test_04_get_item_not_found() {
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/items/999", BASE_URL))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 404);

    println!("Got expected 404");
    println!("Check server logs for:");
    println!("  - WARN level log");
    println!("  - 'Item not found' message");
}

#[tokio::test]
#[ignore = "requires running server"]
async fn test_05_multiple_requests_different_ids() {
    let client = reqwest::Client::new();

    // Make several requests
    for _ in 0..3 {
        let _ = client
            .get(format!("{}/items", BASE_URL))
            .send()
            .await;
    }

    println!("Made 3 requests");
    println!("Check server logs - each should have a unique request_id");
}

#[tokio::test]
#[ignore = "requires running server"]
async fn test_06_verify_json_format() {
    // This test just makes a request - verify JSON format manually
    let client = reqwest::Client::new();

    let _ = client
        .get(format!("{}/items/1", BASE_URL))
        .send()
        .await;

    println!("Request sent. Verify server output is valid JSON.");
    println!("Expected format:");
    println!(r#"  {{"timestamp":"...","level":"INFO","fields":{{"message":"..."}}}}"#);
}
