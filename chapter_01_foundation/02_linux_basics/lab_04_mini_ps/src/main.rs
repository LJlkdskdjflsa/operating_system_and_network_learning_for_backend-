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

    // 1. List all PIDs
    let mut pids = list_pids();
    pids.sort(); // Sort by PID for consistent output

    // 2. For each PID, get info and print
    for pid in pids {
        // Get status info, skip if process disappeared
        let Some((name, state, ppid, memory_kb)) = get_status(pid) else {
            continue;
        };

        // Get command line, fallback to name (for kernel threads)
        let cmdline = get_cmdline(pid).unwrap_or_else(|| format!("[{}]", name));

        // Format memory: None -> "?", Some(kb) -> human readable
        let memory = match memory_kb {
            Some(kb) if kb >= 1024 => format!("{:.1}M", kb as f64 / 1024.0),
            Some(kb) => format!("{}K", kb),
            None => "?".to_string(),
        };

        println!(
            "{:>7} {:>7} {:>5} {:>8}   {}",
            pid, ppid, state, memory, cmdline
        );
    }
}

// TODO: Implement these helper functions

/// List all PIDs
fn list_pids() -> Vec<u32> {
    let mut pids = Vec::new();

    for entry in fs::read_dir("/proc").unwrap() {
        if let Ok(entry) = entry {
            if let Some(name_str) = entry.file_name().to_str() {
                if let Ok(pid) = name_str.parse::<u32>() {
                    pids.push(pid);
                }
            }
        }
    }

    pids
}

/// Read process command line
fn get_cmdline(pid: u32) -> Option<String> {
    // Read /proc/[pid]/cmdline
    let path = format!("/proc/{}/cmdline", pid);
    let content = fs::read_to_string(&path).ok()?;

    // If empty (kernel thread), return None
    if content.is_empty() {
        return None;
    }

    // Replace \0 with spaces and trim
    let cmdline = content.replace('\0', " ").trim().to_string();
    Some(cmdline)
}

/// Read process status
/// Returns (name, state, ppid, memory_kb)
fn get_status(pid: u32) -> Option<(String, String, u32, Option<u64>)> {
    // Read /proc/[pid]/status
    let path = format!("/proc/{}/status", pid);
    let content = fs::read_to_string(&path).ok()?;

    let mut name = String::new();
    let mut state = String::new();
    let mut ppid: u32 = 0;
    let mut memory_kb: Option<u64> = None;

    // Parse each line (format: "Key:\tValue")
    for line in content.lines() {
        // split_once splits "Name:  bash" into ("Name", "  bash")
        if let Some((key, value)) = line.split_once(':') {
            let value = value.trim();

            match key {
                "Name" => name = value.to_string(),
                "State" => {
                    // "S (sleeping)" -> just take "S"
                    // R (Running)
                    state = value.chars().next().unwrap_or('?').to_string();
                }
                "PPid" => {
                    ppid = value.parse().unwrap_or(0);
                }
                "VmRSS" => {
                    // "3256 kB" -> extract 3256
                    if let Some(num_str) = value.split_whitespace().next() {
                        memory_kb = num_str.parse().ok();
                    }
                }
                _ => {} // ignore other fields
            }
        }
    }

    // Return None if we couldn't get basic info
    if name.is_empty() {
        return None;
    }

    Some((name, state, ppid, memory_kb))
}
