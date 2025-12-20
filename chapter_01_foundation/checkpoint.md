# Chapter 1 Checkpoint

Use this checklist to verify you've mastered the core concepts of this chapter.

---

## Self-Assessment Questions

### Rust Fundamentals (1.1)

**Ownership and Borrowing**
- [x] Can explain what "ownership transfer" (move) means
- [x] Can explain the difference between `&T` and `&mut T`
- [x] Can explain why you can't have multiple `&mut T` simultaneously

**Error Handling**
- [x] Can explain the purpose of `Result<T, E>`
- [x] Can correctly use the `?` operator
- [x] Know when to use `unwrap()` vs `expect()` vs `?`

**Multithreading**
- [x] Can explain the purpose of `Arc`
- [x] Can explain the purpose of `Mutex`
- [x] Can explain when to use `mpsc::channel`
- [x] Know what `move` closures mean

### Linux Basics (1.2)

**Basic Concepts**
- [x] Can explain the difference between process and thread
- [x] Can explain what a file descriptor (fd) is
- [x] Know what fd 0, 1, 2 represent

**System Tools**
- [x] Can use `ps aux` to view processes
- [x] Can use `htop` to monitor the system
- [x] Can use `strace` to trace system calls
- [x] Know what `/proc` is

---

## Implementation Verification

### Lab 1: Mini Cat/Grep ✓

```bash
# Test commands
cd chapter_01_foundation/01_rust_fundamentals/lab_01_mini_cat
cargo run -- test.txt
cargo run -- test.txt error
cargo run -- test.txt error -n
```

Acceptance criteria:
- [x] Can correctly read and display file contents
- [x] Can filter lines by keyword
- [x] Can display line numbers
- [x] Shows friendly error message when file doesn't exist

### Lab 2: Parallel Computation ✓

```bash
cd chapter_01_foundation/01_rust_fundamentals/lab_02_parallel_sum
cargo run --release
```

Acceptance criteria:
- [x] All three versions calculate correct results
- [x] Performance comparison completed
- [x] Can explain differences between versions

### Lab 3: strace Observation ✓

```bash
cd chapter_01_foundation/02_linux_basics/lab_03_strace
cargo build --release
strace ./target/release/strace_demo
```

Acceptance criteria:
- [x] Can identify open/read/write/close syscalls
- [x] Can trace multithreaded programs (`strace -f`)
- [x] Can get syscall statistics (`strace -c`)

### Lab 4: Mini PS ✓

```bash
cd chapter_01_foundation/02_linux_basics/lab_04_mini_ps
cargo run  # Requires Linux environment
```

Acceptance criteria:
- [x] Can list all processes
- [x] Can display PID, PPID, STATE, MEMORY
- [x] Can display command line

---

## Concept Connection Quiz

Answer the following questions (one or two sentences):

1. **What's the relationship between Rust's ownership system and Linux's fd?**

   _Hint: Think about what happens when a `File` is dropped_

   Your answer:
   ```
   Rust 的**所有權（ownership）系統**和 Linux 的 **fd（file descriptor）**之間，關係可以用一句話抓住重點：

**Rust 用「誰擁有值、何時釋放」的規則，把「fd 何時 close、避免洩漏或重複 close」這種傳統上很容易出錯的資源管理，變成編譯期就能幫你兜住的大部分問題。**  
下面用幾個面向把它對齊到 Linux 的 fd 行為。

---

## 🧭 核心對應：fd 是資源；Ownership/Drop 是自動釋放機制
在 Linux 裡：

- `open()` / `socket()` 回傳一個整數 fd
- 你必須在適當時機 `close(fd)`
- 忘記 close → **fd leak**
- close 太早或重複 close → **用到無效 fd**、甚至關錯資源（因為 fd 號碼可能被重用）

在 Rust 裡（標準庫與常見 crate 的設計慣例）：

- 會把 fd 包在一個型別裡（例如 `std::fs::File`、`std::net::TcpStream`、`OwnedFd`）
- 這個型別通常遵循 **RAII**：  
  **離開作用域（scope）就會自動執行 `Drop`，在 `Drop` 裡 `close()`**
- 所有權規則確保：**同一個「擁有者」在生命週期結束時負責釋放**，降低重複 close 的風險

這就是「ownership 管資源」的典型例子：fd 只是最常見的一種資源。

---

## 🔍 借用（Borrowing）如何對應「暫時使用 fd」
很多時候你想「用一下 fd」但不想接管它的生命週期（不想負責 close）。這正是借用擅長的部分。

### 借用語意：用，但不擁有
- Rust 允許你拿到 `&File` / `&TcpStream` 之類的**借用**
- 或拿到一個「借用的 fd 觀點」：像 `BorrowedFd<'a>`（概念上）
- 你可以把它傳給需要「讀/寫/設定選項」的函式
- 但因為你沒有所有權：**你不能（也不該）關閉它**

