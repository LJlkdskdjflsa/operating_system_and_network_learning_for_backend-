//! Lab 2 Tests
//!
//! Run with: cargo test
//!
//! Note: This test calls functions defined in main.rs directly
//! So you need to make the functions pub, or put the tests inside main.rs

// Since functions are in main.rs, we test by running the program
use std::process::Command;

/// Run the program and check output
fn run_parallel_sum() -> String {
    let output = Command::new("cargo")
        .args(["run", "--release", "--quiet"])
        .output()
        .expect("Failed to execute program");

    String::from_utf8_lossy(&output.stdout).to_string()
}

#[test]
fn test_01_program_runs() {
    // Test: program runs without panicking
    let output = run_parallel_sum();
    assert!(
        !output.is_empty() || output.contains("todo"),
        "Program should have output"
    );
}

#[test]
fn test_02_correct_expected_value() {
    // Test: displayed Expected value is correct
    let output = run_parallel_sum();

    // Answer for N = 100,000,000 is 5000000050000000
    if output.contains("Expected") {
        assert!(
            output.contains("5000000050000000"),
            "Expected value should be 5000000050000000"
        );
    }
}

#[test]
fn test_03_results_match_expected() {
    // Test: all versions produce correct results
    let output = run_parallel_sum();

    // Skip this test if not yet implemented
    if output.contains("not yet implemented") || output.contains("todo!()") {
        return;
    }

    // Check if all Results are the correct answer
    let expected = "5000000050000000";
    let result_lines: Vec<&str> = output
        .lines()
        .filter(|line| line.contains("Result:"))
        .collect();

    for line in result_lines {
        assert!(
            line.contains(expected),
            "Result should be {}, but got: {}",
            expected,
            line
        );
    }
}

// ============================================================
// If you want more precise unit tests, add the following code
// to the end of main.rs
// ============================================================
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     const TEST_N: u64 = 1000;
//     const EXPECTED: u64 = 500_500;  // 1000 * 1001 / 2
//
//     #[test]
//     fn test_sequential() {
//         assert_eq!(sum_sequential(TEST_N), EXPECTED);
//     }
//
//     #[test]
//     fn test_mutex() {
//         assert_eq!(sum_with_mutex(TEST_N, 4), EXPECTED);
//     }
//
//     #[test]
//     fn test_channel() {
//         assert_eq!(sum_with_channel(TEST_N, 4), EXPECTED);
//     }
// }
