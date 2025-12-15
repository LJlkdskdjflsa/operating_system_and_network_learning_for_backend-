//! Lab 3: Redis Basics
//!
//! ## Goal
//! Learn basic Redis operations using the redis crate
//!
//! ## Prerequisites
//! Start Redis: docker run -d --name redis-lab -p 6379:6379 redis:7
//!
//! ## Requirements
//! 1. Connect to Redis
//! 2. Perform string operations (GET, SET, DEL)
//! 3. Use TTL (expiration)
//! 4. Work with hashes
//! 5. Work with lists
//!
//! ## Expected Behavior
//! ```
//! $ cargo run
//! === String Operations ===
//! Set 'greeting' = 'Hello, Redis!'
//! Get 'greeting': Hello, Redis!
//!
//! === TTL Operations ===
//! Set 'temp' with 5 second TTL
//! TTL of 'temp': 5
//! (wait 2 seconds)
//! TTL of 'temp': 3
//!
//! === Hash Operations ===
//! Set user:1 hash
//! Get user:1 name: Alice
//! Get all user:1: {name: Alice, email: alice@example.com}
//!
//! === List Operations ===
//! Push to queue
//! Pop from queue: task3
//! ```
//!
//! ## Hints
//! - Use `redis::Client::open` to create client
//! - Use `get_async_connection` for async operations
//! - Import `redis::AsyncCommands` trait for commands
//!
//! ## Acceptance Criteria
//! - [ ] Can connect to Redis
//! - [ ] String GET/SET works
//! - [ ] TTL is respected
//! - [ ] Hash operations work
//! - [ ] List operations work

use redis::AsyncCommands;
use serde::{Deserialize, Serialize};

// ============================================================
// TODO: Implement Redis operations
// ============================================================

/// User data structure for hash demo
#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    email: String,
}

/// Demonstrate string operations
async fn demo_strings(con: &mut redis::aio::Connection) -> redis::RedisResult<()> {
    // TODO: Implement
    // 1. SET a key
    // 2. GET the key
    // 3. DELETE the key
    // 4. Verify deletion

    todo!("Implement demo_strings")
}

/// Demonstrate TTL operations
async fn demo_ttl(con: &mut redis::aio::Connection) -> redis::RedisResult<()> {
    // TODO: Implement
    // 1. SET with expiration (SET_EX)
    // 2. Check TTL
    // 3. Wait
    // 4. Check TTL again

    todo!("Implement demo_ttl")
}

/// Demonstrate hash operations
async fn demo_hash(con: &mut redis::aio::Connection) -> redis::RedisResult<()> {
    // TODO: Implement
    // 1. HSET multiple fields
    // 2. HGET single field
    // 3. HGETALL

    todo!("Implement demo_hash")
}

/// Demonstrate list operations
async fn demo_list(con: &mut redis::aio::Connection) -> redis::RedisResult<()> {
    // TODO: Implement
    // 1. LPUSH multiple items
    // 2. RPOP items
    // 3. LRANGE to view

    todo!("Implement demo_list")
}

#[tokio::main]
async fn main() -> redis::RedisResult<()> {
    // TODO: Implement
    // 1. Connect to Redis
    // 2. Run each demo function
    // 3. Clean up (delete test keys)

    todo!("Implement main")
}
