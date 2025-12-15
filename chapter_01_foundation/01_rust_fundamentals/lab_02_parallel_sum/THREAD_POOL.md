# Thread Pool（固定大小工作池）設計說明

本文件解釋本專案 `src/thread_pool.rs` 的 thread pool 實作邏輯：如何提交工作、worker 如何取工作並執行、以及如何安全關機（Drop）。

對照檔案：
- `src/thread_pool.rs`
- `src/main.rs`（`sum_with_thread_pool` 使用範例）

---

## 1. Thread Pool 想解決什麼問題？

如果每次有任務都直接 `thread::spawn`：
- 建立/回收 thread 有固定成本（OS 資源、排程、堆疊等）。
- 任務很多時會產生大量 thread，造成排程負擔，甚至把機器拖慢。

Thread pool 的做法是：
- 先建立固定數量的 worker threads（例如 4、8）。
- 後續任務以「訊息」方式排隊（queue）送進池子。
- worker 不斷從 queue 取出任務並執行。

一句話：**把「建 thread」的成本攤平到多個任務上**。

---

## 2. 這份實作的整體架構（文字圖解）

這份 thread pool 使用 `std::sync::mpsc` 做「工作佇列」：

```
                   (Message::NewJob / Message::Terminate)
    execute()  ───────────────────────────────▶  [ mpsc channel queue ]
       │                                              │
       │                                              ▼
       │                                      Arc<Mutex<Receiver>>
       │                                              │
       │                   lock + recv()              │
       └──────────────────────────────────────────────┘
                              │
                              ▼
                ┌─────────────────────────────┐
                │ worker thread 1: loop { ... }│
                │ worker thread 2: loop { ... }│
                │ worker thread 3: loop { ... }│
                │ ...                          │
                └─────────────────────────────┘
```

要點：
- 所有 worker 共享同一個 `Receiver`，所以要用 `Arc` 共享所有權。
- `Receiver` 不是可以被多執行緒同時 `recv` 的型別，因此用 `Mutex` 確保一次只有一個 worker 能 `recv`。

---

## 3. 元件與責任分工

### 3.1 `Job`：要執行的任務

```rust
type Job = Box<dyn FnOnce() + Send + 'static>;
```

含義：
- `FnOnce()`：任務可能會 move 捕獲資料，只能執行一次（很常見）。
- `Send`：允許把任務從主執行緒移動到 worker thread。
- `'static`：任務不能借用短生命週期引用（避免 worker 還在跑，但引用的資料已離開作用域）。
- `Box<dyn ...>`：把不同型別的 closure 統一裝箱成同一種型別，才能放進同一條 queue。

### 3.2 `Message`：Thread pool 的「協議」

```rust
enum Message {
    NewJob(Job),
    Terminate,
}
```

這是 pool 跟 worker 溝通的訊息種類：
- `NewJob(Job)`：排入一個工作。
- `Terminate`：請 worker 結束迴圈並退出 thread（用於關機）。

### 3.3 `ThreadPool`：對外介面與資源擁有者

```rust
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}
```

責任：
- `new(size)`: 建好 channel + 建好固定數量 worker。
- `execute(f)`: 把工作包成 `Job`，送進 queue。
- `Drop`: 當 pool 被丟掉時，負責通知 worker 停止並 `join`，避免背景 thread 殘留。

### 3.4 `Worker`：實際跑在背景的執行單位

```rust
struct Worker {
    handle: Option<thread::JoinHandle<()>>,
}
```

責任：
- `Worker::new(receiver)`: spawn 一條 thread，進入 loop 持續 `recv()`。
- 收到 `NewJob(job)` 就執行 `job()`。
- 收到 `Terminate`（或 channel 關閉）就 break 結束。

---

## 4. 工作提交與執行流程

### 4.1 提交工作：`ThreadPool::execute`

概念流程：
1. 呼叫端提供一個 closure（要做的事）。
2. pool 把 closure 裝箱成 `Job`。
3. 透過 `sender.send(Message::NewJob(job))` 丟進 queue。

