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
    let peer_address = stream
        .peer_addr()
        .map(|addr| addr.to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    println!("[{}]", peer_address);
    // 2. Create a buffer for reading
    let mut buffer = [0u8; 1024];
    // 3. Loop:
    loop {
        //    - Read data from stream with .await
        match stream.read(&mut buffer).await {
            Ok(0) => {
                println!("[{}] Dissconnected", peer_address);
                break;
            }
            Ok(n) => {
                if let Err(err) = stream.write_all(&buffer[..n]).await {
                    println!("[{}] Write erro{}", peer_address, err);
                }
            }
            Err(err) => {
                println!("[{}] Read erro{}", peer_address, err);
            }
        }
        //    - If read returns 0, client disconnected - break
        //    - Echo data back with write_all().await
        // 4. Log when connection closes
    }
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

    let listener = TcpListener::bind(addr).await.expect("Failed to bind");

    println!("Async Echo Server (Tokio)");
    println!("Listening on {}", addr);

    loop {
        match listener.accept().await {
            Ok((_socket, addr)) => {
                println!("New connection from {}", addr);
                tokio::spawn(async move {
                    handle_client(_socket).await;
                });
            }
            Err(err) => {
                eprintln!("Accept error: {}", err);
            }
        }
    }
}
