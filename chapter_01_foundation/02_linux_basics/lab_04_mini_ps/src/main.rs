//! Mini PS - 一個讀取 /proc 的程序列表工具
//!
//! 這個程式需要在 Linux 環境執行

use std::collections::HashMap;
use std::fs;

/// 程序資訊結構
struct ProcessInfo {
    pid: u32,
    ppid: u32,
    name: String,
    state: String,
    memory_kb: Option<u64>,
    cmdline: Option<String>,
}

impl ProcessInfo {
    /// 從 PID 讀取程序資訊
    fn from_pid(pid: u32) -> Option<Self> {
        let status = parse_status(pid)?;

        Some(ProcessInfo {
            pid,
            ppid: status.get("PPid")?.parse().ok()?,
            name: status.get("Name")?.clone(),
            state: status
                .get("State")?
                .chars()
                .next()?
                .to_string(),
            memory_kb: status
                .get("VmRSS")
                .and_then(|s| s.split_whitespace().next())
                .and_then(|s| s.parse().ok()),
            cmdline: get_cmdline(pid),
        })
    }

    /// 格式化記憶體大小
    fn format_memory(&self) -> String {
        match self.memory_kb {
            Some(kb) if kb >= 1024 * 1024 => format!("{:.1}G", kb as f64 / 1024.0 / 1024.0),
            Some(kb) if kb >= 1024 => format!("{:.1}M", kb as f64 / 1024.0),
            Some(kb) => format!("{}K", kb),
            None => "-".to_string(),
        }
    }
}

/// 列出所有 PID
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

/// 讀取程序的命令列
fn get_cmdline(pid: u32) -> Option<String> {
    let path = format!("/proc/{}/cmdline", pid);

    match fs::read(&path) {
        Ok(content) => {
            // cmdline 用 \0 分隔參數
            let cmdline: String = content
                .iter()
                .map(|&b| if b == 0 { ' ' } else { b as char })
                .collect();

            let cmdline = cmdline.trim().to_string();

            if cmdline.is_empty() {
                None // kernel thread 沒有 cmdline
            } else {
                Some(cmdline)
            }
        }
        Err(_) => None,
    }
}

/// 解析 /proc/[pid]/status 檔案
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
    // 檢查是否在 Linux 環境
    if !std::path::Path::new("/proc").exists() {
        eprintln!("錯誤：找不到 /proc 目錄");
        eprintln!("這個程式需要在 Linux 環境執行（原生 Linux、WSL2、或 Docker）");
        std::process::exit(1);
    }

    // 印出標題
    println!(
        "{:>7} {:>7} {:>5} {:>8}   {}",
        "PID", "PPID", "STATE", "MEMORY", "COMMAND"
    );
    println!("{}", "-".repeat(70));

    // 取得所有 PID 並讀取資訊
    let pids = list_pids();

    for pid in pids {
        // 程序可能在讀取過程中消失，所以用 if let
        if let Some(info) = ProcessInfo::from_pid(pid) {
            // 顯示命令列或名稱
            let display_cmd = info
                .cmdline
                .as_ref()
                .map(|c| {
                    // 如果命令太長，截斷
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

// ============================================================
// 測試（只在 Linux 上有意義）
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(target_os = "linux")]
    fn test_list_pids_contains_self() {
        let pids = list_pids();
        let self_pid = std::process::id();
        assert!(pids.contains(&self_pid));
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_get_self_cmdline() {
        let self_pid = std::process::id();
        let cmdline = get_cmdline(self_pid);
        assert!(cmdline.is_some());
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_parse_self_status() {
        let self_pid = std::process::id();
        let status = parse_status(self_pid);
        assert!(status.is_some());

        let status = status.unwrap();
        assert!(status.contains_key("Name"));
        assert!(status.contains_key("Pid"));
        assert!(status.contains_key("PPid"));
    }

    #[test]
    fn test_format_memory() {
        let info = ProcessInfo {
            pid: 1,
            ppid: 0,
            name: "test".to_string(),
            state: "S".to_string(),
            memory_kb: Some(1024),
            cmdline: None,
        };
        assert_eq!(info.format_memory(), "1.0M");

        let info2 = ProcessInfo {
            memory_kb: Some(512),
            ..info
        };
        assert_eq!(info2.format_memory(), "512K");
    }
}
