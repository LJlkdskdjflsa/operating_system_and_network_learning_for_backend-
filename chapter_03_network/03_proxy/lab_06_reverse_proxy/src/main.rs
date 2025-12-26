//! Lab 6: Reverse Proxy
//!
//! ## Goal
//! Build a simple HTTP reverse proxy with round-robin load balancing
//!
//! ## Requirements
//! 1. Listen on port 8080
//! 2. Forward requests to multiple backend servers
//! 3. Implement round-robin load balancing
//! 4. Add X-Forwarded-For header
//! 5. Handle backend failures gracefully
//!
//! ## Architecture
//! ```
//!                                    ┌─> [Backend 1 :8081]
//! [Client] --> [Proxy :8080] ────────┼─> [Backend 2 :8082]
//!                                    └─> [Backend 3 :8083]
//! ```
//!
//! ## Expected Behavior
//! ```bash
//! # Start 2 backends (use echo servers or any HTTP server)
//! # Backend 1:
//! cargo run --manifest-path=../lab_03_raw_http/Cargo.toml &
//!
//! # Start proxy
//! cargo run
//!
//! # Test - requests alternate between backends
//! curl http://localhost:8080/
//! curl http://localhost:8080/
//! ```
//!
//! ## Hints
//! - Read HTTP request from client
//! - Select backend using round-robin
//! - Connect to backend and forward request
//! - Read response from backend
//! - Forward response to client
//! - Use AtomicUsize for round-robin counter
//!
//! ## Acceptance Criteria
//! - [ ] Requests are forwarded to backends
//! - [ ] Round-robin balances across backends
//! - [ ] X-Forwarded-For header is added
//! - [ ] Backend failures don't crash proxy

use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

// ============================================================
// TODO: Implement the reverse proxy
// ============================================================

/// Backend servers to balance across
const BACKENDS: &[&str] = &["127.0.0.1:8081", "127.0.0.1:8082"];

/// Round-robin counter
static COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Select next backend using round-robin
fn next_backend() -> &'static str {
    next_backend_with_count().0
}

/// Select next backend and return the request count
fn next_backend_with_count() -> (&'static str, usize) {
    let count = COUNTER.fetch_add(1, Ordering::Relaxed);
    let idx = count % BACKENDS.len();
    (BACKENDS[idx], count + 1)
}

/// Add x forwarded for
///
/// Add X-Forwarded-For to header in http request  
///
fn add_x_forwarded_for(request: &[u8], client_addr: &str) -> Vec<u8> {
    let delimiter = b"\r\n\r\n";
    let header_end = request.windows(4).position(|w| w == delimiter);
    let Some(end_idx) = header_end else {
        return request.to_vec();
    };

    let (head, body_with_delim) = request.split_at(end_idx);
    let head_str = String::from_utf8_lossy(head);

    // buffer of new HTTP request
    let mut out: Vec<u8> = Vec::with_capacity(request.len() + client_addr.len() + 32);

    let mut added = false;

    for line in head_str.split("\r\n") {
        if !added && line.to_ascii_lowercase().starts_with("x-forwarded-for:") {
            out.extend_from_slice(line.as_bytes());
            out.extend_from_slice(b", ");
            out.extend_from_slice(client_addr.as_bytes());
            out.extend_from_slice(b"\r\n");
            added = true;
        } else {
            out.extend_from_slice(line.as_bytes());
            out.extend_from_slice(b"\r\n");
        }
    }

    if !added {
        out.extend_from_slice(b"X-Forwarded-For: ");
        out.extend_from_slice(client_addr.as_bytes());
        out.extend_from_slice(b"\r\n");
    }

    out.extend_from_slice(b"\r\n");
    out.extend_from_slice(&body_with_delim[4..]);
    out
}
/// Forward request to backend and return response
async fn forward_request(request: &[u8], backend: &str, client_addr: &str) -> Option<Vec<u8>> {
    // TODO: Implement
    // 1. Connect to backend
    let mut backend_stream = TcpStream::connect(backend).await.ok()?;
    // 2. Add/modify X-Forwarded-For header
    let forwarded = add_x_forwarded_for(request, client_addr);
    backend_stream.write_all(&forwarded).await.ok()?;
    // 3. Read response from backend (headers + optional body)
    let mut response = Vec::with_capacity(4096);
    let mut tmp = [0u8; 1024];
    let mut header_end = None;
    while header_end.is_none() {
        let n = backend_stream.read(&mut tmp).await.ok()?;
        if n == 0 {
            return Some(response);
        }
        response.extend_from_slice(&tmp[..n]);
        header_end = response.windows(4).position(|w| w == b"\r\n\r\n");
        if response.len() > 64 * 1024 {
            return Some(response);
        }
    }

    let end_idx = header_end?;
    let header_bytes = &response[..end_idx];
    let header_str = String::from_utf8_lossy(header_bytes);
    let mut content_length = None;
    for line in header_str.split("\r\n") {
        if line.to_ascii_lowercase().starts_with("content-length:") {
            if let Some(v) = line.split(':').nth(1) {
                content_length = v.trim().parse::<usize>().ok();
            }
        }
    }

    if let Some(len) = content_length {
        let expected_len = end_idx + 4 + len;
        while response.len() < expected_len {
            let n = backend_stream.read(&mut tmp).await.ok()?;
            if n == 0 {
                break;
            }
            response.extend_from_slice(&tmp[..n]);
        }
    } else {
        while let Ok(n) = backend_stream.read(&mut tmp).await {
            if n == 0 {
                break;
            }
            response.extend_from_slice(&tmp[..n]);
        }
    }

    Some(response)
}

