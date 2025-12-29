//! Lab 4: Cache Patterns
//!
//! ## Goal
//! Implement cache-aside pattern with TTL and fallback
//! (Using in-memory HashMap to simulate Redis)
//!
//! ## Requirements
//! 1. Implement cache-aside pattern
//! 2. Support TTL (expiration)
//! 3. Handle cache misses
//! 4. Fallback to "database" on cache failure
//! 5. Track cache hit/miss statistics
//!
//! ## Expected Behavior
//! ```
//! $ cargo run
//! === Cache-Aside Pattern Demo ===
//!
//! First request (cache miss):
//!   Cache miss for user:1
//!   Fetching from database...
//!   Stored in cache with 5s TTL
//!   User { id: 1, name: "Alice" }
//!
//! Second request (cache hit):
//!   Cache hit for user:1
//!   User { id: 1, name: "Alice" }
//!
//! After TTL expires:
//!   Cache miss for user:1
//!   Fetching from database...
//!
//! Stats: hits=5, misses=3, hit_rate=62.5%
//! ```
//!
//! ## Hints
//! - Use HashMap<String, (Value, Instant)> for cache with expiry
//! - Check expiration on read
//! - Simulate database with a HashMap
//!
//! ## Acceptance Criteria
//! - [ ] Cache-aside pattern works correctly
//! - [ ] TTL causes automatic invalidation
//! - [ ] Cache misses fetch from database
//! - [ ] Statistics are tracked accurately

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

// ============================================================
// TODO: Implement cache-aside pattern
// ============================================================

/// User data
#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: i64,
    name: String,
    email: String,
}

/// Cache entry with expiration
struct CacheEntry {
    value: String, // JSON serialized
    expires_at: Instant,
}

/// Simple in-memory cache (simulates Redis)
struct Cache {
    data: Mutex<HashMap<String, CacheEntry>>,
    stats: Mutex<CacheStats>,
}

/// Cache statistics
#[derive(Default)]
struct CacheStats {
    hits: u64,
    misses: u64,
}

impl Cache {
    fn new() -> Self {
        // TODO: Initialize cache
        Cache {
            data: Mutex::new(HashMap::new()),
            stats: Mutex::new(CacheStats::default()),
        }
    }

    /// Get value from cache
    fn get<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Option<T> {
        let mut data = self.data.lock().unwrap();
        if let Some(entry) = data.get(key) {
            if entry.expires_at > Instant::now() {
                if let Ok(value) = serde_json::from_str::<T>(&entry.value) {
                    let mut stats = self.stats.lock().unwrap();
                    stats.hits += 1;
                    println!("  Cache hit for {}", key);
                    return Some(value);
                }
                data.remove(key);
            } else {
                data.remove(key);
            }
        }

        let mut stats = self.stats.lock().unwrap();
        stats.misses += 1;
        println!("  Cache miss for {}", key);
        None
    }

    /// Set value in cache with TTL
    fn set<T: Serialize>(&self, key: &str, value: &T, ttl: Duration) {
        // TODO: Implement
        // 1. Serialize value
        // 2. Calculate expiration time
        // 3. Store in cache
        let mut data = self.data.lock().unwrap();
        let entry = CacheEntry {
            value: serde_json::to_string(value).unwrap(),
            expires_at: Instant::now() + ttl,
        };
        data.insert(key.to_string(), entry);
        println!("  Stored in cache with {}s TTL", ttl.as_secs());
    }

    /// Get cache statistics
    fn stats(&self) -> (u64, u64, f64) {
        let stats = self.stats.lock().unwrap();
        let total = stats.hits + stats.misses;
        let hit_rate = if total == 0 {
            0.0
        } else {
            (stats.hits as f64 / total as f64) * 100.0
        };
        (stats.hits, stats.misses, hit_rate)
    }
}

/// Simulated database
struct Database {
    users: HashMap<i64, User>,
}

impl Database {
    fn new() -> Self {
        // TODO: Create with some sample users
        let mut users = HashMap::new();
        users.insert(
            1,
            User {
                id: 1,
                name: "Alice".to_string(),
                email: "alice@example.com".to_string(),
            },
        );
        users.insert(
            2,
            User {
                id: 2,
                name: "Bob".to_string(),
                email: "bob@example.com".to_string(),
            },
        );
        users.insert(
            3,
            User {
                id: 3,
                name: "Charlie".to_string(),
                email: "charlie@example.com".to_string(),
            },
        );
        Database { users }
    }

    /// Simulate slow database query
    async fn get_user(&self, id: i64) -> Option<User> {
        // TODO: Implement with simulated delay
        println!("  Fetching from database...");
        tokio::time::sleep(Duration::from_millis(100)).await;
        self.users.get(&id).cloned()
    }
}

/// Cache-aside implementation
async fn get_user_cached(cache: &Cache, db: &Database, id: i64, ttl: Duration) -> Option<User> {
    // TODO: Implement cache-aside pattern
    // 1. Try cache first
    let key = format!("user:{}", id);

    if let Some(user) = cache.get::<User>(&key) {
        return Some(user);
    }
    // 2. On miss, fetch from database
    // 3. Store in cache
    // 4. Return result
    let user = db.get_user(id).await?;
    cache.set(&key, &user, ttl);
    Some(user)
}

#[tokio::main]
async fn main() {
    // TODO: Implement demo
    println!("=== Cache-Aside Pattern Demo ===\n");
    // 1. Create cache and database
    let cache = Cache::new();
    let db = Database::new();
    let ttl = Duration::from_secs(5);
    // 2. Make requests (observe hits/misses)
    println!("First request (cache miss):");
    let user = get_user_cached(&cache, &db, 1, ttl).await;
    println!("  {:?}\n", user.unwrap());

    println!("Second request (cache hit):");
    let user = get_user_cached(&cache, &db, 1, ttl).await;
    println!("  {:?}\n", user.unwrap());
    println!("Additional requests:");
    let user = get_user_cached(&cache, &db, 1, ttl).await;
    println!("  {:?}", user.unwrap());
    let user = get_user_cached(&cache, &db, 2, ttl).await;
    println!("  {:?}", user.unwrap());
    let user = get_user_cached(&cache, &db, 2, ttl).await;
    println!("  {:?}", user.unwrap());
    let user = get_user_cached(&cache, &db, 1, ttl).await;
    println!("  {:?}\n", user.unwrap());
    // 3. Wait for TTL to expire
    println!("After TTL expires:");
    tokio::time::sleep(Duration::from_secs(6)).await;
    let user = get_user_cached(&cache, &db, 1, ttl).await;
    println!("  {:?}\n", user.unwrap());
    let user = get_user_cached(&cache, &db, 1, ttl).await;
    println!("  {:?}\n", user.unwrap());
    // 4. Make more requests
    // 5. Print statistics
    let (hits, misses, hit_rate) = cache.stats();
    println!(
        "Stats: hits={}, misses={}, hit_rate={:.1}%",
        hits, misses, hit_rate
    );
}
