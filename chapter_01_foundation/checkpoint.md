# Chapter 1 Checkpoint

Use this checklist to verify you've mastered the core concepts of this chapter.

---

## Self-Assessment Questions

### Rust Fundamentals (1.1)

**Ownership and Borrowing**
- [ ] Can explain what "ownership transfer" (move) means
- [ ] Can explain the difference between `&T` and `&mut T`
- [ ] Can explain why you can't have multiple `&mut T` simultaneously

**Error Handling**
- [ ] Can explain the purpose of `Result<T, E>`
- [ ] Can correctly use the `?` operator
- [ ] Know when to use `unwrap()` vs `expect()` vs `?`

**Multithreading**
- [ ] Can explain the purpose of `Arc`
- [ ] Can explain the purpose of `Mutex`
- [ ] Can explain when to use `mpsc::channel`
- [ ] Know what `move` closures mean

### Linux Basics (1.2)

**Basic Concepts**
- [ ] Can explain the difference between process and thread
- [ ] Can explain what a file descriptor (fd) is
- [ ] Know what fd 0, 1, 2 represent

**System Tools**
- [ ] Can use `ps aux` to view processes
- [ ] Can use `htop` to monitor the system
- [ ] Can use `strace` to trace system calls
- [ ] Know what `/proc` is

---

## Implementation Verification

### Lab 1: Mini Cat/Grep ✓

```bash
# Test commands
cd chapter_01_foundation/01_rust_fundamentals/lab_01_mini_cat
cargo run -- test.txt
cargo run -- test.txt error
cargo run -- test.txt error -n
```

Acceptance criteria:
- [ ] Can correctly read and display file contents
- [ ] Can filter lines by keyword
- [ ] Can display line numbers
- [ ] Shows friendly error message when file doesn't exist

### Lab 2: Parallel Computation ✓

```bash
cd chapter_01_foundation/01_rust_fundamentals/lab_02_parallel_sum
cargo run --release
```

Acceptance criteria:
- [ ] All three versions calculate correct results
- [ ] Performance comparison completed
- [ ] Can explain differences between versions

### Lab 3: strace Observation ✓

```bash
cd chapter_01_foundation/02_linux_basics/lab_03_strace
cargo build --release
strace ./target/release/strace_demo
```

Acceptance criteria:
- [ ] Can identify open/read/write/close syscalls
- [ ] Can trace multithreaded programs (`strace -f`)
- [ ] Can get syscall statistics (`strace -c`)

### Lab 4: Mini PS ✓

```bash
cd chapter_01_foundation/02_linux_basics/lab_04_mini_ps
cargo run  # Requires Linux environment
```

Acceptance criteria:
- [ ] Can list all processes
- [ ] Can display PID, PPID, STATE, MEMORY
- [ ] Can display command line

---

## Concept Connection Quiz

Answer the following questions (one or two sentences):

1. **What's the relationship between Rust's ownership system and Linux's fd?**

   _Hint: Think about what happens when a `File` is dropped_

   Your answer:
   ```

   ```

2. **Why can `Arc<Mutex<T>>` be shared between threads?**

   _Hint: What problems do `Arc` and `Mutex` each solve?_

   Your answer:
   ```

   ```

3. **What does `strace` showing `write(1, "Hello\n", 6)` mean?**

   Your answer:
   ```

   ```

4. **Why is reading large files with `BufReader` more efficient than direct `read`?**

   _Hint: Think about the cost of system calls_

   Your answer:
   ```

   ```

---

## Reference Answers

<details>
<summary>Click to expand answers</summary>

1. **Ownership and fd**

   When a Rust `File` object is dropped, it automatically calls the `close()` system call to close the corresponding fd. This is Rust's RAII (Resource Acquisition Is Initialization) mechanism — the resource's lifetime is bound to the object's lifetime, no manual management needed.

2. **Arc + Mutex**

   - `Arc` (Atomic Reference Counting) solves the "multiple owners" problem: allows multiple threads to hold references to the same data
   - `Mutex` solves the "concurrent writes" problem: ensures only one thread can access the data at a time

3. **write(1, "Hello\n", 6)**

   - `write` is the system call for writing data
   - `1` is the file descriptor, representing stdout
   - `"Hello\n"` is the content to write
   - `6` is the number of bytes to write

4. **BufReader efficiency**

   Each system call has context switch overhead (switching from user space to kernel space). `BufReader` reads a larger chunk (default 8KB) into a buffer at once. Subsequent reads can be served directly from the buffer, reducing the number of system calls.

</details>

---

## Before Moving to Next Chapter

After confirming all items above are checked, you can proceed to Chapter 2: **OS: Process / Thread / Memory / I/O**.

Chapter 2 will dive deeper into:
- System-level differences between Process vs Thread
- Memory management and virtual memory
- I/O models: blocking, non-blocking, async
