//! Lab 4 Reference Answer

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// User data
#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: i64,
    name: String,
    email: String,
}

/// Cache entry with expiration
struct CacheEntry {
    value: String,
    expires_at: Instant,
}

/// Cache statistics
#[derive(Default, Debug)]
struct CacheStats {
    hits: u64,
    misses: u64,
}

/// Simple in-memory cache (simulates Redis)
struct Cache {
    data: Mutex<HashMap<String, CacheEntry>>,
    stats: Mutex<CacheStats>,
}

impl Cache {
    fn new() -> Self {
        Cache {
            data: Mutex::new(HashMap::new()),
            stats: Mutex::new(CacheStats::default()),
        }
    }

    /// Get value from cache
    fn get<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Option<T> {
        let mut data = self.data.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();

        if let Some(entry) = data.get(key) {
            // Check if expired
            if entry.expires_at > Instant::now() {
                stats.hits += 1;
                println!("  [CACHE HIT] {}", key);
                return serde_json::from_str(&entry.value).ok();
            } else {
                // Remove expired entry
                data.remove(key);
                println!("  [CACHE EXPIRED] {}", key);
            }
        }

        stats.misses += 1;
        println!("  [CACHE MISS] {}", key);
        None
    }

    /// Set value in cache with TTL
    fn set<T: Serialize>(&self, key: &str, value: &T, ttl: Duration) {
        let mut data = self.data.lock().unwrap();

        let entry = CacheEntry {
            value: serde_json::to_string(value).unwrap(),
            expires_at: Instant::now() + ttl,
        };

        data.insert(key.to_string(), entry);
        println!("  [CACHE SET] {} (TTL: {:?})", key, ttl);
    }

    /// Delete from cache
    fn delete(&self, key: &str) {
        let mut data = self.data.lock().unwrap();
        data.remove(key);
        println!("  [CACHE DELETE] {}", key);
    }

    /// Get cache statistics
    fn stats(&self) -> (u64, u64, f64) {
        let stats = self.stats.lock().unwrap();
        let total = stats.hits + stats.misses;
        let hit_rate = if total > 0 {
            (stats.hits as f64 / total as f64) * 100.0
        } else {
            0.0
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
        println!("  [DATABASE] Fetching user {} (slow operation)...", id);
        // Simulate database latency
        tokio::time::sleep(Duration::from_millis(100)).await;
        self.users.get(&id).cloned()
    }

    /// Update user in database
    async fn update_user(&mut self, user: User) {
        println!("  [DATABASE] Updating user {}...", user.id);
        tokio::time::sleep(Duration::from_millis(50)).await;
        self.users.insert(user.id, user);
    }
}

/// Cache-aside pattern: get user with caching
async fn get_user_cached(cache: &Cache, db: &Database, id: i64, ttl: Duration) -> Option<User> {
    let key = format!("user:{}", id);

    // 1. Try cache first
    if let Some(user) = cache.get::<User>(&key) {
        return Some(user);
    }

    // 2. Cache miss - fetch from database
    let user = db.get_user(id).await?;

    // 3. Store in cache
    cache.set(&key, &user, ttl);

    Some(user)
}

/// Update user with cache invalidation
async fn update_user_cached(
    cache: &Cache,
    db: &mut Database,
    user: User,
) {
    let key = format!("user:{}", user.id);

    // Update database
    db.update_user(user).await;

    // Invalidate cache
    cache.delete(&key);
}

#[tokio::main]
async fn main() {
    println!("=== Cache-Aside Pattern Demo ===\n");

    let cache = Cache::new();
    let mut db = Database::new();
    let ttl = Duration::from_secs(3); // Short TTL for demo

    // First request - cache miss
    println!("1. First request for user 1:");
    let user = get_user_cached(&cache, &db, 1, ttl).await;
    println!("   Result: {:?}\n", user);

    // Second request - cache hit
    println!("2. Second request for user 1:");
    let user = get_user_cached(&cache, &db, 1, ttl).await;
    println!("   Result: {:?}\n", user);

    // Request different user - cache miss
    println!("3. Request for user 2:");
    let user = get_user_cached(&cache, &db, 2, ttl).await;
    println!("   Result: {:?}\n", user);

    // Multiple hits
    println!("4. Multiple requests (should be cache hits):");
    for i in 1..=3 {
        let user = get_user_cached(&cache, &db, 1, ttl).await;
        println!("   Request {}: {:?}", i, user.map(|u| u.name));
    }
    println!();

    // Request non-existent user
    println!("5. Request for non-existent user 999:");
    let user = get_user_cached(&cache, &db, 999, ttl).await;
    println!("   Result: {:?}\n", user);

    // Wait for TTL to expire
    println!("6. Waiting for TTL to expire (3 seconds)...");
    tokio::time::sleep(Duration::from_secs(4)).await;

    // Request after expiration - cache miss again
    println!("7. Request after TTL expired:");
    let user = get_user_cached(&cache, &db, 1, ttl).await;
    println!("   Result: {:?}\n", user);

    // Demonstrate cache invalidation on update
    println!("8. Update user and invalidate cache:");
    let updated_user = User {
        id: 1,
        name: "Alice Updated".to_string(),
        email: "alice.new@example.com".to_string(),
    };
    update_user_cached(&cache, &mut db, updated_user).await;

    // Next request will fetch from database
    println!("\n9. Request after update (should be cache miss):");
    let user = get_user_cached(&cache, &db, 1, ttl).await;
    println!("   Result: {:?}\n", user);

    // Print final statistics
    let (hits, misses, hit_rate) = cache.stats();
    println!("=== Final Statistics ===");
    println!("Cache hits:    {}", hits);
    println!("Cache misses:  {}", misses);
    println!("Hit rate:      {:.1}%", hit_rate);

    println!("\n=== Key Takeaways ===");
    println!("1. Cache-aside reduces database load");
    println!("2. TTL prevents serving stale data forever");
    println!("3. Invalidate cache on writes for consistency");
    println!("4. First request is always slower (cache miss)");
}

// Key concepts demonstrated:
//
// 1. CACHE-ASIDE PATTERN:
//    - Check cache first
//    - On miss, fetch from source
//    - Store in cache for future requests
//
// 2. TTL (Time To Live):
//    - Automatic expiration
//    - Prevents stale data
//    - Balance between freshness and hit rate
//
// 3. CACHE INVALIDATION:
//    - Delete cache on update
//    - Ensures consistency
//    - Alternative: update cache (write-through)
//
// 4. STATISTICS:
//    - Track hit/miss ratio
//    - Monitor cache effectiveness
//    - Tune TTL based on hit rate

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_hit() {
        let cache = Cache::new();
        let db = Database::new();
        let ttl = Duration::from_secs(60);

        // First request - miss
        let _ = get_user_cached(&cache, &db, 1, ttl).await;

        // Second request - hit
        let user = get_user_cached(&cache, &db, 1, ttl).await;

        assert!(user.is_some());
        let (hits, misses, _) = cache.stats();
        assert_eq!(hits, 1);
        assert_eq!(misses, 1);
    }

    #[tokio::test]
    async fn test_cache_expiration() {
        let cache = Cache::new();
        let db = Database::new();
        let ttl = Duration::from_millis(100);

        // First request
        let _ = get_user_cached(&cache, &db, 1, ttl).await;

        // Wait for expiration
        tokio::time::sleep(Duration::from_millis(150)).await;

        // Should be a miss
        let _ = get_user_cached(&cache, &db, 1, ttl).await;

        let (_, misses, _) = cache.stats();
        assert_eq!(misses, 2);
    }
}
