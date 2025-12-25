# Chapter 3 Checkpoint

## Self-Assessment Questions

Answer these questions to verify your understanding:

### TCP/UDP (Lab 1-2)

1. **What are the key differences between TCP and UDP?**
   - Connection-oriented vs connectionless
   - Reliability guarantees
   - Ordering guarantees
   - Use cases

2. **How does TCP ensure reliable delivery?**
   - Sequence numbers
   - Acknowledgments
   - Retransmission
   - Flow control

3. **When would you choose UDP over TCP?**
   - Video streaming
   - Gaming
   - DNS queries
   - When latency matters more than reliability

4. **How do you handle multiple clients in a chat server?**
   - Shared state management
   - Broadcasting messages
   - Client tracking

### HTTP (Lab 3-5)

5. **What are the components of an HTTP request?**
   - Method (GET, POST, etc.)
   - Path
   - Headers
   - Body

6. **What do common HTTP status codes mean?**
   - 200, 201, 204
   - 301, 302, 304
   - 400, 401, 403, 404
   - 500, 502, 503

7. **What is REST?**
   - Resource-based URLs
   - HTTP methods for CRUD
   - Statelessness
   - JSON payloads

8. **How does Axum route requests?**
   - Router setup
   - Path parameters
   - Query parameters
   - Request extractors

### Proxy (Lab 6)

9. **What is a reverse proxy?**
   - Client-facing server
   - Forwards to backends
   - Benefits (SSL termination, load balancing, caching)

10. **What are common load balancing strategies?**
    - Round-robin
    - Least connections
    - IP hash
    - Weighted

## Concept Quiz

### Question 1: TCP vs UDP
Which protocol would you use for a real-time multiplayer game?
- A) TCP for all communication
- B) UDP for all communication
- C) TCP for login/state, UDP for movement
- D) Neither

<details>
<summary>Answer</summary>
C) TCP for login/state, UDP for movement

TCP ensures important data (login credentials, game state) arrives reliably.
UDP is better for real-time updates where occasional packet loss is acceptable
and low latency is critical.
</details>

### Question 2: HTTP Methods
Which HTTP method should you use to update a specific field of a resource?
- A) GET
- B) POST
- C) PUT
- D) PATCH

<details>
<summary>Answer</summary>
D) PATCH

PUT replaces the entire resource.
PATCH updates specific fields.
POST creates new resources.
GET retrieves resources.
</details>

### Question 3: TCP Connection
What is the TCP three-way handshake?
- A) SYN -> ACK -> FIN
- B) SYN -> SYN-ACK -> ACK
- C) ACK -> SYN -> FIN
- D) FIN -> ACK -> RST

<details>
<summary>Answer</summary>
B) SYN -> SYN-ACK -> ACK

1. Client sends SYN (synchronize)
2. Server responds with SYN-ACK
3. Client sends ACK (acknowledge)
Connection is now established.
</details>

### Question 4: HTTP Status
A client receives status code 503. What happened?
- A) Resource not found
- B) Client sent bad request
- C) Server is temporarily unavailable
- D) Authentication required

<details>
<summary>Answer</summary>
C) Server is temporarily unavailable

503 Service Unavailable indicates the server cannot handle the request
temporarily (maintenance, overload, etc.).

404 = Not found
400 = Bad request
401 = Authentication required
</details>

### Question 5: Load Balancing
Which load balancing strategy ensures the same client always goes to the same backend?
- A) Round-robin
- B) Least connections
- C) Random
- D) IP hash

<details>
<summary>Answer</summary>
D) IP hash

IP hash computes a hash of the client's IP address to determine which
backend to use. This provides session affinity (sticky sessions).

Round-robin and random distribute requests without considering the client.
Least connections routes to the server with fewest active connections.
</details>

## Practical Verification

### TCP Chat Server
```bash
# Start server
cargo run

# Connect multiple clients
nc localhost 8080  # Terminal 1
nc localhost 8080  # Terminal 2

# Type in one terminal, see it in both
# Verify: messages broadcast to all clients
```

### UDP Echo
```bash
# Start server
cargo run

# Send UDP packet
echo "hello" | nc -u localhost 8080

# Verify: response received despite UDP being "unreliable"
```

### Raw HTTP
```bash
# Start server
cargo run

# Test with curl
curl -v http://localhost:8080/
curl -v http://localhost:8080/hello

# Verify: proper HTTP response with headers
```

### Axum API
```bash
# Start server
cargo run

# Test CRUD operations
curl http://localhost:8080/items
curl -X POST -H "Content-Type: application/json" -d '{"name":"test"}' http://localhost:8080/items
curl http://localhost:8080/items/1
curl -X DELETE http://localhost:8080/items/1

# Verify: JSON responses, proper status codes
```

### Reverse Proxy
```bash
# Start backend server(s)
# Start proxy
cargo run

# Send requests through proxy
curl http://localhost:8080/

# Verify: requests forwarded to backend
```

## Key Takeaways

1. **TCP provides reliability** - at the cost of latency
2. **UDP provides speed** - at the cost of reliability
3. **HTTP is text-based** - can be debugged with simple tools
4. **Frameworks abstract complexity** - but understand what's underneath
5. **Proxies add flexibility** - load balancing, SSL termination, caching
