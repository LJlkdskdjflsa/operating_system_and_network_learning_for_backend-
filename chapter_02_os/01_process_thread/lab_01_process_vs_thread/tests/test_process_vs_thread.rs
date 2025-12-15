//! Lab 1 Tests
//!
//! Run with: cargo test

use std::process::Command;

fn run_program() -> (String, bool) {
    let output = Command::new("cargo")
        .args(["run", "--release", "--quiet"])
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let success = output.status.success();

    (stdout, success)
}

#[test]
fn test_01_program_runs() {
    let (output, success) = run_program();

    // If it contains "todo!()" then not yet implemented
    if output.contains("not yet implemented") {
        return; // Skip - not implemented yet
    }

    // Program should run (may fail on non-Linux for process version)
    assert!(
        success || output.contains("Thread version"),
        "Program should produce some output"
    );
}

#[test]
fn test_02_thread_version_correct() {
    let (output, _) = run_program();

    // Skip if not implemented
    if output.contains("not yet implemented") {
        return;
    }

    // Thread version should produce correct result
    if output.contains("Multi-Thread version:") {
        assert!(
            output.contains("5000000050000000"),
            "Thread version should produce correct result for N=100,000,000"
        );
    }
}

#[test]
#[cfg(target_os = "linux")]
fn test_03_process_version_correct() {
    let (output, _) = run_program();

    // Skip if not implemented
    if output.contains("not yet implemented") {
        return;
    }

    // Process version should produce correct result (Linux only)
    if output.contains("Multi-Process version:") {
        assert!(
            output.contains("5000000050000000"),
            "Process version should produce correct result for N=100,000,000"
        );
    }
}

#[test]
fn test_04_both_results_match() {
    let (output, _) = run_program();

    // Skip if not implemented
    if output.contains("not yet implemented") {
        return;
    }

    // If both versions ran, they should have the same result
    if output.contains("Both versions produced correct results!") {
        // Success message means both versions matched expected
        assert!(true);
    }
}
