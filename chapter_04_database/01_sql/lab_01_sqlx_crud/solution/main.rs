//! Lab 1 Reference Answer

use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use sqlx::FromRow;

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
    // Insert and get the last insert rowid
    let result = sqlx::query(
        "INSERT INTO users (name, email) VALUES (?, ?)"
    )
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
    sqlx::query_as::<_, User>("SELECT id, name, email FROM users WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
}

/// Get all users
async fn list_users(pool: &SqlitePool) -> Result<Vec<User>, sqlx::Error> {
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
    let result = sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("SQLx CRUD Demo\n");

    // Create in-memory SQLite pool
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite::memory:")
        .await?;

    // Initialize database
    println!("=== Initializing Database ===");
    init_db(&pool).await?;

    // CREATE
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

    // READ
    println!("\n=== READ ===");
    let user = get_user(&pool, alice.id).await?;
    println!("Get user {}: {:?}", alice.id, user);

    let nonexistent = get_user(&pool, 999).await?;
    println!("Get user 999: {:?}", nonexistent);

    // LIST
    println!("\n=== LIST ===");
    let users = list_users(&pool).await?;
    println!("All users:");
    for user in &users {
        println!("  - {:?}", user);
    }

    // UPDATE
    println!("\n=== UPDATE ===");
    let updated = update_user(&pool, alice.id, "Alice Smith".to_string()).await?;
    println!("Updated user: {:?}", updated);

    // Verify update
    let user = get_user(&pool, alice.id).await?;
    println!("After update: {:?}", user);

    // DELETE
    println!("\n=== DELETE ===");
    let deleted = delete_user(&pool, bob.id).await?;
    println!("Deleted user {}: {}", bob.id, deleted);

    // Verify delete
    let user = get_user(&pool, bob.id).await?;
    println!("After delete: {:?}", user);

    // Final list
    println!("\n=== FINAL STATE ===");
    let users = list_users(&pool).await?;
    println!("Remaining users:");
    for user in &users {
        println!("  - {:?}", user);
    }

    // Demonstrate error handling
    println!("\n=== ERROR HANDLING ===");
    match create_user(
        &pool,
        CreateUser {
            name: "Alice Clone".to_string(),
            email: "alice@example.com".to_string(), // Duplicate email
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

// Key concepts demonstrated:
//
// 1. CONNECTION POOL:
//    - SqlitePoolOptions for configuration
//    - In-memory database for testing
//
// 2. SCHEMA MANAGEMENT:
//    - CREATE TABLE IF NOT EXISTS
//    - Proper column types and constraints
//
// 3. CRUD OPERATIONS:
//    - INSERT with last_insert_rowid()
//    - SELECT with fetch_optional/fetch_all
//    - UPDATE with rows_affected check
//    - DELETE with boolean return
//
// 4. ERROR HANDLING:
//    - UNIQUE constraint violation
//    - Graceful handling of not found
//
// 5. TYPE SAFETY:
//    - FromRow derive for automatic mapping
//    - query_as for type-safe results

#[cfg(test)]
mod tests {
    use super::*;

    async fn setup_pool() -> SqlitePool {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        init_db(&pool).await.unwrap();
        pool
    }

    #[tokio::test]
    async fn test_create_user() {
        let pool = setup_pool().await;

        let user = create_user(
            &pool,
            CreateUser {
                name: "Test".to_string(),
                email: "test@example.com".to_string(),
            },
        )
        .await
        .unwrap();

        assert_eq!(user.name, "Test");
        assert_eq!(user.email, "test@example.com");
        assert!(user.id > 0);
    }

    #[tokio::test]
    async fn test_get_user() {
        let pool = setup_pool().await;

        let created = create_user(
            &pool,
            CreateUser {
                name: "Test".to_string(),
                email: "test@example.com".to_string(),
            },
        )
        .await
        .unwrap();

        let found = get_user(&pool, created.id).await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Test");

        let not_found = get_user(&pool, 999).await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    async fn test_update_user() {
        let pool = setup_pool().await;

        let created = create_user(
            &pool,
            CreateUser {
                name: "Original".to_string(),
                email: "test@example.com".to_string(),
            },
        )
        .await
        .unwrap();

        let updated = update_user(&pool, created.id, "Updated".to_string())
            .await
            .unwrap();

        assert!(updated.is_some());
        assert_eq!(updated.unwrap().name, "Updated");
    }

    #[tokio::test]
    async fn test_delete_user() {
        let pool = setup_pool().await;

        let created = create_user(
            &pool,
            CreateUser {
                name: "ToDelete".to_string(),
                email: "delete@example.com".to_string(),
            },
        )
        .await
        .unwrap();

        let deleted = delete_user(&pool, created.id).await.unwrap();
        assert!(deleted);

        let found = get_user(&pool, created.id).await.unwrap();
        assert!(found.is_none());
    }
}
