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

    // TODO: Experiment 1 - File Reading
    // Create a test file, then read it
    // Hint: Use File::create to create, File::open to read
    println!("[Experiment 1] File Reading");
    println!("  TODO: Implement file reading");

    // TODO: Experiment 2 - File Writing
    // Create a new file, write some content
    println!("\n[Experiment 2] File Writing");
    println!("  TODO: Implement file writing");

    // TODO: Experiment 3 - Sleep
    // Use thread::sleep to sleep for a while
    println!("\n[Experiment 3] Sleep");
    println!("  TODO: Implement sleep");

    // TODO: Experiment 4 - Multi-threading
    // Create a few threads, observe clone syscall
    println!("\n[Experiment 4] Multi-threading");
    println!("  TODO: Implement multi-threading");

    println!("\n=== Experiment Complete ===");
}
