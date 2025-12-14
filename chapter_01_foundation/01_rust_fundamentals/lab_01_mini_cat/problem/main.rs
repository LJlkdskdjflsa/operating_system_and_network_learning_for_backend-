//! Lab 1: Mini Cat/Grep
//!
//! ## Goal
//! Implement a command-line tool that reads files and filters lines containing specific keywords
//!
//! ## Requirements
//! 1. `cargo run -- test.txt`           → Display entire file contents
//! 2. `cargo run -- test.txt error`     → Only show lines containing "error"
//! 3. `cargo run -- test.txt error -n`  → Add line numbers
//!
//! ## Hints
//! - Use `std::env::args()` to get command-line arguments
//! - Use `std::fs::File` and `std::io::BufReader` to read files
//! - Use `anyhow` crate for error handling (already in Cargo.toml)
//!
//! ## Verification
//! ```bash
//! cargo test          # Run automated tests
//! cargo run -- test.txt
//! cargo run -- test.txt error
//! cargo run -- test.txt error -n
//! ```
//!
//! ## Acceptance Criteria
//! - [ ] `cargo test` all pass
//! - [ ] Can correctly read and display file contents
//! - [ ] Can filter lines by keyword
//! - [ ] Can display line numbers
//! - [ ] Shows friendly error message when file doesn't exist
//!
//! Check solution/main.rs after completing

use anyhow::Result;

fn main() -> Result<()> {
    // TODO: Implement your mini cat/grep
    //
    // Suggested steps:
    // 1. Parse command-line arguments
    // 2. Open file
    // 3. Read line by line
    // 4. Filter and output based on conditions

    println!("Please implement mini cat/grep!");

    Ok(())
}
