# 1.1 Rust 強化：後端必備的語言能力

## 本節目標

> 掌握 Rust 中與「系統程式設計」和「並行處理」最相關的核心概念

完成本節後，你將能夠：

- 用所有權系統寫出記憶體安全的程式
- 正確處理錯誤而不 panic
- 寫出安全的多執行緒程式

---

## 1. 所有權 (Ownership)

### 為什麼要學這個？

在 C/C++ 中，記憶體管理是 bug 的溫床：

- **Use after free**：記憶體釋放後還在用
- **Double free**：同一塊記憶體釋放兩次
- **Memory leak**：忘記釋放記憶體

Rust 透過「所有權」在 **編譯期** 就擋住這些問題。

### 核心規則（只有三條）

```rust
// 規則 1: 每個值都有一個「所有者」(owner)
let s1 = String::from("hello");  // s1 是 "hello" 的所有者

// 規則 2: 同一時間只能有一個所有者
let s2 = s1;                     // 所有權轉移 (move) 給 s2
// println!("{}", s1);           // 編譯錯誤！s1 已經無效

// 規則 3: 所有者離開作用域時，值會被丟棄 (drop)
{
    let s3 = String::from("world");
}   // s3 離開作用域，記憶體自動釋放
```

### 與 OS 的關聯

當你 `drop` 一個擁有系統資源的物件時（如 `File`），Rust 會自動呼叫 `close()` system call：

```rust
{
    let file = File::open("test.txt")?;
    // 使用 file...
}   // file 離開作用域 → 自動呼叫 close(fd)
```

用 `strace` 可以觀察到這個行為（Lab 3 會做）。

---

## 2. 借用 (Borrowing)

### 問題：如果每次傳參數都要轉移所有權？

```rust
fn print_length(s: String) {
    println!("Length: {}", s.len());
}   // s 被 drop

fn main() {
    let s = String::from("hello");
    print_length(s);
    // println!("{}", s);  // 錯誤！s 已經被 move 走了
}
```

### 解法：借用

```rust
fn print_length(s: &String) {   // 借用，不取得所有權
    println!("Length: {}", s.len());
}

fn main() {
    let s = String::from("hello");
    print_length(&s);           // 借出去
    println!("{}", s);          // 還能用！
}
```

### 借用規則

```rust
let mut s = String::from("hello");

// 規則 1: 可以有多個不可變借用
let r1 = &s;
let r2 = &s;
println!("{} {}", r1, r2);  // OK

// 規則 2: 或者只能有一個可變借用
let r3 = &mut s;
r3.push_str(" world");

// 規則 3: 不可變和可變借用不能同時存在
// let r4 = &s;             // 如果 r3 還在用，這會錯誤
```

### 為什麼這很重要？

這個規則防止了 **data race**（在並行程式中非常致命）：

- 兩個執行緒同時寫同一個記憶體 → 未定義行為
- Rust 在編譯期就不讓這種情況發生

---

## 3. 生命週期 (Lifetime)

### 問題：借用能活多久？

```rust
fn longest(x: &str, y: &str) -> &str {  // 編譯錯誤！
    if x.len() > y.len() { x } else { y }
}
```

編譯器不知道回傳的參照應該活多久。

### 解法：生命週期標註

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

`'a` 告訴編譯器：「回傳值的生命週期與輸入參數一樣長」。

### 實務心法

1. 大多數情況編譯器會自動推導（lifetime elision）
2. 當編譯器報錯時，通常是在告訴你有潛在的懸空參照
3. 如果生命週期標註很複雜，考慮改用 `String`（擁有所有權）

---

## 4. 錯誤處理

### Rust 的錯誤處理哲學

- **可恢復錯誤**：用 `Result<T, E>`
- **不可恢復錯誤**：用 `panic!`（程式直接崩潰）

後端服務幾乎都用 `Result`，因為你不希望一個錯誤就讓整個服務掛掉。

### Result 基本用法

```rust
use std::fs::File;
use std::io::Read;

fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;  // ? 會提早回傳錯誤
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn main() {
    match read_file("test.txt") {
        Ok(content) => println!("Content: {}", content),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### 錯誤處理的演進

```rust
// 階段 1: 手動 match（繁瑣但清楚）
let file = match File::open(path) {
    Ok(f) => f,
    Err(e) => return Err(e),
};

// 階段 2: 用 ? 運算子（簡潔）
let file = File::open(path)?;

// 階段 3: 用 anyhow（更靈活，推薦）
use anyhow::{Context, Result};

fn read_config() -> Result<Config> {
    let content = std::fs::read_to_string("config.toml")
        .context("Failed to read config file")?;
    // ...
}
```

---

## 5. 多執行緒基礎

### 為什麼後端需要多執行緒？

- 利用多核 CPU
- 平行處理多個請求
- 執行背景任務

### 基本的 thread spawn

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Hello from thread!");
    });

    handle.join().unwrap();  // 等待執行緒結束
}
```

### 問題：如何在執行緒間共享資料？

```rust
// 這樣不行！
let counter = 0;
let handle = thread::spawn(|| {
    counter += 1;  // 錯誤：不能在閉包中修改外部變數
});
```

