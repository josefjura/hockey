use chrono::{DateTime, Duration, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
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

    pub fn refresh_expiry(&mut self) {
        self.expires_at = Utc::now() + Duration::days(7);
    }
}

/// In-memory session store
/// TODO: Replace with Redis or persistent storage for production
#[derive(Debug, Clone)]
pub struct SessionStore {
    sessions: Arc<RwLock<HashMap<String, Session>>>,
}

impl SessionStore {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create a new session
    pub async fn create_session(&self, user_id: i64, email: String, name: String) -> Session {
        let session = Session::new(user_id, email, name);
        let mut sessions = self.sessions.write().await;
        sessions.insert(session.id.clone(), session.clone());
        session
    }

    /// Get a session by ID
    pub async fn get_session(&self, session_id: &str) -> Option<Session> {
        let sessions = self.sessions.read().await;
        sessions.get(session_id).cloned()
    }

    /// Validate and get session (checks expiry)
    pub async fn validate_session(&self, session_id: &str) -> Option<Session> {
        let session = self.get_session(session_id).await?;

        if session.is_expired() {
            // Remove expired session
            self.delete_session(session_id).await;
            return None;
        }

        Some(session)
    }

    /// Refresh session expiry
    pub async fn refresh_session(&self, session_id: &str) -> Option<Session> {
        let mut sessions = self.sessions.write().await;

        if let Some(session) = sessions.get_mut(session_id) {
            if !session.is_expired() {
                session.refresh_expiry();
                return Some(session.clone());
            }
        }

        None
    }

    /// Delete a session
    pub async fn delete_session(&self, session_id: &str) {
        let mut sessions = self.sessions.write().await;
        sessions.remove(session_id);
    }

    /// Clean up expired sessions (should be called periodically)
    pub async fn cleanup_expired(&self) {
        let mut sessions = self.sessions.write().await;
        sessions.retain(|_, session| !session.is_expired());
    }

    /// Get number of active sessions
    #[allow(dead_code)]
    pub async fn session_count(&self) -> usize {
        let sessions = self.sessions.read().await;
        sessions.len()
    }
}

impl Default for SessionStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_session_creation() {
        let store = SessionStore::new();
        let session = store
            .create_session(1, "test@example.com".to_string(), "Test User".to_string())
            .await;

        assert_eq!(session.user_id, 1);
        assert_eq!(session.user_email, "test@example.com");
        assert!(!session.is_expired());
    }

    #[tokio::test]
    async fn test_session_retrieval() {
        let store = SessionStore::new();
        let session = store
            .create_session(1, "test@example.com".to_string(), "Test User".to_string())
            .await;

        let retrieved = store.get_session(&session.id).await;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().user_id, 1);
    }

    #[tokio::test]
    async fn test_session_deletion() {
        let store = SessionStore::new();
        let session = store
            .create_session(1, "test@example.com".to_string(), "Test User".to_string())
            .await;

        assert_eq!(store.session_count().await, 1);
        store.delete_session(&session.id).await;
        assert_eq!(store.session_count().await, 0);
    }

    #[tokio::test]
    async fn test_session_validation() {
        let store = SessionStore::new();
        let session = store
            .create_session(1, "test@example.com".to_string(), "Test User".to_string())
            .await;

        // Valid session
        let validated = store.validate_session(&session.id).await;
        assert!(validated.is_some());

        // Invalid session ID
        let invalid = store.validate_session("invalid-id").await;
        assert!(invalid.is_none());
    }
}
