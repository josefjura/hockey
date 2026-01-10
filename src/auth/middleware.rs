use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use axum_extra::extract::CookieJar;

use super::session::Session;
use super::signing::verify_signed_session_id;
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
    // Get session cookie (contains signed session ID: "session_id.signature")
    let signed_session_id = jar
        .get(SESSION_COOKIE_NAME)
        .map(|cookie| cookie.value().to_string());

    if let Some(signed_session_id) = signed_session_id {
        // Verify signature and extract session ID
        if let Some(session_id) =
            verify_signed_session_id(&signed_session_id, &state.session_secret)
        {
            // Validate session
            if let Some(session) = state.sessions.validate_session(&session_id).await {
                // Refresh session expiry on each request
                if let Err(e) = state.sessions.refresh_session(&session_id).await {
                    tracing::error!("Failed to refresh session {}: {}", session_id, e);
                }

                // Add session to request extensions
                request.extensions_mut().insert(session);

                // Continue to the route handler
                return Ok(next.run(request).await);
            }
        }
    }

    // No valid session - redirect to login
    Err(Redirect::to("/auth/login").into_response())
}

/// Middleware that optionally extracts session if present
/// Does not redirect - just adds session to extensions if available
#[allow(dead_code)]
pub async fn optional_auth(
    State(state): State<AppState>,
    jar: CookieJar,
    mut request: Request,
    next: Next,
) -> Response {
    let signed_session_id = jar
        .get(SESSION_COOKIE_NAME)
        .map(|cookie| cookie.value().to_string());

    if let Some(signed_session_id) = signed_session_id {
        // Verify signature and extract session ID
        if let Some(session_id) =
            verify_signed_session_id(&signed_session_id, &state.session_secret)
        {
            if let Some(session) = state.sessions.validate_session(&session_id).await {
                if let Err(e) = state.sessions.refresh_session(&session_id).await {
                    tracing::error!("Failed to refresh session {}: {}", session_id, e);
                }
                request.extensions_mut().insert(session);
            }
        }
    }

    next.run(request).await
}

/// Extract session from request extensions
/// Returns 401 if no session found (for use in handlers)
#[allow(dead_code)]
pub fn get_session(request: &Request) -> Result<Session, StatusCode> {
    request
        .extensions()
        .get::<Session>()
        .cloned()
        .ok_or(StatusCode::UNAUTHORIZED)
}
