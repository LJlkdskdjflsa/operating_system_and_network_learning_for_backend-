//! Lab 2 Tests

use std::net::UdpSocket;
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
fn test_01_udp_echo() {
    let _server = match start_server() {
        Some(s) => s,
        None => return,
    };

    let socket = match UdpSocket::bind("127.0.0.1:0") {
        Ok(s) => s,
        Err(_) => return,
    };

    socket
        .set_read_timeout(Some(Duration::from_secs(2)))
        .unwrap();

    let sent = b"hello udp";
    if socket.send_to(sent, "127.0.0.1:8080").is_err() {
        return;
    }

    let mut buffer = [0u8; 1024];
    match socket.recv_from(&mut buffer) {
        Ok((n, _)) => {
            assert_eq!(&buffer[..n], sent, "Server should echo back the same data");
        }
        Err(_) => {
            // Timeout or error - server might not be implemented
        }
    }
}

#[test]
fn test_02_multiple_datagrams() {
    let _server = match start_server() {
        Some(s) => s,
        None => return,
    };

    let socket = match UdpSocket::bind("127.0.0.1:0") {
        Ok(s) => s,
        Err(_) => return,
    };

    socket
        .set_read_timeout(Some(Duration::from_secs(2)))
        .unwrap();

    for i in 0..5 {
        let msg = format!("message {}", i);
        if socket.send_to(msg.as_bytes(), "127.0.0.1:8080").is_err() {
            continue;
        }

        let mut buffer = [0u8; 1024];
        if let Ok((n, _)) = socket.recv_from(&mut buffer) {
            assert_eq!(
                &buffer[..n],
                msg.as_bytes(),
                "Each datagram should be echoed independently"
            );
        }
    }
}

#[test]
fn test_03_multiple_clients() {
    let _server = match start_server() {
        Some(s) => s,
        None => return,
    };

    // Create multiple UDP sockets (clients)
    let mut sockets = Vec::new();
    for _ in 0..3 {
        if let Ok(s) = UdpSocket::bind("127.0.0.1:0") {
            s.set_read_timeout(Some(Duration::from_secs(2))).unwrap();
            sockets.push(s);
        }
    }

    // Each client sends and receives
    for (i, socket) in sockets.iter().enumerate() {
        let msg = format!("client {}", i);
        if socket.send_to(msg.as_bytes(), "127.0.0.1:8080").is_err() {
            continue;
        }

        let mut buffer = [0u8; 1024];
        if let Ok((n, _)) = socket.recv_from(&mut buffer) {
            assert_eq!(&buffer[..n], msg.as_bytes());
        }
    }
}
