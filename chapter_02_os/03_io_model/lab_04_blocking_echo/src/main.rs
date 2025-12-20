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
    let peer_addr = stream
        .peer_addr()
        .map(|addr| addr.to_string())
        .unwrap_or_else(|_| "<unkown>".to_string());
    let mut buffer = [0u8; 1024];

    loop {
        let bytes_read = match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => n,
            Err(err) => {
                eprintln!("Read error from {peer_addr}: {err}");
                break;
            }
        };

        if let Err(err) = stream.write_all(&buffer[..bytes_read]) {
            eprintln!("Write error to {peer_addr}: {err}");
            break;
        }
    }
}

fn main() {
    let addr = "127.0.0.1:8080";

    // TODO: Implement
    // 3. Loop accepting connections:
    //    - On accept, print "New connection from {addr}"
    //    - Spawn a thread to handle the client
    //    - (Don't wait for the thread - let it run independently)

    let listener = TcpListener::bind(addr).expect("failed to bind TCP listener");
    println!("Listening on {addr}");

    // let _ = listener;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let peer_addr = stream
                    .peer_addr()
                    .map(|addr| addr.to_string())
                    .unwrap_or_else(|_| "<unknow>".to_string());
                println!("New connection from {peer_addr}");
                thread::spawn(|| handle_client(stream));
            }
            Err(err) => {
                eprint!("Accept error: {err}")
            }
        }
    }
    // todo!("Implement main server loop")
}
