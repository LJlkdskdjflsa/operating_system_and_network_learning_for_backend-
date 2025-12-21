# TCP/UDP Fundamentals

## Overview

TCP (Transmission Control Protocol) and UDP (User Datagram Protocol) are the two main transport layer protocols. Understanding their differences is crucial for making the right choice in your applications.

## The Transport Layer

```
Application Layer    (HTTP, DNS, SSH, etc.)
        ↓
Transport Layer      (TCP, UDP)  ← This chapter
        ↓
Network Layer        (IP)
        ↓
Link Layer           (Ethernet, WiFi)
```

The transport layer provides:

- **Multiplexing**: Multiple applications sharing one IP address (via ports)
- **Data delivery**: Getting bytes from A to B

## TCP: Transmission Control Protocol

### Key Characteristics

1. **Connection-oriented**: Must establish connection before data transfer
2. **Reliable**: Guarantees delivery (or reports failure)
3. **Ordered**: Data arrives in the order it was sent
4. **Flow control**: Sender won't overwhelm receiver
5. **Congestion control**: Won't overwhelm the network

### TCP Three-Way Handshake

```
Client                          Server
   |                               |
   |----------- SYN ------------->|  "I want to connect"
   |                               |
   |<-------- SYN-ACK ------------|  "OK, I'm ready too"
   |                               |
   |----------- ACK ------------->|  "Great, let's go"
   |                               |
   |====== Connection Open =======|
```

### TCP Segment Structure

```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|          Source Port          |       Destination Port        |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                        Sequence Number                        |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                    Acknowledgment Number                      |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
| Offset|  Res  |U|A|P|R|S|F|            Window                 |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|           Checksum            |         Urgent Pointer        |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                    Options (if any)                           |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                             Data                              |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

Key fields:

- **Sequence Number**: Position of this data in the stream
- **Acknowledgment Number**: Next expected byte from other side
- **Flags**: SYN, ACK, FIN, RST, etc.
- **Window**: How much data receiver can accept

### TCP Reliability Mechanisms

#### Sequence Numbers and ACKs

```
Sender                           Receiver
   |                                |
   |---- Seq=100, Data="Hello" --->|
   |                                |
   |<-------- ACK=105 -------------|  "Got it, next expect 105"
   |                                |
   |---- Seq=105, Data="World" --->|
   |                                |
   |<-------- ACK=110 -------------|
```

#### Retransmission

```
Sender                           Receiver
   |                                |
   |---- Seq=100, Data="Hi" ------>|
   |                                |
   |       (ACK lost!)              |
   |                                |
   |  (timeout, retransmit)         |
   |---- Seq=100, Data="Hi" ------>|
   |                                |
   |<-------- ACK=102 -------------|
```

### TCP State Machine (Simplified)

```
                    CLOSED
                       |
                    (listen)
                       ↓
                    LISTEN ←------ Server waits
                       |
                 (SYN received)
                       ↓
                  SYN_RECEIVED
                       |
                 (ACK received)
                       ↓
                  ESTABLISHED ←--- Data transfer happens here
                       |
                 (FIN sent/received)
                       ↓
                  FIN_WAIT / CLOSE_WAIT
                       |
                       ↓
                    CLOSED
```

### TCP in Rust

```rust
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

// Server
let listener = TcpListener::bind("127.0.0.1:8080")?;
for stream in listener.incoming() {
    let mut stream = stream?;
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer)?;
    stream.write_all(&buffer[..n])?;
}

// Client
let mut stream = TcpStream::connect("127.0.0.1:8080")?;
stream.write_all(b"Hello")?;
let mut response = [0; 1024];
let n = stream.read(&mut response)?;
```

## UDP: User Datagram Protocol

### Key Characteristics

1. **Connectionless**: No handshake required
2. **Unreliable**: No delivery guarantee
3. **Unordered**: Packets may arrive out of order
4. **No flow/congestion control**: Application must handle
5. **Lightweight**: Less overhead than TCP

### UDP Datagram Structure

```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|          Source Port          |       Destination Port        |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|            Length             |           Checksum            |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                             Data                              |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

