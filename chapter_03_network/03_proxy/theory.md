# Proxy and Load Balancing

## Overview

Proxies and load balancers are essential components in modern backend architecture. They enable scaling, security, and reliability for web services.

## What is a Proxy?

A proxy is an intermediary server that sits between clients and backend servers.

### Forward Proxy

```
[Client] --> [Forward Proxy] --> [Internet] --> [Server]

- Client knows about proxy
- Proxy fetches resources on behalf of client
- Use cases: caching, anonymity, access control
```

### Reverse Proxy

```
[Client] --> [Reverse Proxy] --> [Backend Servers]

- Client doesn't know about backend servers
- Proxy routes requests to appropriate backend
- Use cases: load balancing, SSL termination, caching
```

## Reverse Proxy Benefits

### 1. Load Balancing
```
                              ┌─> [Server 1]
[Clients] --> [Reverse Proxy] ├─> [Server 2]
                              └─> [Server 3]
```

Distribute requests across multiple servers.

### 2. SSL/TLS Termination
```
[Client] --HTTPS--> [Proxy] --HTTP--> [Backend]

- Proxy handles encryption/decryption
- Backend servers don't need SSL certificates
- Reduces backend CPU load
```

### 3. Caching
```
[Client] --> [Proxy (cached)] --> [Backend]
                    |
              [Cache Storage]

- Proxy caches responses
- Reduces backend load
- Faster response times
```

### 4. Security
- Hide backend infrastructure
- DDoS protection
- Web Application Firewall (WAF)
- Rate limiting

### 5. Compression
- Compress responses before sending to client
- Reduces bandwidth usage

## Load Balancing Algorithms

### 1. Round Robin

```rust
let servers = ["server1", "server2", "server3"];
let mut index = 0;

fn next_server() -> &str {
    let server = servers[index];
    index = (index + 1) % servers.len();
    server
}

// Request 1 -> server1
// Request 2 -> server2
// Request 3 -> server3
// Request 4 -> server1 (cycles back)
```

**Pros**: Simple, fair distribution
**Cons**: Doesn't consider server load

### 2. Weighted Round Robin

```rust
struct Server {
    addr: String,
    weight: u32,
}

let servers = [
    Server { addr: "big-server", weight: 3 },
    Server { addr: "small-server", weight: 1 },
];

// big-server gets 3x more requests
```

**Pros**: Account for different server capacities
**Cons**: Weights are static

### 3. Least Connections

```rust
fn next_server(servers: &[Server]) -> &Server {
    servers.iter()
        .min_by_key(|s| s.active_connections)
        .unwrap()
}
```

**Pros**: Adapts to actual server load
**Cons**: More complex to track

### 4. IP Hash (Session Affinity)

```rust
fn next_server(client_ip: &str, servers: &[Server]) -> &Server {
    let hash = hash(client_ip);
    let index = hash % servers.len();
    &servers[index]
}

// Same client always goes to same server
```

**Pros**: Session persistence without cookies
**Cons**: Uneven distribution if many clients share IP

### 5. Random

```rust
fn next_server(servers: &[Server]) -> &Server {
    let index = rand::random::<usize>() % servers.len();
    &servers[index]
}
```

**Pros**: Simple, no state needed
**Cons**: Not predictable

### 6. Least Response Time

```rust
fn next_server(servers: &[Server]) -> &Server {
    servers.iter()
        .min_by_key(|s| s.avg_response_time)
        .unwrap()
}
```

**Pros**: Routes to fastest server
**Cons**: Requires monitoring

## Health Checks

Ensure requests only go to healthy servers.

### Passive Health Checks

Monitor responses to real requests:

```rust
if response.status() >= 500 {
    server.failures += 1;
    if server.failures > threshold {
        server.mark_unhealthy();
    }
} else {
    server.failures = 0;
}
```

### Active Health Checks

Periodically probe servers:

```rust
async fn health_check(server: &Server) -> bool {
    match client.get(&format!("{}/health", server.addr)).await {
        Ok(resp) => resp.status() == 200,
        Err(_) => false,
    }
}

// Run every 10 seconds
loop {
    for server in &mut servers {
        server.healthy = health_check(server).await;
    }
    sleep(Duration::from_secs(10)).await;
}
```

## Building a Simple Reverse Proxy

### Basic Implementation

