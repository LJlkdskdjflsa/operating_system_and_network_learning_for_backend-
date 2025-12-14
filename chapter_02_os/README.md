# Chapter 2: Operating System Fundamentals

> Deep understanding of OS resource management from a backend developer's perspective

---

## Learning Objectives

After completing this chapter, you will be able to:

- Understand the difference between Process, Thread, and Async Task
- Explain context switching and its performance implications
- Understand virtual memory, page faults, and cache locality
- Implement different I/O models (blocking, non-blocking, async)
- Use system tools to observe and diagnose program behavior

---

## Chapter Structure

```
chapter_02_os/
├── README.md                    ← You are here
├── checkpoint.md                ← Self-assessment
├── 01_process_thread/
│   ├── theory.md               ← Process, Thread, Context Switch
│   ├── lab_01_process_vs_thread/  ← Compare process vs thread
│   └── lab_02_thread_pool/        ← Implement a thread pool
├── 02_memory/
│   ├── theory.md               ← Virtual memory, Stack vs Heap
│   └── lab_03_locality/           ← Cache locality experiment
└── 03_io_model/
    ├── theory.md               ← Blocking, Non-blocking, Async
    ├── lab_04_blocking_echo/      ← Thread-per-connection server
    └── lab_05_async_echo/         ← Tokio async server
```

---

## Topics Overview

### 2.1 Process & Thread

| Concept | What You'll Learn |
|---------|-------------------|
| Process vs Thread | Memory isolation, shared resources, creation cost |
| Context Switch | Why it's expensive (cache, TLB invalidation) |
| Kernel vs User Thread | How OS schedules work |
| Coroutine/Async Task | Lightweight concurrency |

**Labs:**
- Lab 1: Compare multi-process vs multi-thread performance
- Lab 2: Build a simple thread pool from scratch

### 2.2 Memory

| Concept | What You'll Learn |
|---------|-------------------|
| Stack vs Heap | Where data lives, allocation speed |
| Virtual Memory | Address translation, page tables |
| Page Fault | What happens on memory access |
| Cache Locality | Why memory access patterns matter |

**Labs:**
- Lab 3: Measure cache locality impact on performance

### 2.3 I/O Model

| Concept | What You'll Learn |
|---------|-------------------|
| Blocking I/O | Simple but doesn't scale |
| Non-blocking + poll/epoll | Event-driven I/O |
| Async I/O (Tokio) | Rust's async runtime |
| Why Nginx uses epoll | Connection scaling |

**Labs:**
- Lab 4: Build a blocking echo server (thread-per-connection)
- Lab 5: Build an async echo server (Tokio)

---

## Prerequisites

Before starting this chapter, ensure you:

- [ ] Completed Chapter 1 (Rust fundamentals + Linux basics)
- [ ] Comfortable with Rust ownership and borrowing
- [ ] Can use `Arc`, `Mutex`, and channels
- [ ] Know basic Linux commands (`ps`, `htop`, `strace`)

---

## Recommended Learning Order

```
1. Read 01_process_thread/theory.md
      ↓
2. Complete Lab 1: Process vs Thread
      ↓
3. Complete Lab 2: Thread Pool
      ↓
4. Read 02_memory/theory.md
      ↓
5. Complete Lab 3: Memory Locality
      ↓
6. Read 03_io_model/theory.md
      ↓
7. Complete Lab 4: Blocking Echo Server
      ↓
8. Complete Lab 5: Async Echo Server
      ↓
9. Complete checkpoint.md self-assessment
```

---

## Tools You'll Use

| Tool | Purpose |
|------|---------|
| `htop` | Monitor CPU, memory, thread count |
| `strace` | Trace system calls (`clone`, `fork`, `read`, `write`) |
| `perf` | Performance profiling (cache misses, page faults) |
| `time` | Measure execution time |
| `/proc/[pid]/` | Read process information |

---

## Key Takeaways

By the end of this chapter, you should understand:

1. **When to use processes vs threads**
   - Processes for isolation (security, fault tolerance)
   - Threads for shared memory and lower overhead

2. **Why async is powerful for I/O-bound workloads**
   - Thousands of connections with few threads
   - Cooperative scheduling avoids context switch overhead

3. **How memory access patterns affect performance**
   - Sequential access >> random access
   - Cache-friendly code can be 10-100x faster

4. **How to observe system behavior**
   - Use `strace` to see what syscalls your program makes
   - Use `htop` to see resource usage in real-time
