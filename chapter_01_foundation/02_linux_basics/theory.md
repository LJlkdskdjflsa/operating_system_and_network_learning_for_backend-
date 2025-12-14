# 1.2 Linux 環境熟悉：用系統視角看你的程式

## 本節目標

> 學會用 Linux 工具觀察程式的系統行為，建立「程式碼 ↔ OS」的思維連結

完成本節後，你將能夠：
- 使用基本指令查看系統狀態
- 用 `strace` 追蹤程式的 system call
- 理解 `/proc` 虛擬檔案系統
- 從「系統層面」理解程式在做什麼

---

## 1. 為什麼後端工程師需要懂 Linux？

你寫的後端服務最終會跑在 Linux 上。當出問題時：

```
"服務怎麼變慢了？" → 用 top/htop 看 CPU、記憶體
"連線怎麼建不起來？" → 用 ss/netstat 看 socket 狀態
"檔案讀取怎麼這麼慢？" → 用 strace 看是哪個 syscall 卡住
"為什麼會 OOM？" → 用 /proc/[pid]/status 看記憶體用量
```

**理解 Linux = 能診斷問題 = 值錢的能力**

---

## 2. 基本指令速覽

### 程序相關

| 指令 | 用途 | 常用參數 |
|------|------|----------|
| `ps` | 查看程序 | `ps aux` 看所有程序 |
| `top` | 即時監控 | 按 `1` 看各 CPU，`M` 按記憶體排序 |
| `htop` | 更好的 top | 按 `F5` 看樹狀結構 |
| `kill` | 送訊號 | `kill -9 PID` 強制終止 |

### 檔案相關

| 指令 | 用途 | 例子 |
|------|------|------|
| `ls` | 列出檔案 | `ls -la` 看詳細資訊 |
| `cat` | 顯示檔案 | `cat file.txt` |
| `less` | 分頁查看 | `less big_file.log` |
| `grep` | 搜尋文字 | `grep "error" log.txt` |
| `find` | 搜尋檔案 | `find . -name "*.rs"` |

### 網路相關（下一章會詳細講）

| 指令 | 用途 | 例子 |
|------|------|------|
| `ss` | 查看 socket | `ss -tulpn` |
| `netstat` | 舊版 ss | `netstat -an` |
| `curl` | HTTP 請求 | `curl http://localhost:8080` |

---

## 3. 理解 Process 和 Thread

### 用 ps 查看程序

```bash
# 查看所有程序
$ ps aux
USER    PID  %CPU %MEM    VSZ   RSS TTY   STAT START   TIME COMMAND
root      1   0.0  0.1 168000 12000 ?     Ss   10:00   0:01 /sbin/init
user   1234   2.0  0.5 500000 50000 pts/0 Sl+  10:05   0:10 ./my_server

# 各欄位意義：
# PID   - Process ID
# %CPU  - CPU 使用率
# %MEM  - 記憶體使用率
# VSZ   - 虛擬記憶體大小（KB）
# RSS   - 實際使用的實體記憶體（KB）
# STAT  - 狀態（S=睡眠, R=運行, Z=僵屍）
```

### 用 htop 即時監控

```
  CPU[||||||||                    25.0%]   Tasks: 89, 320 thr
  Mem[||||||||||||||||||      2.0G/8.0G]   Load average: 1.20 0.80 0.60
  Swp[                          0K/2.0G]   Uptime: 5 days, 03:24:12

    PID USER      PRI  NI  VIRT   RES   SHR S CPU%  MEM%   TIME+  Command
   1234 user       20   0  500M   50M   10M S  2.0   0.6   0:10.5 ./my_server
   1235 user       20   0  500M   10M    5M S  0.5   0.1   0:02.1 └─ worker_1
   1236 user       20   0  500M   10M    5M S  0.5   0.1   0:02.0 └─ worker_2
```

### 關鍵概念

```
┌─────────────────────────────────────────────────────────────┐
│                        Process                               │
│  - 獨立的記憶體空間                                           │
│  - 獨立的 file descriptor table                              │
│  - 有自己的 PID                                              │
│                                                              │
│    ┌─────────┐  ┌─────────┐  ┌─────────┐                    │
│    │ Thread 1│  │ Thread 2│  │ Thread 3│                    │
│    │ (main)  │  │(worker) │  │(worker) │                    │
│    └─────────┘  └─────────┘  └─────────┘                    │
│         │            │            │                          │
│         └────────────┼────────────┘                          │
│                      │                                       │
│              共享同一份記憶體                                  │
│              共享同一份 fd table                              │
└─────────────────────────────────────────────────────────────┘
```

---

## 4. System Call（系統呼叫）

### 什麼是 System Call？

你的程式不能直接操作硬體。要做任何「真實」的事情（讀檔案、開網路連線、分配記憶體），都必須請求 OS 幫忙：

```
┌─────────────────┐
│   你的程式       │  User Space
│   （Rust 程式）  │
└────────┬────────┘
         │ System Call
         ▼
┌─────────────────┐
│   Linux Kernel  │  Kernel Space
│   （作業系統）   │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│   硬體          │
│ (CPU, Disk, NIC)│
└─────────────────┘
```

### 常見的 System Call

