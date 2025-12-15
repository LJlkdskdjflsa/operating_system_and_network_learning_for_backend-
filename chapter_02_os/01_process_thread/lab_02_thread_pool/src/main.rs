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
    // - workers: Vec<Worker>
    // - sender: mpsc::Sender<Job> (or Option<Sender> for shutdown)
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

        // TODO: Implement
        // 1. Create a channel
        // 2. Wrap receiver in Arc<Mutex<...>>
        // 3. Create `size` workers, each with a clone of the receiver
        // 4. Return ThreadPool with workers and sender

        todo!("Implement ThreadPool::new")
    }

    /// Execute a closure on a worker thread
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // TODO: Implement
        // 1. Box the closure
        // 2. Send it through the channel

        todo!("Implement ThreadPool::execute")
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // TODO: Implement graceful shutdown
        // 1. Drop the sender (so workers know to stop)
        // 2. Join all worker threads

        // Note: You might need to change sender to Option<Sender>
        // so you can drop it here
    }
}

impl Worker {
    /// Create a new worker that listens for jobs on the receiver
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // TODO: Implement
        // 1. Spawn a thread
        // 2. In the thread, loop:
        //    - Lock the receiver
        //    - Wait for a job (blocking)
        //    - Execute the job
        //    - If channel is closed, break

        todo!("Implement Worker::new")
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
