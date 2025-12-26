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

use std::time::{SystemTime, UNIX_EPOCH};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

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
    let mut parts = raw.splitn(2, "\r\n\r\n");
    let head = parts.next()?;
    let body = parts.next().unwrap_or("").to_string();

    let mut lines = head.split("\r\n");
    let request_line = lines.next()?;
    let mut request_parts = request_line.split_whitespace();
    let method = request_parts.next()?.to_string();
    let path = request_parts.next()?.to_string();

    let mut headers = Vec::new();
    for line in lines {
        if let Some((key, value)) = line.split_once(':') {
            headers.push((key.trim().to_string(), value.trim().to_string()));
        }
    }

    Some(HttpRequest {
        method,
        path,
        headers,
        body,
    })
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
    let reason = match status_code {
        200 => "OK",
        400 => "Bad Request",
        404 => "Not Found",
        500 => "Internal Server Error",
        _ => "OK",
    };

    format!(
        "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        status_code,
        reason,
        content_type,
        body.as_bytes().len(),
        body
    )
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
    let mut buffer = [0u8; 4096];
    // print!("buffer before read: {:#?}", buffer);
    let bytes_read = match stream.read(&mut buffer).await {
        Ok(0) => return,
        Ok(n) => n,
        Err(_) => return,
    };
    // print!("buffer after read: {:#?}", buffer);
    let raw_request = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("raw request:\n {:#?}", raw_request);

    let request = match parse_request(&raw_request) {
        Some(req) => req,
        None => {
            let response = build_response(400, "text/plain", "400 Bad Request");
            let _ = stream.write_all(response.as_bytes()).await;
            println!("bad request");
            return;
        }
    };
    println!("method={} path={}", request.method, request.path);
    let (status_code, body) = if request.method == "GET" && request.path == "/" {
        (200, "Hello, World!".to_string())
    } else if request.method == "GET" && request.path.starts_with("/hello/") {
        let name = request.path.trim_start_matches("/hello/");
        if name.is_empty() {
            (404, "404 Not Found".to_string())
        } else {
            (200, format!("Hello, {}!", name))
        }
    } else if request.method == "GET" && request.path == "/time" {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        (200, format!("Current time: {}", now.as_secs()))
    } else {
        (404, "404 Not Found".to_string())
    };

    let response = build_response(status_code, "text/plain", &body);
    let _ = stream.write_all(response.as_bytes()).await;
}

#[tokio::main]
async fn main() {
    let port = std::env::args().nth(1).unwrap_or_else(|| "8080".to_string());
    let addr = format!("127.0.0.1:{}", port);

    println!("start server at: {:#?}", addr);
    // TODO: Implement
    // 1. Bind listener
    let listener = TcpListener::bind(addr).await.expect("Failed to bind");
    loop {
        let (stream, _) = match listener.accept().await {
            Ok(pair) => {
                print!("pair: {:#?}", pair);
                pair
            }
            Err(_) => continue,
        };

        tokio::spawn(async move {
            handle_connection(stream).await;
        });
    }
    // 2. Accept connections
    // 3. Spawn handler for each
}
