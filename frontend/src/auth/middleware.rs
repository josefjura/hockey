use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use axum_extra::extract::CookieJar;

use super::session::Session;
use crate::app_state::AppState;

pub const SESSION_COOKIE_NAME: &str = "hockey_session";

/// Middleware that requires authentication
/// Redirects to /auth/login if not authenticated
pub async fn require_auth(
    State(state): State<AppState>,
    jar: CookieJar,
    mut request: Request,
    next: Next,
) -> Result<Response, Response> {
    // Get session cookie
    let session_id = jar
        .get(SESSION_COOKIE_NAME)
        .map(|cookie| cookie.value().to_string());

    if let Some(session_id) = session_id {
        // Validate session
        if let Some(session) = state.sessions.validate_session(&session_id).await {
            // Refresh session expiry on each request
            state.sessions.refresh_session(&session_id).await;

            // Add session to request extensions
            request.extensions_mut().insert(session);

            // Continue to the route handler
            return Ok(next.run(request).await);
        }
    }

    // No valid session - redirect to login
    Err(Redirect::to("/auth/login").into_response())
}

/// Middleware that optionally extracts session if present
/// Does not redirect - just adds session to extensions if available
pub async fn optional_auth(
    State(state): State<AppState>,
    jar: CookieJar,
    mut request: Request,
    next: Next,
) -> Response {
    let session_id = jar
        .get(SESSION_COOKIE_NAME)
        .map(|cookie| cookie.value().to_string());

    if let Some(session_id) = session_id {
        if let Some(session) = state.sessions.validate_session(&session_id).await {
            state.sessions.refresh_session(&session_id).await;
            request.extensions_mut().insert(session);
        }
    }

    next.run(request).await
}

/// Extract session from request extensions
/// Returns 401 if no session found (for use in handlers)
pub fn get_session(request: &Request) -> Result<Session, StatusCode> {
    request
        .extensions()
        .get::<Session>()
        .cloned()
        .ok_or(StatusCode::UNAUTHORIZED)
}
