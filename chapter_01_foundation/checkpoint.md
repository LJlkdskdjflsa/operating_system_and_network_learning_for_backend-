# 第一章 檢核點

用這份清單確認你已經掌握本章的核心概念。

---

## 自我檢測問題

### Rust 基礎（1.1）

**所有權與借用**
- [ ] 能解釋什麼是「所有權轉移」(move)
- [ ] 能解釋 `&T` 和 `&mut T` 的差別
- [ ] 能解釋為什麼不能同時有多個 `&mut T`

**錯誤處理**
- [ ] 能解釋 `Result<T, E>` 的用途
- [ ] 能正確使用 `?` 運算子
- [ ] 知道何時該用 `unwrap()` vs `expect()` vs `?`

**多執行緒**
- [ ] 能解釋 `Arc` 的用途
- [ ] 能解釋 `Mutex` 的用途
- [ ] 能解釋 `mpsc::channel` 的使用場景
- [ ] 知道 `move` closure 的意義

### Linux 基礎（1.2）

**基本概念**
- [ ] 能解釋 process 和 thread 的差別
- [ ] 能解釋什麼是 file descriptor (fd)
- [ ] 知道 fd 0, 1, 2 分別代表什麼

**系統工具**
- [ ] 會用 `ps aux` 查看程序
- [ ] 會用 `htop` 監控系統
- [ ] 會用 `strace` 追蹤 system call
- [ ] 知道 `/proc` 是什麼

---

## 實作驗收

### Lab 1: Mini Cat/Grep ✓

```bash
# 測試指令
cd chapter_01_foundation/01_rust_fundamentals/lab_01_mini_cat
cargo run -- test.txt
cargo run -- test.txt error
cargo run -- test.txt error -n
```

驗收項目：
- [ ] 能顯示檔案內容
- [ ] 能過濾包含關鍵字的行
- [ ] 能顯示行號
- [ ] 檔案不存在時顯示友善錯誤訊息

### Lab 2: 平行計算 ✓

```bash
cd chapter_01_foundation/01_rust_fundamentals/lab_02_parallel_sum
cargo run --release
```

驗收項目：
- [ ] 三種版本都能正確計算結果
- [ ] 有做效能比較
- [ ] 能解釋各版本的差異

### Lab 3: strace 觀察 ✓

```bash
cd chapter_01_foundation/02_linux_basics/lab_03_strace
cargo build --release
strace ./target/release/strace_demo
```

驗收項目：
- [ ] 能辨識 open/read/write/close
- [ ] 能追蹤多執行緒 (`strace -f`)
- [ ] 能統計 syscall (`strace -c`)

### Lab 4: Mini PS ✓

```bash
cd chapter_01_foundation/02_linux_basics/lab_04_mini_ps
cargo run  # 需要在 Linux 環境
```

驗收項目：
- [ ] 能列出所有程序
- [ ] 能顯示 PID, PPID, STATE, MEMORY
- [ ] 能顯示命令列

---

## 概念連結測驗

回答以下問題（可以用一兩句話）：

1. **Rust 的所有權系統和 Linux 的 fd 有什麼關聯？**

   _提示：想想 `File` 被 drop 時會發生什麼_

   你的答案：
   ```

   ```

2. **為什麼 `Arc<Mutex<T>>` 可以在多執行緒間共享？**

   _提示：`Arc` 和 `Mutex` 各自解決什麼問題？_

   你的答案：
   ```

   ```

3. **`strace` 顯示 `write(1, "Hello\n", 6)` 是什麼意思？**

   你的答案：
   ```

   ```

4. **為什麼用 `BufReader` 讀取大檔案比直接 `read` 更有效率？**

   _提示：想想 system call 的成本_

   你的答案：
   ```

   ```

---

## 參考答案

<details>
<summary>點擊展開答案</summary>

1. **所有權與 fd**

   當 Rust 的 `File` 物件被 drop 時，會自動呼叫 `close()` system call 關閉對應的 fd。這就是 Rust 的 RAII (Resource Acquisition Is Initialization) 機制——資源的生命週期與物件的生命週期綁定，不需要手動管理。

2. **Arc + Mutex**

   - `Arc`（Atomic Reference Counting）解決「多個所有者」的問題：讓多個執行緒可以持有同一份資料的參照
   - `Mutex` 解決「同時寫入」的問題：確保同一時間只有一個執行緒能存取資料

3. **write(1, "Hello\n", 6)**

   - `write` 是寫入資料的 system call
   - `1` 是 file descriptor，代表 stdout
   - `"Hello\n"` 是要寫入的內容
   - `6` 是要寫入的 byte 數

4. **BufReader 的效率**

   每次 system call 都有 context switch 的成本（從 user space 切換到 kernel space）。`BufReader` 一次讀取較大的區塊（預設 8KB）到緩衝區，之後的讀取可以直接從緩衝區取得，減少 system call 次數。

</details>

---

## 進入下一章前

確認以上所有項目都打勾後，你就可以進入第二章：**OS 向：Process / Thread / Memory / I/O**。

第二章會更深入探討：
- Process vs Thread 的系統層面差異
- 記憶體管理和虛擬記憶體
- I/O 模型：blocking, non-blocking, async
