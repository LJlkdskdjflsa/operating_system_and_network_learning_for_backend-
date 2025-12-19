//! Lab 3: strace Observation Experiment
//!
//! ## Goal
//! Write a program, then observe its system calls using strace
//!
//! ## Requirements
//! Implement a program that includes the following operations:
//! 1. Read a file
//! 2. Write a file
//! 3. Sleep for a period of time
//! 4. Create multiple threads
//!
//! ## How to Run
//! ```bash
//! cargo build --release
//! strace ./target/release/strace_demo
//! strace -f ./target/release/strace_demo  # Trace multi-threading
//! strace -c ./target/release/strace_demo  # Statistics of syscalls
//! ```
//!
//! ## Key Observations
//! Look for these in strace output:
//! - `openat` - Open file
//! - `read` - Read data
//! - `write` - Write data
//! - `close` - Close file
//! - `nanosleep` - Sleep
//! - `clone` - Create thread
//!
//! ## Verification
//! ```bash
//! cargo test                                    # Basic tests
//! cargo build --release
//! strace ./target/release/strace_demo          # Observe syscalls
//! strace -f ./target/release/strace_demo       # Trace multi-threading
//! strace -c ./target/release/strace_demo       # Statistics of syscalls
//! ```
//!
//! ## Acceptance Criteria
//! - [ ] `cargo test` passes
//! - [ ] Can run program with strace
//! - [ ] Can identify open/read/write/close
//! - [ ] Can explain what fd is
//! - [ ] Can use -f to trace multi-threading
//!
//! Check solution/main.rs after completing

use std::fs::File;
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== strace Observation Experiment ===\n");

    println!("[Experiment 1] File Reading");
    println!("  -> Observe: openat, read, close");
    file_read_experiment();

    println!("\n[Experiment 2] File Writing");
    println!("  -> Observe: openat (with O_CREAT), write, close");
    file_write_experiment();

    println!("\n[Experiment 3] Sleep 500ms");
    println!("  -> Observe: nanosleep");
    thread::sleep(Duration::from_millis(500));

    println!("\n[Experiment 4] Multi-threading");
    println!("  -> Observe: clone (use strace -f)");
    multi_thread_experiment();

    println!("\n=== Experiment Complete ===");
}

fn file_read_experiment() {
    let test_content = "This is test content for strace experiment.\n";
    std::fs::write("test.txt", test_content).expect("Failed to create test file");

    let mut file = File::open("test.txt").expect("Failed to open file");
    let mut content = String::new();
    let bytes_read = file.read_to_string(&mut content).expect("Failed to read");

    println!("  Read {} bytes", bytes_read);
}

fn file_write_experiment() {
    let mut file = File::create("output.txt").expect("Failed to create file");

    for i in 1..=3 {
        let line = format!("Line {}: Hello from Rust!\n", i);
        file.write_all(line.as_bytes()).expect("Failed to write");
    }

    println!("  Wrote 3 lines");
}

fn multi_thread_experiment() {
    let handles: Vec<_> = (1..=3)
        .map(|i| {
            thread::spawn(move || {
                println!("  Thread {} started", i);
                thread::sleep(Duration::from_millis(50));
                println!("  Thread {} finished", i);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}
