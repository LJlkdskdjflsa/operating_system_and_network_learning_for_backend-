//! Lab 3 Reference Answer

use redis::AsyncCommands;
use std::collections::HashMap;

/// Demonstrate string operations
async fn demo_strings(con: &mut redis::aio::Connection) -> redis::RedisResult<()> {
    println!("\n=== String Operations ===\n");

    // SET
    let _: () = con.set("greeting", "Hello, Redis!").await?;
    println!("SET 'greeting' = 'Hello, Redis!'");

    // GET
    let value: String = con.get("greeting").await?;
    println!("GET 'greeting': {}", value);

    // SET only if not exists
    let result: bool = con.set_nx("greeting", "New value").await?;
    println!("SETNX 'greeting': {} (should be false, key exists)", result);

    // INCR (counter)
    let _: () = con.set("counter", 0).await?;
    let new_val: i64 = con.incr("counter", 1).await?;
    println!("INCR 'counter': {}", new_val);
    let new_val: i64 = con.incr("counter", 5).await?;
    println!("INCRBY 'counter' 5: {}", new_val);

    // DELETE
    let deleted: i32 = con.del("greeting").await?;
    println!("DEL 'greeting': {} key(s) deleted", deleted);

    // Verify deletion
    let exists: bool = con.exists("greeting").await?;
    println!("EXISTS 'greeting': {}", exists);

    // Cleanup
    let _: () = con.del("counter").await?;

    Ok(())
}

/// Demonstrate TTL operations
async fn demo_ttl(con: &mut redis::aio::Connection) -> redis::RedisResult<()> {
    println!("\n=== TTL Operations ===\n");

    // SET with expiration
    let _: () = con.set_ex("temp_key", "temporary value", 5).await?;
    println!("SET 'temp_key' with 5 second TTL");

    // Check TTL
    let ttl: i64 = con.ttl("temp_key").await?;
    println!("TTL 'temp_key': {} seconds", ttl);

    // Wait
    println!("Waiting 2 seconds...");
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    // Check TTL again
    let ttl: i64 = con.ttl("temp_key").await?;
    println!("TTL 'temp_key': {} seconds", ttl);

    // Set TTL on existing key
    let _: () = con.set("persistent", "value").await?;
    println!("\nSET 'persistent' (no TTL)");

    let ttl: i64 = con.ttl("persistent").await?;
    println!("TTL 'persistent': {} (-1 means no expiration)", ttl);

    let _: () = con.expire("persistent", 10).await?;
    println!("EXPIRE 'persistent' 10");

    let ttl: i64 = con.ttl("persistent").await?;
    println!("TTL 'persistent': {} seconds", ttl);

    // Cleanup
    let _: () = con.del("temp_key").await?;
    let _: () = con.del("persistent").await?;

    Ok(())
}

/// Demonstrate hash operations
async fn demo_hash(con: &mut redis::aio::Connection) -> redis::RedisResult<()> {
    println!("\n=== Hash Operations ===\n");

    let key = "user:1";

    // HSET multiple fields
    let _: () = con.hset(key, "name", "Alice").await?;
    let _: () = con.hset(key, "email", "alice@example.com").await?;
    let _: () = con.hset(key, "age", "30").await?;
    println!("HSET {} name='Alice' email='alice@example.com' age='30'", key);

    // HGET single field
    let name: String = con.hget(key, "name").await?;
    println!("HGET {} name: {}", key, name);

    // HGETALL
    let all: HashMap<String, String> = con.hgetall(key).await?;
    println!("HGETALL {}: {:?}", key, all);

    // HMSET (multiple fields at once)
    let _: () = redis::cmd("HMSET")
        .arg("user:2")
        .arg("name")
        .arg("Bob")
        .arg("email")
        .arg("bob@example.com")
        .query_async(con)
        .await?;
    println!("\nHMSET user:2 name='Bob' email='bob@example.com'");

    let user2: HashMap<String, String> = con.hgetall("user:2").await?;
    println!("HGETALL user:2: {:?}", user2);

    // HDEL (delete field)
    let _: () = con.hdel(key, "age").await?;
    println!("\nHDEL {} age", key);

    let all: HashMap<String, String> = con.hgetall(key).await?;
    println!("HGETALL {} (after HDEL): {:?}", key, all);

    // Cleanup
    let _: () = con.del(key).await?;
    let _: () = con.del("user:2").await?;

    Ok(())
}