/// Get client address as a string
fn get_client_address(stream: &TcpStream) -> Option<String> {
    stream.peer_addr().ok().map(|addr| addr.to_string())
}

/// Read an HTTP request from the client stream
async fn read_request(stream: &mut TcpStream) -> Option<Vec<u8>> {
    let mut buf = Vec::with_capacity(4096);
    let mut tmp = [0u8; 1024];
    let mut header_end = None;

    while header_end.is_none() {
        let n = match stream.read(&mut tmp).await {
            Ok(0) => return None,
            Ok(n) => n,
            Err(_) => return None,
        };
        buf.extend_from_slice(&tmp[..n]);
        header_end = buf.windows(4).position(|w| w == b"\r\n\r\n");
        if buf.len() > 64 * 1024 {
            return None;
        }
    }

    let end_idx = header_end?;
    let header_bytes = &buf[..end_idx];
    let header_str = String::from_utf8_lossy(header_bytes);
    let mut content_length = 0usize;
    for line in header_str.split("\r\n") {
        if line.to_ascii_lowercase().starts_with("content-length:") {
            if let Some(v) = line.split(':').nth(1) {
                content_length = v.trim().parse().unwrap_or(0);
            }
        }
    }

    let expected_len = end_idx + 4 + content_length;
    while buf.len() < expected_len {
        let n = match stream.read(&mut tmp).await {
            Ok(0) => break,
            Ok(n) => n,
            Err(_) => return None,
        };
        buf.extend_from_slice(&tmp[..n]);
    }

    Some(buf)
}

/// Handle incoming client connection
async fn handle_client(mut stream: TcpStream) {
    // TODO: Implement
    // 1. Get client address
    let client_addr = match get_client_address(&stream) {
        Some(addr) => addr,
        None => {
            eprintln!("failed to get client address");
            return;
        }
    };
    println!("client connected: {}", client_addr);
    // 2. Read request
    let request = match read_request(&mut stream).await {
        Some(req) => req,
        None => return,
    };
    
    // 3. Select backend
    let (backend, count) = next_backend_with_count();
    println!("round-robin count: {}", count);
    println!("request redirect to backend: {}", backend);
    // 4. Forward request
    let response = match forward_request(&request, backend, &client_addr).await {
        Some(resp) => resp,
        None => {
            let msg = b"HTTP/1.1 502 Bad Gateway\r\nContent-Length: 11\r\n\r\nBad Gateway";
            let _ = stream.write_all(msg).await;
            return;
        }
    };
    // 5. Send response to client
    let _ = stream.write_all(&response).await;
    // 6. Handle errors gracefully
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";

    // TODO: Implement
    // 1. Bind listener
    let listener = TcpListener::bind(addr).await.expect("Failed to bind");
    // 2. Print startup info
    println!("start proxy server at: {:#?}", addr);
    // 3. Accept and handle connections
    loop {
        let (stream, _) = match listener.accept().await {
            Ok(pair) => {
                print!("pair: {:#?}", pair);
                pair
            }
            Err(_) => continue,
        };

        tokio::spawn(async move {
            handle_client(stream).await;
        });
    }
}
