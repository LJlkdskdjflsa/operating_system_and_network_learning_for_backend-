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

use anyhow::{Context, Result};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    // println!("Input Parameter:");
    // println!("==========================");
    // println!("param 0: {:?}", args.get(0));
    // println!("param 1: {:?}", args.get(1));
    // println!("param 2: {:?}", args.get(2));
    // println!("param 3: {:?}", args.get(3));
    // println!("==========================");
    let program_name = args.get(0).map(String::as_str).unwrap_or("mini_cat");
    let file_name = match args.get(1) {
        Some(name) => name,
        None => {
            eprintln!("usage: {program_name} <file> [keyword] [-n]");
            std::process::exit(1);
        }
    };
    let mut keyword: Option<&str> = None;
    let mut show_line_numbers = false;

    for arg in args.iter().skip(2) {
        match arg.as_str() {
            "-n" | "--line-numbers" => show_line_numbers = true,
            other if keyword.is_none() => keyword = Some(other),
            _ => {}
        }
    }

    let file = File::open(file_name)
        .with_context(|| format!("Failed to open file: {file_name}"))?;
    let reader = BufReader::new(file);
    // lines() returns an Iterator<Item = io::Result<String>>
    for (line_number, line_result) in reader.lines().enumerate() {
        let line = line_result
            .with_context(|| format!("Error reading line {}", line_number + 1))?;

        if let Some(keyword_internal) = keyword {
            // When a keyword is provided, only print matching lines (case-insensitive)

            if line
                .to_lowercase()
                .contains(&keyword_internal.to_lowercase())
            {
                if show_line_numbers {
                    println!("{:>3}: {}", line_number + 1, line);
                } else {
                    println!("{}", line);
                }
            }
        } else {
            // With no keyword, print every line
            if show_line_numbers {
                println!("{:>3}: {}", line_number + 1, line);
            } else {
                println!("{}", line);
            }
        }
    }

    Ok(())
}