/// Demonstrate list operations
async fn demo_list(con: &mut redis::aio::Connection) -> redis::RedisResult<()> {
    println!("\n=== List Operations ===\n");

    let key = "task_queue";

    // LPUSH (add to front)
    let _: () = con.lpush(key, "task1").await?;
    let _: () = con.lpush(key, "task2").await?;
    let _: () = con.lpush(key, "task3").await?;
    println!("LPUSH {} task1, task2, task3", key);

    // LRANGE (view list)
    let items: Vec<String> = con.lrange(key, 0, -1).await?;
    println!("LRANGE {} 0 -1: {:?}", key, items);

    // RPOP (remove from back - FIFO queue)
    let task: String = con.rpop(key, None).await?;
    println!("RPOP {}: {}", key, task);

    let items: Vec<String> = con.lrange(key, 0, -1).await?;
    println!("LRANGE {} 0 -1: {:?}", key, items);

    // LLEN (list length)
    let len: i64 = con.llen(key).await?;
    println!("LLEN {}: {}", key, len);

    // RPUSH (add to back)
    let _: () = con.rpush(key, "task4").await?;
    println!("RPUSH {} task4", key);

    let items: Vec<String> = con.lrange(key, 0, -1).await?;
    println!("LRANGE {} 0 -1: {:?}", key, items);

    // Cleanup
    let _: () = con.del(key).await?;

    Ok(())
}

/// Demonstrate sorted set operations
async fn demo_sorted_set(con: &mut redis::aio::Connection) -> redis::RedisResult<()> {
    println!("\n=== Sorted Set Operations (Leaderboard) ===\n");

    let key = "leaderboard";

    // ZADD (add with score)
    let _: () = con.zadd(key, "player1", 100).await?;
    let _: () = con.zadd(key, "player2", 250).await?;
    let _: () = con.zadd(key, "player3", 150).await?;
    let _: () = con.zadd(key, "player4", 200).await?;
    println!("ZADD {} player1=100, player2=250, player3=150, player4=200", key);

    // ZRANGE (ascending order)
    let items: Vec<String> = con.zrange(key, 0, -1).await?;
    println!("ZRANGE {} 0 -1: {:?}", key, items);

    // ZREVRANGE (descending order - top players)
    let items: Vec<String> = con.zrevrange(key, 0, 2).await?;
    println!("ZREVRANGE {} 0 2 (top 3): {:?}", key, items);

    // ZSCORE (get score)
    let score: f64 = con.zscore(key, "player2").await?;
    println!("ZSCORE {} player2: {}", key, score);

    // ZRANK (get rank - 0-indexed)
    let rank: i64 = con.zrevrank(key, "player2").await?;
    println!("ZREVRANK {} player2: {} (0 = first place)", key, rank);

    // ZINCRBY (increment score)
    let new_score: f64 = con.zincr(key, "player1", 200).await?;
    println!("ZINCRBY {} player1 200: new score = {}", key, new_score);

    let items: Vec<String> = con.zrevrange(key, 0, 2).await?;
    println!("New top 3: {:?}", items);

    // Cleanup
    let _: () = con.del(key).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> redis::RedisResult<()> {
    println!("Redis Basics Demo");
    println!("==================");
    println!("\nConnecting to Redis...");

    // Connect to Redis
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_async_connection().await?;

    println!("Connected!");

    // Run demos
    demo_strings(&mut con).await?;
    demo_ttl(&mut con).await?;
    demo_hash(&mut con).await?;
    demo_list(&mut con).await?;
    demo_sorted_set(&mut con).await?;

    println!("\n=== Demo Complete ===");
    println!("\nKey commands summary:");
    println!("  Strings: SET, GET, DEL, INCR, SETNX");
    println!("  TTL:     EXPIRE, TTL, SET_EX");
    println!("  Hashes:  HSET, HGET, HGETALL, HDEL");
    println!("  Lists:   LPUSH, RPUSH, LPOP, RPOP, LRANGE");
    println!("  ZSets:   ZADD, ZRANGE, ZREVRANGE, ZSCORE, ZRANK");

    Ok(())
}

// Key concepts demonstrated:
//
// 1. STRINGS:
//    - Basic key-value storage
//    - Atomic increment
//    - Conditional set (SETNX)
//
// 2. TTL:
//    - Automatic expiration
//    - Set at creation or later
//    - Check remaining time
//
// 3. HASHES:
//    - Store objects as field-value pairs
//    - Efficient for partial updates
//    - No need to serialize/deserialize entire object
//
// 4. LISTS:
//    - Ordered collections
//    - Use as queue (LPUSH + RPOP)
//    - Use as stack (LPUSH + LPOP)
//
// 5. SORTED SETS:
//    - Ordered by score
//    - Perfect for leaderboards
//    - O(log N) insert, O(1) rank lookup
