//! Lab 2 Reference Answer

use tokio::net::UdpSocket;
use std::sync::atomic::{AtomicU64, Ordering};

// Statistics counters
static PACKETS_RECEIVED: AtomicU64 = AtomicU64::new(0);
static BYTES_PROCESSED: AtomicU64 = AtomicU64::new(0);

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";

    // Create UDP socket
    let socket = UdpSocket::bind(addr)
        .await
        .expect("Failed to bind UDP socket");

    println!("UDP Echo Server");
    println!("Listening on {}", addr);
    println!("\nTest with: echo 'hello' | nc -u localhost 8080");
    println!("Or: nc -u localhost 8080  (then type and press Enter)\n");

    let mut buffer = [0u8; 65535]; // Max UDP datagram size

    loop {
        // Receive datagram and sender address
        match socket.recv_from(&mut buffer).await {
            Ok((n, src_addr)) => {
                // Update statistics
                let packets = PACKETS_RECEIVED.fetch_add(1, Ordering::Relaxed) + 1;
                let bytes = BYTES_PROCESSED.fetch_add(n as u64, Ordering::Relaxed) + n as u64;

                // Log the received data
                let data = String::from_utf8_lossy(&buffer[..n]);
                println!(
                    "[Packet #{}] Received {} bytes from {}: {:?}",
                    packets,
                    n,
                    src_addr,
                    data.trim()
                );

                // Echo back to sender
                match socket.send_to(&buffer[..n], src_addr).await {
                    Ok(sent) => {
                        println!("  -> Echoed {} bytes back", sent);
                    }
                    Err(e) => {
                        eprintln!("  -> Send error: {}", e);
                    }
                }

                // Print statistics periodically
                if packets % 10 == 0 {
                    println!(
                        "\n--- Stats: {} packets, {} bytes total ---\n",
                        packets, bytes
                    );
                }
            }
            Err(e) => {
                eprintln!("Receive error: {}", e);
            }
        }
    }
}

// Key concepts demonstrated:
//
// 1. UDP IS CONNECTIONLESS:
//    - No accept(), no handshake
//    - Each recv_from() is independent
//    - Must track sender address to reply
//
// 2. DATAGRAM BOUNDARIES:
//    - Each recv_from() gets exactly one datagram
//    - Unlike TCP streams, no need for message framing
//    - If datagram > buffer size, excess is discarded!
//
// 3. NO GUARANTEED DELIVERY:
//    - Packets may be lost, duplicated, or reordered
//    - Application must handle reliability if needed
//    - Good for: real-time data where stale packets are useless
//
// 4. SIMPLICITY:
//    - Much simpler than TCP server
//    - No connection state to manage
//    - Single socket handles all clients
//
// Compare with TCP:
// - TCP: listener.accept() -> per-connection stream
// - UDP: socket.recv_from() -> immediate datagram + sender address

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{timeout, Duration};

    async fn start_test_server() -> u16 {
        let socket = UdpSocket::bind("127.0.0.1:0").await.unwrap();
        let port = socket.local_addr().unwrap().port();

        tokio::spawn(async move {
            let mut buffer = [0u8; 1024];
            loop {
                if let Ok((n, addr)) = socket.recv_from(&mut buffer).await {
                    let _ = socket.send_to(&buffer[..n], addr).await;
                }
            }
        });

        tokio::time::sleep(Duration::from_millis(100)).await;
        port
    }

    #[tokio::test]
    async fn test_echo() {
        let port = start_test_server().await;

        let client = UdpSocket::bind("127.0.0.1:0").await.unwrap();
        let server_addr = format!("127.0.0.1:{}", port);

        let sent = b"hello udp";
        client.send_to(sent, &server_addr).await.unwrap();

        let mut buffer = [0u8; 1024];
        let result = timeout(Duration::from_secs(1), client.recv_from(&mut buffer)).await;

        assert!(result.is_ok());
        let (n, _) = result.unwrap().unwrap();
        assert_eq!(&buffer[..n], sent);
    }

    #[tokio::test]
    async fn test_multiple_datagrams() {
        let port = start_test_server().await;

        let client = UdpSocket::bind("127.0.0.1:0").await.unwrap();
        let server_addr = format!("127.0.0.1:{}", port);

        // Send multiple datagrams
        for i in 0..5 {
            let msg = format!("message {}", i);
            client.send_to(msg.as_bytes(), &server_addr).await.unwrap();

            let mut buffer = [0u8; 1024];
            let result = timeout(Duration::from_secs(1), client.recv_from(&mut buffer)).await;

            assert!(result.is_ok());
            let (n, _) = result.unwrap().unwrap();
            assert_eq!(&buffer[..n], msg.as_bytes());
        }
    }
}
