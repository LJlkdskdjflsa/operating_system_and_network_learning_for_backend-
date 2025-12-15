//! Lab 2 Reference Answer

use std::sync::{mpsc, Arc, Mutex};
use std::thread;

/// A job is a boxed closure that can be sent across threads
type Job = Box<dyn FnOnce() + Send + 'static>;

/// Thread pool that manages a fixed number of worker threads
pub struct ThreadPool {
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
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0, "Thread pool size must be > 0");

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    /// Execute a closure on a worker thread
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        if let Some(ref sender) = self.sender {
            sender.send(job).expect("Failed to send job to worker");
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Drop the sender first, so workers know to stop
        drop(self.sender.take());

        // Wait for all workers to finish
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().expect("Failed to join worker thread");
            }
        }
    }
}

impl Worker {
    /// Create a new worker that listens for jobs on the receiver
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // Lock the receiver and wait for a job
            // The lock is released as soon as we get the job
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    // Execute the job
                    job();
                }
                Err(_) => {
                    // Channel closed, time to shut down
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

fn main() {
    println!("=== Thread Pool Demo ===\n");

    let pool = ThreadPool::new(4);
    println!("Created thread pool with 4 workers\n");

    println!("Submitting 8 tasks...");
    for i in 0..8 {
        pool.execute(move || {
            println!("  Task {} starting on thread {:?}", i, thread::current().id());
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_pool_creation() {
        let pool = ThreadPool::new(4);
        assert_eq!(pool.workers.len(), 4);
    }

    #[test]
    #[should_panic]
    fn test_pool_zero_size() {
        ThreadPool::new(0);
    }

    #[test]
    fn test_execute_single_task() {
        let pool = ThreadPool::new(2);
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = Arc::clone(&counter);

        pool.execute(move || {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        thread::sleep(std::time::Duration::from_millis(100));
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_execute_multiple_tasks() {
        let pool = ThreadPool::new(4);
        let counter = Arc::new(AtomicUsize::new(0));

        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            pool.execute(move || {
                counter_clone.fetch_add(1, Ordering::SeqCst);
            });
        }

        thread::sleep(std::time::Duration::from_millis(200));
        assert_eq!(counter.load(Ordering::SeqCst), 10);
    }

    #[test]
    fn test_graceful_shutdown() {
        let counter = Arc::new(AtomicUsize::new(0));

        {
            let pool = ThreadPool::new(2);
            for _ in 0..5 {
                let counter_clone = Arc::clone(&counter);
                pool.execute(move || {
                    thread::sleep(std::time::Duration::from_millis(10));
                    counter_clone.fetch_add(1, Ordering::SeqCst);
                });
            }
            // Pool dropped here - should wait for all tasks
        }

        // All tasks should have completed
        assert_eq!(counter.load(Ordering::SeqCst), 5);
    }
}
