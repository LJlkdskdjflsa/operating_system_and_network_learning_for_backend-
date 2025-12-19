//! Lab 4 Tests - Mini PS
//!
//! Run with: cargo test
//!
//! Note: These tests require a Linux environment

use std::process::Command;

/// Check if running in Linux environment
fn is_linux() -> bool {
    std::path::Path::new("/proc").exists()
}

/// Helper to run the program and get output
fn run_program() -> (String, String, bool) {
    let output = Command::new("cargo")
        .args(["run", "--quiet"])
        .output()
        .expect("Failed to execute program");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let success = output.status.success();

    (stdout, stderr, success)
}

// ============================================================
// Basic Tests
// ============================================================

#[test]
fn test_01_program_runs() {
    let (_, stderr, success) = run_program();

    // In non-Linux environment, should show error message but not panic
    if !is_linux() {
        assert!(
            stderr.contains("/proc") || stderr.contains("Linux"),
            "In non-Linux environment, should show message requiring Linux"
        );
        return;
    }

    assert!(success, "Program should execute successfully");
}

#[test]
#[cfg(target_os = "linux")]
fn test_02_shows_header() {
    let (stdout, _, _) = run_program();

    // Should have all required header columns
    assert!(stdout.contains("PID"), "Header should contain PID");
    assert!(stdout.contains("PPID"), "Header should contain PPID");
    assert!(stdout.contains("STATE"), "Header should contain STATE");
    assert!(stdout.contains("MEMORY"), "Header should contain MEMORY");
    assert!(stdout.contains("COMMAND"), "Header should contain COMMAND");
}

// ============================================================
// Process Listing Tests
// ============================================================

#[test]
#[cfg(target_os = "linux")]
fn test_03_lists_multiple_processes() {
    let (stdout, _, _) = run_program();
    let lines: Vec<&str> = stdout.lines().collect();

    // Skip header and separator, count actual process lines
    // A process line should start with a number (after trimming)
    let process_lines: Vec<&str> = lines
        .iter()
        .filter(|line| {
            let trimmed = line.trim();
            trimmed
                .split_whitespace()
                .next()
                .map(|first| first.parse::<u32>().is_ok())
                .unwrap_or(false)
        })
        .copied()
        .collect();

    assert!(
        process_lines.len() >= 5,
        "Should list at least 5 processes, but only found {}.\n\
         Make sure you're calling list_pids() and iterating over processes.\n\
         Output:\n{}",
        process_lines.len(),
        stdout
    );
}

#[test]
#[cfg(target_os = "linux")]
fn test_04_shows_pid_1() {
    let (stdout, _, _) = run_program();

    // PID 1 should exist (init/systemd)
    let has_pid_1 = stdout.lines().any(|line| {
        let trimmed = line.trim();
        // First column should be "1"
        trimmed.split_whitespace().next() == Some("1")
    });

    assert!(
        has_pid_1,
        "Should list PID 1 (init/systemd).\n\
         Make sure list_pids() returns PID 1 and you're printing it.\n\
         Output:\n{}",
        stdout
    );
}

#[test]
#[cfg(target_os = "linux")]
fn test_05_shows_current_process() {
    let (stdout, _, _) = run_program();

    // Our own process or cargo should appear
    // Look for "cargo" or "mini_ps" in the output
    let has_self = stdout.lines().any(|line| {
        line.contains("cargo") || line.contains("mini_ps") || line.contains("target")
    });

    assert!(
        has_self,
        "Should be able to see our own process (cargo/mini_ps) in the list.\n\
         Make sure get_cmdline() is working and being called.\n\
         Output:\n{}",
        stdout
    );
}

// ============================================================
// Output Format Tests
// ============================================================

#[test]
#[cfg(target_os = "linux")]
fn test_06_output_has_correct_columns() {
    let (stdout, _, _) = run_program();

    // Find a process line (not header, not separator)
    let process_line = stdout.lines().find(|line| {
        let trimmed = line.trim();
        trimmed
            .split_whitespace()
            .next()
            .map(|first| first.parse::<u32>().is_ok())
            .unwrap_or(false)
    });

    assert!(
        process_line.is_some(),
        "Should have at least one process line in output"
    );

    let line = process_line.unwrap();
    let columns: Vec<&str> = line.split_whitespace().collect();

    // Should have at least 5 columns: PID, PPID, STATE, MEMORY, COMMAND
    assert!(
        columns.len() >= 5,
        "Each process line should have at least 5 columns (PID, PPID, STATE, MEMORY, COMMAND).\n\
         Found {} columns in line: '{}'\n\
         Columns: {:?}",
        columns.len(),
        line,
        columns
    );

    // First column (PID) should be a number
    assert!(
        columns[0].parse::<u32>().is_ok(),
        "First column should be PID (a number), got: '{}'",
        columns[0]
    );

    // Second column (PPID) should be a number
    assert!(
        columns[1].parse::<u32>().is_ok(),
        "Second column should be PPID (a number), got: '{}'",
        columns[1]
    );

    // Third column (STATE) should be a single character like S, R, Z, D, T
    assert!(
        columns[2].len() <= 2 && columns[2].chars().all(|c| c.is_ascii_alphabetic()),
        "Third column should be STATE (like S, R, Z), got: '{}'",
        columns[2]
    );
}

