# Lab 3: 用 strace 觀察 Rust 程式

## 實作目標

> 透過 strace 觀察 Rust 程式的 system call，建立「程式碼 ↔ syscall」的對應關係

完成後你將學會：
- 使用 strace 追蹤程式
- 辨識常見的 system call
- 理解 Rust 程式碼如何轉換成 syscall
- 分析程式的 I/O 行為

---

## 環境準備

### 安裝 strace

```bash
# Ubuntu/Debian
sudo apt install strace

# macOS（使用 dtruss 替代，需要 sudo）
# macOS 上沒有 strace，但有類似的 dtruss
# 注意：macOS 需要關閉 SIP 才能完整使用 dtruss

# 確認安裝
strace --version
```

---

## 實驗一：觀察基本的檔案操作

### 步驟 1：建立測試程式

```rust
// src/main.rs
use std::fs::File;
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== 程式開始 ===");

    // 1. 讀取檔案
    println!("正在讀取檔案...");
    let mut file = File::open("test.txt").expect("無法開啟檔案");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("無法讀取");
    println!("讀取了 {} bytes", content.len());

    // 2. 寫入檔案
    println!("正在寫入檔案...");
    let mut output = File::create("output.txt").expect("無法建立檔案");
    output.write_all(b"Hello from Rust!\n").expect("無法寫入");

    // 3. 睡眠
    println!("睡眠 1 秒...");
    thread::sleep(Duration::from_secs(1));

    println!("=== 程式結束 ===");
}
```

### 步驟 2：準備測試檔案

```bash
echo "This is test content for strace experiment." > test.txt
```

### 步驟 3：編譯並用 strace 執行

```bash
# 編譯（建議用 release 減少雜訊）
cargo build --release

# 用 strace 執行
strace ./target/release/strace_demo 2>&1 | less
```

### 步驟 4：觀察重點

在輸出中尋找這些 syscall：

```bash
# 1. 打開 test.txt
openat(AT_FDCWD, "test.txt", O_RDONLY|O_CLOEXEC) = 3
#      ↑ 目前目錄    ↑ 檔案名      ↑ 唯讀           ↑ 回傳 fd=3

# 2. 讀取檔案內容
read(3, "This is test content...", 8192) = 44
#    ↑ fd=3                              ↑ 讀了 44 bytes

# 3. 關閉 test.txt
close(3) = 0

# 4. 建立 output.txt
openat(AT_FDCWD, "output.txt", O_WRONLY|O_CREAT|O_TRUNC, 0666) = 3
#                               ↑ 寫入   ↑ 不存在就建立  ↑ 清空

# 5. 寫入內容
write(3, "Hello from Rust!\n", 17) = 17

# 6. 睡眠
nanosleep({tv_sec=1, tv_nsec=0}, ...) = 0
#          ↑ 1 秒
```

### 思考問題

1. `println!` 對應哪個 syscall？fd 是多少？
2. 為什麼 `File::open` 回傳的 fd 是 3 而不是 0？
3. 你有看到 `close(3)` 嗎？是誰呼叫的？（提示：Rust 的 Drop）

---

## 實驗二：觀察多執行緒

### 測試程式

```rust
use std::thread;

fn main() {
    println!("Main thread starting...");

    let handles: Vec<_> = (0..3)
        .map(|i| {
            thread::spawn(move || {
                println!("Thread {} running", i);
                thread::sleep(std::time::Duration::from_millis(100));
                println!("Thread {} done", i);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("All threads done");
}
```

### 用 strace 追蹤

```bash
# -f 會追蹤 fork/clone 出來的子程序/執行緒
strace -f ./target/release/thread_demo 2>&1 | less
```

### 觀察重點

```bash
# 主執行緒 clone 出新執行緒
clone(child_stack=0x7f..., flags=CLONE_VM|CLONE_FS|...) = 12345
#                          ↑ CLONE_VM 表示共享記憶體空間（這就是 thread）

# 不同執行緒的輸出會有 PID 標註
[pid 12345] write(1, "Thread 0 running\n", 17) = 17
[pid 12346] write(1, "Thread 1 running\n", 17) = 17
```

