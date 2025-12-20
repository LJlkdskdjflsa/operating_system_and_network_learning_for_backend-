//! Lab 2: Thread Pool Implementation
//!
//! ## Goal
//! Build a reusable thread pool from scratch
//!
//! ## Requirements
//! 1. Create a `ThreadPool` struct with a fixed number of workers
//! 2. Workers should wait for jobs (not busy-wait/spin)
//! 3. `execute()` method to submit tasks
//! 4. Graceful shutdown when pool is dropped
//!
//! ## Expected Usage
//! ```rust
//! let pool = ThreadPool::new(4);
//!
//! for i in 0..8 {
//!     pool.execute(move || {
//!         println!("Task {} running on some worker", i);
//!     });
//! }
//!
//! // Pool shuts down when dropped
//! ```
//!
//! ## Hints
//! - Use `mpsc::channel` to send jobs to workers
//! - Jobs are `Box<dyn FnOnce() + Send + 'static>`
//! - Workers loop, receiving jobs from shared receiver
//! - Use `Arc<Mutex<Receiver>>` to share receiver among workers
//! - For shutdown: send special message or drop sender
//!
//! ## Verification
//! ```bash
//! cargo test              # Run automated tests
//! cargo run               # Run demo
//! ```
//!
//! ## Acceptance Criteria
//! - [ ] `cargo test` all pass
//! - [ ] Pool executes all submitted tasks
//! - [ ] Workers don't busy-wait (use blocking receive)
//! - [ ] Pool can be reused for multiple batches of tasks
//! - [ ] Graceful shutdown (no panics, all tasks complete)
//!
//! Check solution/main.rs after completing

use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// ============================================================
// TODO: Implement ThreadPool and Worker
// ============================================================

/// A job is a boxed closure that can be sent across threads
type Job = Box<dyn FnOnce() + Send + 'static>;

/// Thread pool that manages a fixed number of worker threads
pub struct ThreadPool {
    // TODO: Add fields
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

/// A worker that runs in its own thread
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl ThreadPool {
    /// Create a new ThreadPool with `size` workers
    ///
    /// # Panics
    /// Panics if size is 0
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0, "Thread pool size must be > 0");

        let (sender, receiver) = mpsc::channel();
        let shared_receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&shared_receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    /// Execute a closure on a worker thread
    pub fn execute<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(job);
        if let Some(sender) = &self.sender {
            sender.send(job).expect("Failed to send job to worker");
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("ThreadPool shutting down");
        self.sender.take(); // We call take() to explicitly drop the Sender. Dropping it closes the channel, so each worker’s recv() returns Err and the worker can exit.

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                // removes the handle, leaving None, so we own the handle and can join it., only join if the worker still has a thread.
                thread.join().expect("Failed to join worker thread"); // waits for that worker thread to finish.
            }
        }
    }
}

impl Worker {
    /// Create a new worker that listens for jobs on the receiver
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // TODO: Implement
        // 1. Spawn a thread
        let thread = thread::spawn(move || {
            println!("Worker {} started", id);
            loop {
                let message = receiver.lock().expect("Failed to lock receiver").recv();
                //recv() returns exactly one message each time it’s called.
                match message {
                    Ok(job) => {
                        println!("Worker {} got a job; executing", id);
                        job(); // Wait for a job (blocking)
                    }
                    Err(_) => {
                        println!("Worker {} shutting down", id);
                        break; // If channel is closed, break
                    }
                }
            }
        });
        //: move || ...: a closure that takes ownership of captured variables (so it can run safely in the new thread)
        //: loop {}`: an infinite loop inside that thread (here, empty)
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

// ============================================================
// Demo (no modification needed)
// ============================================================

fn main() {
    println!("=== Thread Pool Demo ===\n");

    let pool = ThreadPool::new(4);
    println!("Created thread pool with 4 workers\n");

    println!("Submitting 8 tasks...");
    for i in 0..8 {
        pool.execute(move || {
            println!("  Task {} starting", i);
            thread::sleep(std::time::Duration::from_millis(100));
            println!("  Task {} done", i);
        });
    }

    // Give tasks time to complete
    thread::sleep(std::time::Duration::from_millis(500));

    println!("\nSubmitting 4 more tasks...");
    for i in 8..12 {
        pool.execute(move || {
            println!("  Task {} starting", i);
            thread::sleep(std::time::Duration::from_millis(50));
            println!("  Task {} done", i);
        });
    }

    thread::sleep(std::time::Duration::from_millis(300));

    println!("\nDropping pool (should trigger graceful shutdown)...");
    drop(pool);

    println!("Pool dropped successfully!");
}
