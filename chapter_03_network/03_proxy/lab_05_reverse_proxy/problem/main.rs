//! Lab 5: Reverse Proxy
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

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::atomic::{AtomicUsize, Ordering};

// ============================================================
// TODO: Implement the reverse proxy
// ============================================================

/// Backend servers to balance across
const BACKENDS: &[&str] = &[
    "127.0.0.1:8081",
    "127.0.0.1:8082",
];

/// Round-robin counter
static COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Select next backend using round-robin
fn next_backend() -> &'static str {
    // TODO: Implement round-robin selection
    todo!("Implement next_backend")
}

/// Forward request to backend and return response
async fn forward_request(
    request: &[u8],
    backend: &str,
    client_addr: &str,
) -> Option<Vec<u8>> {
    // TODO: Implement
    // 1. Connect to backend
    // 2. Add/modify X-Forwarded-For header
    // 3. Send request to backend
    // 4. Read response from backend
    // 5. Return response

    todo!("Implement forward_request")
}

/// Handle incoming client connection
async fn handle_client(mut stream: TcpStream) {
    // TODO: Implement
    // 1. Get client address
    // 2. Read request
    // 3. Select backend
    // 4. Forward request
    // 5. Send response to client
    // 6. Handle errors gracefully

    todo!("Implement handle_client")
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";

    // TODO: Implement
    // 1. Bind listener
    // 2. Print startup info
    // 3. Accept and handle connections

    todo!("Implement main")
}
