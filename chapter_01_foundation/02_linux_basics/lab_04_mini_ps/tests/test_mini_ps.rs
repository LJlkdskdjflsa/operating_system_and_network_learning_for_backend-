//! Lab 4 Tests
//!
//! Run with: cargo test
//!
//! Note: These tests require a Linux environment

use std::process::Command;

/// Check if running in Linux environment
fn is_linux() -> bool {
    std::path::Path::new("/proc").exists()
}

#[test]
fn test_01_program_runs() {
    let output = Command::new("cargo")
        .args(["run", "--quiet"])
        .output()
        .expect("Failed to execute program");

    // In non-Linux environment, should show error message but not panic
    if !is_linux() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("/proc") || stderr.contains("Linux"),
            "In non-Linux environment, should show message requiring Linux"
        );
        return;
    }

    assert!(output.status.success(), "Program should execute successfully");
}

#[test]
#[cfg(target_os = "linux")]
fn test_02_shows_header() {
    let output = Command::new("cargo")
        .args(["run", "--quiet"])
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should have header row
    assert!(
        stdout.contains("PID") && stdout.contains("COMMAND"),
        "Should display PID and COMMAND headers"
    );
}

#[test]
#[cfg(target_os = "linux")]
fn test_03_shows_processes() {
    let output = Command::new("cargo")
        .args(["run", "--quiet"])
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    // Should have multiple lines of output (header + at least some processes)
    assert!(
        lines.len() > 2,
        "Should list multiple processes, but only got {} lines",
        lines.len()
    );
}

#[test]
#[cfg(target_os = "linux")]
fn test_04_shows_init_process() {
    let output = Command::new("cargo")
        .args(["run", "--quiet"])
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // PID 1 should exist (init/systemd)
    // Check for lines starting with whitespace then "1"
    let has_pid_1 = stdout.lines().any(|line| {
        let trimmed = line.trim_start();
        trimmed.starts_with("1 ") || trimmed.starts_with("1\t")
    });

    assert!(has_pid_1, "Should be able to see PID 1 (init/systemd)");
}

// ============================================================
// Manual Verification
// ============================================================
//
// Compare your output with the system's ps command:
//
// ```bash
// cargo run
// ps aux | head -20
// ```
//
// Check:
// - Is the PID count similar?
// - Can you see common processes (init, systemd, bash, etc.)?
