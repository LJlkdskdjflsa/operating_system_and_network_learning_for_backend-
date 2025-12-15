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

    todo!("Implement init_db")
}

/// Create a new user
async fn create_user(
    pool: &SqlitePool,
    user: CreateUser,
) -> Result<User, sqlx::Error> {
    // TODO: Insert user and return the created user with id

    todo!("Implement create_user")
}

/// Get user by ID
async fn get_user(
    pool: &SqlitePool,
    id: i64,
) -> Result<Option<User>, sqlx::Error> {
    // TODO: Select user by id, return None if not found

    todo!("Implement get_user")
}

/// Get all users
async fn list_users(pool: &SqlitePool) -> Result<Vec<User>, sqlx::Error> {
    // TODO: Select all users

    todo!("Implement list_users")
}

/// Update user
async fn update_user(
    pool: &SqlitePool,
    id: i64,
    name: String,
) -> Result<Option<User>, sqlx::Error> {
    // TODO: Update user name, return updated user

    todo!("Implement update_user")
}

/// Delete user
async fn delete_user(
    pool: &SqlitePool,
    id: i64,
) -> Result<bool, sqlx::Error> {
    // TODO: Delete user, return true if deleted

    todo!("Implement delete_user")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Implement
    // 1. Create in-memory SQLite pool
    // 2. Initialize database
    // 3. Demonstrate CRUD operations

    todo!("Implement main")
}