**生命週期（lifetime）**在這裡對應的就是：「你借用 fd 的有效時間，不能比擁有者活得更久」。  
這直接避免了很常見的 bug：拿著一個已被 close 的 fd 還在用。

---

## 🧩 “一個 fd 數字”不等於“一份唯一所有權”：dup/clone 的重要性
Linux 裡要小心一點：fd 是「整數」沒錯，但它背後有兩層概念：

- **fd number（檔案描述符號碼）**：每個行程自己的表格索引
- **open file description（核心中的開啟狀態）**：包含 offset、flags 等

當你 `dup()` 一個 fd，你得到**另一個 fd number**，但通常指向**同一份 open file description**（共享 offset 等狀態）。

Rust 的所有權能保證「某個 Rust 值只 Drop 一次」，但你要自己清楚：

- 你如果只是複製了整數 fd（例如把 `RawFd` 亂拷貝），**Rust 無法阻止你重複 close**
- 正確做法是：  
  - 想要「多一份可獨立關閉的句柄」→ 用 `dup` / `try_clone` 等產生新的 owned handle  
  - 想要「只是借來用一下」→ 用借用型態或 `AsRawFd` 類介面拿參考

一句話：**Ownership 能保證「Rust 物件」的唯一 Drop，不保證你不會把裸 fd 亂複製造成邏輯上的雙重 close。**

---

## ⚠️ RawFd vs 安全封裝：為什麼 Rust 強調型別很重要
你會看到幾種常見介面（概念層級）：

- `AsRawFd`：**借用視角**拿到 `RawFd`（你不擁有它，別 close）
- `IntoRawFd`：**把所有權交出去**，Rust 之後不會在 Drop 時 close（責任轉移）
- `FromRawFd`：**從裸 fd 接管所有權**（之後 Drop 會 close）

危險點在 `FromRawFd`：  
如果你用同一個裸 fd 呼叫兩次 `FromRawFd`，就等於製造了兩個「都以為自己擁有」的物件，最後必然 **double close**。  
這也是為什麼 Rust 把這類操作設計得比較「需要你很確定才用」。

---

## 🔒 併發（threads）下的關係：Send/Sync 與「同時關閉」的競態
在多執行緒中，fd 的問題常見是：

- 一個執行緒在 `read/write`，另一個執行緒把 fd `close` 了 → 競態條件

Rust 的型別系統（`Send`/`Sync`）與所有權可以幫你把很多「不該共享就別共享」的情況擋在編譯期，但這裡要分清楚兩件事：

- **是否能安全地把一個 I/O 物件跨執行緒移動或共享**（由型別的 `Send/Sync` 決定）
- **你的程式邏輯是否會在不對的時間 close**（仍需要設計，例如用 `Arc` 管理共享生命週期、用協定/取消機制讓關閉時機一致）

Rust 讓「共享」變成顯式（例如 `Arc<T>`），讓「誰負責 Drop」更清楚，從而降低競態 bug 機率。

---

## 🧰 和 Linux 行為的幾個實用對照點
### `fork/exec` 與 `CLOEXEC`
- Linux：若 fd 沒設 `FD_CLOEXEC`，`exec()` 後可能把 fd 洩漏到新程式
- Rust 生態常見做法：建立 fd 時就偏好設 `CLOEXEC`（視 API/平台而定），減少洩漏

