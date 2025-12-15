//! Lab 4: Prometheus Metrics Tests
//!
//! These tests require the server to be running on localhost:3000
//! Run with: cargo test -- --ignored

const BASE_URL: &str = "http://localhost:3000";

#[tokio::test]
#[ignore = "requires running server"]
async fn test_01_metrics_endpoint_exists() {
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/metrics", BASE_URL))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 200);

    let content_type = response
        .headers()
        .get("content-type")
        .expect("Missing content-type");

    assert!(
        content_type.to_str().unwrap().contains("text/plain"),
        "Should return text/plain content type"
    );
}

#[tokio::test]
#[ignore = "requires running server"]
async fn test_02_metrics_contain_request_counter() {
    let client = reqwest::Client::new();

    // Make some requests first
    let _ = client.get(format!("{}/items", BASE_URL)).send().await;
    let _ = client.get(format!("{}/items/1", BASE_URL)).send().await;

    // Check metrics
    let response = client
        .get(format!("{}/metrics", BASE_URL))
        .send()
        .await
        .expect("Failed to get metrics");

    let body = response.text().await.expect("Failed to read body");

    assert!(
        body.contains("http_requests_total"),
        "Should contain request counter metric"
    );
    assert!(
        body.contains("# TYPE http_requests_total counter"),
        "Should have TYPE annotation"
    );
}

#[tokio::test]
#[ignore = "requires running server"]
async fn test_03_metrics_contain_histogram() {
    let client = reqwest::Client::new();

    // Make a request to generate histogram data
    let _ = client.get(format!("{}/items", BASE_URL)).send().await;

    // Check metrics
    let response = client
        .get(format!("{}/metrics", BASE_URL))
        .send()
        .await
        .expect("Failed to get metrics");

    let body = response.text().await.expect("Failed to read body");

    assert!(
        body.contains("http_request_duration_seconds"),
        "Should contain duration histogram"
    );
    assert!(
        body.contains("http_request_duration_seconds_bucket"),
        "Should have histogram buckets"
    );
    assert!(
        body.contains("http_request_duration_seconds_sum"),
        "Should have histogram sum"
    );
    assert!(
        body.contains("http_request_duration_seconds_count"),
        "Should have histogram count"
    );
}

#[tokio::test]
#[ignore = "requires running server"]
async fn test_04_metrics_have_labels() {
    let client = reqwest::Client::new();

    // Make requests with different methods/paths
    let _ = client.get(format!("{}/items", BASE_URL)).send().await;
    let _ = client.get(format!("{}/items/1", BASE_URL)).send().await;

    // Check metrics
    let response = client
        .get(format!("{}/metrics", BASE_URL))
        .send()
        .await
        .expect("Failed to get metrics");

    let body = response.text().await.expect("Failed to read body");

    assert!(
        body.contains("method=\"GET\""),
        "Should have method label"
    );
    assert!(
        body.contains("path=\"/items\"") || body.contains("path=\"/items/:id\""),
        "Should have path label"
    );
    assert!(
        body.contains("status=\"200\""),
        "Should have status label"
    );
}

#[tokio::test]
#[ignore = "requires running server"]
async fn test_05_counter_increments() {
    let client = reqwest::Client::new();

    // Get initial metrics
    let initial = client
        .get(format!("{}/metrics", BASE_URL))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    // Make requests
    for _ in 0..5 {
        let _ = client.get(format!("{}/items", BASE_URL)).send().await;
    }

    // Get updated metrics
    let updated = client
        .get(format!("{}/metrics", BASE_URL))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    // Parse and compare (simplified - just check the output changes)
    assert_ne!(initial, updated, "Metrics should change after requests");

    println!("Metrics after 5 requests to /items:");
    for line in updated.lines() {
        if line.contains("http_requests_total") && line.contains("/items") {
            println!("  {}", line);
        }
    }
}

#[tokio::test]
#[ignore = "requires running server"]
async fn test_06_404_status_recorded() {
    let client = reqwest::Client::new();

    // Make a request that returns 404
    let response = client
        .get(format!("{}/items/999", BASE_URL))
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), 404);

    // Check metrics for 404 status
    let metrics = client
        .get(format!("{}/metrics", BASE_URL))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    assert!(
        metrics.contains("status=\"404\""),
        "Should record 404 status in metrics"
    );
}

#[tokio::test]
#[ignore = "requires running server"]
async fn test_07_histogram_buckets() {
    let client = reqwest::Client::new();

    // Make several requests
    for i in 1..=10 {
        let _ = client
            .get(format!("{}/items/{}", BASE_URL, i))
            .send()
            .await;
    }

    // Check histogram buckets
    let metrics = client
        .get(format!("{}/metrics", BASE_URL))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    // Should have multiple bucket boundaries
    let bucket_count = metrics
        .lines()
        .filter(|line| line.contains("http_request_duration_seconds_bucket"))
        .count();

    assert!(
        bucket_count >= 5,
        "Should have multiple histogram buckets, found {}",
        bucket_count
    );

    println!("Found {} histogram buckets", bucket_count);
}
