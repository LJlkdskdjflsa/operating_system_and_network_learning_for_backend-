//! Lab 1 Tests
//!
//! Run with: cargo test

use std::process::Command;

/// Helper function: execute program and get output
fn run_mini_cat(args: &[&str]) -> (String, String, bool) {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--"])
        .args(args)
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let success = output.status.success();

    (stdout, stderr, success)
}

#[test]
fn test_01_read_file() {
    // Test: can read file contents
    let (stdout, _, success) = run_mini_cat(&["test.txt"]);

    assert!(success, "Program should execute successfully");
    assert!(
        stdout.contains("error"),
        "Output should contain test.txt contents (including the word 'error')"
    );
}

#[test]
fn test_02_filter_pattern() {
    // Test: can filter by pattern
    let (stdout, _, success) = run_mini_cat(&["test.txt", "error"]);

    assert!(success, "Program should execute successfully");

    // Should only have lines containing "error"
    for line in stdout.lines() {
        assert!(
            line.contains("error"),
            "After filtering, every line should contain 'error', but found: {}",
            line
        );
    }

    // Should have output (test.txt contains error)
    assert!(!stdout.is_empty(), "Should have output");
}

#[test]
fn test_03_line_numbers() {
    // Test: -n shows line numbers
    let (stdout, _, success) = run_mini_cat(&["test.txt", "-n"]);

    assert!(success, "Program should execute successfully");

    // Check for line number format (digit + colon)
    let has_line_numbers = stdout.lines().any(|line| {
        line.trim_start()
            .chars()
            .next()
            .map(|c| c.is_ascii_digit())
            .unwrap_or(false)
    });

    assert!(has_line_numbers, "Should show line numbers when using -n");
}

#[test]
fn test_04_file_not_found() {
    // Test: should error when file doesn't exist
    let (_, stderr, success) = run_mini_cat(&["this_file_does_not_exist.txt"]);

    assert!(!success || !stderr.is_empty(), "Should error or output error message when file doesn't exist");
}

#[test]
fn test_05_filter_with_line_numbers() {
    // Test: use pattern and -n together
    let (stdout, _, success) = run_mini_cat(&["test.txt", "error", "-n"]);

    assert!(success, "Program should execute successfully");

    // Every line should contain "error"
    for line in stdout.lines() {
        if !line.is_empty() {
            assert!(
                line.contains("error"),
                "After filtering, every line should contain 'error'"
            );
        }
    }
}