---

## 實驗三：觀察網路操作

### 測試程式

```rust
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    // 在另一個執行緒啟動 server
    thread::spawn(|| {
        let listener = TcpListener::bind("127.0.0.1:9999").unwrap();
        println!("Server listening on 9999");

        if let Ok((mut stream, _)) = listener.accept() {
            let mut buf = [0u8; 1024];
            let n = stream.read(&mut buf).unwrap();
            println!("Server received: {}", String::from_utf8_lossy(&buf[..n]));
            stream.write_all(b"Hello from server!").unwrap();
        }
    });

    // 等待 server 啟動
    thread::sleep(std::time::Duration::from_millis(100));

    // Client 連線
    let mut client = TcpStream::connect("127.0.0.1:9999").unwrap();
    client.write_all(b"Hello from client!").unwrap();

    let mut response = String::new();
    client.read_to_string(&mut response).unwrap();
    println!("Client received: {}", response);
}
```

### 觀察重點

```bash
# 建立 socket
socket(AF_INET, SOCK_STREAM|SOCK_CLOEXEC, IPPROTO_TCP) = 3

# 綁定地址
bind(3, {sa_family=AF_INET, sin_port=htons(9999), sin_addr=inet_addr("127.0.0.1")}, 16) = 0

# 開始監聽
listen(3, 128) = 0

# 接受連線
accept4(3, ...) = 4  # 新連線的 fd=4

# Client 端連線
connect(5, {sa_family=AF_INET, sin_port=htons(9999), ...}, 16) = 0
```

---

## 實驗四：效能分析

### 統計 syscall 次數

```bash
# -c 會統計每種 syscall 的呼叫次數和時間
strace -c ./target/release/your_program
```

輸出範例：
```
% time     seconds  usecs/call     calls    errors syscall
------ ----------- ----------- --------- --------- ----------------
 35.00    0.000350          35        10           write
 25.00    0.000250          25        10           read
 20.00    0.000200          40         5           openat
 10.00    0.000100          20         5           close
 10.00    0.000100         100         1           nanosleep
------ ----------- ----------- --------- --------- ----------------
100.00    0.001000                    31           total
```

### 只追蹤特定 syscall

```bash
# 只看檔案相關
strace -e trace=open,openat,read,write,close ./program

# 只看網路相關
strace -e trace=socket,bind,listen,accept,connect,send,recv ./program

# 只看記憶體相關
strace -e trace=mmap,munmap,brk,mprotect ./program
```

---

## 常見 System Call 對照表

| Rust 程式碼 | System Call |
|------------|-------------|
| `File::open(path)` | `openat(AT_FDCWD, path, O_RDONLY)` |
| `File::create(path)` | `openat(AT_FDCWD, path, O_WRONLY\|O_CREAT\|O_TRUNC)` |
| `file.read(&mut buf)` | `read(fd, buf, len)` |
| `file.write(data)` | `write(fd, data, len)` |
| `drop(file)` | `close(fd)` |
| `println!(...)` | `write(1, ...)` |
| `eprintln!(...)` | `write(2, ...)` |
| `thread::spawn(...)` | `clone(...)` |
| `thread::sleep(...)` | `nanosleep(...)` |
| `TcpListener::bind(...)` | `socket() + bind() + listen()` |
| `listener.accept()` | `accept4(...)` |
| `TcpStream::connect(...)` | `socket() + connect()` |

---

## 驗收標準

- [ ] 能用 strace 追蹤自己的程式
- [ ] 能辨識 open/read/write/close 等基本 syscall
- [ ] 能解釋 fd 的概念
- [ ] 能用 `-c` 統計 syscall
- [ ] 能用 `-f` 追蹤多執行緒

---

## 延伸閱讀

- `man strace`
- `man syscalls` - Linux syscall 列表
- [Julia Evans: strace zine](https://wizardzines.com/zines/strace/)