### “關閉”不等於 “對端知道立刻結束”
- socket/pipe 的關閉、半關閉（shutdown）等語意在 Linux 很細
- Rust 會把它包成方法（例如 `shutdown`），但底層語意仍是 OS 的

---

## 💡 重點收束（把兩者連起來）
- **Linux fd**：你手上拿的是一個「需要手動管理生命週期」的資源代號。  
- **Rust ownership + Drop（RAII）**：把「資源釋放」綁定在「值的生命週期」上，讓 `close()` 變成自動且可靠的預設行為。  
- **借用與生命週期**：讓「暫時使用 fd」變安全，避免 use-after-close。  
- **仍需理解 OS 細節**：像 `dup`、`fork/exec`、close 競態等，Rust 幫你減少錯誤，但不會改變 Linux 的基本語意。

如果你正在寫偏底層的 Rust（直接跟 `RawFd` 打交道），記住一條保命準則：**能用安全封裝就不用裸 fd；需要裸 fd 時，清楚區分「借用」和「接管所有權」。**

   ```

2. **Why can `Arc<Mutex<T>>` be shared between threads?**

   _Hint: What problems do `Arc` and `Mutex` each solve?_

   Your answer:
   ```
   `Arc<Mutex<T>>` 之所以能在多執行緒之間共享，是因為它把「**共享所有權**」和「**同步可變存取**」這兩件事拆開處理：`Arc` 負責讓很多執行緒都能拿到同一份資料的「把手」，`Mutex` 負責保證同一時間只有一個執行緒能動那份資料。下面用 Rust 的 `Send/Sync` 規則把它講清楚。

---

## 🧭 Rust 跨執行緒在看什麼：`Send` 與 `Sync`

Rust 用兩個 trait 來約束跨執行緒安全：

- **`Send`**：代表「值可以被**移動**到另一個執行緒」。
- **`Sync`**：代表「`&T`（共享引用）可以被**多個執行緒同時持有**而不出事」。

要讓某個型別能被多執行緒「分享著用」，通常需要它具備某種組合的 `Send/Sync`。

---

## 🔩 `Arc`：解決「共享所有權」的問題

`Arc<T>` = Atomic Reference Counted pointer（原子引用計數智慧指標）。

- 多執行緒都可以 `clone()` 同一個 `Arc`，每個執行緒都拿到一個指向同一份資料的指標。
- **引用計數的加減是原子操作**，所以同時在不同執行緒 clone/drop 都不會把計數弄壞。
- 對比：`Rc<T>` 不是原子計數，所以**不能**跨執行緒共享。

重點：`Arc` 讓「很多人同時擁有」變得安全，但它本身**不保證可變存取安全**。

---

## 🔒 `Mutex`：解決「可變存取」的問題（避免 data race）

`Mutex<T>` 提供互斥鎖：

- 想讀/改裡面的 `T`，必須先 `lock()`。
- `lock()` 成功後會回傳一個 **guard**（鎖守衛），透過它你才能拿到 `&mut T`（概念上是「在鎖保護下的可變存取」）。
- guard 被 drop 時自動解鎖（RAII），因此很符合 Rust 的安全模型。

重點：`Mutex` 讓「同一時間只有一個執行緒能改資料」成為強制規則，因此**避免資料競爭（data race）**。

---

## ✅ 為什麼「兩個合在一起」就能跨執行緒共享？

把它串起來看：

- `Arc`：讓多執行緒能**共享同一個** `Mutex<T>`（共享所有權）。
- `Mutex`：讓多執行緒在任何時刻對 `T` 的存取都被**序列化**（一次只允許一個執行緒進入臨界區）。

所以 `Arc<Mutex<T>>` 的典型使用模式是：

1. 每個執行緒拿到 `Arc` 的一份 clone（大家都指向同一把鎖）。
2. 需要用資料時就 `lock()`。
3. 取得 guard 後安全地讀/改 `T`。
4. guard drop 後釋放鎖，換別的執行緒。

---

## 📌 關鍵限制：不是任何 `T` 都可以

通常你會看到這個條件：

