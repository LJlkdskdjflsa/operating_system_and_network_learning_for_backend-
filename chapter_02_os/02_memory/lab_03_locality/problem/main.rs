//! Lab 3: Cache Locality Experiment
//!
//! ## Goal
//! Demonstrate the performance impact of cache locality
//!
//! ## Requirements
//! Implement and compare:
//! 1. Sequential array access (good locality)
//! 2. Random array access (poor locality)
//! 3. Row-major vs column-major 2D array access
//!
//! ## Expected Output
//! ```
//! Array size: 10,000,000 elements (40 MB)
//!
//! Sequential sum: xxx ms, result: ...
//! Random sum:     xxx ms, result: ...
//! Speedup: Nx faster
//!
//! 2D Array (1000x1000):
//! Row-major sum:    xxx ms
//! Column-major sum: xxx ms
//! Speedup: Nx faster
//! ```
//!
//! ## Hints
//! - Use large arrays to see the effect (millions of elements)
//! - Use `rand::seq::SliceRandom` to shuffle indices
//! - Run in release mode: `cargo run --release`
//! - Make sure to actually use the results (prevent optimization)
//!
//! ## Verification
//! ```bash
//! cargo run --release      # Must use release mode!
//! perf stat -e cache-misses,cache-references ./target/release/locality_demo
//! ```
//!
//! ## Acceptance Criteria
//! - [ ] Sequential access is measurably faster than random
//! - [ ] Row-major is faster than column-major for 2D arrays
//! - [ ] Can explain why based on cache behavior
//!
//! Check solution/main.rs after completing

use rand::seq::SliceRandom;
use std::time::Instant;

// ============================================================
// TODO: Implement these benchmark functions
// ============================================================

/// Sum array elements sequentially (good locality)
fn sum_sequential(arr: &[i64]) -> i64 {
    // TODO: Simply sum all elements in order
    // This has excellent spatial locality
    todo!("Implement sequential sum")
}

/// Sum array elements in random order (poor locality)
fn sum_random(arr: &[i64], indices: &[usize]) -> i64 {
    // TODO: Sum elements using the shuffled indices
    // This has poor spatial locality
    todo!("Implement random sum")
}

/// Sum 2D array in row-major order (good locality for Rust arrays)
fn sum_row_major(arr: &[[i64; 1000]; 1000]) -> i64 {
    // TODO: Iterate rows first, then columns
    // for i in 0..1000 { for j in 0..1000 { ... } }
    todo!("Implement row-major sum")
}

/// Sum 2D array in column-major order (poor locality for Rust arrays)
fn sum_column_major(arr: &[[i64; 1000]; 1000]) -> i64 {
    // TODO: Iterate columns first, then rows
    // for j in 0..1000 { for i in 0..1000 { ... } }
    todo!("Implement column-major sum")
}

// ============================================================
// Benchmark helpers (no modification needed)
// ============================================================

fn benchmark<F, T>(name: &str, mut f: F) -> T
where
    F: FnMut() -> T,
{
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();
    println!("{:25} {:?}", name, duration);
    result
}

fn main() {
    // Make sure we're running in release mode
    #[cfg(debug_assertions)]
    {
        eprintln!("WARNING: Running in debug mode!");
        eprintln!("Results will be much slower and less meaningful.");
        eprintln!("Please run with: cargo run --release\n");
    }

    const SIZE: usize = 10_000_000;

    println!("=== Cache Locality Experiment ===\n");
    println!("Array size: {} elements ({} MB)\n", SIZE, SIZE * 8 / 1_000_000);

    // Create and initialize array
    println!("Creating array...");
    let arr: Vec<i64> = (0..SIZE as i64).collect();

    // Create shuffled indices for random access
    println!("Shuffling indices...");
    let mut indices: Vec<usize> = (0..SIZE).collect();
    let mut rng = rand::thread_rng();
    indices.shuffle(&mut rng);

    println!("Running benchmarks...\n");

    // Part 1: Sequential vs Random
    println!("--- 1D Array Access ---");
    let seq_result = benchmark("Sequential sum:", || sum_sequential(&arr));
    let rand_result = benchmark("Random sum:", || sum_random(&arr, &indices));

    // Verify results match
    assert_eq!(seq_result, rand_result, "Results should match!");
    println!("Results match: {}\n", seq_result);

    // Part 2: Row-major vs Column-major
    println!("--- 2D Array Access (1000x1000) ---");
    println!("Creating 2D array...");

    // Create 2D array on heap (too big for stack)
    let arr_2d: Box<[[i64; 1000]; 1000]> = {
        let mut arr = Box::new([[0i64; 1000]; 1000]);
        for i in 0..1000 {
            for j in 0..1000 {
                arr[i][j] = (i * 1000 + j) as i64;
            }
        }
        arr
    };

    let row_result = benchmark("Row-major sum:", || sum_row_major(&arr_2d));
    let col_result = benchmark("Column-major sum:", || sum_column_major(&arr_2d));

    assert_eq!(row_result, col_result, "Results should match!");
    println!("Results match: {}\n", row_result);

    println!("=== Analysis ===");
    println!("Sequential access is faster because:");
    println!("  - CPU fetches 64-byte cache lines");
    println!("  - Sequential access uses the entire cache line");
    println!("  - Random access wastes most of each cache line\n");

    println!("Row-major is faster because:");
    println!("  - Rust arrays are stored in row-major order");
    println!("  - Row-major iteration is sequential in memory");
    println!("  - Column-major jumps 1000 elements (8000 bytes) each time\n");

    println!("Try: perf stat -e cache-misses ./target/release/locality_demo");
}
