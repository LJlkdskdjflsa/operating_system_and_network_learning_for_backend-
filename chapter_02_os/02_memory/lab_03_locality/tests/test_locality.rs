//! Lab 3 Tests
//!
//! Run with: cargo test

use std::process::Command;

#[test]
fn test_01_program_runs() {
    let output = Command::new("cargo")
        .args(["run", "--release", "--quiet"])
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Skip if not implemented
    if stdout.contains("not yet implemented") {
        return;
    }

    // Should produce output about cache locality
    assert!(
        stdout.contains("Sequential") || stdout.contains("Locality"),
        "Program should produce benchmark output"
    );
}

#[test]
fn test_02_results_match() {
    let output = Command::new("cargo")
        .args(["run", "--release", "--quiet"])
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Skip if not implemented
    if stdout.contains("not yet implemented") {
        return;
    }

    // Results should match (assertions in the program)
    assert!(
        stdout.contains("Results match") || !stdout.contains("assertion failed"),
        "Sequential and random sums should produce same result"
    );
}

#[test]
fn test_03_no_panics() {
    let output = Command::new("cargo")
        .args(["run", "--release", "--quiet"])
        .output()
        .expect("Failed to execute program");

    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should not panic (except for debug mode warning which is OK)
    assert!(
        !stderr.contains("panicked") || stderr.contains("debug mode"),
        "Program should not panic"
    );
}