文字圖解：

```
caller thread
    │  execute(|| do_something())
    ▼
ThreadPool::execute
    │  sender.send(NewJob(job))
    ▼
channel queue
```

### 4.2 取工作與執行：`Worker` 的 loop

每個 worker thread 會一直做：
1. `lock()` 住 `Receiver`（確保一次只有一個 worker 在 `recv`）。
2. `recv()` 等待下一個 `Message`（阻塞）。
3. 依訊息類型：
   - `NewJob(job)` → `job()` 執行任務
   - `Terminate` 或 `Err(_)` → 結束 thread

重點：**取工作的動作被 mutex 序列化，但執行 job 是各自並行**。

---

## 5. 關機（Drop）為什麼需要？怎麼做？

如果你不管 worker thread：
- pool 被 drop 後，worker 可能還在背景跑或阻塞等待 `recv`。
- 程式結束時可能出現資源未回收、或你想要的「所有工作完成」語意不成立。

這份實作在 `Drop for ThreadPool` 做了兩件事：

### 5.1 發送 Terminate：要求每個 worker 退出

```text
for _ in workers:
    sender.send(Terminate)
```

要送「跟 worker 數量一樣多」的 `Terminate`，確保每個 worker 最終都能收到一個退出訊號。

### 5.2 join：等待 worker thread 真的結束

```text
for each worker:
    join(handle)
```

`join` 的語意是：「等這條 thread 跑完」，確保 drop 返回前，所有 worker 都退出。

---

## 6. 為什麼 `Receiver` 要用 `Arc<Mutex<Receiver<_>>>`？

因為同時滿足兩件事：

1. **多個 worker 需要共享同一個 `Receiver`**
   - `Receiver` 只能有一個（mpsc 是 single-consumer），但 worker 有很多個。
   - 用 `Arc` 讓所有 worker 都能持有它的共享所有權。

2. **避免多個 worker 同時 `recv`**
   - `std::sync::mpsc::Receiver` 不是 `Sync`，不能被多執行緒同時安全呼叫 `recv`。
   - 用 `Mutex` 讓 `recv` 在任一時間點只會發生在一個 worker 上。

結果：worker 會排隊拿鎖 → 拿到鎖的那位 worker 才能 `recv()` 下一個工作。

---

## 7. 跟本 Lab 的「平行加總」怎麼接起來？

在 `src/main.rs` 的 `sum_with_thread_pool` 中：
- pool 的 worker 數量 = `num_threads`
- 每個 chunk 會被包成一個 job，丟進 pool
- job 算完 partial sum 後，透過另一條 channel（`result_tx/result_rx`）回報結果給主執行緒

因此整體是「雙 channel」：

```
job queue channel:     ThreadPool::execute  ─▶ worker 執行 job
result channel:        worker tx.send(sum)  ─▶ main rx.iter().sum()
```

---

## 8. 這份 thread pool 的特性與限制（實務理解）

- **固定大小**：worker 數量在 `new(size)` 時決定，不會動態擴縮。
- **單一工作佇列**：所有任務進同一條 queue。
- **取工作序列化**：因為 `Arc<Mutex<Receiver>>`，同時間只有一個 worker 能 `recv`；任務很短且非常密集時，這把鎖可能成為瓶頸。
- **關機語意**：Drop 透過 `Terminate` 讓 worker 退出；一般能確保 thread 被回收，但不提供「清空所有排隊工作才退出」的強保證（若需要，可設計成「先關閉 sender/不再接收新工作，然後讓 worker 把 queue 消耗完再退出」等策略）。

---

## 9. 你可以用來自我檢查的問題

- 為什麼 `Job` 需要 `Send + 'static`？
- 為什麼 worker 要用 loop + `recv()`，而不是每個 job 都 `spawn`？
- `Drop` 裡不 `join` 會怎樣？
- 為什麼需要送出「N 個」`Terminate`？
- 為什麼接收端要 `Mutex`，而不是每個 worker 各自一個 `Receiver`？