- **`T: Send`** 是必要條件（常見且重要）

理由直覺是：即使有鎖，`T` 仍可能被不同執行緒「輪流」存取；如果 `T` 本質上不能安全地跨執行緒使用（不是 `Send`），那鎖也救不了它。

因此在 trait 層面上常可概括成：

- 只要 **`T: Send`**，`Mutex<T>` 通常就能在多執行緒情境中安全使用；
- 而 `Arc` 又能安全地共享那個 `Mutex<T>`；
- 最終使得 **`Arc<Mutex<T>>` 可以在多執行緒之間共享與傳遞**（具備需要的 `Send/Sync` 性質）。

   ```

3. **What does `strace` showing `write(1, "Hello\n", 6)` mean?**

   Your answer:
   ```
   `strace` 顯示 `write(1, "Hello\n", 6)` 的意思是：你的程式剛剛呼叫了一次 **Linux 的 `write` 系統呼叫（syscall）**，把 **6 個位元組**（`Hello` 加上換行 `\n`）寫到 **檔案描述符 1**（也就是標準輸出 stdout）。下面把每一段拆開解釋。

---

## 🧱 這行 `write(1, "Hello\n", 6)` 分別代表什麼？

`write(fd, buf, count)` 是 POSIX/Linux 常見的系統呼叫，參數意義如下：

- `write`：系統呼叫名稱，用來把資料寫到某個「檔案/裝置/管線/socket」。
- `1`：**檔案描述符（file descriptor, fd）**
  - `0` = stdin（標準輸入）
  - `1` = stdout（標準輸出）
  - `2` = stderr（標準錯誤）
- `"Hello\n"`：要寫出去的緩衝區內容（strace 會把它用可讀字串顯示出來）
- `6`：要寫的位元組數
  - `"Hello\n"` 長度就是 6：`H e l l o \n`

---

## 🔍 `strace` 在做什麼？

`strace` 會「攔截並列印」程式對作業系統發出的系統呼叫。

所以看到這行，表示程式並不是「直接把字印到螢幕」，而是：

1. 呼叫 `write(1, ...)`
2. 核心（kernel）收到要求
3. 核心把資料寫到 stdout 對應的目標（通常是你的終端機；也可能被重導到檔案或 pipe）

---

## ⚙️ 這行通常後面還會有回傳值（代表寫了幾個 bytes）

在完整的 `strace` 輸出中，常見格式會像：

- `write(1, "Hello\n", 6) = 6`

最後的 `= 6` 是 **系統呼叫回傳值**，代表實際成功寫了 6 bytes。

如果是：

- `= -1 EPIPE (Broken pipe)`：寫到管線但讀端已關閉
- `= -1 EBADF (Bad file descriptor)`：fd 無效
- `= 3`：只寫了 3 bytes（少見但可能，稱為 *partial write*；對 pipe/socket/非阻塞 I/O 更常見）

---

## 🧠 小補充：為什麼一定是 stdout（fd=1）就等於螢幕？

不一定「永遠是螢幕」，而是 stdout 指向哪裡就寫到哪裡：

- 正常跑在終端機：stdout → 終端機 → 你看到在螢幕上
- 重導：`./a.out > out.txt`，stdout → 檔案
- 管線：`./a.out | grep H`，stdout → pipe → 下一個程式

所以 `write(1, ...)` 的本質是「寫到 stdout」，而 stdout 的去向取決於你怎麼執行程式。

---

   ```

4. **Why is reading large files with `BufReader` more efficient than direct `read`?**

   _Hint: Think about the cost of system calls_

   Your answer:
   ```
用 `BufReader` 讀大型檔案通常更有效率，核心原因是：**它用「一次多讀一點」把昂貴的系統呼叫（syscall）次數降到很少**，之後你的程式再從記憶體緩衝區拿資料，成本就低很多。這在「你用很小的單位一直讀」時特別明顯（例如逐位元組、逐字元、逐行）。

---

## 🧠 為什麼 `BufReader` 會更快？

### 1) 大幅減少 `read()` 系統呼叫次數（最大差別）
對檔案的 `read()` 最終會進入作業系統核心，是一次 syscall。syscall 需要從使用者態切到核心態，成本比一般記憶體操作高很多。