| System Call | 用途 | Rust 對應 |
|-------------|------|-----------|
| `open` | 打開檔案 | `File::open()` |
| `read` | 讀取資料 | `file.read()` |
| `write` | 寫入資料 | `file.write()` / `println!()` |
| `close` | 關閉檔案 | 自動（Drop） |
| `socket` | 建立 socket | `TcpListener::bind()` |
| `accept` | 接受連線 | `listener.accept()` |
| `mmap` | 記憶體映射 | `Vec` 的記憶體分配 |
| `nanosleep` | 睡眠 | `thread::sleep()` |

### 用 strace 觀察

```bash
# 追蹤程式的所有 system call
$ strace ./my_program

# 輸出範例：
execve("./my_program", ...) = 0
mmap(NULL, 4096, ...) = 0x7f...          # 分配記憶體
open("test.txt", O_RDONLY) = 3           # 打開檔案，回傳 fd=3
read(3, "Hello World\n", 4096) = 12      # 讀取 12 bytes
write(1, "Hello World\n", 12) = 12       # 寫到 stdout (fd=1)
close(3) = 0                             # 關閉檔案
exit_group(0) = ?                        # 程式結束
```

---

## 5. File Descriptor（檔案描述子）

### 什麼是 fd？

fd 是一個整數，代表一個打開的「資源」：
- 檔案
- Socket 連線
- Pipe
- 甚至 `/dev/null`

### 預設的 fd

| fd | 名稱 | 用途 |
|----|------|------|
| 0 | stdin | 標準輸入 |
| 1 | stdout | 標準輸出 |
| 2 | stderr | 標準錯誤 |

```rust
// println! 最終會變成 write(1, ...)
println!("Hello");  // → write(1, "Hello\n", 6)

// eprintln! 會寫到 stderr
eprintln!("Error"); // → write(2, "Error\n", 6)
```

### 用 lsof 查看程式打開的 fd

```bash
# 查看 PID 1234 打開的所有 fd
$ lsof -p 1234

COMMAND  PID USER   FD   TYPE DEVICE SIZE/OFF NODE NAME
my_prog 1234 user  cwd    DIR    8,1     4096  123 /home/user
my_prog 1234 user  txt    REG    8,1   100000  456 /home/user/my_prog
my_prog 1234 user    0u   CHR  136,0      0t0    3 /dev/pts/0
my_prog 1234 user    1u   CHR  136,0      0t0    3 /dev/pts/0
my_prog 1234 user    2u   CHR  136,0      0t0    3 /dev/pts/0
my_prog 1234 user    3r   REG    8,1    12345  789 /home/user/data.txt
my_prog 1234 user    4u  IPv4  12345      0t0  TCP *:8080 (LISTEN)
```

---

## 6. /proc 虛擬檔案系統

### 什麼是 /proc？

Linux 把 kernel 的資訊以「檔案」的形式暴露出來。不是真的檔案，是虛擬的介面。

### 常用的 /proc 路徑

```bash
# 系統資訊
/proc/cpuinfo      # CPU 資訊
/proc/meminfo      # 記憶體資訊
/proc/loadavg      # 系統負載

# 特定程序的資訊（把 [pid] 換成實際的 PID）
/proc/[pid]/status    # 程序狀態（名稱、記憶體用量等）
/proc/[pid]/cmdline   # 啟動時的命令列
/proc/[pid]/fd/       # 打開的 file descriptors
/proc/[pid]/maps      # 記憶體映射
```

### 實際查看

```bash
# 查看目前 shell 的 PID
$ echo $$
1234

# 查看這個程序的狀態
$ cat /proc/1234/status
Name:   bash
State:  S (sleeping)
Pid:    1234
PPid:   1000
Threads: 1
VmSize: 12000 kB
VmRSS:  4000 kB
...

# 查看這個程序打開的 fd
$ ls -la /proc/1234/fd/
lrwx------ 1 user user 64 Jan  1 10:00 0 -> /dev/pts/0
lrwx------ 1 user user 64 Jan  1 10:00 1 -> /dev/pts/0
lrwx------ 1 user user 64 Jan  1 10:00 2 -> /dev/pts/0
```

---

## 7. 實用組合技

### 找出吃最多 CPU 的程序

```bash
ps aux --sort=-%cpu | head -10
```

### 找出吃最多記憶體的程序

```bash
ps aux --sort=-%mem | head -10
```

### 追蹤特定程式的 syscall（只看 open/read/write）

```bash
strace -e trace=open,read,write ./my_program
```

### 追蹤正在運行的程式

```bash
# 附加到 PID 1234
strace -p 1234
```

### 查看程式的網路連線

```bash
# 需要知道 PID
ss -tulpn | grep 1234
```

---

## 小結：建立系統視角

```
┌────────────────────────────────────────────────────────────────┐
│                     你的思維模式                                │
├────────────────────────────────────────────────────────────────┤
│                                                                │
│   程式碼層面                    系統層面                        │
│   ─────────                    ────────                        │
│   File::open("x.txt")    ──→   open("x.txt") → fd=3           │
│   file.read(&mut buf)    ──→   read(3, buf, size) → N bytes   │
│   println!("Hello")      ──→   write(1, "Hello", 5)           │
│   drop(file)             ──→   close(3)                       │
│                                                                │
│   thread::spawn(...)     ──→   clone() → 新的 thread          │
│   TcpListener::bind()    ──→   socket() + bind() + listen()   │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

當你能同時看到這兩個層面，debug 和效能調優就會變得容易很多。

---

## 接下來

完成理論閱讀後，請進入實作：
1. **Lab 3**: 用 strace 觀察你的 Rust 程式
2. **Lab 4**: 實作一個讀取 /proc 的 mini ps
