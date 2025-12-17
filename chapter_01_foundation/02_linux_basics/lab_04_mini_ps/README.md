# Lab 4: 實作 Mini PS — 讀取 /proc 虛擬檔案系統

## 實作目標

> 寫一個能列出系統程序的工具，透過讀取 Linux 的 /proc 目錄

完成後你將學會：
- 理解 `/proc` 虛擬檔案系統的結構
- 用 Rust 讀取目錄和解析文字
- 了解 Linux 如何暴露程序資訊

---

## ⚠️ 平台注意

這個 Lab 需要在 **Linux** 環境執行（原生 Linux、WSL2、或 Docker）。

macOS 沒有 `/proc` 檔案系統（macOS 用的是 `sysctl` 和其他介面）。

### macOS / Windows：用 Docker 跑 Linux（推薦）

在 repo 根目錄執行：

```bash
docker run --rm -it -v "$PWD":/work -w /work rust:bookworm bash
```

在容器內執行：

```bash
cd chapter_01_foundation/02_linux_basics/lab_04_mini_ps
cargo test
cargo run --quiet
```

---

## /proc 檔案系統介紹

### 結構概覽

```
/proc/
├── 1/                    # PID 1 的資訊（通常是 init/systemd）
│   ├── cmdline          # 啟動命令
│   ├── status           # 詳細狀態
│   ├── stat             # 統計資訊（一行）
│   ├── fd/              # 打開的 file descriptors
│   └── ...
├── 1234/                 # PID 1234 的資訊
│   └── ...
├── self/                 # 指向當前程序的符號連結
├── cpuinfo              # CPU 資訊
├── meminfo              # 記憶體資訊
└── loadavg              # 系統負載
```

### 關鍵檔案

**`/proc/[pid]/cmdline`**
- 程序啟動時的命令列參數
- 以 null byte (`\0`) 分隔
- 例如：`/usr/bin/python3\0script.py\0--verbose\0`

**`/proc/[pid]/status`**
- 人類可讀的程序狀態
- 包含：Name, State, Pid, PPid, VmSize, VmRSS 等

**`/proc/[pid]/stat`**
- 一行數字，用空格分隔
- 格式：`pid (comm) state ppid pgrp session ...`
- 用於程式化讀取（但 cmdline 有括號時要小心）

---

## 階段一：列出所有 PID

### 任務
讀取 `/proc` 目錄，找出所有代表程序的子目錄（數字命名的目錄）。

### 提示

```rust
use std::fs;

fn list_pids() -> Vec<u32> {
    let mut pids = Vec::new();

    if let Ok(entries) = fs::read_dir("/proc") {
        for entry in entries.flatten() {
            // 取得目錄名稱
            if let Some(name) = entry.file_name().to_str() {
                // 嘗試解析為數字（只有 PID 目錄是純數字）
                if let Ok(pid) = name.parse::<u32>() {
                    pids.push(pid);
                }
            }
        }
    }

    pids.sort();
    pids
}
```

---

## 階段二：讀取程序命令列

### 任務
讀取每個 PID 的 `/proc/[pid]/cmdline`。

### 提示

```rust
use std::fs;

fn get_cmdline(pid: u32) -> Option<String> {
    let path = format!("/proc/{}/cmdline", pid);

    match fs::read(&path) {
        Ok(content) => {
            // cmdline 用 \0 分隔參數，把它們換成空格
            let cmdline: String = content
                .iter()
                .map(|&b| if b == 0 { ' ' } else { b as char })
                .collect();

            let cmdline = cmdline.trim().to_string();

            if cmdline.is_empty() {
                None  // kernel thread 沒有 cmdline
            } else {
                Some(cmdline)
            }
        }
        Err(_) => None,  // 程序可能已經結束
    }
}
```

---

## 階段三：讀取程序狀態

### 任務
從 `/proc/[pid]/status` 讀取更多資訊：Name, State, PPid, VmRSS 等。

### /proc/[pid]/status 範例

```
Name:   bash
State:  S (sleeping)
Tgid:   1234
Pid:    1234
PPid:   1000
TracerPid:      0
Uid:    1000    1000    1000    1000
Gid:    1000    1000    1000    1000
VmSize:    12345 kB
VmRSS:      4567 kB
Threads:        1
...
```

### 提示

```rust
use std::fs;
use std::collections::HashMap;

fn parse_status(pid: u32) -> Option<HashMap<String, String>> {
    let path = format!("/proc/{}/status", pid);
    let content = fs::read_to_string(&path).ok()?;

    let mut info = HashMap::new();

    for line in content.lines() {
        // 每行格式：Key:\tValue
        if let Some((key, value)) = line.split_once(':') {
            info.insert(
                key.trim().to_string(),
                value.trim().to_string()
            );
        }
    }

    Some(info)
}
```

---

## 階段四：完整的 Mini PS

### 預期輸出

```
PID    PPID   STATE  MEMORY     COMMAND
  1       0   S          ?      /sbin/init splash
123       1   S       4.5M      /usr/lib/systemd/...
456     123   S       2.1M      bash
789     456   R       1.2M      ./mini_ps
```

### 整合所有資訊

```rust
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
            state: status.get("State")?
                .chars()
                .next()?
                .to_string(),
            memory_kb: status.get("VmRSS")
                .and_then(|s| s.split_whitespace().next())
                .and_then(|s| s.parse().ok()),
            cmdline: get_cmdline(pid),
        })
    }
}
```

---

## 進階挑戰（選做）

### 1. 加上排序選項

```bash
mini_ps --sort=memory  # 按記憶體排序
mini_ps --sort=pid     # 按 PID 排序
```

### 2. 顯示程序樹

```
├─ systemd (1)
│  ├─ sshd (123)
│  │  └─ bash (456)
│  │     └─ mini_ps (789)
│  └─ nginx (234)
```

### 3. 即時更新（類似 top）

使用 terminal 控制碼清除畫面，每秒更新一次。

### 4. 顯示 CPU 使用率

需要讀取 `/proc/[pid]/stat` 並計算：
- 讀取兩次，間隔一段時間
- 計算 CPU time 的差值

---

## 常用 /proc 檔案

| 路徑 | 內容 |
|------|------|
| `/proc/[pid]/cmdline` | 命令列參數 |
| `/proc/[pid]/status` | 詳細狀態 |
| `/proc/[pid]/stat` | 一行統計（適合程式解析） |
| `/proc/[pid]/fd/` | 打開的 file descriptors |
| `/proc/[pid]/maps` | 記憶體映射 |
| `/proc/[pid]/cwd` | 工作目錄（symlink） |
| `/proc/[pid]/exe` | 執行檔路徑（symlink） |
| `/proc/cpuinfo` | CPU 資訊 |
| `/proc/meminfo` | 記憶體資訊 |
| `/proc/loadavg` | 系統負載 |

---

## 驗收標準

- [ ] 能列出所有 PID
- [ ] 能顯示每個程序的命令列
- [ ] 能顯示程序狀態（Name, State, PPid）
- [ ] 能顯示記憶體用量
- [ ] 能處理程序在執行途中消失的情況

---

## 延伸閱讀

- `man proc` - /proc 檔案系統的完整文件
- [procfs crate](https://docs.rs/procfs/latest/procfs/) - Rust 的 /proc 讀取函式庫