- **沒有緩衝**：如果你每次只 `read()` 很小的量（1 byte、幾個 byte），就會造成**超多次 syscall**。
- **使用 `BufReader`**：它會先向 OS 一次讀進一大塊（例如 8KB/64KB）到內部 buffer，然後：
  - 你每次要 1 byte / 一行 / 一段小資料時
  - 多半只是從 buffer 取，不需要 syscall
  - buffer 用完才再向 OS 補貨一次

也就是把「很多小讀取」變成「少數大讀取」。

### 2) 從記憶體取資料比跟核心要資料便宜
資料進了 buffer 後：
- 多數操作是 user space 的記憶體讀取/切片/拷貝
- 少了 context switch、核心檢查、檔案系統路徑等固定開銷  
所以 CPU 時間更集中在你的處理邏輯，而不是 I/O 呼叫成本。

### 3) `read_line` / `lines()` 這類 API 需要緩衝才會高效
Rust 的 `BufRead` 提供 `read_line`、`read_until`、`lines()` 等高階功能。

如果沒有 buffer，這些功能常會退化成「為了找換行而一直讀很小塊」，導致 syscall 爆炸。`BufReader` 正是為了讓這些模式變得有效率。

---

## 🔍 什麼情況下「直接 `read`」其實也不慢？

如果你本來就用「大 buffer」手動分塊讀，例如一次讀 64KB：

```rust
let mut buf = vec![0u8; 64 * 1024];
loop {
    let n = file.read(&mut buf)?;
    if n == 0 { break; }
    // 處理 buf[..n]
}
```

那你其實已經做到 `BufReader` 最重要的事：**降低 syscall 次數**。  
這時 `BufReader` 可能幫助不大，甚至多一層包裝會有一點點額外開銷（通常很小）。

---

## 📌 簡單對照表（直覺版）

| 讀取方式 | syscall 次數 | 常見效能 |
|---|---:|---|
| 每次讀很小（1 byte/幾個 byte）直接 `read` | 非常多 | 慢 |
| `BufReader` + 小單位讀（逐行/逐字元） | 很少 | 快 |
| 直接 `read` 但每次讀很大塊 | 很少 | 快（常接近 `BufReader`） |

---

## 💡 重點結論
- `BufReader` 更快的主要原因是：**把多次小 `read()` 合併成少次大 `read()`**，把 syscall 成本攤薄。
- 它最適合：逐行解析、逐字元處理、tokenization 這種「常常拿一小段」的程式。
- 如果你本來就一次讀很大塊，`BufReader` 的提升就沒那麼明顯，因為你已經在「自己做 buffering」了。
   ```

---

## Reference Answers

<details>
<summary>Click to expand answers</summary>

1. **Ownership and fd**

   When a Rust `File` object is dropped, it automatically calls the `close()` system call to close the corresponding fd. This is Rust's RAII (Resource Acquisition Is Initialization) mechanism — the resource's lifetime is bound to the object's lifetime, no manual management needed.

2. **Arc + Mutex**

   - `Arc` (Atomic Reference Counting) solves the "multiple owners" problem: allows multiple threads to hold references to the same data
   - `Mutex` solves the "concurrent writes" problem: ensures only one thread can access the data at a time

3. **write(1, "Hello\n", 6)**

   - `write` is the system call for writing data
   - `1` is the file descriptor, representing stdout
   - `"Hello\n"` is the content to write
   - `6` is the number of bytes to write

4. **BufReader efficiency**

   Each system call has context switch overhead (switching from user space to kernel space). `BufReader` reads a larger chunk (default 8KB) into a buffer at once. Subsequent reads can be served directly from the buffer, reducing the number of system calls.

</details>

---

## Before Moving to Next Chapter

After confirming all items above are checked, you can proceed to Chapter 2: **OS: Process / Thread / Memory / I/O**.

Chapter 2 will dive deeper into:
- System-level differences between Process vs Thread
- Memory management and virtual memory
- I/O models: blocking, non-blocking, async
