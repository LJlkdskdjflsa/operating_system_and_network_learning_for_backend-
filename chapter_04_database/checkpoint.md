# Chapter 4 Checkpoint

## Self-Assessment Questions

### SQL & Database Access (Lab 1-2)

1. **What is the N+1 query problem?**
   - Fetching related data in a loop
   - Each iteration makes a new query
   - Solution: JOIN or batch fetching

2. **Why use connection pooling?**
   - Creating connections is expensive (TCP handshake, auth)
   - Reuse existing connections
   - Limit maximum connections to database

3. **What is the difference between `query!` and `query_as!` in SQLx?**
   - `query!` returns anonymous records
   - `query_as!` maps to a specific struct
   - Both are compile-time checked

4. **How do you handle database transactions in SQLx?**
   - `pool.begin()` starts a transaction
   - `tx.commit()` commits changes
   - Rollback on drop if not committed

### Caching (Lab 3-4)

5. **What are the main caching strategies?**
   - Cache-aside: Application manages cache
   - Read-through: Cache manages reads
   - Write-through: Cache manages writes
   - Write-behind: Async writes to database

6. **What is cache invalidation and why is it hard?**
   - Keeping cache consistent with source
   - "Two hard problems: cache invalidation and naming things"
   - Strategies: TTL, event-driven, version-based

7. **When should you NOT use caching?**
   - Data changes frequently
   - Cache hit rate is low
   - Consistency is critical
   - Data is already fast to fetch

8. **What happens when Redis is unavailable?**
   - Application should fallback to database
   - Log the failure for monitoring
   - Consider circuit breaker pattern

## Concept Quiz

### Question 1: Connection Pool Size
What happens if your connection pool is too small?
- A) Queries fail immediately
- B) Queries wait for available connections
- C) New connections are created
- D) Database crashes

<details>
<summary>Answer</summary>
B) Queries wait for available connections

With a fixed-size pool, requests queue up waiting for connections.
This can cause latency spikes under load. Pool size should be tuned
based on workload and database limits.
</details>

### Question 2: Cache-Aside Pattern
In cache-aside, when does the application write to cache?
- A) Before writing to database
- B) After writing to database
- C) After reading from database (on cache miss)
- D) Never (cache auto-populates)

<details>
<summary>Answer</summary>
C) After reading from database (on cache miss)

Cache-aside pattern:
1. Check cache first
2. On miss, read from database
3. Write result to cache
4. Return result

Writes go directly to database, cache is invalidated or updated separately.
</details>

### Question 3: TTL Strategy
You have data that changes every 5 minutes on average. What TTL would you set?
- A) 1 second
- B) 30 seconds to 1 minute
- C) 10 minutes
- D) No TTL (never expires)

<details>
<summary>Answer</summary>
B) 30 seconds to 1 minute

TTL should be shorter than update frequency to ensure reasonable freshness.
Too short = low cache hit rate, defeats caching purpose.
Too long = stale data.
30s-1min balances freshness with cache efficiency.
</details>

### Question 4: SQL Injection
Which SQLx approach prevents SQL injection?
- A) `format!("SELECT * FROM users WHERE id = {}", user_input)`
- B) `sqlx::query!("SELECT * FROM users WHERE id = $1", user_input)`
- C) Both are equally safe
- D) Neither is safe

<details>
<summary>Answer</summary>
B) `sqlx::query!("SELECT * FROM users WHERE id = $1", user_input)`

Parameterized queries ($1, $2) separate SQL from data.
The database treats parameters as values, not SQL code.
String formatting allows attackers to inject SQL.
</details>

### Question 5: Redis Data Structures
Which Redis data structure would you use for a leaderboard?
- A) String
- B) List
- C) Set
- D) Sorted Set (ZSET)

<details>
<summary>Answer</summary>
D) Sorted Set (ZSET)

Sorted Sets store members with scores:
- ZADD leaderboard 100 "player1"
- ZRANGE leaderboard 0 9 (top 10)
- ZRANK leaderboard "player1" (get rank)

Perfect for rankings, rate limiting counters, etc.
</details>

## Practical Verification

### SQLx CRUD
```bash
# Start PostgreSQL
docker run -d --name postgres-lab -e POSTGRES_PASSWORD=password -p 5432:5432 postgres:15

# Run lab
cd lab_01_sqlx_crud
cargo run

# Verify: Check that CRUD operations work
# - Create user
# - Read user
# - Update user
# - Delete user
```

### Connection Pool
```bash
# Run lab with concurrent requests
cd lab_02_connection_pool
cargo run

# Verify: Pool reuses connections
# - Watch connection count in PostgreSQL
# - Check pool statistics
```

### Redis Basics
```bash
# Start Redis
docker run -d --name redis-lab -p 6379:6379 redis:7

# Run lab
cd lab_03_redis_basics
cargo run

# Verify with redis-cli
redis-cli
> KEYS *
> GET some_key
> TTL some_key
```

### Cache Patterns
```bash
# Run lab
cd lab_04_cache_patterns
cargo run

# Verify:
# - First request: cache miss, fetches from "database"
# - Second request: cache hit, returns immediately
# - After TTL: cache miss again
```

## Key Takeaways

1. **Connection pools are essential** - Never create connections per request
2. **SQLx provides compile-time safety** - Catch SQL errors at compile time
3. **Cache strategically** - Not everything benefits from caching
4. **Plan for cache failures** - Always have a fallback
5. **Monitor cache hit rates** - Low hit rate means wasted resources
6. **TTL prevents stale data** - But too short defeats caching purpose
