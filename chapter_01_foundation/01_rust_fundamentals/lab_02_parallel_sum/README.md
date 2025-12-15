# Lab 2: 平行計算 — 多執行緒加總

## 實作目標

> 計算 1 到 N 的總和，使用多執行緒平行處理，並比較不同實作方式

完成後你將學會：
- 使用 `std::thread` 建立執行緒
- 使用 `Arc<Mutex<T>>` 共享可變狀態
- 使用 `mpsc::channel` 傳遞訊息
- 觀察多執行緒的實際行為

---

## 問題定義

計算 `1 + 2 + 3 + ... + N` 的總和。

數學公式：`N * (N + 1) / 2`

但我們要用這個簡單問題來練習多執行緒，把範圍切成多段，讓每個執行緒各算一段。

---

## 階段一：單執行緒版本（基準）

### 任務
先寫一個單執行緒版本，作為效能比較的基準。

```rust
fn sum_sequential(start: u64, end: u64) -> u64 {
    (start..=end).sum()
}

fn main() {
    let n: u64 = 100_000_000;

    let start_time = std::time::Instant::now();
    let result = sum_sequential(1, n);
    let duration = start_time.elapsed();

    println!("Result: {}", result);
    println!("Time: {:?}", duration);
}
```

---

## 階段二：Arc + Mutex 版本

### 概念圖

```
    Main Thread
         │
         ├─── spawn ──→ Thread 1: 計算 1..25M      ─┐
         ├─── spawn ──→ Thread 2: 計算 25M..50M    ─┼── 都寫入 Arc<Mutex<sum>>
         ├─── spawn ──→ Thread 3: 計算 50M..75M    ─┤
         └─── spawn ──→ Thread 4: 計算 75M..100M   ─┘
         │
         ▼
    join all threads
         │
         ▼
    讀取最終結果
```

### 任務
1. 將範圍 `1..N` 切成 `num_threads` 段
2. 每個執行緒計算自己那段的總和
3. 用 `Arc<Mutex<u64>>` 累加結果

### 提示

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn sum_with_mutex(n: u64, num_threads: usize) -> u64 {
    let sum = Arc::new(Mutex::new(0u64));
    let chunk_size = n / num_threads as u64;
    let mut handles = vec![];

    for i in 0..num_threads {
        let sum = Arc::clone(&sum);
        let start = i as u64 * chunk_size + 1;
        let end = if i == num_threads - 1 { n } else { (i + 1) as u64 * chunk_size };

        let handle = thread::spawn(move || {
            let partial_sum: u64 = (start..=end).sum();
            // TODO: 把 partial_sum 加到共享的 sum
            let mut total = sum.lock().unwrap();
            *total += partial_sum;
        });

        handles.push(handle);
    }

    // TODO: 等待所有執行緒完成
    for handle in handles {
        handle.join().unwrap();
    }

    // TODO: 回傳結果
    *sum.lock().unwrap()
}
```

### 學習重點

1. **為什麼需要 Arc？**
   - 多個執行緒要共享同一份資料
   - `Arc` 提供原子參照計數，讓資料可以安全地被多個所有者持有

2. **為什麼需要 Mutex？**
   - 多個執行緒要修改同一份資料
   - `Mutex` 確保同時只有一個執行緒能存取資料

3. **這個版本的問題是什麼？**
   - 鎖競爭：每個執行緒都要搶同一把鎖
   - 但因為我們先算好 partial_sum 才加上去，所以競爭不嚴重

---

## 階段三：Channel 版本

### 概念圖

```
    Main Thread
         │
         ├─── spawn ──→ Thread 1: 計算 1..25M      ─── tx.send(partial_sum)
         ├─── spawn ──→ Thread 2: 計算 25M..50M    ─── tx.send(partial_sum)
         ├─── spawn ──→ Thread 3: 計算 50M..75M    ─── tx.send(partial_sum)
         └─── spawn ──→ Thread 4: 計算 75M..100M   ─── tx.send(partial_sum)
                                                           │
                                                           ▼
                                                    ┌──────────┐
                                                    │ Channel  │
                                                    └──────────┘
                                                           │
         rx.recv() ◄───────────────────────────────────────┘
         │
         ▼
    加總所有 partial_sum
