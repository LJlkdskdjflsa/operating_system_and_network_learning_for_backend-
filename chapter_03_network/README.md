# Chapter 3: Network

## Learning Objectives

After completing this chapter, you will be able to:

1. **TCP/UDP Fundamentals**
   - Understand the difference between TCP and UDP
   - Implement reliable communication with TCP
   - Know when to use UDP vs TCP
   - Handle multiple clients with a chat server

2. **HTTP Protocol**
   - Parse and construct HTTP requests/responses manually
   - Understand HTTP methods, headers, status codes
   - Build a REST API with Axum
   - Handle JSON serialization/deserialization

3. **Proxy and Load Balancing**
   - Implement a reverse proxy
   - Understand load balancing strategies
   - Handle connection pooling
   - Manage upstream server health

## Chapter Structure

```
chapter_03_network/
├── README.md                    # This file
├── checkpoint.md                # Self-assessment
├── 01_tcp_udp/
│   ├── theory.md               # TCP vs UDP, sockets, protocols
│   ├── lab_01_chat_server/     # Multi-client TCP chat
│   └── lab_02_udp_echo/        # UDP echo with packet loss simulation
├── 02_http/
│   ├── theory.md               # HTTP protocol, REST, headers
│   ├── lab_03_raw_http/        # HTTP server from scratch
│   ├── lab_04_axum_api/        # REST API with Axum
│   └── lab_05_streaming_http/  # Streaming HTTP responses
└── 03_proxy/
    ├── theory.md               # Reverse proxy, load balancing
    └── lab_06_reverse_proxy/   # Simple reverse proxy
```

## Prerequisites

- Completed Chapter 2 (especially I/O model concepts)
- Understanding of async/await in Rust
- Basic knowledge of networking concepts (IP, ports)

## Labs Overview

| Lab | Topic | Key Concepts |
|-----|-------|--------------|
| Lab 1 | TCP Chat Server | TCP, broadcast, shared state |
| Lab 2 | UDP Echo | UDP, datagrams, packet handling |
| Lab 3 | Raw HTTP Server | HTTP parsing, request/response |
| Lab 4 | Axum REST API | Framework, routing, JSON |
| Lab 5 | Streaming HTTP | Chunked, SSE, streaming responses |
| Lab 6 | Reverse Proxy | Proxying, load balancing |

## Tools for Observation

```bash
# Network monitoring
netstat -an | grep 8080        # View connections
ss -tlnp                       # Socket statistics
lsof -i :8080                  # Who's using port

# Packet capture (requires root)
sudo tcpdump -i lo port 8080   # Capture packets

# HTTP testing
curl -v http://localhost:8080  # Verbose HTTP request
curl -X POST -d '{}' URL       # POST request

# Load testing
wrk -t12 -c400 -d30s URL       # HTTP benchmark
ab -n 1000 -c 100 URL          # Apache Bench
```

## Recommended Reading

- Beej's Guide to Network Programming
- RFC 793 (TCP), RFC 768 (UDP)
- RFC 7230-7235 (HTTP/1.1)
- Tokio documentation (async networking)

## Time Estimate

- Theory reading: 2-3 hours
- Labs: 4-6 hours
- Total: 6-9 hours
