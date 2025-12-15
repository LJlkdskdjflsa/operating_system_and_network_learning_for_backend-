//! Lab 4 Reference Answer

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

/// Handle a single client connection
fn handle_client(mut stream: TcpStream) {
    let peer_addr = stream
        .peer_addr()
        .map(|a| a.to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    println!("[{}] Connected", peer_addr);

    let mut buffer = [0u8; 1024];

    loop {
        // Read data from client (blocks until data arrives)
        match stream.read(&mut buffer) {
            Ok(0) => {
                // Client disconnected
                println!("[{}] Disconnected", peer_addr);
                break;
            }
            Ok(n) => {
                // Echo data back
                if let Err(e) = stream.write_all(&buffer[..n]) {
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

fn main() {
    let addr = "127.0.0.1:8080";

    let listener = TcpListener::bind(addr).expect("Failed to bind");
    println!("Blocking Echo Server");
    println!("Listening on {}", addr);
    println!("Each connection gets its own thread\n");
    println!("Test with: nc localhost 8080");
    println!("Watch threads with: htop (press H to show threads)\n");

    let mut connection_count = 0;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                connection_count += 1;
                let peer_addr = stream
                    .peer_addr()
                    .map(|a| a.to_string())
                    .unwrap_or_else(|_| "unknown".to_string());

                println!(
                    "New connection #{} from {} (spawning thread)",
                    connection_count, peer_addr
                );

                // Spawn a new thread for each connection
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Accept error: {}", e);
            }
        }
    }
}

// Note: This server has some limitations:
// 1. Each connection needs its own thread (~2-8 MB stack)
// 2. With 10,000 connections, you need 10,000 threads
// 3. Most threads are just waiting (blocked on read)
//
// This is fine for small numbers of connections but doesn't scale.
// See lab_05 for the async version that handles many more connections.
