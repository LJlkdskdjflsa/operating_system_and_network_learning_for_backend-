//! strace 觀察實驗程式
//!
//! 執行方式：
//!   cargo build --release
//!   strace ./target/release/strace_demo

use std::fs::File;
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== strace 觀察實驗 ===\n");

    // 實驗 1: 檔案讀取
    println!("[實驗 1] 檔案讀取");
    println!("  → 觀察: openat, read, close");
    file_read_experiment();

    // 實驗 2: 檔案寫入
    println!("\n[實驗 2] 檔案寫入");
    println!("  → 觀察: openat (with O_CREAT), write, close");
    file_write_experiment();

    // 實驗 3: 睡眠
    println!("\n[實驗 3] 睡眠 500ms");
    println!("  → 觀察: nanosleep");
    thread::sleep(Duration::from_millis(500));

    // 實驗 4: 多執行緒
    println!("\n[實驗 4] 多執行緒");
    println!("  → 觀察: clone (用 strace -f)");
    multi_thread_experiment();

    println!("\n=== 實驗結束 ===");
}

fn file_read_experiment() {
    // 先確保測試檔案存在
    let test_content = "This is test content for strace experiment.\n";
    std::fs::write("test_input.txt", test_content).expect("無法建立測試檔案");

    // 讀取檔案
    let mut file = File::open("test_input.txt").expect("無法開啟檔案");
    let mut content = String::new();
    let bytes_read = file.read_to_string(&mut content).expect("無法讀取");

    println!("  讀取了 {} bytes", bytes_read);

    // file 在這裡被 drop，會觸發 close syscall
}

fn file_write_experiment() {
    let mut file = File::create("test_output.txt").expect("無法建立檔案");

    // 寫入多次，觀察多個 write syscall
    for i in 1..=3 {
        let line = format!("Line {}: Hello from Rust!\n", i);
        file.write_all(line.as_bytes()).expect("無法寫入");
    }

    println!("  寫入了 3 行");

    // file 在這裡被 drop
}

fn multi_thread_experiment() {
    let handles: Vec<_> = (1..=3)
        .map(|i| {
            thread::spawn(move || {
                println!("  Thread {} 開始", i);
                thread::sleep(Duration::from_millis(50));
                println!("  Thread {} 結束", i);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}
