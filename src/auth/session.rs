use chrono::{DateTime, Duration, Utc};
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Session {
    pub id: String,
    #[allow(dead_code)]
    pub user_id: i64,
    pub user_email: String,
    pub user_name: String,
    #[allow(dead_code)]
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    #[allow(dead_code)]
    pub csrf_token: String,
}

impl Session {
    pub fn new(user_id: i64, user_email: String, user_name: String) -> Self {
        let now = Utc::now();
        let expires_at = now + Duration::days(7); // 7 day session

        Self {
            id: Uuid::new_v4().to_string(),
            user_id,
            user_email,
            user_name,
            created_at: now,
            expires_at,
            csrf_token: Uuid::new_v4().to_string(),
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    #[allow(dead_code)]
    pub fn refresh_expiry(&mut self) {
        self.expires_at = Utc::now() + Duration::days(7);
    }
}

/// SQLite-backed session store for production use
#[derive(Debug, Clone)]
pub struct SessionStore {
    db: SqlitePool,
}

impl SessionStore {
    pub fn new(db: SqlitePool) -> Self {
        Self { db }
    }

    /// Create a new session
    pub async fn create_session(
        &self,
        user_id: i64,
        email: String,
        name: String,
    ) -> Result<Session, sqlx::Error> {
        let session = Session::new(user_id, email, name);

        // Store in database
        sqlx::query!(
            r#"
            INSERT INTO sessions (id, user_id, user_email, user_name, csrf_token, created_at, expires_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            "#,
            session.id,
            session.user_id,
            session.user_email,
            session.user_name,
            session.csrf_token,
            session.created_at,
            session.expires_at
        )
        .execute(&self.db)
        .await?;

        Ok(session)
    }

    /// Get a session by ID
    pub async fn get_session(&self, session_id: &str) -> Option<Session> {
        let row = sqlx::query!(
            r#"
            SELECT id, user_id, user_email, user_name, csrf_token, created_at, expires_at
            FROM sessions
            WHERE id = ?1
            "#,
            session_id
        )
        .fetch_optional(&self.db)
        .await
        .ok()??;

        // Parse the TEXT timestamps into DateTime<Utc>
        let created_at = chrono::DateTime::parse_from_rfc3339(&row.created_at)
            .ok()?
            .with_timezone(&Utc);
        let expires_at = chrono::DateTime::parse_from_rfc3339(&row.expires_at)
            .ok()?
            .with_timezone(&Utc);

        Some(Session {
            id: row.id,
            user_id: row.user_id,
            user_email: row.user_email,
            user_name: row.user_name,
            csrf_token: row.csrf_token,
            created_at,
            expires_at,
        })
    }

    /// Validate and get session (checks expiry)
    pub async fn validate_session(&self, session_id: &str) -> Option<Session> {
        let session = self.get_session(session_id).await?;

        if session.is_expired() {
            // Remove expired session
            if let Err(e) = self.delete_session(session_id).await {
                tracing::error!("Failed to delete expired session {}: {}", session_id, e);
            }
            return None;
        }

        Some(session)
    }

    /// Refresh session expiry
    pub async fn refresh_session(&self, session_id: &str) -> Result<Option<Session>, sqlx::Error> {
        let session = match self.get_session(session_id).await {
            Some(s) => s,
            None => return Ok(None),
        };

        if session.is_expired() {
            return Ok(None);
        }

        let new_expires_at = Utc::now() + Duration::days(7);

        sqlx::query!(
            r#"
            UPDATE sessions
            SET expires_at = ?1
            WHERE id = ?2
            "#,
            new_expires_at,
            session_id
        )
        .execute(&self.db)
        .await?;

        Ok(Some(Session {
            expires_at: new_expires_at,
            ..session
        }))
    }

    /// Delete a session
    pub async fn delete_session(&self, session_id: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM sessions
            WHERE id = ?1
            "#,
            session_id
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }

    /// Clean up expired sessions (should be called periodically)
    pub async fn cleanup_expired(&self) -> Result<(), sqlx::Error> {
        let now = Utc::now();
        sqlx::query!(
            r#"
            DELETE FROM sessions
            WHERE expires_at < ?1
            "#,
            now
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }

    /// Get number of active sessions
    #[allow(dead_code)]
    pub async fn session_count(&self) -> usize {
        let result = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM sessions
            "#
        )
        .fetch_one(&self.db)
        .await;

        result.map(|r| r.count as usize).unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;

    #[sqlx::test(migrations = "./migrations", fixtures("users"))]
    async fn test_session_creation(pool: SqlitePool) {
        let store = SessionStore::new(pool);
        let session = store
            .create_session(1, "test@example.com".to_string(), "Test User".to_string())
            .await
            .expect("Failed to create session");

        assert_eq!(session.user_id, 1);
        assert_eq!(session.user_email, "test@example.com");
        assert!(!session.is_expired());
    }

    #[sqlx::test(migrations = "./migrations", fixtures("users"))]
    async fn test_session_retrieval(pool: SqlitePool) {
        let store = SessionStore::new(pool);
        let session = store
            .create_session(1, "test@example.com".to_string(), "Test User".to_string())
            .await
            .expect("Failed to create session");

        let retrieved = store.get_session(&session.id).await;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().user_id, 1);
    }

    #[sqlx::test(migrations = "./migrations", fixtures("users"))]
    async fn test_session_deletion(pool: SqlitePool) {
        let store = SessionStore::new(pool);
        let session = store
            .create_session(1, "test@example.com".to_string(), "Test User".to_string())
            .await
            .expect("Failed to create session");

        assert_eq!(store.session_count().await, 1);
        store
            .delete_session(&session.id)
            .await
            .expect("Failed to delete session");
        assert_eq!(store.session_count().await, 0);
    }

    #[sqlx::test(migrations = "./migrations", fixtures("users"))]
    async fn test_session_validation(pool: SqlitePool) {
        let store = SessionStore::new(pool);
        let session = store
            .create_session(1, "test@example.com".to_string(), "Test User".to_string())
            .await
            .expect("Failed to create session");

        // Valid session
        let validated = store.validate_session(&session.id).await;
        assert!(validated.is_some());

        // Invalid session ID
        let invalid = store.validate_session("invalid-id").await;
        assert!(invalid.is_none());
    }
}
