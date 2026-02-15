use serde::Serialize;
use sqlx::sqlite::SqlitePool;
use sqlx::FromRow;

use crate::error::{AppError, AppResult};

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub password_hash: String,
    pub created_at: String,
}

/// Create a new user with the given email and pre-hashed password.
/// Returns `AppError::Conflict` if the email already exists.
pub async fn create_user(
    pool: &SqlitePool,
    email: &str,
    password_hash: &str,
) -> AppResult<User> {
    let result = sqlx::query(
        "INSERT INTO users (email, password_hash) VALUES (?, ?) RETURNING id, email, password_hash, created_at",
    )
    .bind(email)
    .bind(password_hash)
    .fetch_one(pool)
    .await;

    match result {
        Ok(row) => {
            use sqlx::Row;
            Ok(User {
                id: row.get("id"),
                email: row.get("email"),
                password_hash: row.get("password_hash"),
                created_at: row.get("created_at"),
            })
        }
        Err(sqlx::Error::Database(db_err)) if db_err.message().contains("UNIQUE constraint") => {
            Err(AppError::Conflict(format!(
                "A user with email '{email}' already exists"
            )))
        }
        Err(e) => Err(AppError::Database(e)),
    }
}

/// Look up a user by email address. Returns `None` if not found.
pub async fn get_user_by_email(pool: &SqlitePool, email: &str) -> AppResult<Option<User>> {
    let user = sqlx::query_as::<_, User>(
        "SELECT id, email, password_hash, created_at FROM users WHERE email = ?",
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

/// Look up a user by their integer ID. Returns `None` if not found.
pub async fn get_user_by_id(pool: &SqlitePool, id: i64) -> AppResult<Option<User>> {
    let user = sqlx::query_as::<_, User>(
        "SELECT id, email, password_hash, created_at FROM users WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};

    async fn setup_pool() -> SqlitePool {
        let opts = SqliteConnectOptions::new()
            .filename(":memory:")
            .create_if_missing(true)
            .pragma("foreign_keys", "ON");

        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(opts)
            .await
            .expect("Failed to create test pool");

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to run migrations");

        pool
    }

    #[tokio::test]
    async fn test_create_and_get_user_by_email() {
        let pool = setup_pool().await;
        let user = create_user(&pool, "test@example.com", "hashed_pw")
            .await
            .expect("create_user should succeed");

        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.password_hash, "hashed_pw");
        assert!(user.id > 0);

        let found = get_user_by_email(&pool, "test@example.com")
            .await
            .expect("get_user_by_email should succeed");
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, user.id);
    }

    #[tokio::test]
    async fn test_get_user_by_id() {
        let pool = setup_pool().await;
        let user = create_user(&pool, "id-test@example.com", "hashed_pw")
            .await
            .expect("create_user should succeed");

        let found = get_user_by_id(&pool, user.id)
            .await
            .expect("get_user_by_id should succeed");
        assert!(found.is_some());
        assert_eq!(found.unwrap().email, "id-test@example.com");
    }

    #[tokio::test]
    async fn test_get_nonexistent_user_returns_none() {
        let pool = setup_pool().await;

        let by_email = get_user_by_email(&pool, "nobody@example.com")
            .await
            .expect("should not error");
        assert!(by_email.is_none());

        let by_id = get_user_by_id(&pool, 9999)
            .await
            .expect("should not error");
        assert!(by_id.is_none());
    }

    #[tokio::test]
    async fn test_duplicate_email_returns_conflict() {
        let pool = setup_pool().await;
        create_user(&pool, "dup@example.com", "hash1")
            .await
            .expect("first create should succeed");

        let result = create_user(&pool, "dup@example.com", "hash2").await;
        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::Conflict(msg) => {
                assert!(msg.contains("dup@example.com"));
            }
            other => panic!("Expected Conflict, got: {other:?}"),
        }
    }
}
