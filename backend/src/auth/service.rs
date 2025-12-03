use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};
use sqlx::SqlitePool;

use crate::errors::AppError;

use super::User;

pub async fn authenticate_user(
    db: &SqlitePool,
    email: &str,
    password: &str,
) -> Result<User, AppError> {
    let user = sqlx::query_as::<_, User>("SELECT id, email, name FROM users WHERE email = ?")
        .bind(email)
        .fetch_optional(db)
        .await?;

    let user = user.ok_or_else(|| AppError::unauthorized())?;

    let password_hash: String =
        sqlx::query_scalar("SELECT password_hash FROM users WHERE email = ?")
            .bind(email)
            .fetch_one(db)
            .await?;

    if !verify(password, &password_hash)? {
        return Err(AppError::unauthorized());
    }

    Ok(user)
}

/// Hashes a token using SHA-256
///
/// # Arguments
/// * `token` - Token to hash
///
/// # Returns
/// Hex-encoded SHA-256 hash of the token
fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Stores a refresh token in the database with hashing
///
/// # Arguments
/// * `db` - Database pool
/// * `token` - Raw refresh token to store (will be hashed)
/// * `user_id` - User ID associated with this token
/// * `expires_at` - Expiration timestamp
pub async fn store_refresh_token(
    db: &SqlitePool,
    token: &str,
    user_id: i64,
    expires_at: DateTime<Utc>,
) -> Result<(), AppError> {
    // Hash the token before storing using SHA-256
    let token_hash = hash_token(token);

    sqlx::query("INSERT INTO refresh_tokens (token, user_id, expires_at) VALUES (?, ?, ?)")
        .bind(token_hash)
        .bind(user_id)
        .bind(expires_at.to_rfc3339())
        .execute(db)
        .await?;

    Ok(())
}

/// Validates a refresh token and returns the user ID if valid
///
/// # Arguments
/// * `db` - Database pool
/// * `token` - Raw refresh token to validate
///
/// # Returns
/// * `Ok(user_id)` if token is valid and not expired/revoked
/// * `Err(AppError)` if token is invalid, expired, or revoked
pub async fn validate_refresh_token(db: &SqlitePool, token: &str) -> Result<i64, AppError> {
    // Hash the provided token using SHA-256
    let token_hash = hash_token(token);

    // Look up the token hash in the database
    let user_id: Option<i64> = sqlx::query_scalar(
        "SELECT user_id FROM refresh_tokens
         WHERE token = ?
         AND revoked_at IS NULL
         AND expires_at > datetime('now')",
    )
    .bind(token_hash)
    .fetch_optional(db)
    .await?;

    user_id.ok_or_else(|| AppError::unauthorized())
}

/// Revokes a refresh token
///
/// # Arguments
/// * `db` - Database pool
/// * `token` - Raw refresh token to revoke
pub async fn revoke_refresh_token(db: &SqlitePool, token: &str) -> Result<(), AppError> {
    // Hash the provided token using SHA-256
    let token_hash = hash_token(token);

    // Revoke the token if it exists and is not already revoked
    let result = sqlx::query(
        "UPDATE refresh_tokens
         SET revoked_at = datetime('now')
         WHERE token = ? AND revoked_at IS NULL",
    )
    .bind(token_hash)
    .execute(db)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::unauthorized());
    }

    Ok(())
}

/// Revokes all refresh tokens for a specific user
///
/// # Arguments
/// * `db` - Database pool
/// * `user_id` - User ID whose tokens should be revoked
pub async fn revoke_all_user_tokens(db: &SqlitePool, user_id: i64) -> Result<(), AppError> {
    sqlx::query(
        "UPDATE refresh_tokens SET revoked_at = datetime('now')
         WHERE user_id = ? AND revoked_at IS NULL",
    )
    .bind(user_id)
    .execute(db)
    .await?;

    Ok(())
}

/// Cleans up expired refresh tokens
///
/// Removes all tokens that have expired, regardless of revocation status
///
/// # Arguments
/// * `db` - Database pool
///
/// # Returns
/// Number of tokens deleted
pub async fn cleanup_expired_tokens(db: &SqlitePool) -> Result<u64, AppError> {
    let result = sqlx::query("DELETE FROM refresh_tokens WHERE expires_at <= datetime('now')")
        .execute(db)
        .await?;

    Ok(result.rows_affected())
}

