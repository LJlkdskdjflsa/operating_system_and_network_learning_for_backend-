# Chapter 2 Checkpoint

Use this to verify your understanding after completing all labs.

---

## Self-Assessment Questions

### Process & Thread

- [ ] Can explain the difference between process and thread
- [ ] Can explain what context switching is and why it's expensive
- [ ] Can explain when to use multi-process vs multi-thread
- [ ] Can implement a basic thread pool

### Memory

- [ ] Can explain stack vs heap allocation
- [ ] Can explain what a page fault is
- [ ] Can explain why cache locality matters for performance
- [ ] Can predict which memory access patterns will be faster

### I/O Model

- [ ] Can explain blocking vs non-blocking I/O
- [ ] Can explain how epoll enables handling many connections
- [ ] Can explain the relationship between Tokio and epoll
- [ ] Can implement both blocking and async network servers

---

## Lab Verification

### Lab 1: Process vs Thread

- [ ] Both versions produce correct results
- [ ] Can observe process/thread creation with `htop`
- [ ] Can explain the performance difference

### Lab 2: Thread Pool

- [ ] Thread pool correctly executes submitted tasks
- [ ] Workers wait for tasks without busy-waiting
- [ ] Pool can be gracefully shut down

### Lab 3: Memory Locality

- [ ] Sequential access is faster than random access
- [ ] Can explain why based on cache behavior
- [ ] Can use `perf stat` to observe cache misses (optional)

### Lab 4: Blocking Echo Server

- [ ] Server handles multiple clients
- [ ] Each client gets its own thread
- [ ] Can observe thread count in `htop`

### Lab 5: Async Echo Server

- [ ] Server handles multiple clients with few threads
- [ ] Uses Tokio async/await
- [ ] Can handle many more connections than blocking version

---

## Concept Connection Quiz

### 1. Process vs Thread Memory

**Question**: If thread A modifies a global variable, can thread B see the change? What about if process A modifies a variable - can process B see it?

<details>
<summary>Answer</summary>

- **Thread**: Yes, threads share the same memory space. Thread B will see the change (with proper synchronization).
- **Process**: No, each process has its own memory space. Process B cannot see process A's changes unless they use IPC (pipes, shared memory, sockets, etc.).

This is why threads need `Mutex` for synchronization, while processes need explicit IPC mechanisms.
</details>

### 2. Context Switch Cost

**Question**: Why is context switching expensive? What gets invalidated?

<details>
<summary>Answer</summary>

Context switching is expensive because:
1. **CPU caches** (L1, L2, L3) become "cold" - the new process/thread's data isn't cached
2. **TLB (Translation Lookaside Buffer)** entries for virtual-to-physical address mapping may be invalidated
3. **Branch predictor** state is lost
4. **CPU registers** must be saved and restored

For threads in the same process, TLB invalidation may be avoided (same address space), making thread switches cheaper than process switches.
</details>

### 3. Why Async for Servers

**Question**: Why do high-performance servers (Nginx, Tokio) use async I/O instead of thread-per-connection?

<details>
<summary>Answer</summary>

1. **Memory**: Each thread needs a stack (typically 2-8MB). 10,000 connections = 20-80GB just for stacks!
2. **Context switches**: With many threads, CPU spends time switching instead of doing work
3. **Scalability**: epoll can monitor thousands of file descriptors efficiently

Async I/O with epoll:
- Few threads (usually = CPU cores)
- Each thread handles many connections
- Only active connections consume CPU time
- Memory usage is proportional to actual work, not connection count
</details>

### 4. Cache Locality Impact

**Question**: You have a 2D array `arr[1000][1000]`. Which loop order is faster and why?

```rust
// Version A
for i in 0..1000 {
    for j in 0..1000 {
        sum += arr[i][j];
    }
}

// Version B
for j in 0..1000 {
    for i in 0..1000 {
        sum += arr[i][j];
    }
}
```

<details>
<summary>Answer</summary>

**Version A is faster** (often 10-100x faster for large arrays).

In Rust (and C), 2D arrays are stored in row-major order: `arr[0][0], arr[0][1], arr[0][2], ...`

- **Version A**: Accesses memory sequentially (good locality). When `arr[i][0]` is loaded, `arr[i][1]` to `arr[i][63]` (assuming 64-byte cache lines and 4-byte integers) are already in cache.

- **Version B**: Jumps 1000 elements between accesses (poor locality). Each access likely causes a cache miss.

This is why understanding memory layout matters for performance!
</details>

### 5. Syscalls in Echo Server

**Question**: What syscalls does a blocking echo server make for each client?

<details>
<summary>Answer</summary>

```
accept()      # Accept new connection, returns new socket fd
clone()       # Create new thread (if thread-per-connection)
read()        # Read data from client (blocks until data arrives)
write()       # Send data back to client
close()       # Close client socket when done
```

You can observe these with: `strace -f ./echo_server`

The `-f` flag follows child threads/processes.
</details>

---

## Practical Exercises

### Exercise 1: Measure Context Switch Time

```bash
# Run the process vs thread lab multiple times
# Compare the overhead when thread/process count increases

# Observe with htop while running
htop
```

### Exercise 2: Observe Page Faults

```bash
# Run the locality lab
# Use perf to see page faults

perf stat -e page-faults,cache-misses ./target/release/locality_demo
```

### Exercise 3: Compare Server Scalability

```bash
# Start blocking echo server
cargo run --release

# In another terminal, create many connections
# (You might need to write a simple load test or use a tool like `wrk`)

# Observe thread count
htop

# Now compare with async server
```

---

## Summary

If you can answer all the quiz questions and complete all labs with understanding, you have a solid grasp of:

1. How the OS manages processes and threads
2. How memory works (virtual memory, caching)
3. How I/O models affect server scalability
4. How to observe and measure system behavior

You're ready for Chapter 3: Network!
