//! Lab 2 Tests
//!
//! Run with: cargo test

use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[test]
fn test_01_program_runs() {
    let output = Command::new("cargo")
        .args(["run", "--quiet"])
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // If not implemented yet, skip
    if stdout.contains("not yet implemented") {
        return;
    }

    // Should see some task output
    assert!(
        stdout.contains("Task") || stdout.contains("pool"),
        "Program should produce task-related output"
    );
}

#[test]
fn test_02_tasks_execute() {
    let output = Command::new("cargo")
        .args(["run", "--quiet"])
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // If not implemented yet, skip
    if stdout.contains("not yet implemented") {
        return;
    }

    // Should see tasks starting and completing
    let task_mentions = stdout.matches("Task").count();
    assert!(
        task_mentions >= 4,
        "Should see multiple task executions, found {}",
        task_mentions
    );
}

#[test]
fn test_03_graceful_shutdown() {
    let output = Command::new("cargo")
        .args(["run", "--quiet"])
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // If not implemented yet, skip
    if stdout.contains("not yet implemented") {
        return;
    }

    // Should see successful shutdown message
    assert!(
        stdout.contains("dropped successfully") || stdout.contains("Shutting down"),
        "Should show graceful shutdown"
    );
}

#[test]
fn test_04_no_panics() {
    let output = Command::new("cargo")
        .args(["run", "--quiet"])
        .output()
        .expect("Failed to execute program");

    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should not panic
    assert!(
        !stderr.contains("panic") && !stderr.contains("PANIC"),
        "Program should not panic"
    );
}
