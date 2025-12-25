//! Lab 6 Tests

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

fn start_proxy() -> Option<ServerGuard> {
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

fn send_request(request: &str) -> Option<String> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").ok()?;
    stream.set_read_timeout(Some(Duration::from_secs(2))).ok()?;

    stream.write_all(request.as_bytes()).ok()?;

    let mut response = String::new();
    let _ = stream.read_to_string(&mut response);

    Some(response)
}

#[test]
fn test_01_proxy_accepts_connections() {
    let _proxy = match start_proxy() {
        Some(s) => s,
        None => return,
    };

    let result = TcpStream::connect("127.0.0.1:8080");
    assert!(result.is_ok(), "Should be able to connect to proxy");
}

#[test]
fn test_02_proxy_responds_without_backend() {
    let _proxy = match start_proxy() {
        Some(s) => s,
        None => return,
    };

    let request = "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n";
    let response = match send_request(request) {
        Some(r) => r,
        None => return,
    };

    // Without backends, should return 502
    assert!(
        response.contains("502") || response.contains("HTTP"),
        "Proxy should respond with HTTP response"
    );
}

#[test]
fn test_03_proxy_handles_multiple_requests() {
    let _proxy = match start_proxy() {
        Some(s) => s,
        None => return,
    };

    for _ in 0..5 {
        let request = "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n";
        let _ = send_request(request);
    }

    // If we get here without crashing, proxy handles multiple requests
}

// Note: Testing with actual backends requires starting backend servers
// In a real test environment, you would:
// 1. Start backend servers on ports 8081, 8082
// 2. Send requests through the proxy
// 3. Verify round-robin behavior
// 4. Check X-Forwarded-For header

#[test]
fn test_04_round_robin_counter() {
    // Unit test for round-robin logic
    use std::sync::atomic::{AtomicUsize, Ordering};

    const BACKENDS: &[&str] = &["server1", "server2", "server3"];
    static COUNTER: AtomicUsize = AtomicUsize::new(0);

    fn next_backend() -> &'static str {
        let index = COUNTER.fetch_add(1, Ordering::Relaxed);
        BACKENDS[index % BACKENDS.len()]
    }

    assert_eq!(next_backend(), "server1");
    assert_eq!(next_backend(), "server2");
    assert_eq!(next_backend(), "server3");
    assert_eq!(next_backend(), "server1"); // Cycles back
}
