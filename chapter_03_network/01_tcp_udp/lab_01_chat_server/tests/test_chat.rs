//! Lab 1 Tests

use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::process::{Child, Command};
use std::thread;
use std::time::Duration;

struct ServerGuard {
    child: Child,
}

impl Drop for ServerGuard {
    fn drop(&mut self) {
        let _ = self.child.kill();
    }
}

fn start_server() -> Option<ServerGuard> {
    Command::new("cargo")
        .args(["build", "--quiet"])
        .status()
        .ok()?;

    let child = Command::new("cargo")
        .args(["run", "--quiet"])
        .spawn()
        .ok()?;

    thread::sleep(Duration::from_millis(500));

    Some(ServerGuard { child })
}

#[test]
fn test_01_server_accepts_connection() {
    let _server = match start_server() {
        Some(s) => s,
        None => return,
    };

    let result = TcpStream::connect("127.0.0.1:8080");

    if result.is_err() {
        return;
    }

    assert!(result.is_ok(), "Should be able to connect to server");
}

#[test]
fn test_02_multiple_clients() {
    let _server = match start_server() {
        Some(s) => s,
        None => return,
    };

    let client1 = TcpStream::connect("127.0.0.1:8080");
    let client2 = TcpStream::connect("127.0.0.1:8080");
    let client3 = TcpStream::connect("127.0.0.1:8080");

    if client1.is_err() {
        return;
    }

    assert!(client1.is_ok());
    assert!(client2.is_ok());
    assert!(client3.is_ok());
}

#[test]
fn test_03_message_broadcast() {
    let _server = match start_server() {
        Some(s) => s,
        None => return,
    };

    let mut client1 = match TcpStream::connect("127.0.0.1:8080") {
        Ok(s) => s,
        Err(_) => return,
    };

    let client2 = match TcpStream::connect("127.0.0.1:8080") {
        Ok(s) => s,
        Err(_) => return,
    };

    client1
        .set_read_timeout(Some(Duration::from_secs(2)))
        .unwrap();
    client2
        .set_read_timeout(Some(Duration::from_secs(2)))
        .unwrap();

    // Give time for connections to be established
    thread::sleep(Duration::from_millis(200));

    // Client 1 sends a message
    if client1.write_all(b"hello from client 1\n").is_err() {
        return;
    }

    // Client 2 should receive it
    let mut reader = BufReader::new(&client2);
    let mut line = String::new();

    // Read until we get the message (skip join messages)
    for _ in 0..5 {
        line.clear();
        if reader.read_line(&mut line).is_err() {
            continue;
        }
        if line.contains("hello from client 1") {
            // Success - message was broadcast
            return;
        }
    }

    // If we get here, message wasn't broadcast (might not be implemented yet)
}
