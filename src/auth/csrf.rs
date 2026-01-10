use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

use super::Session;

/// Validates a CSRF token against the session's token
///
/// # Arguments
/// * `form_token` - The CSRF token from the form submission
/// * `session` - The current user session containing the expected token
///
/// # Returns
/// * `Ok(())` if tokens match
/// * `Err(Response)` with 403 Forbidden if tokens don't match or are missing
pub fn validate_csrf_token(form_token: &str, session: &Session) -> Result<(), impl IntoResponse> {
    if form_token.is_empty() {
        tracing::warn!("CSRF validation failed: missing token in form");
        return Err((
            StatusCode::FORBIDDEN,
            Html("<h1>403 Forbidden</h1><p>Missing CSRF token</p>"),
        ));
    }

    if form_token != session.csrf_token {
        tracing::warn!(
            "CSRF validation failed: token mismatch for user {}",
            session.user_email
        );
        return Err((
            StatusCode::FORBIDDEN,
            Html("<h1>403 Forbidden</h1><p>Invalid CSRF token</p>"),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};

    fn create_test_session(csrf_token: String) -> Session {
        Session {
            id: "test-session".to_string(),
            user_id: 1,
            user_email: "test@example.com".to_string(),
            user_name: "Test User".to_string(),
            created_at: Utc::now(),
            expires_at: Utc::now() + Duration::days(7),
            csrf_token,
        }
    }

    #[test]
    fn test_valid_csrf_token() {
        let session = create_test_session("valid-token".to_string());
        let result = validate_csrf_token("valid-token", &session);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_csrf_token() {
        let session = create_test_session("valid-token".to_string());
        let result = validate_csrf_token("wrong-token", &session);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_csrf_token() {
        let session = create_test_session("valid-token".to_string());
        let result = validate_csrf_token("", &session);
        assert!(result.is_err());
    }
}
