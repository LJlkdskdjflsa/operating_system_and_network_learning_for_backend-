//! Lab 5 Reference Answer

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

/// Handle a single client connection (async version)
async fn handle_client(mut stream: TcpStream) {
    let peer_addr = stream
        .peer_addr()
        .map(|a| a.to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    println!("[{}] Connected", peer_addr);

    let mut buffer = [0u8; 1024];

    loop {
        // Read data from client (non-blocking, yields to scheduler)
        match stream.read(&mut buffer).await {
            Ok(0) => {
                // Client disconnected
                println!("[{}] Disconnected", peer_addr);
                break;
            }
            Ok(n) => {
                // Echo data back
                if let Err(e) = stream.write_all(&buffer[..n]).await {
                    println!("[{}] Write error: {}", peer_addr, e);
                    break;
                }
            }
            Err(e) => {
                println!("[{}] Read error: {}", peer_addr, e);
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";

    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind");

    println!("Async Echo Server (Tokio)");
    println!("Listening on {}", addr);
    println!("Connections are handled by async tasks, not threads!\n");
    println!("Test with: nc localhost 8080");
    println!("Watch threads with: htop (press H to show threads)");
    println!("Notice: thread count stays low even with many connections!\n");

    let mut connection_count = 0u64;

    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                connection_count += 1;
                println!(
                    "New connection #{} from {} (spawning async task)",
                    connection_count, addr
                );

                // Spawn an async task (NOT a thread!)
                // This task runs on the Tokio runtime's thread pool
                tokio::spawn(async move {
                    handle_client(stream).await;
                });
            }
            Err(e) => {
                eprintln!("Accept error: {}", e);
            }
        }
    }
}

// Key differences from blocking version:
//
// 1. THREADS:
//    - Blocking: 1 thread per connection
//    - Async: Few threads (= CPU cores), many tasks
//
// 2. MEMORY:
//    - Blocking: ~2-8 MB per connection (thread stack)
//    - Async: ~few KB per connection (task state)
//
// 3. SYSCALLS:
//    - Blocking: read() blocks the thread
//    - Async: epoll_wait() + non-blocking read()
//
// 4. SCALABILITY:
//    - Blocking: ~1000 connections before issues
//    - Async: 10,000+ connections easily
//
// Run with strace to see the difference:
//   Blocking: Many threads, each doing read() syscalls
//   Async: Few threads, epoll_wait() returns when data ready

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpStream;
    use tokio::time::{timeout, Duration};

    async fn start_test_server() -> u16 {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();

        tokio::spawn(async move {
            loop {
                if let Ok((stream, _)) = listener.accept().await {
                    tokio::spawn(handle_client(stream));
                }
            }
        });

        // Give server time to start
        tokio::time::sleep(Duration::from_millis(100)).await;
        port
    }

    #[tokio::test]
    async fn test_echo() {
        let port = start_test_server().await;

        let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port))
            .await
            .unwrap();

        let sent = b"hello async world";
        stream.write_all(sent).await.unwrap();

        let mut buffer = [0u8; 1024];
        let n = timeout(Duration::from_secs(1), stream.read(&mut buffer))
            .await
            .unwrap()
            .unwrap();

        assert_eq!(&buffer[..n], sent);
    }

    #[tokio::test]
    async fn test_multiple_clients() {
        let port = start_test_server().await;

        let mut handles = vec![];

        for i in 0..5 {
            let handle = tokio::spawn(async move {
                let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port))
                    .await
                    .unwrap();

                let msg = format!("client {}", i);
                stream.write_all(msg.as_bytes()).await.unwrap();

                let mut buffer = [0u8; 1024];
                let n = stream.read(&mut buffer).await.unwrap();

                assert_eq!(&buffer[..n], msg.as_bytes());
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }
    }
}
