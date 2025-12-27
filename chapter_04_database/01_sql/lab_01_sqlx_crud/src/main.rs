//! Lab 1: SQLx CRUD Operations
//!
//! ## Goal
//! Learn to perform CRUD operations using SQLx with SQLite
//!
//! ## Requirements
//! 1. Create a User table with id, name, email fields
//! 2. Implement Create, Read, Update, Delete operations
//! 3. Use SQLx query macros for type safety
//! 4. Handle errors gracefully
//!
//! ## Expected Behavior
//! ```
//! $ cargo run
//! Creating table...
//! Creating user: Alice
//! Created user with id: 1
//! Reading user 1: Some(User { id: 1, name: "Alice", email: "alice@example.com" })
//! Updating user 1...
//! Updated user: User { id: 1, name: "Alice Smith", email: "alice@example.com" }
//! Listing all users: [...]
//! Deleting user 1...
//! User 1 after delete: None
//! ```
//!
//! ## Hints
//! - Use `sqlx::SqlitePool` for connection pool
//! - Use `query_as` to map results to structs
//! - Use `execute` for INSERT/UPDATE/DELETE
//! - SQLite uses ? for parameters (not $1)
//!
//! ## Acceptance Criteria
//! - [ ] Table is created on startup
//! - [ ] Can create new users
//! - [ ] Can read users by ID
//! - [ ] Can update existing users
//! - [ ] Can delete users
//! - [ ] Can list all users

use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use sqlx::FromRow;

// ============================================================
// TODO: Implement CRUD operations
// ============================================================

/// User model
#[derive(Debug, Clone, FromRow)]
struct User {
    id: i64,
    name: String,
    email: String,
}

/// Create request (no id)
struct CreateUser {
    name: String,
    email: String,
}

/// Initialize database schema
async fn init_db(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // TODO: Create users table if not exists
    // Columns: id (INTEGER PRIMARY KEY), name (TEXT), email (TEXT UNIQUE)
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT UNIQUE NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    println!("Database initialized");
    Ok(())
}

/// Create a new user
async fn create_user(pool: &SqlitePool, user: CreateUser) -> Result<User, sqlx::Error> {
    // TODO: Insert user and return the created user with id
    let result = sqlx::query("INSERT INTO users (name, email) VALUES (?, ?)")
        .bind(&user.name)
        .bind(&user.email)
        .execute(pool)
        .await?;

    let id = result.last_insert_rowid();

    Ok(User {
        id,
        name: user.name,
        email: user.email,
    })
}

/// Get user by ID
async fn get_user(pool: &SqlitePool, id: i64) -> Result<Option<User>, sqlx::Error> {
    // TODO: Select user by id, return None if not found

    sqlx::query_as::<_, User>("SELECT id, name, email FROM users WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
}

/// Get all users
async fn list_users(pool: &SqlitePool) -> Result<Vec<User>, sqlx::Error> {
    // TODO: Select all users
    sqlx::query_as::<_, User>("SELECT id, name, email FROM users ORDER BY id")
        .fetch_all(pool)
        .await
}

/// Update user
async fn update_user(
    pool: &SqlitePool,
    id: i64,
    name: String,
) -> Result<Option<User>, sqlx::Error> {
    // TODO: Update user name, return updated user
    let result = sqlx::query("UPDATE users SET name = ? WHERE id = ?")
        .bind(&name)
        .bind(id)
        .execute(pool)
        .await?;
    if result.rows_affected() == 0 {
        return Ok(None);
    }

    get_user(pool, id).await
}

/// Delete user
async fn delete_user(pool: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
    // TODO: Delete user, return true if deleted
    let result = sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Implement
    println!("SQLx CRUD Demo\n");

    // 1. Create in-memory SQLite pool
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite::memory:")
        .await?;

    // 2. Initialize database
    println!("=== Initializing Database ===");
    init_db(&pool).await?;
    // 3. Demonstrate CRUD operations
    println!("\n=== CREATE ===");
    let alice = create_user(
        &pool,
        CreateUser {
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        },
    )
    .await?;
    println!("Created user: {:?}", alice);
    let bob = create_user(
        &pool,
        CreateUser {
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
        },
    )
    .await?;
    println!("Created user: {:?}", bob);

    println!("\n=== READ ===");
    let user = get_user(&pool, alice.id).await?;
    println!("Get user {}: {:?}", alice.id, user);

    let nonexistent = get_user(&pool, 999).await?;
    println!("Get user 999: {:?}", nonexistent);

    println!("\n=== LIST ===");
    let users = list_users(&pool).await?;
    println!("All users:");
    for user in &users {
        println!("  - {:?}", user);
    }

    println!("\n=== UPDATE ===");
    let updated = update_user(&pool, alice.id, "Alice Smith".to_string()).await?;
    println!("Updated user: {:?}", updated);

    let user = get_user(&pool, alice.id).await?;
    println!("After update: {:?}", user);

    println!("\n=== DELETE ===");
    let deleted = delete_user(&pool, bob.id).await?;
    println!("Deleted user {}: {}", bob.id, deleted);

    let user = get_user(&pool, bob.id).await?;
    println!("After delete: {:?}", user);

    println!("\n=== FINAL STATE ===");
    let users = list_users(&pool).await?;
    println!("Remaining users:");
    for user in &users {
        println!("  - {:?}", user);
    }

    println!("\n=== ERROR HANDLING ===");
    match create_user(
        &pool,
        CreateUser {
            name: "Alice Clone".to_string(),
            email: "alice@example.com".to_string(),
        },
    )
    .await
    {
        Ok(user) => println!("Created: {:?}", user),
        Err(e) => println!("Error (expected - duplicate email): {}", e),
    }

    println!("\nDemo complete!");
    Ok(())
}