#[test]
#[cfg(target_os = "linux")]
fn test_07_memory_column_format() {
    let (stdout, _, _) = run_program();

    // Find process lines
    let process_lines: Vec<&str> = stdout
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            trimmed
                .split_whitespace()
                .next()
                .map(|first| first.parse::<u32>().is_ok())
                .unwrap_or(false)
        })
        .collect();

    assert!(!process_lines.is_empty(), "Should have process lines");

    // Check that at least some processes have memory info
    // Memory column (4th) should be either "?" or a number with unit (like "4.5M", "123K")
    let valid_memory_count = process_lines
        .iter()
        .filter(|line| {
            let columns: Vec<&str> = line.split_whitespace().collect();
            if columns.len() < 4 {
                return false;
            }
            let mem = columns[3];
            // Valid formats: "?", "123K", "4.5M", "1.2G", or just numbers
            mem == "?"
                || mem.ends_with('K')
                || mem.ends_with('M')
                || mem.ends_with('G')
                || mem.ends_with('B')
                || mem.parse::<u64>().is_ok()
        })
        .count();

    assert!(
        valid_memory_count > 0,
        "Memory column should be '?' or have a unit (K, M, G).\n\
         Make sure get_status() returns memory info and you're formatting it.\n\
         Sample lines:\n{}",
        process_lines.iter().take(5).copied().collect::<Vec<_>>().join("\n")
    );
}

// ============================================================
// Helper Function Tests (via output validation)
// ============================================================

#[test]
#[cfg(target_os = "linux")]
fn test_08_ppid_is_valid() {
    let (stdout, _, _) = run_program();

    // PID 1 should have PPID 0
    let pid_1_line = stdout.lines().find(|line| {
        let trimmed = line.trim();
        trimmed.split_whitespace().next() == Some("1")
    });

    if let Some(line) = pid_1_line {
        let columns: Vec<&str> = line.split_whitespace().collect();
        if columns.len() >= 2 {
            assert_eq!(
                columns[1], "0",
                "PID 1 should have PPID 0, got: '{}'.\n\
                 Make sure get_status() correctly parses PPid from /proc/[pid]/status",
                columns[1]
            );
        }
    }
}

#[test]
#[cfg(target_os = "linux")]
fn test_09_no_todo_in_output() {
    let (stdout, _, _) = run_program();

    assert!(
        !stdout.to_lowercase().contains("todo"),
        "Output should not contain 'TODO' - remove the placeholder message.\n\
         Output:\n{}",
        stdout
    );
}

#[test]
#[cfg(target_os = "linux")]
fn test_10_handles_kernel_threads() {
    let (stdout, _, _) = run_program();

    // Kernel threads (like kthreadd, PID 2) often have empty cmdline
    // They should still appear with their name from /proc/[pid]/status
    let pid_2_line = stdout.lines().find(|line| {
        let trimmed = line.trim();
        trimmed.split_whitespace().next() == Some("2")
    });

    // In Docker containers, PID 2 (kthreadd) is not visible
    // Skip this test if we're in a container (no PID 2)
    if pid_2_line.is_none() {
        // Check if we're likely in a container by checking if PID 1 is bash/sh
        let pid_1_is_shell = stdout.lines().any(|line| {
            let trimmed = line.trim();
            trimmed.split_whitespace().next() == Some("1")
                && (line.contains("bash") || line.contains("sh"))
        });
        if pid_1_is_shell {
            // We're in a container, skip this test
            println!("Skipping kernel thread test - running in container");
            return;
        }
    }

    // On real Linux, PID 2 should exist (kthreadd)
    if let Some(line) = pid_2_line {
        let columns: Vec<&str> = line.split_whitespace().collect();
        assert!(
            columns.len() >= 5,
            "PID 2 line should have all columns including command name"
        );
        // Should show [kthreadd] since cmdline is empty for kernel threads
        assert!(
            line.contains("["),
            "Kernel thread should show [name] format when cmdline is empty"
        );
    }
}

// ============================================================
// Edge Case Tests
// ============================================================

#[test]
#[cfg(target_os = "linux")]
fn test_11_consistent_line_count() {
    // Run twice and check we get similar results (processes don't disappear due to bugs)
    let (stdout1, _, _) = run_program();
    let (stdout2, _, _) = run_program();

    let count1 = stdout1
        .lines()
        .filter(|l| {
            l.trim()
                .split_whitespace()
                .next()
                .map(|f| f.parse::<u32>().is_ok())
                .unwrap_or(false)
        })
        .count();
    let count2 = stdout2
        .lines()
        .filter(|l| {
            l.trim()
                .split_whitespace()
                .next()
                .map(|f| f.parse::<u32>().is_ok())
                .unwrap_or(false)
        })
        .count();

    // Allow some variance (processes start/stop) but should be similar
    let diff = (count1 as i32 - count2 as i32).abs();
    assert!(
        diff < 20,
        "Process count should be relatively stable between runs.\n\
         Run 1: {} processes, Run 2: {} processes.\n\
         Large variance might indicate error handling issues.",
        count1,
        count2
    );
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
// - Does memory usage look reasonable?
