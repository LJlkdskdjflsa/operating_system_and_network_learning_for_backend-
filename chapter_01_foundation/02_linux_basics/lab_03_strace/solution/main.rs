//! Lab 3 Reference Answer

use std::fs::File;
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== strace Observation Experiment ===\n");

    // Experiment 1: File Reading
    println!("[Experiment 1] File Reading");
    println!("  -> Observe: openat, read, close");
    file_read_experiment();

    // Experiment 2: File Writing
    println!("\n[Experiment 2] File Writing");
    println!("  -> Observe: openat (with O_CREAT), write, close");
    file_write_experiment();

    // Experiment 3: Sleep
    println!("\n[Experiment 3] Sleep 500ms");
    println!("  -> Observe: nanosleep");
    thread::sleep(Duration::from_millis(500));

    // Experiment 4: Multi-threading
    println!("\n[Experiment 4] Multi-threading");
    println!("  -> Observe: clone (use strace -f)");
    multi_thread_experiment();

    println!("\n=== Experiment Complete ===");
}

fn file_read_experiment() {
    // First ensure test file exists
    let test_content = "This is test content for strace experiment.\n";
    std::fs::write("test_input.txt", test_content).expect("Failed to create test file");

    // Read the file
    let mut file = File::open("test_input.txt").expect("Failed to open file");
    let mut content = String::new();
    let bytes_read = file.read_to_string(&mut content).expect("Failed to read");

    println!("  Read {} bytes", bytes_read);
    // file is dropped here, triggering close syscall
}

fn file_write_experiment() {
    let mut file = File::create("test_output.txt").expect("Failed to create file");

    // Write multiple times, observe multiple write syscalls
    for i in 1..=3 {
        let line = format!("Line {}: Hello from Rust!\n", i);
        file.write_all(line.as_bytes()).expect("Failed to write");
    }

    println!("  Wrote 3 lines");
    // file is dropped here
}

fn multi_thread_experiment() {
    let handles: Vec<_> = (1..=3)
        .map(|i| {
            thread::spawn(move || {
                println!("  Thread {} started", i);
                thread::sleep(Duration::from_millis(50));
                println!("  Thread {} finished", i);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}
