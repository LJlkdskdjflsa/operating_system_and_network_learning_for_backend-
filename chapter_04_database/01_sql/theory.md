# SQL & Database Access in Rust

## Overview

Database access is fundamental to backend development. This section covers using SQLx for type-safe, async database operations in Rust.

## Why SQLx?

SQLx is a Rust SQL toolkit that provides:
- **Compile-time checked queries** - SQL errors caught at compile time
- **Async-first** - Built for async runtimes (Tokio, async-std)
- **Database agnostic** - Supports PostgreSQL, MySQL, SQLite
- **Pure Rust** - No C dependencies for SQLite

## Basic Concepts

### Database Connections

```rust
use sqlx::postgres::PgPoolOptions;

// Single connection (not recommended for production)
let conn = PgConnection::connect("postgres://user:pass@localhost/db").await?;

// Connection pool (recommended)
let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect("postgres://user:pass@localhost/db")
    .await?;
```

### Connection Strings

```
postgres://username:password@host:port/database
sqlite:path/to/database.db
sqlite::memory:  // In-memory SQLite
mysql://username:password@host:port/database
```

## SQLx Query Macros

### `query!` - Compile-time Verified

```rust
// Returns anonymous record
let row = sqlx::query!(
    "SELECT id, name FROM users WHERE id = $1",
    user_id
)
.fetch_one(&pool)
.await?;

println!("User: {} - {}", row.id, row.name);
```

### `query_as!` - Map to Struct

```rust
#[derive(Debug)]
struct User {
    id: i64,
    name: String,
    email: String,
}

let user = sqlx::query_as!(
    User,
    "SELECT id, name, email FROM users WHERE id = $1",
    user_id
)
.fetch_one(&pool)
.await?;
```

### `query_scalar!` - Single Value

```rust
let count = sqlx::query_scalar!(
    "SELECT COUNT(*) FROM users"
)
.fetch_one(&pool)
.await?;
```

## Fetch Methods

```rust
// Fetch exactly one row (error if 0 or >1)
let user = query.fetch_one(&pool).await?;

// Fetch optional (None if no rows)
let user = query.fetch_optional(&pool).await?;

// Fetch all rows
let users = query.fetch_all(&pool).await?;

// Streaming (for large result sets)
let mut stream = query.fetch(&pool);
while let Some(row) = stream.try_next().await? {
    // Process row
}
```

## CRUD Operations

### Create

```rust
let result = sqlx::query!(
    "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id",
    name,
    email
)
.fetch_one(&pool)
.await?;

let new_id = result.id;
```

### Read

```rust
// Single record
let user = sqlx::query_as!(
    User,
    "SELECT * FROM users WHERE id = $1",
    id
)
.fetch_optional(&pool)
.await?;

// Multiple records with pagination
let users = sqlx::query_as!(
    User,
    "SELECT * FROM users ORDER BY id LIMIT $1 OFFSET $2",
    limit,
    offset
)
.fetch_all(&pool)
.await?;
```

### Update

```rust
let result = sqlx::query!(
    "UPDATE users SET name = $1 WHERE id = $2",
    new_name,
    id
)
.execute(&pool)
.await?;

let rows_affected = result.rows_affected();
```

### Delete

```rust
let result = sqlx::query!(
    "DELETE FROM users WHERE id = $1",
    id
)
.execute(&pool)
.await?;
```

## Transactions

```rust
// Start transaction
let mut tx = pool.begin().await?;

// Execute queries within transaction
sqlx::query!("INSERT INTO orders (user_id, total) VALUES ($1, $2)", user_id, total)
    .execute(&mut *tx)
    .await?;

sqlx::query!("UPDATE inventory SET quantity = quantity - $1 WHERE product_id = $2", qty, product_id)
    .execute(&mut *tx)
    .await?;

// Commit transaction
tx.commit().await?;

// If tx is dropped without commit, it automatically rolls back
```

### Transaction with Error Handling

```rust
async fn transfer_funds(
    pool: &PgPool,
    from: i64,
    to: i64,
    amount: i64,
) -> Result<(), Error> {
    let mut tx = pool.begin().await?;

    // Deduct from source
    let result = sqlx::query!(
        "UPDATE accounts SET balance = balance - $1 WHERE id = $2 AND balance >= $1",
        amount,
        from
    )
    .execute(&mut *tx)
    .await?;

    if result.rows_affected() == 0 {
        return Err(Error::InsufficientFunds);
    }

    // Add to destination
    sqlx::query!(
        "UPDATE accounts SET balance = balance + $1 WHERE id = $2",
        amount,
        to
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(())
}
```

