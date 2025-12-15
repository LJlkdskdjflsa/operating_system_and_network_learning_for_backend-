//! Lab 3: Raw HTTP Server
//!
//! ## Goal
//! Build an HTTP server from scratch to understand the protocol
//!
//! ## Requirements
//! 1. Listen on port 8080
//! 2. Parse HTTP requests manually
//! 3. Route based on method and path
//! 4. Return proper HTTP responses with headers
//!
//! ## Expected Behavior
//! ```
//! $ curl http://localhost:8080/
//! Hello, World!
//!
//! $ curl http://localhost:8080/hello/John
//! Hello, John!
//!
//! $ curl http://localhost:8080/time
//! Current time: 2024-12-15 10:00:00
//!
//! $ curl http://localhost:8080/nonexistent
//! 404 Not Found
//! ```
//!
//! ## Hints
//! - Read raw bytes from TCP stream
//! - Parse request line: "GET /path HTTP/1.1"
//! - Parse headers until empty line
//! - Build response with status line + headers + body
//! - Remember CRLF (\r\n) line endings!
//!
//! ## Verification
//! ```bash
//! cargo run
//! curl -v http://localhost:8080/
//! curl -v http://localhost:8080/hello/World
//! ```
//!
//! ## Acceptance Criteria
//! - [ ] Parses HTTP request line correctly
//! - [ ] Routes to different handlers based on path
//! - [ ] Returns proper HTTP response format
//! - [ ] Handles 404 for unknown paths

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// ============================================================
// TODO: Implement the raw HTTP server
// ============================================================

/// Simple HTTP request structure
struct HttpRequest {
    method: String,
    path: String,
    headers: Vec<(String, String)>,
    body: String,
}

/// Parse raw HTTP request bytes into HttpRequest
fn parse_request(raw: &str) -> Option<HttpRequest> {
    // TODO: Implement
    // 1. Split by \r\n
    // 2. First line is request line: "METHOD PATH VERSION"
    // 3. Following lines are headers until empty line
    // 4. Rest is body

    todo!("Implement parse_request")
}

/// Build HTTP response string
fn build_response(status_code: u16, content_type: &str, body: &str) -> String {
    // TODO: Implement
    // Format:
    // HTTP/1.1 {status_code} {reason}\r\n
    // Content-Type: {content_type}\r\n
    // Content-Length: {body.len()}\r\n
    // Connection: close\r\n
    // \r\n
    // {body}

    todo!("Implement build_response")
}

/// Handle incoming connection
async fn handle_connection(mut stream: TcpStream) {
    // TODO: Implement
    // 1. Read request into buffer
    // 2. Parse request
    // 3. Route to appropriate handler:
    //    - GET / -> "Hello, World!"
    //    - GET /hello/{name} -> "Hello, {name}!"
    //    - GET /time -> current time
    //    - * -> 404 Not Found
    // 4. Send response

    todo!("Implement handle_connection")
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";

    // TODO: Implement
    // 1. Bind listener
    // 2. Accept connections
    // 3. Spawn handler for each

    todo!("Implement main")
}
