# Lab 1: 實作 Mini Cat/Grep

## 實作目標

> 寫一個命令列工具，能讀取檔案並過濾出包含特定關鍵字的行

完成後你將學會：
- 使用 `std::fs` 和 buffered I/O
- 用 `Result` 和 `?` 處理錯誤
- 解析命令列參數
- 寫出符合 Rust 風格的錯誤處理

---

## 預期功能

```bash
# 顯示檔案全部內容（類似 cat）
$ cargo run -- test.txt

# 過濾包含 "error" 的行（類似 grep）
$ cargo run -- test.txt error

# 顯示行號
$ cargo run -- test.txt error --line-numbers
```

---

## 階段一：基本 Cat 功能

### 任務
1. 讀取命令列指定的檔案
2. 將內容輸出到 stdout

### 提示

```rust
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // 取得命令列參數
    let args: Vec<String> = env::args().collect();

    // args[0] 是程式名稱，args[1] 是第一個參數
    if args.len() < 2 {
        eprintln!("Usage: mini_cat <filename>");
        std::process::exit(1);
    }

    let filename = &args[1];

    // TODO: 打開檔案並讀取
}
```

### 學習重點

**為什麼用 BufReader？**

```rust
// 方法 1: 一次讀取全部（簡單但不適合大檔案）
let content = std::fs::read_to_string(filename)?;

// 方法 2: 使用 BufReader（適合大檔案，逐行讀取）
let file = File::open(filename)?;
let reader = BufReader::new(file);
for line in reader.lines() {
    let line = line?;  // lines() 回傳 Result<String>
    println!("{}", line);
}
```

`BufReader` 在內部維護一個緩衝區，減少 system call 次數。

---

## 階段二：加入 Grep 功能

### 任務
1. 接受第二個參數作為搜尋關鍵字
2. 只輸出包含該關鍵字的行

### 提示

```rust
// 檢查字串是否包含子字串
if line.contains(pattern) {
    println!("{}", line);
}
```

---

## 階段三：加入行號

### 任務
1. 支援 `--line-numbers` 或 `-n` 參數
2. 輸出時在每行前面加上行號

### 預期輸出

```
   1: This is line one
   3: This line contains error
   7: Another error here
```

### 提示

```rust
// enumerate() 會給你 (index, value)
for (line_number, line) in reader.lines().enumerate() {
    let line = line?;
    // line_number 從 0 開始，顯示時 +1
    println!("{:4}: {}", line_number + 1, line);
}
```

---

## 階段四：改進錯誤處理

### 任務
使用 `thiserror` 或 `anyhow` 讓錯誤訊息更清楚

### 使用 anyhow（推薦初學者）

```toml
# Cargo.toml
[dependencies]
anyhow = "1.0"
```

```rust
use anyhow::{Context, Result};

fn read_and_filter(filename: &str, pattern: Option<&str>) -> Result<()> {
    let file = File::open(filename)
        .context(format!("Failed to open file: {}", filename))?;
    // ...
    Ok(())
}

fn main() -> Result<()> {
    // 現在 main 也可以回傳 Result
    let args: Vec<String> = env::args().collect();
    // ...
    read_and_filter(&filename, pattern)?;
    Ok(())
}
```

---

## 進階挑戰（選做）

1. **支援正規表達式**
   - 使用 `regex` crate
   - `cargo add regex`

2. **支援多檔案**
   - `mini_cat file1.txt file2.txt`
   - 在每個檔案輸出前顯示檔名

3. **顯示顏色**
   - 用 `colored` crate 讓匹配的關鍵字變色

4. **效能測試**
   - 準備一個大檔案（100MB+）
   - 比較有無 BufReader 的差異

---

## 驗收標準

- [ ] 能正確讀取並顯示檔案內容
- [ ] 能根據關鍵字過濾行
- [ ] 能顯示行號
- [ ] 檔案不存在時顯示友善的錯誤訊息
- [ ] 程式碼沒有 `unwrap()`（除了已知不會失敗的情況）

---

## 起始程式碼

查看 `src/main.rs` 取得起始程式碼框架。

---

## 延伸閱讀

- [std::io::BufReader](https://doc.rust-lang.org/std/io/struct.BufReader.html)
- [The Rust Book: Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [anyhow crate](https://docs.rs/anyhow/latest/anyhow/)
