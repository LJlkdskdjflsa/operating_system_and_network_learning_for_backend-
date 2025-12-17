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