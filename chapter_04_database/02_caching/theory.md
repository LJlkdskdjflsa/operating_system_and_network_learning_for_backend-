# Caching

## Overview

Caching is storing frequently accessed data in a fast storage layer to reduce latency and database load. Understanding caching patterns is essential for building scalable backend systems.

## Why Cache?

```
Without cache:
Client -> Server -> Database (slow, ~10-100ms)

With cache:
Client -> Server -> Cache (fast, ~1ms) -> Database (on miss)
```

Benefits:
- **Lower latency**: Memory is faster than disk
- **Reduced database load**: Fewer queries hit the database
- **Cost savings**: Less database capacity needed
- **Better user experience**: Faster responses

## Caching Layers

```
┌─────────────────────────────────────────────────┐
│                   Client                        │
├─────────────────────────────────────────────────┤
│              Browser Cache (L1)                 │
├─────────────────────────────────────────────────┤
│                CDN Cache (L2)                   │
├─────────────────────────────────────────────────┤
│            Application Cache (L3)               │
│         (Redis, Memcached, In-memory)           │
├─────────────────────────────────────────────────┤
│             Database Cache (L4)                 │
│          (Query cache, buffer pool)             │
├─────────────────────────────────────────────────┤
│                  Database                       │
└─────────────────────────────────────────────────┘
```

## Caching Strategies

### 1. Cache-Aside (Lazy Loading)

Application manages cache explicitly.

```rust
async fn get_user(id: i64) -> User {
    // 1. Check cache
    if let Some(user) = cache.get(&format!("user:{}", id)).await {
        return user;  // Cache hit
    }

    // 2. Cache miss - fetch from database
    let user = db.get_user(id).await;

    // 3. Store in cache
    cache.set(&format!("user:{}", id), &user, TTL).await;

    user
}
```

**Pros**: Simple, only caches what's needed
**Cons**: Initial request is slow (cache miss)

### 2. Read-Through

Cache handles reads automatically.

```
Application -> Cache -> Database
                  ↑
          (Cache fetches on miss)
```

**Pros**: Simpler application code
**Cons**: Requires cache provider support

### 3. Write-Through

Writes go through cache to database.

```rust
async fn update_user(user: User) {
    // Write to cache (cache writes to DB)
    cache.write_through(&format!("user:{}", user.id), &user).await;
}
```

**Pros**: Cache always consistent
**Cons**: Higher write latency

### 4. Write-Behind (Write-Back)

Cache writes to database asynchronously.

```rust
async fn update_user(user: User) {
    // Write to cache immediately
    cache.set(&format!("user:{}", user.id), &user).await;

    // Cache flushes to DB periodically or on eviction
}
```

**Pros**: Fast writes
**Cons**: Risk of data loss, eventual consistency

### 5. Refresh-Ahead

Proactively refresh cache before expiration.

```rust
// Background task
loop {
    for key in hot_keys {
        if cache.ttl(key) < threshold {
            let value = db.get(key).await;
            cache.set(key, value, TTL).await;
        }
    }
    sleep(Duration::from_secs(60)).await;
}
```

**Pros**: Prevents cache stampede
**Cons**: More complex, may refresh unused data

## Cache Invalidation

> "There are only two hard things in Computer Science: cache invalidation and naming things." - Phil Karlton

### Time-Based (TTL)

```rust
// Set with expiration
cache.set_ex("user:123", user, 300).await;  // 5 minutes

// Redis command
SET user:123 "{...}" EX 300
```

### Event-Based

```rust
async fn update_user(user: User) {
    db.update_user(&user).await;

    // Invalidate cache
    cache.del(&format!("user:{}", user.id)).await;
}
```

### Version-Based

```rust
let version = get_data_version();
let key = format!("user:{}:v{}", id, version);

// On update, increment version
increment_data_version();
// Old cache entries expire naturally
```

## Redis

Redis is an in-memory data structure store, commonly used for caching.

### Basic Operations

```rust
use redis::AsyncCommands;

// Connect
let client = redis::Client::open("redis://127.0.0.1/")?;
let mut con = client.get_async_connection().await?;

// String operations
con.set("key", "value").await?;
let value: String = con.get("key").await?;

// With expiration
con.set_ex("key", "value", 300).await?;  // 5 minute TTL

// Delete
con.del("key").await?;

// Check existence
let exists: bool = con.exists("key").await?;
```