```

### 任務
1. 用 `mpsc::channel` 建立一個 channel
2. 每個執行緒計算完後，把結果 send 到 channel
3. 主執行緒 recv 所有結果並加總

### 提示

```rust
use std::sync::mpsc;
use std::thread;

fn sum_with_channel(n: u64, num_threads: usize) -> u64 {
    let (tx, rx) = mpsc::channel();
    let chunk_size = n / num_threads as u64;

    for i in 0..num_threads {
        let tx = tx.clone();  // 每個執行緒需要自己的 sender
        let start = i as u64 * chunk_size + 1;
        let end = if i == num_threads - 1 { n } else { (i + 1) as u64 * chunk_size };

        thread::spawn(move || {
            let partial_sum: u64 = (start..=end).sum();
            tx.send(partial_sum).unwrap();
        });
    }

    // 重要：丟掉原始的 tx，否則 rx 會一直等待
    drop(tx);

    // TODO: 收集所有結果並加總
    rx.iter().sum()
}
```

### 學習重點

1. **為什麼要 clone tx？**
   - `mpsc` 是 multi-producer, single-consumer
   - 每個 producer（執行緒）需要自己的 sender

2. **為什麼要 drop(tx)？**
   - `rx.iter()` 會一直等待，直到所有 sender 都被 drop
   - 如果不 drop 原始的 tx，程式會 deadlock

3. **Channel vs Mutex 的差異？**
   - Channel: 沒有鎖競爭，但有 channel buffer 的開銷
   - Mutex: 直接共享記憶體，但需要處理鎖競爭

---

## 階段四：效能比較

### 任務
1. 跑不同的 num_threads（1, 2, 4, 8, 16）
2. 記錄每個版本的執行時間
3. 觀察 speedup 曲線

### 觀察模式（讓程式跑久一點）

如果你想用 `top/htop/ps` 觀察 thread 行為，可以開啟觀察模式，程式會：
- 把 `N` 預設拉大（也可用環境變數覆寫）
- 在每個 worker 任務開始時 `sleep` 一下（避免瞬間跑完）
- 使用較「不易被最佳化成公式」的加總方式（方便看到 CPU 工作）

```bash
OBSERVE=1 cargo run --release

# 自訂 N / sleep 毫秒數
OBSERVE=1 N=50000000 SLEEP_MS=200 cargo run --release

# 控制 Rayon thread 數（可選）
OBSERVE=1 RAYON_NUM_THREADS=8 cargo run --release
```

### 預期輸出

```
N = 100,000,000

Sequential:     xxx ms
Mutex (1 thread):  xxx ms
Mutex (2 threads): xxx ms
Mutex (4 threads): xxx ms
Mutex (8 threads): xxx ms

Channel (1 thread):  xxx ms
Channel (2 threads): xxx ms
Channel (4 threads): xxx ms
Channel (8 threads): xxx ms
```

### 思考問題

1. 執行緒數量超過 CPU 核心數後，效能還會繼續提升嗎？
2. 為什麼 Mutex 和 Channel 版本的效能差異不大？
3. 如果每個執行緒的計算量很小（例如只加幾個數），結果會如何？

---

## 進階挑戰（選做）

### 1. 實作一個簡單的 Thread Pool

```rust
struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    fn new(size: usize) -> Self { /* TODO */ }
    fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static { /* TODO */ }
}
```

### 2. 用 htop 觀察

```bash
# 在一個終端跑程式
cargo run --release

# 在另一個終端
htop
# 按 H 切換 thread 顯示
```

觀察：
- 有幾個執行緒在跑？
- CPU 使用率如何分布？

### 3. 用 Rayon 重寫

```toml
[dependencies]
rayon = "1.10"
```

```rust
use rayon::prelude::*;

fn sum_with_rayon(n: u64) -> u64 {
    (1..=n).into_par_iter().sum()
}
```

比較 Rayon 和你手寫版本的效能差異。

---

## 驗收標準

- [x] 單執行緒版本正確計算結果
- [x] Arc + Mutex 版本正確運作
- [x] Channel 版本正確運作
- [x] 有做效能比較並記錄結果
- [ ] 能解釋 Arc、Mutex、Channel 各自的用途

---

## 起始程式碼

查看 `src/main.rs` 取得起始程式碼框架。
