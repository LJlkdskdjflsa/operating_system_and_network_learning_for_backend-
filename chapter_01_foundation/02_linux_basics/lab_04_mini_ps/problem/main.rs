//! Lab 4: Mini PS
//!
//! ## Goal
//! Implement a tool that lists system processes by reading Linux's /proc directory
//!
//! ## Requirements
//! Display the following information:
//! ```
//! PID    PPID   STATE  MEMORY     COMMAND
//!   1       0   S          ?      /sbin/init
//! 123       1   S       4.5M      /usr/lib/systemd/...
//! ```
//!
//! ## /proc File Structure
//! - `/proc/[pid]/cmdline` - Command line arguments (separated by \0)
//! - `/proc/[pid]/status` - Detailed status (Name, State, PPid, VmRSS, etc.)
//!
//! ## Hints
//! - Use `std::fs::read_dir("/proc")` to list the directory
//! - PID directories have purely numeric names
//! - Processes may disappear during reading, handle errors gracefully
//!
//! ## Verification
//! ```bash
//! cargo test          # Run automated tests (requires Linux)
//! cargo run           # Run the program
//! ps aux | head -20   # Compare with system ps output
//! ```
//!
//! ## Acceptance Criteria
//! - [ ] `cargo test` all pass
//! - [ ] Can list all PIDs
//! - [ ] Can display each process's command line
//! - [ ] Can display process status (Name, State, PPid)
//! - [ ] Can display memory usage
//!
//! Warning: This lab requires a Linux environment (WSL2, Docker, or native Linux)
//!
//! Check solution/main.rs after completing

use std::fs;

fn main() {
    // Check if running in Linux environment
    if !std::path::Path::new("/proc").exists() {
        eprintln!("Error: /proc directory not found");
        eprintln!("This program requires a Linux environment");
        std::process::exit(1);
    }

    println!(
        "{:>7} {:>7} {:>5} {:>8}   {}",
        "PID", "PPID", "STATE", "MEMORY", "COMMAND"
    );
    println!("{}", "-".repeat(60));

    // TODO: Implement the following steps
    //
    // 1. List all PIDs
    //    - Read the /proc directory
    //    - Find all subdirectory names that are pure numbers
    //
    // 2. For each PID, read the following information:
    //    - /proc/[pid]/cmdline -> command line
    //    - /proc/[pid]/status -> Name, State, PPid, VmRSS
    //
    // 3. Format the output

    println!("TODO: Implement mini ps!");
}

// TODO: Implement these helper functions

/// List all PIDs
fn list_pids() -> Vec<u32> {
    // Hint:
    // fs::read_dir("/proc")
    // Filter out directory names that can be parsed as u32
    todo!()
}

/// Read process command line
fn get_cmdline(pid: u32) -> Option<String> {
    // Hint:
    // Read /proc/[pid]/cmdline
    // Replace \0 with spaces
    todo!()
}

/// Read process status
fn get_status(pid: u32) -> Option<(String, String, u32, Option<u64>)> {
    // Returns (name, state, ppid, memory_kb)
    // Hint:
    // Read /proc/[pid]/status
    // Parse each line's Key: Value format
    todo!()
}
