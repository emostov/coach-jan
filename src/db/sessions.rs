use chrono::{TimeDelta, Utc};
use serde::Serialize;
use sqlx::sqlite::SqlitePool;
use sqlx::FromRow;
use uuid::Uuid;

use crate::error::AppResult;

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Session {
    pub id: String,
    pub user_id: i64,
    pub expires_at: String,
    pub created_at: String,
}

/// Create a new session for the given user. Generates a UUID v4 session ID
/// and sets expiration to 30 days from now.
pub async fn create_session(pool: &SqlitePool, user_id: i64) -> AppResult<Session> {
    let session_id = Uuid::new_v4().to_string();
    let now = Utc::now();
    let expires_at = (now + TimeDelta::days(30)).to_rfc3339();
    let created_at = now.to_rfc3339();

    sqlx::query(
        "INSERT INTO sessions (id, user_id, expires_at, created_at) VALUES (?, ?, ?, ?)",
    )
    .bind(&session_id)
    .bind(user_id)
    .bind(&expires_at)
    .bind(&created_at)
    .execute(pool)
    .await?;

    Ok(Session {
        id: session_id,
        user_id,
        expires_at,
        created_at,
    })
}

/// Retrieve a session by ID, but only if it has not yet expired.
/// Uses SQLite's `datetime('now')` for comparison against `expires_at`.
pub async fn get_valid_session(
    pool: &SqlitePool,
    session_id: &str,
) -> AppResult<Option<Session>> {
    let session = sqlx::query_as::<_, Session>(
        "SELECT id, user_id, expires_at, created_at FROM sessions WHERE id = ? AND expires_at > datetime('now')",
    )
    .bind(session_id)
    .fetch_optional(pool)
    .await?;

    Ok(session)
}

/// Delete a specific session by ID (logout).
pub async fn delete_session(pool: &SqlitePool, session_id: &str) -> AppResult<()> {
    sqlx::query("DELETE FROM sessions WHERE id = ?")
        .bind(session_id)
        .execute(pool)
        .await?;

    Ok(())
}

/// Delete all expired sessions. Returns the number of sessions removed.
pub async fn cleanup_expired_sessions(pool: &SqlitePool) -> AppResult<u64> {
    let result = sqlx::query("DELETE FROM sessions WHERE expires_at <= datetime('now')")
        .execute(pool)
        .await?;

    Ok(result.rows_affected())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::Row;
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

    async fn create_test_user(pool: &SqlitePool) -> i64 {
        let row = sqlx::query(
            "INSERT INTO users (email, password_hash) VALUES ('test@example.com', 'hash') RETURNING id",
        )
        .fetch_one(pool)
        .await
        .expect("create test user");
        row.get("id")
    }

    #[tokio::test]
    async fn test_create_session() {
        let pool = setup_pool().await;
        let user_id = create_test_user(&pool).await;

        let session = create_session(&pool, user_id)
            .await
            .expect("create_session should succeed");

        assert_eq!(session.user_id, user_id);
        assert!(!session.id.is_empty());
        // Session ID should be a valid UUID
        assert!(Uuid::parse_str(&session.id).is_ok());
    }

    #[tokio::test]
    async fn test_get_valid_session() {
        let pool = setup_pool().await;
        let user_id = create_test_user(&pool).await;

        let session = create_session(&pool, user_id)
            .await
            .expect("create_session should succeed");

        let found = get_valid_session(&pool, &session.id)
            .await
            .expect("get_valid_session should succeed");
        assert!(found.is_some());
        assert_eq!(found.unwrap().user_id, user_id);
    }

    #[tokio::test]
    async fn test_get_nonexistent_session_returns_none() {
        let pool = setup_pool().await;

        let found = get_valid_session(&pool, "nonexistent-id")
            .await
            .expect("should not error");
        assert!(found.is_none());
    }

    #[tokio::test]
    async fn test_delete_session() {
        let pool = setup_pool().await;
        let user_id = create_test_user(&pool).await;

        let session = create_session(&pool, user_id)
            .await
            .expect("create_session should succeed");

        delete_session(&pool, &session.id)
            .await
            .expect("delete_session should succeed");

        let found = get_valid_session(&pool, &session.id)
            .await
            .expect("get_valid_session should succeed");
        assert!(found.is_none());
    }

    #[tokio::test]
    async fn test_cleanup_expired_sessions() {
        let pool = setup_pool().await;
        let user_id = create_test_user(&pool).await;

        // Insert a session that is already expired
        let expired_id = Uuid::new_v4().to_string();
        let past = (Utc::now() - TimeDelta::days(1)).to_rfc3339();
        sqlx::query("INSERT INTO sessions (id, user_id, expires_at, created_at) VALUES (?, ?, ?, ?)")
            .bind(&expired_id)
            .bind(user_id)
            .bind(&past)
            .bind(&past)
            .execute(&pool)
            .await
            .expect("insert expired session");

        // Insert a valid session
        let valid = create_session(&pool, user_id)
            .await
            .expect("create valid session");

        let cleaned = cleanup_expired_sessions(&pool)
            .await
            .expect("cleanup should succeed");
        assert_eq!(cleaned, 1);

        // Valid session should still exist
        let found = get_valid_session(&pool, &valid.id)
            .await
            .expect("should find valid session");
        assert!(found.is_some());

        // Expired session should be gone
        let gone = get_valid_session(&pool, &expired_id)
            .await
            .expect("should not error");
        assert!(gone.is_none());
    }
}
