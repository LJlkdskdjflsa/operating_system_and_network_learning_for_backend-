//! Lab 2: UDP Echo Server
//!
//! ## Goal
//! Build a UDP echo server and understand datagram-based communication
//!
//! ## Requirements
//! 1. Listen on UDP port 8080
//! 2. Receive datagrams from any client
//! 3. Echo each datagram back to its sender
//! 4. Print statistics (packets received, bytes processed)
//!
//! ## Expected Behavior
//! ```
//! # Terminal 1: Start server
//! $ cargo run
//! UDP Echo Server listening on 127.0.0.1:8080
//!
//! # Terminal 2: Send UDP packets
//! $ echo "hello" | nc -u localhost 8080
//! hello
//!
//! # Server shows:
//! Received 6 bytes from 127.0.0.1:54321
//! ```
//!
//! ## Key Differences from TCP
//! - No connection establishment
//! - Each recv_from gets exactly one datagram
//! - Must track sender address to reply
//! - No guaranteed delivery or ordering
//!
//! ## Hints
//! - Use `UdpSocket::bind()` to create socket
//! - Use `recv_from()` to get data AND sender address
//! - Use `send_to()` to send response to specific address
//! - No connection tracking needed!
//!
//! ## Verification
//! ```bash
//! cargo run                              # Start server
//! echo "test" | nc -u localhost 8080     # Send UDP
//! ```
//!
//! ## Acceptance Criteria
//! - [ ] Server receives UDP datagrams
//! - [ ] Echoes back to sender
//! - [ ] Shows packet statistics
//! - [ ] Handles multiple clients (no connection state)

use std::sync::atomic::{AtomicU64, Ordering};
use tokio::net::UdpSocket;

// ============================================================
// TODO: Implement the UDP echo server
// ============================================================

// Statistics counters
static PACKETS_RECEIVED: AtomicU64 = AtomicU64::new(0);
static BYTES_PROCESSED: AtomicU64 = AtomicU64::new(0);

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";

    // TODO: Implement
    let socket = UdpSocket::bind(addr)
        .await
        .expect("bind failed");
    // 1. Create UdpSocket bound to addr
    // 2. Loop:
    //    - recv_from() to get datagram and sender address
    //    - Update statistics
    //    - Print received data info
    //    - send_to() to echo back to sender
    // 3. Handle errors gracefully
    let mut buf = [0u8; 2048];

    loop {
        match socket.recv_from(&mut buf).await {
            Ok((len, peer)) => {
                PACKETS_RECEIVED.fetch_add(1, Ordering::Relaxed);
                BYTES_PROCESSED.fetch_add(len as u64, Ordering::Relaxed);

                let msg = String::from_utf8_lossy(&buf[..len]);
                println!("Received {} bytes from {}: {}", len, peer, msg);

                match socket.send_to(&buf[..len], &peer).await {
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!("send_to error: {}", err);
                    }
                }
            }
            Err(err) => {
                eprintln!("recv_from error: {}", err);
            }
        }
    }
}
