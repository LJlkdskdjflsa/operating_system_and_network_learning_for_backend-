//! Lab 3 Reference Answer

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::time::{SystemTime, UNIX_EPOCH};

/// Simple HTTP request structure
#[derive(Debug)]
struct HttpRequest {
    method: String,
    path: String,
    #[allow(dead_code)]
    headers: Vec<(String, String)>,
    #[allow(dead_code)]
    body: String,
}

/// Parse raw HTTP request bytes into HttpRequest
fn parse_request(raw: &str) -> Option<HttpRequest> {
    let mut lines = raw.split("\r\n");

    // Parse request line: "GET /path HTTP/1.1"
    let request_line = lines.next()?;
    let parts: Vec<&str> = request_line.split_whitespace().collect();

    if parts.len() < 2 {
        return None;
    }

    let method = parts[0].to_string();
    let path = parts[1].to_string();

    // Parse headers
    let mut headers = Vec::new();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        if let Some((key, value)) = line.split_once(": ") {
            headers.push((key.to_string(), value.to_string()));
        }
    }

    // Rest is body
    let body: String = lines.collect::<Vec<&str>>().join("\r\n");

    Some(HttpRequest {
        method,
        path,
        headers,
        body,
    })
}

/// Get reason phrase for status code
fn status_reason(code: u16) -> &'static str {
    match code {
        200 => "OK",
        201 => "Created",
        204 => "No Content",
        400 => "Bad Request",
        404 => "Not Found",
        405 => "Method Not Allowed",
        500 => "Internal Server Error",
        _ => "Unknown",
    }
}

/// Build HTTP response string
fn build_response(status_code: u16, content_type: &str, body: &str) -> String {
    format!(
        "HTTP/1.1 {} {}\r\n\
         Content-Type: {}\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\
         \r\n\
         {}",
        status_code,
        status_reason(status_code),
        content_type,
        body.len(),
        body
    )
}

/// Simple router - match path and extract parameters
fn route(method: &str, path: &str) -> (u16, String, String) {
    // Only handle GET for simplicity
    if method != "GET" {
        return (405, "text/plain".into(), "Method Not Allowed".into());
    }

    // Route: GET /
    if path == "/" {
        return (200, "text/plain".into(), "Hello, World!".into());
    }

    // Route: GET /hello/{name}
    if path.starts_with("/hello/") {
        let name = &path[7..]; // Skip "/hello/"
        if !name.is_empty() {
            return (200, "text/plain".into(), format!("Hello, {}!", name));
        }
    }

    // Route: GET /time
    if path == "/time" {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        return (200, "text/plain".into(), format!("Current timestamp: {}", timestamp));
    }

    // Route: GET /json (example JSON response)
    if path == "/json" {
        let json = r#"{"message": "Hello, JSON!", "status": "ok"}"#;
        return (200, "application/json".into(), json.into());
    }

    // Route: GET /html (example HTML response)
    if path == "/html" {
        let html = r#"<!DOCTYPE html>
<html>
<head><title>Raw HTTP Server</title></head>
<body>
    <h1>Hello from Raw HTTP!</h1>
    <p>This response was built manually.</p>
</body>
</html>"#;
        return (200, "text/html".into(), html.into());
    }

    // 404 for unknown paths
    (404, "text/plain".into(), "404 Not Found".into())
}

/// Handle incoming connection
async fn handle_connection(mut stream: TcpStream) {
    let peer_addr = stream
        .peer_addr()
        .map(|a| a.to_string())
        .unwrap_or_else(|_| "unknown".into());

    // Read request
    let mut buffer = [0u8; 4096];
    let n = match stream.read(&mut buffer).await {
        Ok(0) => return,
        Ok(n) => n,
        Err(e) => {
            eprintln!("[{}] Read error: {}", peer_addr, e);
            return;
        }
    };

    let raw_request = String::from_utf8_lossy(&buffer[..n]);

    // Parse request
    let request = match parse_request(&raw_request) {
        Some(req) => req,
        None => {
            let response = build_response(400, "text/plain", "Bad Request");
            let _ = stream.write_all(response.as_bytes()).await;
            return;
        }
    };

    println!("[{}] {} {}", peer_addr, request.method, request.path);

    // Route and generate response
    let (status, content_type, body) = route(&request.method, &request.path);
    let response = build_response(status, &content_type, &body);

    // Send response
    if let Err(e) = stream.write_all(response.as_bytes()).await {
        eprintln!("[{}] Write error: {}", peer_addr, e);
    }
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";

    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind");

    println!("Raw HTTP Server");
    println!("Listening on http://{}", addr);
    println!("\nAvailable routes:");
    println!("  GET /           -> Hello, World!");
    println!("  GET /hello/NAME -> Hello, NAME!");
    println!("  GET /time       -> Current timestamp");
    println!("  GET /json       -> JSON response");
    println!("  GET /html       -> HTML page");
    println!("\nTest with: curl -v http://localhost:8080/\n");

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                tokio::spawn(async move {
                    handle_connection(stream).await;
                });
            }
            Err(e) => {
                eprintln!("Accept error: {}", e);
            }
        }
    }
}

// Key concepts demonstrated:
//
// 1. HTTP REQUEST FORMAT:
//    - Request line: METHOD PATH HTTP/VERSION
//    - Headers: Key: Value
//    - Empty line (CRLF)
//    - Body (optional)
//
// 2. HTTP RESPONSE FORMAT:
//    - Status line: HTTP/VERSION STATUS_CODE REASON
//    - Headers
//    - Empty line
//    - Body
//
// 3. ROUTING:
//    - Match method and path
//    - Extract path parameters
//    - Return appropriate status codes
//
// 4. CONTENT TYPES:
//    - text/plain for plain text
//    - text/html for HTML
//    - application/json for JSON
//
// This is what frameworks like Axum do under the hood!

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_request() {
        let raw = "GET /hello/world HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let req = parse_request(raw).unwrap();

        assert_eq!(req.method, "GET");
        assert_eq!(req.path, "/hello/world");
        assert_eq!(req.headers.len(), 1);
    }

    #[test]
    fn test_build_response() {
        let response = build_response(200, "text/plain", "Hello");

        assert!(response.starts_with("HTTP/1.1 200 OK"));
        assert!(response.contains("Content-Type: text/plain"));
        assert!(response.contains("Content-Length: 5"));
        assert!(response.ends_with("Hello"));
    }

    #[test]
    fn test_route_root() {
        let (status, _, body) = route("GET", "/");
        assert_eq!(status, 200);
        assert_eq!(body, "Hello, World!");
    }

    #[test]
    fn test_route_hello() {
        let (status, _, body) = route("GET", "/hello/Rust");
        assert_eq!(status, 200);
        assert_eq!(body, "Hello, Rust!");
    }

    #[test]
    fn test_route_404() {
        let (status, _, _) = route("GET", "/nonexistent");
        assert_eq!(status, 404);
    }

    #[test]
    fn test_route_method_not_allowed() {
        let (status, _, _) = route("POST", "/");
        assert_eq!(status, 405);
    }
}
