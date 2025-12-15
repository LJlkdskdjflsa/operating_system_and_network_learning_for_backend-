//! Lab 1 Reference Answer

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::broadcast;
use std::net::SocketAddr;

/// Handle a single client connection
async fn handle_client(
    stream: TcpStream,
    addr: SocketAddr,
    tx: broadcast::Sender<(String, SocketAddr)>,
) {
    // Subscribe to receive broadcast messages
    let mut rx = tx.subscribe();

    // Split the stream into reader and writer
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    // Notify about new connection
    let join_msg = format!("[{}] joined the chat\n", addr);
    println!("{}", join_msg.trim());
    let _ = tx.send((join_msg, addr));

    loop {
        tokio::select! {
            // Read from client
            result = reader.read_line(&mut line) => {
                match result {
                    Ok(0) => {
                        // Client disconnected
                        let leave_msg = format!("[{}] left the chat\n", addr);
                        println!("{}", leave_msg.trim());
                        let _ = tx.send((leave_msg, addr));
                        break;
                    }
                    Ok(_) => {
                        // Broadcast message to all clients
                        let msg = format!("[{}]: {}", addr, line);
                        println!("{}", msg.trim());
                        let _ = tx.send((msg, addr));
                        line.clear();
                    }
                    Err(e) => {
                        eprintln!("[{}] Read error: {}", addr, e);
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
                            if let Err(e) = writer.write_all(msg.as_bytes()).await {
                                eprintln!("[{}] Write error: {}", addr, e);
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
    let addr = "127.0.0.1:8080";

    // Create a broadcast channel for message distribution
    // Capacity of 100 messages
    let (tx, _rx) = broadcast::channel::<(String, SocketAddr)>(100);

    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind");

    println!("TCP Chat Server");
    println!("Listening on {}", addr);
    println!("\nTest with: nc localhost 8080");
    println!("Open multiple terminals to chat!\n");

    loop {
        match listener.accept().await {
            Ok((stream, client_addr)) => {
                println!("New connection from {}", client_addr);

                // Clone sender for this client
                let tx_clone = tx.clone();

                // Spawn handler for this client
                tokio::spawn(async move {
                    handle_client(stream, client_addr, tx_clone).await;
                });
            }
            Err(e) => {
                eprintln!("Accept error: {}", e);
            }
        }
    }
}

// Key concepts demonstrated:
//
// 1. BROADCAST CHANNEL:
//    - `tokio::sync::broadcast` allows multiple producers and consumers
//    - Each subscriber gets a copy of every message
//    - Perfect for chat applications
//
// 2. STREAM SPLITTING:
//    - `stream.into_split()` separates read and write halves
//    - Allows concurrent reading and writing
//    - Alternative to wrapping stream in Arc<Mutex<>>
//
// 3. TOKIO SELECT:
//    - `tokio::select!` waits on multiple futures
//    - Proceeds when any one completes
//    - Used to handle both reading from client and receiving broadcasts
//
// 4. MESSAGE FILTERING:
//    - Messages include sender address
//    - Sender skips their own messages
//    - Prevents echo back to originator
//
// Alternative implementations:
// - Use `Arc<Mutex<HashMap<SocketAddr, Writer>>>` to track clients
// - Use `mpsc` channels per client instead of broadcast
// - Use `tokio_util::codec` for framing

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{timeout, Duration};

    async fn start_test_server() -> u16 {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let (tx, _rx) = broadcast::channel::<(String, SocketAddr)>(100);

        tokio::spawn(async move {
            loop {
                if let Ok((stream, addr)) = listener.accept().await {
                    let tx_clone = tx.clone();
                    tokio::spawn(handle_client(stream, addr, tx_clone));
                }
            }
        });

        tokio::time::sleep(Duration::from_millis(100)).await;
        port
    }

    #[tokio::test]
    async fn test_multiple_clients_connect() {
        let port = start_test_server().await;

        let client1 = TcpStream::connect(format!("127.0.0.1:{}", port)).await;
        let client2 = TcpStream::connect(format!("127.0.0.1:{}", port)).await;

        assert!(client1.is_ok());
        assert!(client2.is_ok());
    }

    #[tokio::test]
    async fn test_message_broadcast() {
        let port = start_test_server().await;

        let mut client1 = TcpStream::connect(format!("127.0.0.1:{}", port))
            .await
            .unwrap();
        let mut client2 = TcpStream::connect(format!("127.0.0.1:{}", port))
            .await
            .unwrap();

        // Give time for join messages
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Client 1 sends a message
        client1.write_all(b"hello\n").await.unwrap();

        // Client 2 should receive it
        let mut buffer = [0u8; 1024];
        let result = timeout(Duration::from_secs(1), client2.read(&mut buffer)).await;

        assert!(result.is_ok());
        let n = result.unwrap().unwrap();
        let received = String::from_utf8_lossy(&buffer[..n]);
        assert!(received.contains("hello") || received.contains("joined"));
    }
}
