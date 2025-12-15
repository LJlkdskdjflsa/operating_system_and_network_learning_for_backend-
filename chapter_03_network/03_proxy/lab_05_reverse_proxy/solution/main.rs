//! Lab 5 Reference Answer

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::atomic::{AtomicUsize, Ordering};

/// Backend servers to balance across
const BACKENDS: &[&str] = &[
    "127.0.0.1:8081",
    "127.0.0.1:8082",
];

/// Round-robin counter
static COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Request counter for logging
static REQUEST_COUNT: AtomicUsize = AtomicUsize::new(0);

/// Select next backend using round-robin
fn next_backend() -> &'static str {
    let index = COUNTER.fetch_add(1, Ordering::Relaxed);
    BACKENDS[index % BACKENDS.len()]
}

/// Add X-Forwarded-For header to HTTP request
fn add_forwarded_header(request: &[u8], client_addr: &str) -> Vec<u8> {
    let request_str = String::from_utf8_lossy(request);

    // Find the end of the first line (after request line)
    if let Some(pos) = request_str.find("\r\n") {
        let (first_line, rest) = request_str.split_at(pos + 2);

        // Check if X-Forwarded-For already exists
        let header = if rest.to_lowercase().contains("x-forwarded-for:") {
            // Append to existing header (would need more complex parsing)
            format!("{}X-Forwarded-For: {}\r\n{}", first_line, client_addr, rest)
        } else {
            // Add new header
            format!("{}X-Forwarded-For: {}\r\n{}", first_line, client_addr, rest)
        };

        header.into_bytes()
    } else {
        request.to_vec()
    }
}

/// Forward request to backend and return response
async fn forward_request(
    request: &[u8],
    backend: &str,
    client_addr: &str,
) -> Option<Vec<u8>> {
    // Connect to backend
    let mut backend_conn = match TcpStream::connect(backend).await {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Failed to connect to backend {}: {}", backend, e);
            return None;
        }
    };

    // Add X-Forwarded-For header
    let modified_request = add_forwarded_header(request, client_addr);

    // Send request to backend
    if let Err(e) = backend_conn.write_all(&modified_request).await {
        eprintln!("Failed to send request to backend: {}", e);
        return None;
    }

    // Read response from backend
    let mut response = Vec::new();
    let mut buffer = [0u8; 4096];

    // Read with timeout-like behavior (read until no more data)
    loop {
        match tokio::time::timeout(
            std::time::Duration::from_millis(500),
            backend_conn.read(&mut buffer),
        )
        .await
        {
            Ok(Ok(0)) => break, // Connection closed
            Ok(Ok(n)) => response.extend_from_slice(&buffer[..n]),
            Ok(Err(e)) => {
                eprintln!("Error reading from backend: {}", e);
                break;
            }
            Err(_) => break, // Timeout - assume response complete
        }
    }

    if response.is_empty() {
        None
    } else {
        Some(response)
    }
}

/// Generate error response
fn error_response(status: u16, message: &str) -> Vec<u8> {
    let reason = match status {
        502 => "Bad Gateway",
        503 => "Service Unavailable",
        _ => "Error",
    };

    format!(
        "HTTP/1.1 {} {}\r\n\
         Content-Type: text/plain\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\
         \r\n\
         {}",
        status,
        reason,
        message.len(),
        message
    )
    .into_bytes()
}

/// Handle incoming client connection
async fn handle_client(mut stream: TcpStream) {
    let client_addr = stream
        .peer_addr()
        .map(|a| a.to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    let request_num = REQUEST_COUNT.fetch_add(1, Ordering::Relaxed) + 1;

    // Read request
    let mut buffer = [0u8; 4096];
    let n = match stream.read(&mut buffer).await {
        Ok(0) => return,
        Ok(n) => n,
        Err(e) => {
            eprintln!("[{}] Read error: {}", client_addr, e);
            return;
        }
    };

    // Select backend
    let backend = next_backend();

    // Log the request
    let request_line = String::from_utf8_lossy(&buffer[..n])
        .lines()
        .next()
        .unwrap_or("")
        .to_string();
    println!(
        "[Request #{}] {} -> {} | {}",
        request_num, client_addr, backend, request_line
    );

    // Forward to backend
    let response = match forward_request(&buffer[..n], backend, &client_addr).await {
        Some(resp) => {
            println!("[Request #{}] Backend responded ({} bytes)", request_num, resp.len());
            resp
        }
        None => {
            println!("[Request #{}] Backend failed, returning 502", request_num);
            error_response(502, "Bad Gateway - Backend unavailable")
        }
    };

    // Send response to client
    if let Err(e) = stream.write_all(&response).await {
        eprintln!("[{}] Write error: {}", client_addr, e);
    }
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";

    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind");

    println!("Reverse Proxy Server");
    println!("Listening on http://{}", addr);
    println!("\nBackends (round-robin):");
    for backend in BACKENDS {
        println!("  - {}", backend);
    }
    println!("\nStart backends with:");
    println!("  # Terminal 1:");
    println!("  cd ../lab_03_raw_http && cargo run  # Port 8081");
    println!("  # Terminal 2:");
    println!("  # Start another HTTP server on port 8082");
    println!("\nTest with: curl http://localhost:8080/\n");

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                tokio::spawn(async move {
                    handle_client(stream).await;
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
// 1. ROUND-ROBIN LOAD BALANCING:
//    - AtomicUsize counter for thread-safe selection
//    - Modulo operation to cycle through backends
//    - Each request goes to next server in list
//
// 2. REQUEST FORWARDING:
//    - Read request from client
//    - Connect to selected backend
//    - Send request to backend
//    - Read and forward response
//
// 3. HEADER MODIFICATION:
//    - Add X-Forwarded-For header
//    - Preserves client IP for logging/security
//
// 4. ERROR HANDLING:
//    - Backend connection failures return 502
//    - Proxy stays running even if backends fail
//
// Improvements for production:
// - Connection pooling (reuse backend connections)
// - Health checks (remove unhealthy backends)
// - Request timeout handling
// - Keep-alive support
// - Proper HTTP/1.1 parsing
// - WebSocket support

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_robin() {
        // Reset counter
        COUNTER.store(0, Ordering::Relaxed);

        assert_eq!(next_backend(), "127.0.0.1:8081");
        assert_eq!(next_backend(), "127.0.0.1:8082");
        assert_eq!(next_backend(), "127.0.0.1:8081");
        assert_eq!(next_backend(), "127.0.0.1:8082");
    }

    #[test]
    fn test_add_forwarded_header() {
        let request = b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let modified = add_forwarded_header(request, "192.168.1.1");
        let modified_str = String::from_utf8_lossy(&modified);

        assert!(modified_str.contains("X-Forwarded-For: 192.168.1.1"));
    }

    #[test]
    fn test_error_response() {
        let response = error_response(502, "Backend down");
        let response_str = String::from_utf8_lossy(&response);

        assert!(response_str.starts_with("HTTP/1.1 502"));
        assert!(response_str.contains("Bad Gateway"));
        assert!(response_str.contains("Backend down"));
    }
}