Only 8 bytes of header (vs TCP's 20+ bytes)!

### UDP Communication Model

```
Sender                           Receiver
   |                                |
   |---- Datagram "Hello" -------->|  (might arrive)
   |                                |
   |---- Datagram "World" -------->|  (might not!)
   |                                |
   |---- Datagram "Test" --------->|  (might arrive first!)
   |                                |
   (No acknowledgments, no retries)
```

### UDP in Rust

```rust
use std::net::UdpSocket;

// Server
let socket = UdpSocket::bind("127.0.0.1:8080")?;
let mut buffer = [0; 1024];
loop {
    let (n, src_addr) = socket.recv_from(&mut buffer)?;
    socket.send_to(&buffer[..n], src_addr)?;
}

// Client
let socket = UdpSocket::bind("127.0.0.1:0")?;  // Any available port
socket.send_to(b"Hello", "127.0.0.1:8080")?;
let mut buffer = [0; 1024];
let (n, _) = socket.recv_from(&mut buffer)?;
```

## TCP vs UDP Comparison

| Feature            | TCP               | UDP                |
| ------------------ | ----------------- | ------------------ |
| Connection         | Required          | None               |
| Reliability        | Guaranteed        | Best effort        |
| Ordering           | Guaranteed        | None               |
| Speed              | Slower            | Faster             |
| Header size        | 20+ bytes         | 8 bytes            |
| Flow control       | Yes               | No                 |
| Congestion control | Yes               | No                 |
| Use case           | Files, web, email | Video, gaming, DNS |

## When to Use Which?

### Use TCP When:

- **Data integrity is critical**: File transfers, database queries
- **Order matters**: HTTP requests, SSH sessions
- **You need acknowledgment**: Financial transactions
- **Simplicity**: TCP handles reliability for you

### Use UDP When:

- **Speed > reliability**: Live video, voice chat
- **Small messages**: DNS queries, NTP
- **Broadcast/multicast**: Service discovery
- **Application handles reliability**: Custom game protocols

## Socket Programming Concepts

### Socket Types

```rust
// TCP socket - stream-based
use std::net::TcpListener;
use std::net::TcpStream;

// UDP socket - datagram-based
use std::net::UdpSocket;
```

### Address Binding

```rust
// Specific address
let listener = TcpListener::bind("127.0.0.1:8080")?;

// All interfaces
let listener = TcpListener::bind("0.0.0.0:8080")?;

// Let OS choose port
let socket = UdpSocket::bind("127.0.0.1:0")?;
let addr = socket.local_addr()?;  // Get assigned port
```

### Blocking vs Non-blocking

```rust
use std::time::Duration;

// Set timeout (blocking with limit)
stream.set_read_timeout(Some(Duration::from_secs(5)))?;

// Non-blocking mode
stream.set_nonblocking(true)?;
match stream.read(&mut buffer) {
    Ok(n) => { /* got data */ }
    Err(e) if e.kind() == ErrorKind::WouldBlock => { /* try later */ }
    Err(e) => { /* real error */ }
}
```

## Async Networking with Tokio

```rust
use tokio::net::{TcpListener, TcpStream, UdpSocket};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// TCP
let listener = TcpListener::bind("127.0.0.1:8080").await?;
let (mut stream, addr) = listener.accept().await?;
let n = stream.read(&mut buffer).await?;
stream.write_all(&buffer[..n]).await?;

// UDP
let socket = UdpSocket::bind("127.0.0.1:8080").await?;
let (n, addr) = socket.recv_from(&mut buffer).await?;
socket.send_to(&buffer[..n], addr).await?;
```

## Observing Network Traffic

### Using tcpdump

```bash
# Capture TCP traffic on port 8080
sudo tcpdump -i lo port 8080

# Show TCP flags
sudo tcpdump -i lo port 8080 -tttt

# Example output for TCP handshake:
# 12:00:00 IP localhost.54321 > localhost.8080: Flags [S], seq 1234
# 12:00:00 IP localhost.8080 > localhost.54321: Flags [S.], seq 5678, ack 1235
# 12:00:00 IP localhost.54321 > localhost.8080: Flags [.], ack 5679
```

### Using ss (Socket Statistics)

```bash
# Show all TCP connections
ss -t

# Show listening sockets
ss -tln

# Show with process info
ss -tlnp

# Show UDP sockets
ss -uln
```

### Using netstat

```bash
# Show all connections
netstat -an

# Show TCP connections
netstat -ant

# Show UDP
netstat -anu
```

## Common Patterns

### Chat Server (TCP)

```rust
// Share connected clients
let clients: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));

// Broadcast message to all clients
fn broadcast(clients: &[TcpStream], message: &[u8]) {
    for client in clients {
        let _ = client.write_all(message);
    }
}
```

### Request-Response (UDP)

```rust
// Server processes each datagram independently
loop {
    let (n, src) = socket.recv_from(&mut buffer)?;
    let response = process(&buffer[..n]);
    socket.send_to(&response, src)?;
}
```

## Summary

- **TCP** = Reliable, ordered, connection-based (use for most applications)
- **UDP** = Fast, unreliable, connectionless (use for real-time applications)
- Both use **sockets** with **IP:port** addressing
- Tokio provides **async** versions of both
- Use **tcpdump/ss** to observe network behavior

## Labs

1. **Lab 1: TCP Chat Server** - Multi-client chat with message broadcasting
2. **Lab 2: UDP Echo** - Simple UDP echo server with packet handling