### Data Structures

```rust
// Strings
con.set("user:123:name", "Alice").await?;

// Hashes (for objects)
con.hset("user:123", "name", "Alice").await?;
con.hset("user:123", "email", "alice@example.com").await?;
let name: String = con.hget("user:123", "name").await?;

// Lists (for queues)
con.lpush("queue", "task1").await?;
let task: String = con.rpop("queue").await?;

// Sets (for unique items)
con.sadd("tags:post:1", "rust").await?;
con.sadd("tags:post:1", "programming").await?;
let tags: Vec<String> = con.smembers("tags:post:1").await?;

// Sorted Sets (for rankings)
con.zadd("leaderboard", "player1", 100).await?;
con.zadd("leaderboard", "player2", 200).await?;
let top: Vec<(String, f64)> = con.zrevrange_withscores("leaderboard", 0, 9).await?;
```

### TTL Management

```rust
// Set TTL
con.expire("key", 300).await?;

// Get remaining TTL
let ttl: i64 = con.ttl("key").await?;

// Remove expiration
con.persist("key").await?;

// Set only if not exists
con.set_nx("key", "value").await?;

// Set with expiration only if not exists
con.set_ex_nx("key", "value", 300).await?;
```

## Common Patterns

### Cache Key Design

```rust
// Pattern: {entity}:{id}:{field}
"user:123"              // User object
"user:123:profile"      // User profile
"user:123:posts"        // User's posts list
"post:456:comments"     // Post comments

// Pattern: {entity}:{query_hash}
"users:list:page=1&limit=20"
"search:hash(query)"
```

### Cache Stampede Prevention

When cache expires, many requests hit database simultaneously.

```rust
// Solution 1: Locking
async fn get_with_lock(key: &str) -> Value {
    if let Some(value) = cache.get(key).await {
        return value;
    }

    // Try to acquire lock
    let lock_key = format!("lock:{}", key);
    if cache.set_nx(&lock_key, "1", 10).await {
        // Got lock, fetch from DB
        let value = db.get(key).await;
        cache.set(key, &value, TTL).await;
        cache.del(&lock_key).await;
        return value;
    }

    // Wait and retry
    sleep(Duration::from_millis(100)).await;
    get_with_lock(key).await
}

// Solution 2: Probabilistic early refresh
async fn get_with_early_refresh(key: &str) -> Value {
    if let Some((value, ttl)) = cache.get_with_ttl(key).await {
        // Probabilistically refresh before expiration
        let should_refresh = rand::random::<f64>() < (1.0 / ttl as f64);
        if should_refresh {
            tokio::spawn(refresh_cache(key.to_string()));
        }
        return value;
    }
    // Fetch from DB...
}
```

### Fallback on Cache Failure

```rust
async fn get_user(id: i64) -> Result<User, Error> {
    // Try cache first
    match cache.get(&format!("user:{}", id)).await {
        Ok(Some(user)) => return Ok(user),
        Ok(None) => { /* cache miss */ }
        Err(e) => {
            // Log but don't fail
            log::warn!("Cache error: {}", e);
        }
    }

    // Fallback to database
    db.get_user(id).await
}
```

## Cache Metrics

Important metrics to monitor:

```rust
// Hit rate
hit_rate = cache_hits / (cache_hits + cache_misses)
// Target: > 90%

// Latency
cache_latency_p95 < 5ms
db_latency_p95 < 100ms

// Memory usage
cache_memory_used / cache_memory_max

// Eviction rate
evictions_per_second
```

## When NOT to Cache

- **Frequently changing data**: Low hit rate, stale data
- **User-specific data with low reuse**: Memory waste
- **Large objects**: Memory pressure
- **Consistency-critical data**: Banking, inventory
- **Already fast queries**: Unnecessary complexity

## Summary

- **Cache-aside** is the most common pattern
- **TTL** provides automatic invalidation
- Use **Redis** for distributed caching
- **Monitor hit rates** to validate caching
- Handle **cache failures gracefully**
- Prevent **cache stampede** with locking or early refresh

## Labs

1. **Lab 3: Redis Basics** - Basic Redis operations
2. **Lab 4: Cache Patterns** - Implement cache-aside with fallback
