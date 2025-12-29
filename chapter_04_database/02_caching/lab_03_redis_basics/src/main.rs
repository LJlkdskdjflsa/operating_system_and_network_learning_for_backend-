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
use std::collections::HashMap;
use std::time::Duration;
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
    println!("\n=== String Operations ===");

    // 1. SET a key
    let _: () = con.set("greeting", "Hello, Redis!").await?;
    println!("Set 'greeting' = 'Hello, Redis!'");

    // 2. GET the key
    let value: String = con.get("greeting").await?;
    println!("Get 'greeting': {}", value);

    // 3. DELETE the key
    let _: () = con.del("greeting").await?;

    // 4. Verify deletion
    let exists: bool = con.exists("greeting").await?;
    if !exists {
        println!("(deleted 'greeting')");
    }

    Ok(())
}

/// Demonstrate TTL operations
async fn demo_ttl(con: &mut redis::aio::Connection) -> redis::RedisResult<()> {
    println!("\n=== TTL Operations ===");

    // 1. SET with expiration (SET_EX)
    let _: () = con.set_ex("temp", "temporary", 5).await?;
    println!("Set 'temp' with 5 second TTL");

    // 2. Check TTL
    let ttl: i64 = con.ttl("temp").await?;
    println!("TTL of 'temp': {}", ttl);

    // 3. Wait
    println!("(wait 2 seconds)");
    tokio::time::sleep(Duration::from_secs(2)).await;

    // 4. Check TTL again
    let ttl: i64 = con.ttl("temp").await?;
    println!("TTL of 'temp': {}", ttl);

    Ok(())
}

/// Demonstrate hash operations
async fn demo_hash(con: &mut redis::aio::Connection) -> redis::RedisResult<()> {
    // TODO: Implement
    println!("\n=== Hash Operations ===");

    let key = "user:1";
    let user = User {
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };

    // 1. HSET multiple fields
    let _: () = con.hset(key, "name", &user.name).await?;
    let _: () = con.hset(key, "email", &user.email).await?;
    println!("Set user:1 hash");

    // 2. HGET single field
    let name: String = con.hget(key, "name").await?;
    println!("Get user:1 name: {}", name);

    // 3. HGETALL
    let all: HashMap<String, String> = con.hgetall(key).await?;
    let name = all.get("name").cloned().unwrap_or_default();
    let email = all.get("email").cloned().unwrap_or_default();
    println!("Get all user:1: {{name: {}, email: {}}}", name, email);

    Ok(())
}

/// Demonstrate list operations
async fn demo_list(con: &mut redis::aio::Connection) -> redis::RedisResult<()> {
    // TODO: Implement
    // 1. LPUSH multiple items
    // 2. RPOP items
    // 3. LRANGE to view
    println!("\n=== List Operations ===");

    let key = "queue";

    // 1. LPUSH multiple items (in reverse so RPOP returns task3)
    let _: () = con.lpush(key, "task3").await?;
    let _: () = con.lpush(key, "task2").await?;
    let _: () = con.lpush(key, "task1").await?;
    println!("Push to queue");

    // 2. RPOP item
    let task: String = con.rpop(key, None).await?;
    println!("R Pop from queue: {}", task);
    // 2.5 LPOP item
    let task: String = con.lpop(key, None).await?;
    println!("L Pop from queue: {}", task);

    // 3. LRANGE to view
    let items: Vec<String> = con.lrange(key, 0, -1).await?;
    println!("Queue items: {:?}", items);

    Ok(())
}

#[tokio::main]
async fn main() -> redis::RedisResult<()> {
    // TODO: Implement
    // 1. Connect to Redis
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_async_connection().await?;
    // 2. Run each demo function
    demo_strings(&mut con).await?;
    demo_ttl(&mut con).await?;
    demo_hash(&mut con).await?;
    demo_list(&mut con).await?;
    // 3. Clean up (delete test keys)

    let _: () = con.del(vec!["temp", "user:1", "queue"]).await?;
    Ok(())
}
