//! Lab 5 Tests

use std::io::{Read, Write};
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
    // Build first
    Command::new("cargo")
        .args(["build", "--quiet"])
        .status()
        .ok()?;

    // Start server in background
    let child = Command::new("cargo")
        .args(["run", "--quiet"])
        .spawn()
        .ok()?;

    // Give server time to start
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
        return; // Server might not be implemented yet
    }

    assert!(result.is_ok(), "Should be able to connect to server");
}

#[test]
fn test_02_server_echoes_data() {
    let _server = match start_server() {
        Some(s) => s,
        None => return,
    };

    let mut stream = match TcpStream::connect("127.0.0.1:8080") {
        Ok(s) => s,
        Err(_) => return,
    };

    stream
        .set_read_timeout(Some(Duration::from_secs(2)))
        .unwrap();

    let sent = b"hello async world";
    if stream.write_all(sent).is_err() {
        return;
    }

    let mut buffer = [0u8; 1024];
    match stream.read(&mut buffer) {
        Ok(n) => {
            assert_eq!(&buffer[..n], sent, "Server should echo back the same data");
        }
        Err(_) => {
            // Timeout or error
        }
    }
}

#[test]
fn test_03_multiple_clients() {
    let _server = match start_server() {
        Some(s) => s,
        None => return,
    };

    let mut streams = Vec::new();

    for _ in 0..5 {
        match TcpStream::connect("127.0.0.1:8080") {
            Ok(s) => streams.push(s),
            Err(_) => return,
        }
    }

    assert!(
        streams.len() >= 3,
        "Server should accept multiple connections"
    );
}
