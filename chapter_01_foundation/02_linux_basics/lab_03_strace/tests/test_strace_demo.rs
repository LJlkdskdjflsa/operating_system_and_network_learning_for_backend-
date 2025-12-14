//! Lab 3 Tests
//!
//! Run with: cargo test
//!
//! This lab is mainly about observing strace output, so tests only verify the program runs

use std::process::Command;

#[test]
fn test_01_program_runs() {
    // Test: program can run
    let output = Command::new("cargo")
        .args(["run", "--quiet"])
        .output()
        .expect("Failed to execute program");

    assert!(output.status.success(), "Program should execute successfully");
}

#[test]
fn test_02_creates_output_file() {
    // First run the program
    Command::new("cargo")
        .args(["run", "--quiet"])
        .output()
        .expect("Failed to execute program");

    // If file writing is implemented, test_output.txt should be created
    // (This is optional, depends on your implementation)
}

#[test]
fn test_03_has_experiment_output() {
    // Test: program outputs experiment information
    let output = Command::new("cargo")
        .args(["run", "--quiet"])
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should have experiment titles
    assert!(
        stdout.contains("Experiment") || stdout.contains("strace"),
        "Program should output experiment information"
    );
}

// ============================================================
// The real acceptance is observing with strace
// ============================================================
//
// Run the following commands to observe system calls:
//
// ```bash
// cargo build --release
// strace ./target/release/strace_demo
// ```
//
// You should see:
// - openat(...) - Open file
// - read(...) - Read data
// - write(...) - Write data (including println!)
// - close(...) - Close file
// - nanosleep(...) - Sleep
//
// Use strace -f to see multi-threading clone()
