//! Lab 1: TCP Chat Server
//!
//! ## Goal
//! Build a multi-client TCP chat server where messages from one client
//! are broadcast to all other connected clients.
//!
//! ## Requirements
//! 1. Listen on TCP port 8080
//! 2. Accept multiple clients concurrently
//! 3. When a client sends a message, broadcast it to all other clients
//! 4. Handle client disconnection gracefully
//! 5. Show connection/disconnection notifications
//!
//! ## Expected Behavior
//! ```
//! $ nc localhost 8080  # Client 1
//! > hello everyone!
//!
//! $ nc localhost 8080  # Client 2
//! [user123]: hello everyone!
//! > hi there!
//!
//! # Client 1 sees:
//! [user456]: hi there!
//! ```
//!
//! ## Hints
//! - Use `Arc<Mutex<HashMap>>` or `tokio::sync::broadcast` for shared state
//! - Each client needs a unique ID
//! - Use `tokio::spawn` for each client handler
//! - Consider using channels for message distribution
//!
//! ## Verification
//! ```bash
//! cargo run                    # Start server
//! # Open multiple terminals:
//! nc localhost 8080           # Connect clients
//! # Type in one, see in others
//! ```
//!
//! ## Acceptance Criteria
//! - [ ] Multiple clients can connect
//! - [ ] Messages are broadcast to all other clients
//! - [ ] Disconnection is handled gracefully
//! - [ ] Connection notifications shown

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::broadcast;
use std::net::SocketAddr;

// ============================================================
// TODO: Implement the chat server
// ============================================================

/// Handle a single client connection
async fn handle_client(
    stream: TcpStream,
    addr: SocketAddr,
    tx: broadcast::Sender<(String, SocketAddr)>,
) {
    // TODO: Implement
    // 1. Subscribe to the broadcast channel
    // 2. Split the stream into reader and writer
    // 3. Spawn two tasks:
    //    a. Read from client, broadcast to channel
    //    b. Receive from channel, write to client (skip own messages)
    // 4. Handle disconnection

    todo!("Implement handle_client")
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";

    // TODO: Implement
    // 1. Create a broadcast channel for message distribution
    // 2. Create TcpListener
    // 3. Loop accepting connections
    // 4. Spawn handle_client for each connection

    todo!("Implement main server loop")
}
