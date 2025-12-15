//! Simple target server for load testing
//!
//! Run with: cargo run --bin target_server

use std::net::SocketAddr;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

const RESPONSE: &[u8] = b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 47\r\n\r\n{\"items\":[{\"id\":1,\"name\":\"Widget\"}],\"count\":1}";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "0.0.0.0:3000".parse()?;
    let listener = TcpListener::bind(addr).await?;

    println!("Target server running on http://{}", addr);
    println!("Endpoints:");
    println!("  GET /items - Returns JSON (fast)");
    println!("  GET /slow  - Returns JSON after 100ms delay");
    println!();

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0u8; 1024];
            if let Ok(n) = socket.read(&mut buf).await {
                if n > 0 {
                    let request = String::from_utf8_lossy(&buf[..n]);

                    // Check for /slow endpoint
                    if request.contains("GET /slow") {
                        tokio::time::sleep(Duration::from_millis(100)).await;
                    }

                    let _ = socket.write_all(RESPONSE).await;
                }
            }
        });
    }
}