## Connection Pooling

### Why Connection Pooling?

Creating a database connection is expensive:
1. TCP handshake
2. TLS negotiation (if encrypted)
3. Authentication
4. Connection setup

Connection pools:
- Maintain a set of open connections
- Reuse connections across requests
- Limit total connections (prevent overwhelming database)

### Pool Configuration

```rust
let pool = PgPoolOptions::new()
    // Maximum connections in the pool
    .max_connections(10)

    // Minimum connections to keep open
    .min_connections(2)

    // Maximum time to wait for a connection
    .acquire_timeout(Duration::from_secs(3))

    // Maximum connection lifetime
    .max_lifetime(Duration::from_mins(30))

    // Idle connection timeout
    .idle_timeout(Duration::from_mins(10))

    // Test connection before returning from pool
    .test_before_acquire(true)

    .connect(&database_url)
    .await?;
```

### Pool Sizing Guidelines

```
Optimal pool size ≈ (core_count * 2) + effective_spindle_count

For SSD: connections = cores * 2-4
For HDD: connections = cores * 2 + disk_count
```

Rule of thumb:
- Start small (5-10 connections)
- Monitor and adjust based on metrics
- More connections ≠ better performance

### Pool Metrics

```rust
// Get pool statistics
let pool_size = pool.size();            // Current connections
let idle_count = pool.num_idle();       // Idle connections
```

## Migrations

### Using SQLx CLI

```bash
# Install sqlx-cli
cargo install sqlx-cli

# Create migration
sqlx migrate add create_users_table

# Run migrations
sqlx migrate run

# Revert last migration
sqlx migrate revert
```

### Migration Files

```sql
-- migrations/20231215_create_users_table.sql

-- Up migration
CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_users_email ON users(email);
```

### Embedded Migrations

```rust
// Embed migrations in binary
sqlx::migrate!("./migrations")
    .run(&pool)
    .await?;
```

## Error Handling

```rust
use sqlx::Error;

match sqlx::query!("SELECT * FROM users WHERE id = $1", id)
    .fetch_one(&pool)
    .await
{
    Ok(user) => Ok(user),
    Err(Error::RowNotFound) => Err(AppError::NotFound),
    Err(Error::Database(e)) if e.is_unique_violation() => {
        Err(AppError::DuplicateEntry)
    }
    Err(e) => Err(AppError::DatabaseError(e)),
}
```

## Common Patterns

### Repository Pattern

```rust
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<User>, Error> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn create(&self, name: &str, email: &str) -> Result<User, Error> {
        sqlx::query_as!(
            User,
            "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *",
            name,
            email
        )
        .fetch_one(&self.pool)
        .await
    }
}
```

### Avoiding N+1 Queries

```rust
// BAD: N+1 queries
let posts = get_all_posts(&pool).await?;
for post in &posts {
    let author = get_user(&pool, post.author_id).await?; // N queries!
}

// GOOD: Single query with JOIN
let posts_with_authors = sqlx::query_as!(
    PostWithAuthor,
    r#"
    SELECT p.*, u.name as author_name
    FROM posts p
    JOIN users u ON p.author_id = u.id
    "#
)
.fetch_all(&pool)
.await?;
```

## Runtime vs Compile-time Queries

### Compile-time (Recommended)

```rust
// Requires DATABASE_URL at compile time
// Errors caught at compile time
let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
    .fetch_one(&pool)
    .await?;
```

### Runtime (Dynamic Queries)

```rust
// No compile-time checking
// Useful for dynamic SQL
let user: User = sqlx::query_as("SELECT * FROM users WHERE id = $1")
    .bind(id)
    .fetch_one(&pool)
    .await?;
```

## SQLite vs PostgreSQL

### SQLite (Good for Development)

```rust
// In-memory database
let pool = SqlitePool::connect("sqlite::memory:").await?;

// File-based
let pool = SqlitePool::connect("sqlite:app.db").await?;
```

### PostgreSQL (Good for Production)

```rust
let pool = PgPool::connect("postgres://user:pass@localhost/db").await?;
```

## Summary

- **SQLx** provides compile-time verified SQL queries
- **Connection pools** are essential for production
- **Transactions** ensure data consistency
- **Migrations** manage schema changes
- Use **JOINs** to avoid N+1 queries
- Handle errors gracefully with pattern matching

## Labs

1. **Lab 1: SQLx CRUD** - Implement basic CRUD operations
2. **Lab 2: Connection Pool** - Build and monitor connection pool
