# 2.3 I/O Models: From Blocking to Async

## Section Goals

> Understand different I/O models and why async matters for server performance

After completing this section, you will be able to:

- Explain blocking vs non-blocking I/O
- Understand how epoll enables handling many connections
- Explain how Tokio works under the hood
- Choose the right I/O model for your application

---

## 1. The I/O Problem

### Why I/O is Different from Computation

CPU operations are **fast** (nanoseconds). I/O operations are **slow**:

| Operation          | Time                   |
| ------------------ | ---------------------- |
| CPU instruction    | ~0.3 ns                |
| L1 cache access    | ~1 ns                  |
| Main memory        | ~100 ns                |
| SSD read           | ~100,000 ns (100 μs)   |
| Network round-trip | ~1,000,000 ns (1 ms)   |
| HDD seek           | ~10,000,000 ns (10 ms) |

When your program does I/O, the CPU sits idle waiting. This is wasteful!

### The Challenge: Many Connections

A web server needs to handle many clients simultaneously:

```
Client 1 ──────┐
Client 2 ──────┼──────► Server
Client 3 ──────┤
   ...         │
Client 10000 ──┘
```

How do we handle 10,000 connections without 10,000 threads?

---

## 2. Blocking I/O

### How It Works

The simplest model: when you call `read()`, the thread blocks until data arrives.

```rust
use std::net::TcpStream;
use std::io::Read;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0u8; 1024];

    // This BLOCKS until data arrives or connection closes
    let n = stream.read(&mut buffer).unwrap();

    // Process data...
}
```

### Thread-per-Connection

To handle multiple clients, spawn a thread for each:

```rust
use std::net::TcpListener;
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // New thread for each connection
        thread::spawn(move || {
            handle_client(stream);
        });
    }
}
```

### The Problem with Blocking

```
10,000 connections = 10,000 threads

Each thread needs:
  - Stack: ~2-8 MB
  - Kernel resources
  - Context switch overhead

10,000 threads × 2 MB = 20 GB just for stacks!
```

Plus, most threads are just waiting (blocked on I/O), wasting resources.

---

## 3. Non-Blocking I/O

### The Idea

Make `read()` return immediately, even if no data is available:

```rust
use std::io::ErrorKind;

stream.set_nonblocking(true)?;

loop {
    match stream.read(&mut buffer) {
        Ok(n) => {
            // Got n bytes, process them
            break;
        }
        Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
            // No data yet, do something else
            do_other_work();
        }
        Err(e) => panic!("Error: {}", e),
    }
}
```

### Busy Polling is Bad

Naive non-blocking with a loop wastes CPU:

```rust
// BAD: Busy polling
loop {
    for socket in &sockets {
        if let Ok(data) = socket.try_read() {
            process(data);
        }
    }
    // Spins even when no sockets have data!
}
```

We need a way to **sleep until something is ready**.

---

## 4. I/O Multiplexing: select/poll/epoll

### The Solution

Ask the OS: "Tell me when any of these file descriptors is ready."

```
┌─────────────────────────────────────────────────────────┐
│                    Your Program                          │
│                                                          │
│   "Hey OS, wake me when fd 3, 4, or 5 is readable"      │
│                         │                                │
└─────────────────────────┼────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────┐
│                      Kernel                              │
│                                                          │
│   [fd 3: waiting] [fd 4: DATA READY!] [fd 5: waiting]   │
│                         │                                │
│   "fd 4 is ready!"      │                                │
└─────────────────────────┼────────────────────────────────┘
                          │
                          ▼
                   Program wakes up
                   Reads from fd 4
```

### Evolution of APIs

| API        | Platforms  | Scalability                        |
| ---------- | ---------- | ---------------------------------- |
| `select`   | All Unix   | O(n) - checks all fds every time   |
| `poll`     | All Unix   | O(n) - slightly better than select |
| `epoll`    | Linux      | O(1) - only returns ready fds      |
| `kqueue`   | BSD/macOS  | O(1) - similar to epoll            |
| `io_uring` | Linux 5.1+ | Even better - async syscalls       |

### Why epoll Wins

```
select with 10,000 fds:
  - Copy 10,000 fd structures to kernel
  - Kernel checks all 10,000
  - Copy results back
  - Repeat every call

epoll with 10,000 fds:
  - Register fds once
  - Kernel maintains ready list
  - epoll_wait returns only ready fds
  - Much less copying, O(1) per ready fd
```

---

## 5. Event-Driven Architecture

### The Event Loop Pattern

Instead of thread-per-connection, use one thread with an event loop:

```
┌─────────────────────────────────────────────────────────┐
│                    Event Loop                            │
│                                                          │
│   while true {                                           │
│       events = epoll_wait(...)  // Block until ready     │
│                                                          │
│       for event in events {                              │
│           match event.fd {                               │
│               listener => accept_new_connection()        │
│               client   => handle_client_data()           │
│           }                                              │
│       }                                                  │
│   }                                                      │
│                                                          │
│   Single thread handles thousands of connections!        │
└─────────────────────────────────────────────────────────┘
```

### Example with mio (Low-Level)

```rust
use mio::{Events, Interest, Poll, Token};
use mio::net::TcpListener;

fn main() {
    let mut poll = Poll::new().unwrap();
    let mut events = Events::with_capacity(1024);

    let mut listener = TcpListener::bind("127.0.0.1:8080".parse().unwrap()).unwrap();
    poll.registry().register(&mut listener, Token(0), Interest::READABLE).unwrap();

    loop {
        poll.poll(&mut events, None).unwrap();

        for event in events.iter() {
            match event.token() {
                Token(0) => {
                    // New connection
                    let (conn, addr) = listener.accept().unwrap();
                    println!("New connection from {}", addr);
                    // Register conn for reading...
                }
                _ => {
                    // Handle existing connection...
                }
            }
        }
    }
}
```

