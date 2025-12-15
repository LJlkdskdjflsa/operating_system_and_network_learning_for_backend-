//! Lab 4: Blocking Echo Server
//!
//! ## Goal
//! Build a thread-per-connection echo server to understand blocking I/O
//!
//! ## Requirements
//! 1. Listen on a TCP port (default: 8080)
//! 2. Accept incoming connections
//! 3. For each connection, spawn a thread to handle it
//! 4. Echo back whatever the client sends
//! 5. Handle client disconnection gracefully
//!
//! ## Expected Behavior
//! ```
//! $ cargo run
//! Listening on 127.0.0.1:8080
//! New connection from 127.0.0.1:54321
//! Connection closed: 127.0.0.1:54321
//! ```
//!
//! ## Testing
//! In another terminal:
//! ```bash
//! # Using netcat
//! nc localhost 8080
//! hello    # Type this
//! hello    # Server echoes back
//!
//! # Or using telnet
//! telnet localhost 8080
//! ```
//!
//! ## Hints
//! - Use `TcpListener::bind()` to create the server
//! - Use `listener.accept()` to accept connections (blocks until a client connects)
//! - Use `thread::spawn()` to handle each connection
//! - Use `stream.read()` and `stream.write_all()` for I/O
//! - Handle `read()` returning 0 (client disconnected)
//!
//! ## Verification
//! ```bash
//! cargo run                          # Start server
//! # In another terminal:
//! nc localhost 8080                  # Connect client
//! htop                               # Watch thread count grow
//! ```
//!
//! ## Acceptance Criteria
//! - [ ] Server accepts multiple clients
//! - [ ] Each client gets its own thread (visible in htop)
//! - [ ] Echo works correctly
//! - [ ] Clean disconnect when client closes
//!
//! Check solution/main.rs after completing

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

// ============================================================
// TODO: Implement the echo server
// ============================================================

/// Handle a single client connection
fn handle_client(mut stream: TcpStream) {
    // TODO: Implement
    // 1. Get client address for logging
    // 2. Create a buffer for reading
    // 3. Loop:
    //    - Read data from stream
    //    - If read returns 0, client disconnected - break
    //    - Echo data back (write_all)
    // 4. Log when connection closes

    todo!("Implement handle_client")
}

fn main() {
    let addr = "127.0.0.1:8080";

    // TODO: Implement
    // 1. Create TcpListener bound to addr
    // 2. Print "Listening on {addr}"
    // 3. Loop accepting connections:
    //    - On accept, print "New connection from {addr}"
    //    - Spawn a thread to handle the client
    //    - (Don't wait for the thread - let it run independently)

    todo!("Implement main server loop")
}
