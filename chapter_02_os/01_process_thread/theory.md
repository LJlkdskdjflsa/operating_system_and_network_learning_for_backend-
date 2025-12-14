# 2.1 Process & Thread: The Units of Execution

## Section Goals

> Understand how the OS executes your code and manages concurrent workloads

After completing this section, you will be able to:

- Explain the difference between process, thread, and async task
- Understand context switching and its performance cost
- Choose the right concurrency model for different scenarios
- Implement a thread pool from scratch

---

## 1. What is a Process?

### Definition

A **process** is an instance of a running program. It has:

- Its own **memory space** (code, data, heap, stack)
- Its own **file descriptors** (open files, sockets)
- Its own **PID** (process ID)
- Security context (user, permissions)

```
┌─────────────────────────────────────────────────────────┐
│                     Process A                            │
├─────────────────────────────────────────────────────────┤
│  Memory Space (virtual address space)                    │
│  ┌──────────┬──────────┬──────────┬──────────┐          │
│  │   Code   │   Data   │   Heap   │  Stack   │          │
│  │ (text)   │ (global) │    ↓     │    ↑     │          │
│  └──────────┴──────────┴──────────┴──────────┘          │
│                                                          │
│  File Descriptor Table                                   │
│  ┌────┬─────────────────┐                               │
│  │ 0  │ stdin           │                               │
│  │ 1  │ stdout          │                               │
│  │ 2  │ stderr          │                               │
│  │ 3  │ /home/data.txt  │                               │
│  └────┴─────────────────┘                               │
└─────────────────────────────────────────────────────────┘
```

### Creating a Process: fork() and exec()

On Unix/Linux, new processes are created by:

1. **fork()**: Creates a copy of the current process
2. **exec()**: Replaces the current process with a new program

```
Parent Process
     │
     │ fork()
     ├──────────────► Child Process (copy of parent)
     │                     │
     │                     │ exec("/bin/ls")
     │                     ▼
     │                Child Process (now running ls)
     │
     ▼
Parent continues
```

In Rust, you typically use `std::process::Command` which does fork+exec for you:

```rust
use std::process::Command;

let output = Command::new("ls")
    .arg("-la")
    .output()
    .expect("Failed to execute");

println!("{}", String::from_utf8_lossy(&output.stdout));
```

### Process Isolation

Processes are **isolated** from each other:

```
Process A                     Process B
┌─────────────┐              ┌─────────────┐
│ Memory: 0x1000 = 42       │ Memory: 0x1000 = 99
│             │              │             │
│ Can't see   │   Kernel     │ Can't see   │
│ Process B's ├──────────────┤ Process A's │
│ memory      │              │ memory      │
└─────────────┘              └─────────────┘
```

This isolation provides:
- **Security**: Process A can't read Process B's passwords
- **Stability**: If Process A crashes, Process B keeps running
- **Resource control**: Each process has its own limits

### Inter-Process Communication (IPC)

Since processes can't share memory directly, they communicate via:

| Method | Description | Use Case |
|--------|-------------|----------|
| Pipe | Unidirectional byte stream | Parent-child communication |
| Socket | Network or Unix domain socket | Any processes |
| Shared Memory | Kernel-managed shared region | High-performance IPC |
| Message Queue | Kernel-managed queue | Async messaging |
| File | Read/write to same file | Simple, persistent |

---

## 2. What is a Thread?

### Definition

A **thread** is a unit of execution within a process. Threads in the same process share:

- **Memory space** (code, data, heap)
- **File descriptors**
- **PID**

But each thread has its own:
- **Stack** (local variables, function calls)
- **Thread ID (TID)**
- **CPU registers**

