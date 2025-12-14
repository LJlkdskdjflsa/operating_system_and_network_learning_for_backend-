# 1.1 Rust Core: Essential Language Skills for Backend

## Section Goals

> Master Rust's core concepts most relevant to "systems programming" and "concurrency"

After completing this section, you will be able to:

- Write memory-safe programs using the ownership system
- Handle errors properly without panicking
- Write safe multithreaded programs

---

## 1. Ownership

### Why Learn This?

In C/C++, memory management is a breeding ground for bugs:

- **Use after free**: Using memory after it's been freed
- **Double free**: Freeing the same memory twice
- **Memory leak**: Forgetting to free memory

Rust blocks these problems at **compile time** through "ownership".

### Core Rules (Only Three)

```rust
// Rule 1: Every value has one "owner"
let s1 = String::from("hello");  // s1 is the owner of "hello"

// Rule 2: Only one owner at a time
let s2 = s1;                     // Ownership transferred (move) to s2
// println!("{}", s1);           // Compile error! s1 is no longer valid

// Rule 3: When owner goes out of scope, value is dropped
{
    let s3 = String::from("world");
}   // s3 goes out of scope, memory automatically freed
```

### Relation to OS

When you `drop` an object holding system resources (like `File`), Rust automatically calls the `close()` system call:

```rust
{
    let file = File::open("test.txt")?;
    // use file...
}   // file goes out of scope → automatically calls close(fd)
```

You can observe this behavior with `strace` (Lab 3).

---

## 2. Borrowing

### Problem: What if every parameter pass requires ownership transfer?

```rust
fn print_length(s: String) {
    println!("Length: {}", s.len());
}   // s is dropped

fn main() {
    let s = String::from("hello");
    print_length(s);
    // println!("{}", s);  // Error! s has been moved
}
```

### Solution: Borrowing

```rust
fn print_length(s: &String) {   // Borrow, don't take ownership
    println!("Length: {}", s.len());
}

fn main() {
    let s = String::from("hello");
    print_length(&s);           // Lend it out
    println!("{}", s);          // Still usable!
}
```

### Borrowing Rules

```rust
let mut s = String::from("hello");

// Rule 1: Can have multiple immutable borrows
let r1 = &s;
let r2 = &s;
println!("{} {}", r1, r2);  // OK

// Rule 2: Or only one mutable borrow
let r3 = &mut s;
r3.push_str(" world");

// Rule 3: Immutable and mutable borrows cannot coexist
// let r4 = &s;             // If r3 is still in use, this would error
```

### Why Is This Important?

This rule prevents **data races** (extremely dangerous in concurrent programs):

- Two threads writing to the same memory simultaneously → undefined behavior
- Rust prevents this at compile time

---

## 3. Lifetimes

### Problem: How long can a borrow live?

```rust
fn longest(x: &str, y: &str) -> &str {  // Compile error!
    if x.len() > y.len() { x } else { y }
}
```

The compiler doesn't know how long the returned reference should live.

### Solution: Lifetime Annotations

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

`'a` tells the compiler: "The return value's lifetime is the same as the input parameters".

### Practical Tips

1. Most cases the compiler infers automatically (lifetime elision)
2. When the compiler complains, it's usually warning about potential dangling references
3. If lifetime annotations get complex, consider using `String` (owned) instead

---

## 4. Error Handling

### Rust's Error Handling Philosophy

- **Recoverable errors**: Use `Result<T, E>`
- **Unrecoverable errors**: Use `panic!` (program crashes immediately)

Backend services almost always use `Result`, because you don't want one error to crash the entire service.

### Basic Result Usage

```rust
use std::fs::File;
use std::io::Read;

fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;  // ? early returns on error
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn main() {
    match read_file("test.txt") {
        Ok(content) => println!("Content: {}", content),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### Error Handling Evolution

```rust
// Stage 1: Manual match (verbose but clear)
let file = match File::open(path) {
    Ok(f) => f,
    Err(e) => return Err(e),
};

// Stage 2: Use ? operator (concise)
let file = File::open(path)?;

// Stage 3: Use anyhow (more flexible, recommended)
use anyhow::{Context, Result};

fn read_config() -> Result<Config> {
    let content = std::fs::read_to_string("config.toml")
        .context("Failed to read config file")?;
    // ...
}
```

---

## 5. Multithreading Basics

### Why Do Backends Need Multithreading?

- Utilize multi-core CPUs
- Process multiple requests in parallel
- Run background tasks

### Basic Thread Spawn

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Hello from thread!");
    });

    handle.join().unwrap();  // Wait for thread to finish
}
```

