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

use std::net::SocketAddr;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;

// ============================================================
// TODO: Implement the chat server
// ============================================================

/// Handle a single client connection
async fn handle_client(
    stream: TcpStream,
    addr: SocketAddr,
    sender: broadcast::Sender<(String, SocketAddr)>,
) {
    // Subscribe to receive broadcast messages
    let mut rx = sender.subscribe();

    // Split the stream into reader and writer
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    // Notify about new connection
    let join_msg = format!("[{}] joined the chat\n", addr);
    println!("{}", join_msg.trim());
    let _ = sender.send((join_msg, addr));

    loop {
        tokio::select! {
            // Read from client
            result = reader.read_line(&mut line) => {
                match result {
                    Ok(0) => {
                        // Client disconnected
                        let leave_msg = format!("[{}] left the chat\n", addr);
                        println!("{}", leave_msg.trim());
                        let _ = sender.send((leave_msg, addr));
                        break;
                    }
                    Ok(_) => {
                        // Broadcast message to all clients
                        let msg = format!("[{}]: {}", addr, line);
                        println!("{}", msg.trim());
                        let _ = sender.send((msg, addr));
                        line.clear();
                    }
                    Err(err) => {
                        eprintln!("[{}] Read error: {}", addr, err);
                        break;
                    }
                }
            }

            // Receive broadcast messages and send to this client
            result = rx.recv() => {
                match result {
                    Ok((msg, sender_addr)) => {
                        // Don't send message back to sender
                        if sender_addr != addr {
                            if let Err(err) = writer.write_all(msg.as_bytes()).await {
                                eprintln!("[{}] Write error: {}", addr, err);
                                break;
                            }
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(n)) => {
                        eprintln!("[{}] Lagged {} messages", addr, n);
                    }
                    Err(broadcast::error::RecvError::Closed) => {
                        break;
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // let addr = "127.0.0.1:8080";
    let addr = "0.0.0.0:8080";


    // Create a broadcast channel for message distribution
    let (tx, _rx) = broadcast::channel::<(String, SocketAddr)>(100);

    let listener = TcpListener::bind(addr).await.expect("Failed to bind");

    println!("TCP Chat Server");
    println!("Listening on {}", addr);
    println!("\nTest with: nc localhost 8080");
    println!("Open multiple terminals to chat!\n");
    // 2. Create TcpListener
    // 3. Loop accepting connections
    loop {
        match listener.accept().await {
            Ok((stream, client_addr)) => {
                println!("New connection from {}", client_addr);

                let tx_clone = tx.clone();
                tokio::spawn(async move {
                    handle_client(stream, client_addr, tx_clone).await;
                });
            }
            Err(err) => {
                eprintln!("Accept error: {}", err);
            }
        }
    }
    // 4. Spawn handle_client for each connection
}
