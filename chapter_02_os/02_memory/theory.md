# 2.2 Memory: Stack, Heap, and Cache Locality

## Section Goals

> Understand how memory works and why access patterns matter for performance

After completing this section, you will be able to:

- Explain stack vs heap allocation
- Understand virtual memory and page faults
- Predict which memory access patterns will be faster
- Write cache-friendly code

---

## 1. Memory Layout of a Process

Every process has a virtual address space divided into regions:

```
High Address
┌─────────────────────────────────────┐
│            Stack                     │  ← Grows downward
│         (local variables,            │
│          function calls)             │
│              ↓                       │
├─────────────────────────────────────┤
│                                      │
│         (unmapped space)             │
│                                      │
├─────────────────────────────────────┤
│              ↑                       │
│            Heap                      │  ← Grows upward
│      (dynamic allocation:            │
│       Box, Vec, String)              │
├─────────────────────────────────────┤
│            BSS                       │  ← Uninitialized globals
├─────────────────────────────────────┤
│            Data                      │  ← Initialized globals
├─────────────────────────────────────┤
│            Text (Code)               │  ← Program instructions
└─────────────────────────────────────┘
Low Address
```

---

## 2. Stack vs Heap

### Stack

- **Fast allocation**: Just move stack pointer
- **Automatic cleanup**: Variables freed when function returns
- **Fixed size**: Typically 2-8 MB per thread
- **LIFO order**: Last in, first out

```rust
fn example() {
    let x = 42;           // On stack
    let arr = [0u8; 100]; // On stack (fixed size array)

    // When example() returns, x and arr are automatically freed
}
```

### Heap

- **Slower allocation**: Need to find free space, track allocations
- **Manual control**: Rust uses ownership for automatic cleanup
- **Unlimited size**: Only bounded by system memory
- **Random access**: Can allocate/free in any order

```rust
fn example() {
    let v = vec![1, 2, 3];     // Data on heap, pointer on stack
    let s = String::from("hi"); // Data on heap, pointer on stack
    let b = Box::new(42);       // 42 on heap, pointer on stack

    // When example() returns, Rust automatically frees heap memory
}
```

### Visualization

```
Stack                          Heap
┌────────────────┐            ┌────────────────────────┐
│ v: ptr, len=3, │───────────►│ [1, 2, 3]              │
│     cap=3      │            ├────────────────────────┤
├────────────────┤            │ "hi"                   │
│ s: ptr, len=2, │───────────►│                        │
│     cap=2      │            ├────────────────────────┤
├────────────────┤            │ 42                     │
│ b: ptr         │───────────►│                        │
├────────────────┤            └────────────────────────┘
│ x: 42          │
│ arr: [0; 100]  │
└────────────────┘
```

---

## 3. Virtual Memory

### Why Virtual Memory?

Each process thinks it has the entire address space to itself. The OS and CPU translate virtual addresses to physical addresses.

```
Process A                  Physical Memory           Process B
┌──────────────┐          ┌──────────────┐          ┌──────────────┐
│ 0x1000: data │──┐       │ 0x5000: ...  │       ┌──│ 0x1000: data │
│              │  │       │ 0x6000: A's  │◄──────┘  │              │
│              │  └──────►│        data  │          │              │
│              │          │ 0x7000: B's  │◄─────────│              │
└──────────────┘          │        data  │          └──────────────┘
                          └──────────────┘

Both processes use address 0x1000, but they map to different physical memory!
```

### Benefits

1. **Isolation**: Processes can't access each other's memory
2. **Simplicity**: Each process has consistent address layout
3. **Overcommit**: Can allocate more virtual memory than physical RAM
4. **Shared libraries**: Same code can be mapped into multiple processes

### Page Tables

Memory is divided into **pages** (typically 4KB). The **page table** maps virtual pages to physical frames.

```
Virtual Address: 0x12345678
                 ├────────┼────────┤
                 │Page Num│ Offset │
                 │ 0x12345│  0x678 │
                        │
                        ▼
                 Page Table
                 ┌─────────┬─────────┐
                 │ Virtual │Physical │
                 │  0x12345│ 0xABCDE │
                 └─────────┴─────────┘
                        │
                        ▼
Physical Address: 0xABCDE678
```

---

## 4. Page Faults

A **page fault** occurs when a program accesses memory that isn't currently in physical RAM.

### Types of Page Faults

| Type | Cause | Result |
|------|-------|--------|
| Minor | Page in RAM but not mapped | Update page table (fast) |
| Major | Page on disk (swapped out) | Load from disk (slow!) |
| Invalid | Accessing unmapped address | Segmentation fault |

### Why Major Page Faults Are Expensive

```
Memory access:    ~100 nanoseconds
SSD read:         ~100 microseconds  (1000x slower!)
HDD read:         ~10 milliseconds   (100,000x slower!)
```

### Observing Page Faults

```bash
# Count page faults for a program
perf stat -e page-faults,minor-faults,major-faults ./your_program

# Watch page faults in real-time
vmstat 1
```

---

## 5. CPU Caches

### The Memory Hierarchy

```
┌──────────────────────────────────────────────────────────────┐
│ Speed    │ Size      │ Location                              │
├──────────┼───────────┼───────────────────────────────────────┤
│ ~1 ns    │ ~64 KB    │ L1 Cache (per core)                   │
│ ~4 ns    │ ~256 KB   │ L2 Cache (per core)                   │
│ ~12 ns   │ ~8 MB     │ L3 Cache (shared)                     │
│ ~100 ns  │ ~16 GB    │ Main Memory (RAM)                     │
│ ~100 μs  │ ~1 TB     │ SSD                                   │
│ ~10 ms   │ ~4 TB     │ HDD                                   │
└──────────┴───────────┴───────────────────────────────────────┘
```