---

## 6. Async/Await (Tokio)

### The Problem with Event Loops

Manual event loops are complex:

- Must manage state machines manually
- Callback hell
- Hard to read and maintain

### Async to the Rescue

Rust's async/await lets you write sequential-looking code that's actually event-driven:

```rust
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let mut buf = [0u8; 1024];

            loop {
                let n = socket.read(&mut buf).await.unwrap();
                if n == 0 { return; }

                socket.write_all(&buf[0..n]).await.unwrap();
            }
        });
    }
}
```

### How It Works

```
┌─────────────────────────────────────────────────────────┐
│                    Tokio Runtime                         │
│                                                          │
│   ┌─────────────────────────────────────────────────┐   │
│   │              Task Scheduler                      │   │
│   │   (manages thousands of async tasks)             │   │
│   └─────────────────────────────────────────────────┘   │
│                          │                               │
│                          ▼                               │
│   ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐   │
│   │ Worker  │  │ Worker  │  │ Worker  │  │ Worker  │   │
│   │Thread 1 │  │Thread 2 │  │Thread 3 │  │Thread 4 │   │
│   └────┬────┘  └────┬────┘  └────┬────┘  └────┬────┘   │
│        │            │            │            │         │
│        └────────────┴────────────┴────────────┘         │
│                          │                               │
│                          ▼                               │
│                    epoll (Linux)                         │
│                    kqueue (macOS)                        │
└─────────────────────────────────────────────────────────┘

1. Tasks are lightweight (~few KB each)
2. Few OS threads (= CPU cores)
3. Tasks cooperatively yield at .await points
4. epoll/kqueue handles I/O readiness
```

### Cooperative vs Preemptive

| Threads                              | Async Tasks                          |
| ------------------------------------ | ------------------------------------ |
| Preemptive: OS can interrupt anytime | Cooperative: task yields at `.await` |
| ~2-8 MB stack each                   | ~few KB each                         |
| Expensive context switch             | Cheap task switch                    |
| Blocking is fine                     | Blocking blocks the worker!          |

**Warning**: Don't block in async code!

```rust
// BAD: Blocks the worker thread!
async fn bad() {
    std::thread::sleep(Duration::from_secs(1));
}

// GOOD: Yields to scheduler
async fn good() {
    tokio::time::sleep(Duration::from_secs(1)).await;
}
```

---

## 7. Choosing the Right Model

### Decision Tree

```
┌─────────────────────────────────────────────────────────┐
│              When to Use Each I/O Model                  │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  Few connections (< 100)?                                │
│     YES → Blocking thread-per-connection is fine         │
│     NO  ↓                                                │
│                                                          │
│  CPU-bound work per request?                             │
│     YES → Thread pool with blocking I/O                  │
│     NO  ↓                                                │
│                                                          │
│  Many connections, I/O-bound?                            │
│     YES → Async (Tokio)                                  │
│                                                          │
│  Need ultimate control?                                  │
│     YES → mio (low-level epoll wrapper)                  │
│                                                          │
└─────────────────────────────────────────────────────────┘
```

### Real-World Examples

| Application               | I/O Model   | Why                               |
| ------------------------- | ----------- | --------------------------------- |
| Simple CLI tool           | Blocking    | Simple, few operations            |
| Database server           | Thread pool | CPU-bound query processing        |
| Web server (Nginx)        | epoll       | Many connections, mostly waiting  |
| Rust web framework (Axum) | Tokio       | Ergonomic async, high performance |

---

## 8. Observing I/O Behavior

### Using strace

```bash
# See I/O syscalls
strace -e read,write,accept,epoll_wait ./your_server

# Example output for blocking server:
# accept(3, ...) = 4           # Accept connection
# read(4, ...) = 100           # Read from client (blocks until data)
# write(4, ...) = 100          # Write response

# Example output for async server:
# epoll_wait(3, ...) = 1       # Wait for events
# accept(4, ...) = 5           # Accept (non-blocking)
# epoll_ctl(3, EPOLL_CTL_ADD, 5, ...) = 0  # Register new socket
# epoll_wait(3, ...) = 1       # Wait again
# read(5, ...) = 100           # Read ready data
```

### Using ss

```bash
# See socket states
ss -tulpn | grep your_server

# See connection counts
ss -s
```

---

## Summary

```
┌─────────────────────────────────────────────────────────┐
│                    I/O Model Summary                     │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  Blocking I/O:                                           │
│    + Simple to understand                                │
│    + Works everywhere                                    │
│    - Needs many threads for many connections             │
│    - Doesn't scale well (C10K problem)                   │
│                                                          │
│  Non-blocking + epoll:                                   │
│    + Handles many connections efficiently                │
│    + Low resource usage                                  │
│    - Complex to program manually                         │
│                                                          │
│  Async (Tokio):                                          │
│    + Best of both: scalable + ergonomic                  │
│    + Sequential-looking code                             │
│    - Must use async-aware libraries                      │
│    - Can't block in async code                           │
│                                                          │
└─────────────────────────────────────────────────────────┘
```

---

## Next Steps

After understanding the theory, proceed to hands-on practice:

1. **Lab 4**: Build a blocking echo server (thread-per-connection)
2. **Lab 5**: Build an async echo server (Tokio)

Compare their behavior under load using tools like `htop` and `strace`.
