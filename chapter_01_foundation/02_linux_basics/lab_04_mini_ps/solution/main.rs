//! Lab 4 Reference Answer

use std::collections::HashMap;
use std::fs;

/// Process information structure
struct ProcessInfo {
    pid: u32,
    ppid: u32,
    name: String,
    state: String,
    memory_kb: Option<u64>,
    cmdline: Option<String>,
}

impl ProcessInfo {
    fn from_pid(pid: u32) -> Option<Self> {
        let status = parse_status(pid)?;

        Some(ProcessInfo {
            pid,
            ppid: status.get("PPid")?.parse().ok()?,
            name: status.get("Name")?.clone(),
            state: status.get("State")?.chars().next()?.to_string(),
            memory_kb: status
                .get("VmRSS")
                .and_then(|s| s.split_whitespace().next())
                .and_then(|s| s.parse().ok()),
            cmdline: get_cmdline(pid),
        })
    }

    fn format_memory(&self) -> String {
        match self.memory_kb {
            Some(kb) if kb >= 1024 * 1024 => format!("{:.1}G", kb as f64 / 1024.0 / 1024.0),
            Some(kb) if kb >= 1024 => format!("{:.1}M", kb as f64 / 1024.0),
            Some(kb) => format!("{}K", kb),
            None => "-".to_string(),
        }
    }
}

fn list_pids() -> Vec<u32> {
    let mut pids = Vec::new();

    if let Ok(entries) = fs::read_dir("/proc") {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if let Ok(pid) = name.parse::<u32>() {
                    pids.push(pid);
                }
            }
        }
    }

    pids.sort();
    pids
}

fn get_cmdline(pid: u32) -> Option<String> {
    let path = format!("/proc/{}/cmdline", pid);

    match fs::read(&path) {
        Ok(content) => {
            let cmdline: String = content
                .iter()
                .map(|&b| if b == 0 { ' ' } else { b as char })
                .collect();

            let cmdline = cmdline.trim().to_string();

            if cmdline.is_empty() {
                None
            } else {
                Some(cmdline)
            }
        }
        Err(_) => None,
    }
}

fn parse_status(pid: u32) -> Option<HashMap<String, String>> {
    let path = format!("/proc/{}/status", pid);
    let content = fs::read_to_string(&path).ok()?;

    let mut info = HashMap::new();

    for line in content.lines() {
        if let Some((key, value)) = line.split_once(':') {
            info.insert(key.trim().to_string(), value.trim().to_string());
        }
    }

    Some(info)
}

fn main() {
    if !std::path::Path::new("/proc").exists() {
        eprintln!("Error: /proc directory not found");
        eprintln!("This program requires a Linux environment");
        std::process::exit(1);
    }

    println!(
        "{:>7} {:>7} {:>5} {:>8}   {}",
        "PID", "PPID", "STATE", "MEMORY", "COMMAND"
    );
    println!("{}", "-".repeat(70));

    let pids = list_pids();

    for pid in pids {
        if let Some(info) = ProcessInfo::from_pid(pid) {
            let display_cmd = info
                .cmdline
                .as_ref()
                .map(|c| {
                    if c.len() > 50 {
                        format!("{}...", &c[..47])
                    } else {
                        c.clone()
                    }
                })
                .unwrap_or_else(|| format!("[{}]", info.name));

            println!(
                "{:>7} {:>7} {:>5} {:>8}   {}",
                info.pid,
                info.ppid,
                info.state,
                info.format_memory(),
                display_cmd
            );
        }
    }
}