### Cache Lines

CPUs don't fetch individual bytes - they fetch **cache lines** (typically 64 bytes).

```
When you access arr[0]:
┌─────────────────────────────────────────────────────────────┐
│ arr[0] arr[1] arr[2] arr[3] ... arr[15]                     │
│ ←─────────── 64-byte cache line ────────────►               │
└─────────────────────────────────────────────────────────────┘

All of arr[0] through arr[15] (for 4-byte integers) are now in cache!
```

---

## 6. Cache Locality

### Spatial Locality

Accessing memory near recently accessed memory is fast (already in cache line).

```rust
// GOOD: Sequential access - great spatial locality
let mut sum = 0;
for i in 0..1000000 {
    sum += arr[i];  // arr[i+1] is likely in same cache line
}

// BAD: Large stride - poor spatial locality
let mut sum = 0;
for i in (0..1000000).step_by(64) {
    sum += arr[i];  // Each access likely misses cache
}
```

### Temporal Locality

Accessing recently accessed memory is fast (still in cache).

```rust
// GOOD: Reuse same data
for _ in 0..1000 {
    for item in &small_array {
        process(item);  // small_array stays in cache
    }
}

// BAD: Process huge array multiple times
for _ in 0..1000 {
    for item in &huge_array {
        process(item);  // huge_array doesn't fit in cache
    }
}
```

### 2D Array Access Pattern

Row-major order (Rust/C) vs column-major order matters!

```rust
let arr = [[0i32; 1000]; 1000];

// GOOD: Row-major access (matches memory layout)
for i in 0..1000 {
    for j in 0..1000 {
        process(arr[i][j]);  // Sequential in memory
    }
}

// BAD: Column-major access (jumps around)
for j in 0..1000 {
    for i in 0..1000 {
        process(arr[i][j]);  // Jumps 1000 elements each time!
    }
}
```

Memory layout:
```
arr[0][0], arr[0][1], arr[0][2], ... arr[0][999], arr[1][0], arr[1][1], ...

Row-major iterates: 0, 1, 2, 3, 4, ...            (sequential)
Column-major iterates: 0, 1000, 2000, 3000, ...  (huge jumps!)
```

---

## 7. Measuring Cache Performance

### Using perf

```bash
# Count cache misses
perf stat -e cache-references,cache-misses,L1-dcache-load-misses ./your_program

# Example output:
#  1,000,000 cache-references
#    100,000 cache-misses      # 10% miss rate
```

### Using Rust benchmarks

```rust
use std::time::Instant;

fn benchmark_sequential(arr: &[i32]) -> i32 {
    let start = Instant::now();
    let sum: i32 = arr.iter().sum();
    println!("Sequential: {:?}", start.elapsed());
    sum
}

fn benchmark_random(arr: &[i32], indices: &[usize]) -> i32 {
    let start = Instant::now();
    let sum: i32 = indices.iter().map(|&i| arr[i]).sum();
    println!("Random: {:?}", start.elapsed());
    sum
}
```

---

## 8. Rust Memory Types

### Stack Types (Copy)

```rust
let x: i32 = 42;        // Stack
let arr: [u8; 10] = [0; 10];  // Stack (small, fixed size)
let tuple: (i32, f64) = (1, 2.0);  // Stack
```

### Heap Types

```rust
let v: Vec<i32> = vec![1, 2, 3];  // Heap data, stack metadata
let s: String = String::from("hello");  // Heap data
let b: Box<i32> = Box::new(42);  // Heap (explicit boxing)
```

### Smart Pointers

| Type | Ownership | Use Case |
|------|-----------|----------|
| `Box<T>` | Single owner | Heap allocation, recursive types |
| `Rc<T>` | Multiple owners (single-threaded) | Shared read-only data |
| `Arc<T>` | Multiple owners (thread-safe) | Shared data across threads |

---

## 9. Practical Tips

### For Better Cache Performance

1. **Use contiguous data structures**
   - `Vec<T>` over `LinkedList<T>`
   - Array of structs over struct of arrays (sometimes)

2. **Access data sequentially when possible**
   - Iterate arrays in order
   - Process data in chunks

3. **Keep hot data small**
   - Separate frequently-used fields from rarely-used ones
   - Use smaller data types when possible

4. **Be aware of alignment**
   - Misaligned access can be slower
   - Rust generally handles this well

### Anti-Patterns

```rust
// BAD: Pointer chasing (linked list)
struct Node {
    value: i32,
    next: Option<Box<Node>>,  // Each node is separate allocation
}

// BETTER: Contiguous storage
let values: Vec<i32> = vec![1, 2, 3, 4, 5];  // All data together
```

---

## Summary

```
┌─────────────────────────────────────────────────────────────┐
│                Memory Performance Tips                       │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  1. Stack is faster than heap (no allocation overhead)       │
│                                                              │
│  2. Sequential access >> Random access (cache locality)      │
│                                                              │
│  3. Keep working set small (fit in cache)                    │
│                                                              │
│  4. Avoid major page faults (don't exceed physical RAM)      │
│                                                              │
│  5. Use Vec over LinkedList for most cases                   │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

---

## Next Steps

After understanding the theory, proceed to hands-on practice:

1. **Lab 3**: Measure the impact of cache locality on performance