pub fn hash_password(password: &str) -> Result<String, AppError> {
    hash(password, DEFAULT_COST).map_err(|e| AppError::Internal(e.into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;
    use sqlx::sqlite::SqlitePoolOptions;

    #[test]
    fn test_hash_password() {
        let password = "test123";
        let hash = hash_password(password).unwrap();
        assert!(verify(password, &hash).unwrap());
    }

    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .expect("Failed to create in-memory database");

        // Create users table
        sqlx::query(
            "CREATE TABLE users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                email TEXT NOT NULL UNIQUE,
                name TEXT,
                password_hash TEXT NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .expect("Failed to create users table");

        // Create refresh_tokens table
        sqlx::query(
            "CREATE TABLE refresh_tokens (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                token TEXT NOT NULL UNIQUE,
                user_id INTEGER NOT NULL,
                expires_at DATETIME NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                revoked_at DATETIME,
                FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
            )",
        )
        .execute(&pool)
        .await
        .expect("Failed to create refresh_tokens table");

        // Insert a test user
        let password_hash = hash_password("testpass").unwrap();
        sqlx::query("INSERT INTO users (email, name, password_hash) VALUES (?, ?, ?)")
            .bind("test@example.com")
            .bind("Test User")
            .bind(password_hash)
            .execute(&pool)
            .await
            .expect("Failed to insert test user");

        pool
    }

    #[tokio::test]
    async fn test_store_refresh_token() {
        let db = setup_test_db().await;
        let token = "test_token_123";
        let user_id = 1;
        let expires_at = Utc::now() + Duration::days(7);

        let result = store_refresh_token(&db, token, user_id, expires_at).await;
        assert!(result.is_ok());

        // Verify token was stored (hashed)
        let stored_token: String =
            sqlx::query_scalar("SELECT token FROM refresh_tokens WHERE user_id = ?")
                .bind(user_id)
                .fetch_one(&db)
                .await
                .unwrap();

        // The stored token should be hashed with SHA-256, not the raw token
        assert_ne!(stored_token, token);
        assert_eq!(stored_token, hash_token(token));
    }

    #[tokio::test]
    async fn test_validate_refresh_token_valid() {
        let db = setup_test_db().await;
        let token = "valid_token_456";
        let user_id = 1;
        let expires_at = Utc::now() + Duration::days(7);

        store_refresh_token(&db, token, user_id, expires_at)
            .await
            .unwrap();

        let result = validate_refresh_token(&db, token).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), user_id);
    }

    #[tokio::test]
    async fn test_validate_refresh_token_invalid() {
        let db = setup_test_db().await;
        let token = "invalid_token_789";

        let result = validate_refresh_token(&db, token).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validate_refresh_token_expired() {
        let db = setup_test_db().await;
        let token = "expired_token_101";
        let user_id = 1;
        let expires_at = Utc::now() - Duration::days(1); // Expired

        store_refresh_token(&db, token, user_id, expires_at)
            .await
            .unwrap();

        let result = validate_refresh_token(&db, token).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validate_refresh_token_revoked() {
        let db = setup_test_db().await;
        let token = "revoked_token_202";
        let user_id = 1;
        let expires_at = Utc::now() + Duration::days(7);

        store_refresh_token(&db, token, user_id, expires_at)
            .await
            .unwrap();

        // Revoke the token
        revoke_refresh_token(&db, token).await.unwrap();

        // Validation should fail
        let result = validate_refresh_token(&db, token).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_revoke_refresh_token() {
        let db = setup_test_db().await;
        let token = "to_revoke_303";
        let user_id = 1;
        let expires_at = Utc::now() + Duration::days(7);

        store_refresh_token(&db, token, user_id, expires_at)
            .await
            .unwrap();

        let result = revoke_refresh_token(&db, token).await;
        assert!(result.is_ok());

        // Verify token is revoked
        let revoked_at: Option<String> =
            sqlx::query_scalar("SELECT revoked_at FROM refresh_tokens WHERE user_id = ?")
                .bind(user_id)
                .fetch_one(&db)
                .await
                .unwrap();

        assert!(revoked_at.is_some());
    }

    #[tokio::test]
    async fn test_revoke_all_user_tokens() {
        let db = setup_test_db().await;
        let user_id = 1;
        let expires_at = Utc::now() + Duration::days(7);

        // Store multiple tokens for the user
        store_refresh_token(&db, "token1", user_id, expires_at)
            .await
            .unwrap();
        store_refresh_token(&db, "token2", user_id, expires_at)
            .await
            .unwrap();
        store_refresh_token(&db, "token3", user_id, expires_at)
            .await
            .unwrap();

        let result = revoke_all_user_tokens(&db, user_id).await;
        assert!(result.is_ok());

        // Verify all tokens are revoked
        let revoked_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM refresh_tokens
             WHERE user_id = ? AND revoked_at IS NOT NULL",
        )
        .bind(user_id)
        .fetch_one(&db)
        .await
        .unwrap();

        assert_eq!(revoked_count, 3);
    }

    #[tokio::test]
    async fn test_cleanup_expired_tokens() {
        let db = setup_test_db().await;
        let user_id = 1;

        // Store expired tokens
        let expired_time = Utc::now() - Duration::days(1);
        store_refresh_token(&db, "expired1", user_id, expired_time)
            .await
            .unwrap();
        store_refresh_token(&db, "expired2", user_id, expired_time)
            .await
            .unwrap();

        // Store a valid token
        let future_time = Utc::now() + Duration::days(7);
        store_refresh_token(&db, "valid_token", user_id, future_time)
            .await
            .unwrap();

        // Cleanup expired tokens
        let deleted = cleanup_expired_tokens(&db).await.unwrap();
        assert_eq!(deleted, 2);

        // Verify only valid token remains
        let remaining_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM refresh_tokens")
            .fetch_one(&db)
            .await
            .unwrap();

        assert_eq!(remaining_count, 1);
    }
}
