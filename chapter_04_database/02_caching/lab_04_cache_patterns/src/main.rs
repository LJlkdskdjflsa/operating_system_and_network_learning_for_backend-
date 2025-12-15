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

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

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
    value: String,  // JSON serialized
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

        todo!("Implement Cache::new")
    }

    /// Get value from cache
    fn get<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Option<T> {
        // TODO: Implement
        // 1. Lock data
        // 2. Check if key exists
        // 3. Check if expired (remove if so)
        // 4. Update stats
        // 5. Deserialize and return

        todo!("Implement Cache::get")
    }

    /// Set value in cache with TTL
    fn set<T: Serialize>(&self, key: &str, value: &T, ttl: Duration) {
        // TODO: Implement
        // 1. Serialize value
        // 2. Calculate expiration time
        // 3. Store in cache

        todo!("Implement Cache::set")
    }

    /// Get cache statistics
    fn stats(&self) -> (u64, u64, f64) {
        // TODO: Return (hits, misses, hit_rate)

        todo!("Implement Cache::stats")
    }
}

/// Simulated database
struct Database {
    users: HashMap<i64, User>,
}

impl Database {
    fn new() -> Self {
        // TODO: Create with some sample users

        todo!("Implement Database::new")
    }

    /// Simulate slow database query
    async fn get_user(&self, id: i64) -> Option<User> {
        // TODO: Implement with simulated delay

        todo!("Implement Database::get_user")
    }
}

/// Cache-aside implementation
async fn get_user_cached(
    cache: &Cache,
    db: &Database,
    id: i64,
    ttl: Duration,
) -> Option<User> {
    // TODO: Implement cache-aside pattern
    // 1. Try cache first
    // 2. On miss, fetch from database
    // 3. Store in cache
    // 4. Return result

    todo!("Implement get_user_cached")
}

#[tokio::main]
async fn main() {
    // TODO: Implement demo
    // 1. Create cache and database
    // 2. Make requests (observe hits/misses)
    // 3. Wait for TTL to expire
    // 4. Make more requests
    // 5. Print statistics

    todo!("Implement main")
}
