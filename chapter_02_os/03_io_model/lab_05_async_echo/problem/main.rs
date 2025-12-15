//! Lab 5: Async Echo Server (Tokio)
//!
//! ## Goal
//! Build an async echo server using Tokio to understand async I/O
//!
//! ## Requirements
//! 1. Listen on a TCP port (default: 8080)
//! 2. Accept incoming connections asynchronously
//! 3. For each connection, spawn an async task (not a thread!)
//! 4. Echo back whatever the client sends
//! 5. Handle client disconnection gracefully
//!
//! ## Expected Behavior
//! Same as blocking server, but with fewer OS threads!
//!
//! ## Key Differences from Blocking Version
//! - Uses `tokio::net::TcpListener` instead of `std::net::TcpListener`
//! - Uses `tokio::spawn` instead of `std::thread::spawn`
//! - Uses `.await` for I/O operations
//! - Many connections share few threads
//!
//! ## Hints
//! - Use `#[tokio::main]` attribute on main
//! - Use `TcpListener::bind().await` to create listener
//! - Use `listener.accept().await` to accept connections
//! - Use `AsyncReadExt::read()` and `AsyncWriteExt::write_all()`
//! - Use `tokio::spawn()` to spawn async tasks
//!
//! ## Verification
//! ```bash
//! cargo run                          # Start server
//! # In another terminal:
//! nc localhost 8080                  # Connect client
//! htop                               # Notice: still few threads!
//! ```
//!
//! ## Acceptance Criteria
//! - [ ] Server accepts multiple clients
//! - [ ] Thread count stays low even with many connections (htop)
//! - [ ] Echo works correctly
//! - [ ] Uses async/await (not blocking calls)
//!
//! Check solution/main.rs after completing

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

// ============================================================
// TODO: Implement the async echo server
// ============================================================

/// Handle a single client connection (async version)
async fn handle_client(mut stream: TcpStream) {
    // TODO: Implement (very similar to blocking version, but with .await)
    // 1. Get client address for logging
    // 2. Create a buffer for reading
    // 3. Loop:
    //    - Read data from stream with .await
    //    - If read returns 0, client disconnected - break
    //    - Echo data back with write_all().await
    // 4. Log when connection closes

    todo!("Implement async handle_client")
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";

    // TODO: Implement
    // 1. Create TcpListener bound to addr (use .await)
    // 2. Print "Listening on {addr}"
    // 3. Loop accepting connections:
    //    - On accept (use .await), print "New connection from {addr}"
    //    - Spawn an async task with tokio::spawn() to handle the client
    //    - (The task runs concurrently, not in a new thread)

    todo!("Implement async main server loop")
}
