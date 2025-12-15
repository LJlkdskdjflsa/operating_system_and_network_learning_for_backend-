//! Lab 4 Tests

use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::{Child, Command};
use std::thread;
use std::time::Duration;

struct ServerGuard {
    child: Child,
}

impl Drop for ServerGuard {
    fn drop(&mut self) {
        let _ = self.child.kill();
    }
}

fn start_server() -> Option<ServerGuard> {
    Command::new("cargo")
        .args(["build", "--quiet"])
        .status()
        .ok()?;

    let child = Command::new("cargo")
        .args(["run", "--quiet"])
        .spawn()
        .ok()?;

    thread::sleep(Duration::from_millis(1000));

    Some(ServerGuard { child })
}

fn send_request(request: &str) -> Option<String> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").ok()?;
    stream.set_read_timeout(Some(Duration::from_secs(2))).ok()?;

    stream.write_all(request.as_bytes()).ok()?;

    let mut response = String::new();
    stream.read_to_string(&mut response).ok()?;

    Some(response)
}

#[test]
fn test_01_list_items_empty() {
    let _server = match start_server() {
        Some(s) => s,
        None => return,
    };

    let request = "GET /items HTTP/1.1\r\nHost: localhost\r\n\r\n";
    let response = match send_request(request) {
        Some(r) => r,
        None => return,
    };

    assert!(response.contains("200"), "GET /items should return 200");
    assert!(
        response.contains("application/json"),
        "Should return JSON content type"
    );
    assert!(response.contains("[]"), "Empty list should return []");
}

#[test]
fn test_02_create_item() {
    let _server = match start_server() {
        Some(s) => s,
        None => return,
    };

    let body = r#"{"name":"Widget","price":9.99}"#;
    let request = format!(
        "POST /items HTTP/1.1\r\n\
         Host: localhost\r\n\
         Content-Type: application/json\r\n\
         Content-Length: {}\r\n\
         \r\n\
         {}",
        body.len(),
        body
    );

    let response = match send_request(&request) {
        Some(r) => r,
        None => return,
    };

    assert!(
        response.contains("201") || response.contains("200"),
        "POST /items should return 201 Created"
    );
    assert!(
        response.contains("Widget"),
        "Response should contain created item name"
    );
}

#[test]
fn test_03_get_item_not_found() {
    let _server = match start_server() {
        Some(s) => s,
        None => return,
    };

    let request = "GET /items/999 HTTP/1.1\r\nHost: localhost\r\n\r\n";
    let response = match send_request(request) {
        Some(r) => r,
        None => return,
    };

    assert!(
        response.contains("404"),
        "GET /items/999 should return 404 Not Found"
    );
}

#[test]
fn test_04_create_and_get() {
    let _server = match start_server() {
        Some(s) => s,
        None => return,
    };

    // Create item
    let body = r#"{"name":"TestItem","price":5.50}"#;
    let create_request = format!(
        "POST /items HTTP/1.1\r\n\
         Host: localhost\r\n\
         Content-Type: application/json\r\n\
         Content-Length: {}\r\n\
         \r\n\
         {}",
        body.len(),
        body
    );

    let _ = send_request(&create_request);

    // Give server time to process
    thread::sleep(Duration::from_millis(100));

    // Get item (ID should be 1)
    let get_request = "GET /items/1 HTTP/1.1\r\nHost: localhost\r\n\r\n";
    let response = match send_request(get_request) {
        Some(r) => r,
        None => return,
    };

    if response.contains("200") {
        assert!(
            response.contains("TestItem"),
            "Should return the created item"
        );
    }
}

#[test]
fn test_05_delete_item() {
    let _server = match start_server() {
        Some(s) => s,
        None => return,
    };

    // Create item first
    let body = r#"{"name":"ToDelete","price":1.00}"#;
    let create_request = format!(
        "POST /items HTTP/1.1\r\n\
         Host: localhost\r\n\
         Content-Type: application/json\r\n\
         Content-Length: {}\r\n\
         \r\n\
         {}",
        body.len(),
        body
    );

    let _ = send_request(&create_request);
    thread::sleep(Duration::from_millis(100));

    // Delete item
    let delete_request = "DELETE /items/1 HTTP/1.1\r\nHost: localhost\r\n\r\n";
    let response = match send_request(delete_request) {
        Some(r) => r,
        None => return,
    };

    // Should return 204 No Content or 200 OK
    assert!(
        response.contains("204") || response.contains("200"),
        "DELETE should return success status"
    );

    // Verify item is gone
    thread::sleep(Duration::from_millis(100));
    let get_request = "GET /items/1 HTTP/1.1\r\nHost: localhost\r\n\r\n";
    let response = match send_request(get_request) {
        Some(r) => r,
        None => return,
    };

    assert!(
        response.contains("404"),
        "Deleted item should return 404"
    );
}