```rust
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

async fn proxy_request(
    client: TcpStream,
    backend: &str,
) {
    // Connect to backend
    let mut backend = TcpStream::connect(backend).await?;

    // Split streams
    let (mut client_read, mut client_write) = client.into_split();
    let (mut backend_read, mut backend_write) = backend.into_split();

    // Proxy in both directions
    tokio::select! {
        _ = tokio::io::copy(&mut client_read, &mut backend_write) => {}
        _ = tokio::io::copy(&mut backend_read, &mut client_write) => {}
    }
}
```

### HTTP-Aware Proxy

```rust
async fn handle_request(
    mut client: TcpStream,
    backends: &[String],
    index: &AtomicUsize,
) {
    // Read HTTP request
    let mut buffer = [0u8; 4096];
    let n = client.read(&mut buffer).await?;

    // Select backend (round-robin)
    let idx = index.fetch_add(1, Ordering::Relaxed);
    let backend = &backends[idx % backends.len()];

    // Connect to backend
    let mut backend_conn = TcpStream::connect(backend).await?;

    // Forward request
    backend_conn.write_all(&buffer[..n]).await?;

    // Forward response
    let mut response = Vec::new();
    backend_conn.read_to_end(&mut response).await?;
    client.write_all(&response).await?;
}
```

## Connection Pooling

Reuse connections to backends:

```rust
struct ConnectionPool {
    connections: Mutex<Vec<TcpStream>>,
    backend: String,
    max_size: usize,
}

impl ConnectionPool {
    async fn get(&self) -> TcpStream {
        // Try to reuse existing connection
        if let Some(conn) = self.connections.lock().pop() {
            return conn;
        }

        // Create new connection
        TcpStream::connect(&self.backend).await.unwrap()
    }

    async fn put(&self, conn: TcpStream) {
        let mut connections = self.connections.lock();
        if connections.len() < self.max_size {
            connections.push(conn);
        }
        // Otherwise, connection is dropped
    }
}
```

## Real-World Proxies

### Nginx

```nginx
upstream backend {
    server backend1:8080 weight=3;
    server backend2:8080;
    server backend3:8080 backup;
}

server {
    listen 80;

    location / {
        proxy_pass http://backend;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

### HAProxy

```
frontend http_front
    bind *:80
    default_backend http_back

backend http_back
    balance roundrobin
    server server1 backend1:8080 check
    server server2 backend2:8080 check
    server server3 backend3:8080 check weight 2
```

### Envoy

Modern proxy with advanced features:
- L4/L7 load balancing
- gRPC support
- Service mesh integration
- Observability (metrics, tracing)

## Headers in Proxied Requests

### X-Forwarded-For

Original client IP (may be multiple proxies):

```
X-Forwarded-For: client, proxy1, proxy2
```

### X-Forwarded-Proto

Original protocol:

```
X-Forwarded-Proto: https
```

### X-Real-IP

Single original client IP:

```
X-Real-IP: 192.168.1.100
```

### Host Header

Preserve original host:

```rust
// Proxy should forward original Host header
request.headers().get("Host")
// Or set X-Forwarded-Host
```

## Common Patterns

### Blue-Green Deployment

```
[Proxy] ---> [Blue (v1)] ← current
         └-> [Green (v2)] ← new version

# Switch traffic
[Proxy] ---> [Blue (v1)]
         └-> [Green (v2)] ← current
```

### Canary Deployment

```rust
fn select_backend(request: &Request) -> &Backend {
    if rand::random::<f32>() < 0.1 {
        &canary_backend  // 10% to new version
    } else {
        &stable_backend  // 90% to stable version
    }
}
```

### Circuit Breaker

```rust
struct CircuitBreaker {
    state: State, // Closed, Open, HalfOpen
    failures: u32,
    threshold: u32,
    reset_timeout: Duration,
}

impl CircuitBreaker {
    fn call(&mut self, f: impl Fn() -> Result) -> Result {
        match self.state {
            State::Open => Err("Circuit open"),
            State::Closed | State::HalfOpen => {
                match f() {
                    Ok(r) => {
                        self.failures = 0;
                        self.state = State::Closed;
                        Ok(r)
                    }
                    Err(e) => {
                        self.failures += 1;
                        if self.failures >= self.threshold {
                            self.state = State::Open;
                        }
                        Err(e)
                    }
                }
            }
        }
    }
}
```

## Summary

- **Forward Proxy**: Client-side, for outgoing requests
- **Reverse Proxy**: Server-side, for incoming requests
- **Load Balancing**: Distribute traffic across servers
- **Health Checks**: Ensure traffic goes to healthy servers
- **Connection Pooling**: Reuse backend connections
- **Headers**: Forward client information to backends

## Lab

**Lab 6: Reverse Proxy** - Build a simple HTTP reverse proxy with round-robin load balancing