### Problem: How to Share Data Between Threads?

```rust
// This won't work!
let counter = 0;
let handle = thread::spawn(|| {
    counter += 1;  // Error: can't modify external variable in closure
});
```

### Solution 1: Arc + Mutex (Shared Mutable State)

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Arc: Atomic Reference Counting, lets multiple threads share ownership
    // Mutex: Mutual exclusion lock, ensures only one thread can modify data at a time
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("Thread {} incremented count to {}", i, *num);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

### Solution 2: Channel (Message Passing)

```rust
use std::sync::mpsc;  // multi-producer, single-consumer
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send("Hello from thread!").unwrap();
    });

    let msg = rx.recv().unwrap();
    println!("Received: {}", msg);
}
```

Complex Example

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let worker_count = 3;

    // Multiple threads pass "state increments" and progress via channel
    for id in 1..=worker_count {
        let tx = tx.clone();
        thread::spawn(move || {
            for step in 1..=5 {
                let delta = 1; // Increment by 1 each step
                tx.send((id, step, delta)).unwrap();
                thread::sleep(Duration::from_millis(50));
            }
            // tx is automatically dropped when thread ends
        });
    }
    drop(tx); // Drop the main thread's sender so iteration can end

    // Main thread collects and updates shared state
    let mut state = 0;
    for (id, step, delta) in rx {
        state += delta;
        println!("recv from thread {id} step {step}: delta={delta} -> state={state}");
    }

    println!("Final state: {state}");
}
// Result
// recv from thread 1 step 1: delta=1 -> state=1
// recv from thread 2 step 1: delta=1 -> state=2
// recv from thread 3 step 1: delta=1 -> state=3
// ...
// Final state: 15
```

### Arc vs Mutex vs Channel: When to Use Which?

| Scenario                            | Recommended Solution                    |
| ----------------------------------- | --------------------------------------- |
| Multiple threads read same data     | `Arc<T>` (read-only sharing)            |
| Multiple threads read/write same data | `Arc<Mutex<T>>` or `Arc<RwLock<T>>`   |
| Pass messages/tasks between threads | `mpsc::channel` or `crossbeam_channel`  |
| High concurrency scenarios          | Consider channels to avoid lock contention |

---

## 6. Async Basics

### Why Do We Need Async?

Traditional multithreading:

- One thread per connection
- 10,000 connections = 10,000 threads
- Each thread consumes ~2MB stack → 20GB memory!

Async:

- Few threads handle many tasks
- Tasks yield CPU while waiting for I/O
- 10,000 connections might only need 4-8 threads

### Basic Syntax

```rust
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = fetch_data("https://httpbin.org/ip").await?;
    println!("{body}");
    Ok(())
}

async fn fetch_data(url: &str) -> Result<String, Error> {
    // This is an async function
    let response = reqwest::get(url).await?;  // .await "pauses" until complete
    response.text().await
}

// Result
// {
//   "origin": "36.231.189.249"
// }
```

### Key Concepts

1. **Future**: Represents a "value that will complete in the future"
2. **async fn**: Returns a `Future`
3. **.await**: Pauses current task, waits for Future to complete
4. **Runtime** (like Tokio): Responsible for executing and scheduling Futures

### For Now, Just Understand This

Detailed async will be covered in later chapters (I/O Model). For now, just know:

- Async is for efficiently handling large amounts of I/O operations
- Tokio is the most commonly used async runtime
- `.await` doesn't block the thread, it only pauses the current task

---

## Summary: How These Concepts Connect

```
┌─────────────────────────────────────────────────────┐
│                 Your Backend Service                │
├─────────────────────────────────────────────────────┤
│  Async Runtime (Tokio)                              │
│    ├── Task 1: Handle HTTP request                  │
│    │     └── Use Result for error handling          │
│    ├── Task 2: Handle another request               │
│    └── Shared state: Arc<RwLock<AppState>>          │
├─────────────────────────────────────────────────────┤
│  Ownership system ensures:                          │
│    - No data races                                  │
│    - Resources auto-released (file, socket, memory) │
│    - Most concurrency bugs caught at compile time   │
└─────────────────────────────────────────────────────┘
```

---

## Next Steps

After completing the theory, proceed to hands-on practice:

1. **Lab 1**: Implement mini cat/grep → Practice file I/O and error handling
2. **Lab 2**: Implement parallel computation → Practice Arc/Mutex/Channel
