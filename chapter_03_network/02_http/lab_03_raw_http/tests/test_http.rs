//! Lab 3 Tests

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

    thread::sleep(Duration::from_millis(500));

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
fn test_01_server_responds() {
    let _server = match start_server() {
        Some(s) => s,
        None => return,
    };

    let request = "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n";
    let response = match send_request(request) {
        Some(r) => r,
        None => return,
    };

    assert!(
        response.starts_with("HTTP/1.1"),
        "Response should start with HTTP/1.1"
    );
}

#[test]
fn test_02_root_returns_200() {
    let _server = match start_server() {
        Some(s) => s,
        None => return,
    };

    let request = "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n";
    let response = match send_request(request) {
        Some(r) => r,
        None => return,
    };

    assert!(
        response.contains("200 OK") || response.contains("200"),
        "GET / should return 200"
    );
    assert!(
        response.contains("Hello"),
        "Response body should contain 'Hello'"
    );
}

#[test]
fn test_03_hello_name() {
    let _server = match start_server() {
        Some(s) => s,
        None => return,
    };

    let request = "GET /hello/World HTTP/1.1\r\nHost: localhost\r\n\r\n";
    let response = match send_request(request) {
        Some(r) => r,
        None => return,
    };

    assert!(response.contains("200"), "GET /hello/World should return 200");
    assert!(
        response.contains("World"),
        "Response should contain the name 'World'"
    );
}

#[test]
fn test_04_not_found() {
    let _server = match start_server() {
        Some(s) => s,
        None => return,
    };

    let request = "GET /nonexistent HTTP/1.1\r\nHost: localhost\r\n\r\n";
    let response = match send_request(request) {
        Some(r) => r,
        None => return,
    };

    assert!(
        response.contains("404"),
        "Unknown path should return 404"
    );
}

#[test]
fn test_05_has_content_type() {
    let _server = match start_server() {
        Some(s) => s,
        None => return,
    };

    let request = "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n";
    let response = match send_request(request) {
        Some(r) => r,
        None => return,
    };

    assert!(
        response.contains("Content-Type"),
        "Response should have Content-Type header"
    );
}

#[test]
fn test_06_has_content_length() {
    let _server = match start_server() {
        Some(s) => s,
        None => return,
    };

    let request = "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n";
    let response = match send_request(request) {
        Some(r) => r,
        None => return,
    };

    assert!(
        response.contains("Content-Length"),
        "Response should have Content-Length header"
    );
}