### 解法 1: Arc + Mutex（共享可變狀態）

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Arc: 原子參照計數，讓多個執行緒可以共享所有權
    // Mutex: 互斥鎖，確保同時只有一個執行緒能修改資料
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("Thread {} incremented count to {}", i, *num);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

### 解法 2: Channel（訊息傳遞）

```rust
use std::sync::mpsc;  // multi-producer, single-consumer
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send("Hello from thread!").unwrap();
    });

    let msg = rx.recv().unwrap();
    println!("Received: {}", msg);
}
```

複雜的案例

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let worker_count = 3;

    // 多個線程透過 channel 傳遞「狀態增量」與進度
    for id in 1..=worker_count {
        let tx = tx.clone();
        thread::spawn(move || {
            for step in 1..=5 {
                let delta = 1; // 每步累加 1，可按需調整
                tx.send((id, step, delta)).unwrap();
                thread::sleep(Duration::from_millis(50));
            }
            // 線程結束時會自動 drop tx
        });
    }
    drop(tx); // 主線程持有的最後一個發送端，丟棄以便迭代結束

    // 主線程集中更新共享狀態，並在收到每個訊息時列印中間結果
    let mut state = 0;
    for (id, step, delta) in rx {
        state += delta;
        println!("recv from thread {id} step {step}: delta={delta} -> state={state}");
    }

    println!("Final state: {state}");
}
// Result
// recv from thread 1 step 1: delta=1 -> state=1
// recv from thread 2 step 1: delta=1 -> state=2
// recv from thread 3 step 1: delta=1 -> state=3
// recv from thread 1 step 2: delta=1 -> state=4
// recv from thread 2 step 2: delta=1 -> state=5
// recv from thread 3 step 2: delta=1 -> state=6
// recv from thread 1 step 3: delta=1 -> state=7
// recv from thread 3 step 3: delta=1 -> state=8
// recv from thread 2 step 3: delta=1 -> state=9
// recv from thread 1 step 4: delta=1 -> state=10
// recv from thread 3 step 4: delta=1 -> state=11
// recv from thread 2 step 4: delta=1 -> state=12
// recv from thread 1 step 5: delta=1 -> state=13
// recv from thread 3 step 5: delta=1 -> state=14
// recv from thread 2 step 5: delta=1 -> state=15
// Final state: 15
```

### Arc vs Mutex vs Channel：何時用哪個？

| 場景                     | 推薦方案                               |
| ------------------------ | -------------------------------------- |
| 多個執行緒讀取同一份資料 | `Arc<T>`（唯讀共享）                   |
| 多個執行緒讀寫同一份資料 | `Arc<Mutex<T>>` 或 `Arc<RwLock<T>>`    |
| 執行緒間傳遞訊息/任務    | `mpsc::channel` 或 `crossbeam_channel` |
| 高併發場景               | 考慮用 channel 避免鎖競爭              |

---

## 6. Async 基礎概念

### 為什麼需要 async？

傳統多執行緒：

- 每個連線一個執行緒
- 10,000 連線 = 10,000 執行緒
- 每個執行緒消耗 ~2MB stack → 20GB 記憶體！

Async：

- 少量執行緒處理大量任務
- 任務在等待 I/O 時讓出 CPU
- 10,000 連線可能只需要 4-8 個執行緒

### 基本語法

```rust

use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = fetch_data("https://httpbin.org/ip").await?;
    println!("{body}");
    Ok(())
}

async fn fetch_data(url: &str) -> Result<String, Error> {
    // 這是一個非同步函式
    let response = reqwest::get(url).await?;  // .await 會「暫停」直到完成
    response.text().await
}

// Result
// {
//   "origin": "36.231.189.249"
// }

```

### 關鍵概念

1. **Future**：代表一個「未來會完成的值」
2. **async fn**：回傳一個 `Future`
3. **.await**：暫停當前任務，等待 Future 完成
4. **Runtime**（如 Tokio）：負責執行和調度這些 Future

### 現在只需理解這些

詳細的 async 會在後面的章節（I/O 模型）深入學習。現在只要知道：

- async 是為了高效處理大量 I/O 操作
- Tokio 是最常用的 async runtime
- `.await` 不會阻塞執行緒，只會暫停當前任務

---

## 小結：這些概念如何串在一起？

```
┌─────────────────────────────────────────────────────┐
│                    你的後端服務                      │
├─────────────────────────────────────────────────────┤
│  Async Runtime (Tokio)                              │
│    ├── Task 1: 處理 HTTP 請求                        │
│    │     └── 用 Result 處理可能的錯誤                │
│    ├── Task 2: 處理另一個請求                        │
│    └── 共享狀態: Arc<RwLock<AppState>>              │
├─────────────────────────────────────────────────────┤
│  所有權系統確保：                                    │
│    - 沒有 data race                                 │
│    - 資源自動釋放（file, socket, memory）            │
│    - 編譯期就抓出大部分並行 bug                      │
└─────────────────────────────────────────────────────┘
```

---

## 接下來

完成理論閱讀後，請進入實作：

1. **Lab 1**: 實作 mini cat/grep → 練習檔案 I/O 和錯誤處理
2. **Lab 2**: 實作平行計算 → 練習 Arc/Mutex/Channel