```
┌─────────────────────────────────────────────────────────┐
│                       Process                            │
├─────────────────────────────────────────────────────────┤
│  Shared Memory (code, data, heap)                       │
│  ┌──────────┬──────────┬──────────────────────┐         │
│  │   Code   │   Data   │        Heap          │         │
│  └──────────┴──────────┴──────────────────────┘         │
│                                                          │
│  Thread 1        Thread 2        Thread 3               │
│  ┌──────┐        ┌──────┐        ┌──────┐               │
│  │Stack │        │Stack │        │Stack │               │
│  │  ↑   │        │  ↑   │        │  ↑   │               │
│  │ TID=1│        │ TID=2│        │ TID=3│               │
│  └──────┘        └──────┘        └──────┘               │
└─────────────────────────────────────────────────────────┘
```

### Creating Threads in Rust

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Hello from thread!");
    });

    handle.join().unwrap();
}
```

### Why Threads Share Memory

This is both powerful and dangerous:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Shared counter between threads
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;  // All threads modify the same counter
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());  // 10
}
```

Without proper synchronization, this leads to **data races**.

---

## 3. Process vs Thread: Comparison

| Aspect | Process | Thread |
|--------|---------|--------|
| Memory | Isolated | Shared |
| Creation cost | High (copy memory mappings) | Low (only stack) |
| Communication | IPC (pipes, sockets) | Direct memory access |
| Crash impact | Only that process | Entire process |
| Context switch | Expensive (TLB flush) | Cheaper (same address space) |

### When to Use Each

**Use Processes when:**
- Need isolation (security, fault tolerance)
- Running untrusted code
- Different programming languages
- Worker processes that might crash

**Use Threads when:**
- Need to share large amounts of data
- Low latency communication needed
- Same codebase, controlled environment
- Fine-grained parallelism

### Example: Web Server Architectures

```
Multi-Process (Apache prefork):
┌──────────┐
│  Master  │
│ Process  │
└────┬─────┘
     │ fork()
     ├───► Worker Process 1 (handles request)
     ├───► Worker Process 2 (handles request)
     └───► Worker Process 3 (handles request)

Multi-Thread (Java Tomcat):
┌──────────────────────────┐
│       Single Process      │
│  ┌───────┐  ┌───────┐    │
│  │Thread1│  │Thread2│    │
│  │(req)  │  │(req)  │    │
│  └───────┘  └───────┘    │
└──────────────────────────┘

Async (Nginx, Tokio):
┌──────────────────────────┐
│       Single Process      │
│  ┌───────────────────┐   │
│  │  Event Loop       │   │
│  │  (few threads)    │   │
│  │  handles 10000+   │   │
│  │  connections      │   │
│  └───────────────────┘   │
└──────────────────────────┘
```

---

## 4. Context Switching

### What is Context Switching?

When the OS switches from running one process/thread to another, it must:

1. Save the current state (registers, program counter)
2. Load the new state
3. Possibly flush caches (for processes)

```
┌─────────────────────────────────────────────────────────┐
│ Time ──────────────────────────────────────────────────►│
│                                                          │
│ Thread A:  ████████░░░░░░░░░░████████░░░░░░░░░░          │
│ Thread B:  ░░░░░░░░████████░░░░░░░░░░████████░░          │
│                    │       │        │       │            │
│                    └───────┴────────┴───────┘            │
│                     Context switches (overhead)          │
└─────────────────────────────────────────────────────────┘
```

### Why is it Expensive?

1. **Direct costs:**
   - Save/restore registers
   - Update kernel structures

2. **Indirect costs (the real killer):**
   - **Cache invalidation**: CPU caches contain old process's data
   - **TLB invalidation**: Address translation cache must be flushed (for process switch)
   - **Branch predictor**: Predictions for old code are useless

```
Cost hierarchy (approximate):

Register save/restore:     ~100 cycles
Kernel mode switch:        ~1000 cycles
Cache warmup:              ~10,000-100,000 cycles (varies greatly)
TLB refill:                ~10,000 cycles per page access
```

### Observing Context Switches

```bash
# See context switches for a process
cat /proc/[pid]/status | grep ctxt

# Or use perf
perf stat -e context-switches ./your_program
```

---

## 5. Kernel Threads vs User Threads

### Kernel Threads (1:1 model)

Each user thread maps to one kernel thread. The OS schedules them.

```
User Space    │    Kernel Space
              │
Thread 1 ─────┼────► Kernel Thread 1
              │
Thread 2 ─────┼────► Kernel Thread 2
              │
Thread 3 ─────┼────► Kernel Thread 3
```

**Rust's std::thread uses this model.**

Pros:
- True parallelism on multiple CPUs
- If one thread blocks, others continue

Cons:
- Context switch goes through kernel
- Creating thousands of threads is expensive

### User Threads / Green Threads (M:N model)

Many user threads map to fewer kernel threads. A runtime schedules them.

```
User Space    │    Kernel Space
              │
Task 1 ─┐     │
Task 2 ─┼──►  │    Kernel Thread 1
Task 3 ─┘     │
              │
Task 4 ─┐     │
Task 5 ─┼──►  │    Kernel Thread 2
Task 6 ─┘     │
```

**Tokio async tasks use this model.**

Pros:
- Very lightweight (small stack, fast switching)
- Can have millions of tasks

Cons:
- Need runtime support
- If task blocks (not awaits), it blocks the kernel thread

---

## 6. Thread Pool Pattern

### Why Thread Pools?

Creating threads is expensive. Instead:

1. Create a fixed number of worker threads at startup
2. Submit tasks to a queue
3. Workers pick up and execute tasks

```
┌─────────────────────────────────────────────────────────┐
│                     Thread Pool                          │
│                                                          │
│   Task Queue                                             │
│   ┌────┬────┬────┬────┬────┐                            │
│   │Task│Task│Task│Task│... │◄─── Submit tasks           │
│   └──┬─┴──┬─┴──┬─┴──┬─┴────┘                            │
│      │    │    │    │                                    │
│      ▼    ▼    ▼    ▼                                    │
│   ┌──────┬──────┬──────┬──────┐                         │
│   │Worker│Worker│Worker│Worker│ (fixed number)          │
│   │  1   │  2   │  3   │  4   │                         │
│   └──────┴──────┴──────┴──────┘                         │
└─────────────────────────────────────────────────────────┘
```

### Basic Implementation Sketch

```rust
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {} executing job", id);
            job();
        });

        Worker { id, thread }
    }
}
```

---

## 7. Observing Processes and Threads

### Using htop

```
Press F5 to see tree view (parent-child relationships)
Press H to show/hide threads

  PID USER   VIRT   RES   SHR S  CPU%  MEM%  Command
 1234 user   100M   50M   10M S   2.0   0.6  ./my_server
 1235 user   100M   10M    5M S   0.5   0.1  └─ worker_1
 1236 user   100M   10M    5M S   0.5   0.1  └─ worker_2
 1237 user   100M   10M    5M S   0.3   0.1  └─ worker_3
```

### Using strace

```bash
# See clone() syscall for thread creation
strace -f -e clone ./threaded_program

# Output:
clone(child_stack=0x7f..., flags=CLONE_VM|CLONE_FS|...) = 1235
clone(child_stack=0x7f..., flags=CLONE_VM|CLONE_FS|...) = 1236
```

The `CLONE_VM` flag means threads share memory (it's a thread, not a process).

### Using /proc

```bash
# List threads of process 1234
ls /proc/1234/task/
# Output: 1234 1235 1236 1237

# See thread count
cat /proc/1234/status | grep Threads
# Output: Threads: 4
```

---

## Summary: Choosing the Right Model

```
┌─────────────────────────────────────────────────────────┐
│             Concurrency Model Decision Tree              │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  Need isolation/security?                                │
│     YES → Use processes                                  │
│     NO  ↓                                                │
│                                                          │
│  CPU-bound work?                                         │
│     YES → Use threads (= CPU cores)                      │
│     NO  ↓                                                │
│                                                          │
│  Many concurrent I/O operations?                         │
│     YES → Use async (Tokio)                              │
│     NO  → Use threads                                    │
│                                                          │
└─────────────────────────────────────────────────────────┘
```

---

## Next Steps

After understanding the theory, proceed to hands-on practice:

1. **Lab 1**: Compare multi-process vs multi-thread performance
2. **Lab 2**: Build a thread pool from scratch
