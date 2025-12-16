use axum::{
    extract::State,
    response::{Html, IntoResponse, Redirect},
    Form,
};
use axum_extra::extract::{cookie::{Cookie, SameSite}, CookieJar};
use serde::Deserialize;
use sqlx::Row;

use crate::app_state::AppState;
use crate::auth::{verify_password, SESSION_COOKIE_NAME};
use crate::views::pages::auth::login_page;

#[derive(Debug, Deserialize)]
pub struct LoginForm {
    email: String,
    password: String,
    csrf_token: String,
}

/// GET /auth/login - Show login page
pub async fn login_get() -> Html<String> {
    // Generate a simple CSRF token for the form
    let csrf_token = uuid::Uuid::new_v4().to_string();

    let html = login_page(None, &csrf_token);
    Html(html.into_string())
}

/// POST /auth/login - Handle login form submission
pub async fn login_post(
    State(state): State<AppState>,
    jar: CookieJar,
    Form(form): Form<LoginForm>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    // Verify user credentials using raw query
    let user_row = sqlx::query(
        "SELECT id, email, password_hash, name FROM users WHERE email = ?"
    )
    .bind(&form.email)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error during login: {}", e);
        (
            jar.clone(),
            Html(login_page(Some("An error occurred. Please try again.".to_string()), &form.csrf_token).into_string())
        )
    })?;

    // Check if user exists
    let user_row = match user_row {
        Some(row) => row,
        None => {
            return Err((
                jar,
                Html(login_page(Some("Invalid email or password".to_string()), &form.csrf_token).into_string())
            ));
        }
    };

    // Extract user data
    let user_id: i64 = user_row.get("id");
    let user_email: String = user_row.get("email");
    let user_name: String = user_row.get("name");
    let password_hash: String = user_row.get("password_hash");

    // Verify password
    let password_valid = verify_password(&form.password, &password_hash)
        .map_err(|e| {
            tracing::error!("Password verification error: {}", e);
            (
                jar.clone(),
                Html(login_page(Some("An error occurred. Please try again.".to_string()), &form.csrf_token).into_string())
            )
        })?;

    if !password_valid {
        return Err((
            jar,
            Html(login_page(Some("Invalid email or password".to_string()), &form.csrf_token).into_string())
        ));
    }

    // Create session
    let session = state.sessions
        .create_session(user_id, user_email.clone(), user_name)
        .await;

    tracing::info!("User {} logged in successfully", user_email);

    // Set session cookie
    let session_cookie = Cookie::build((SESSION_COOKIE_NAME, session.id))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Strict)
        .max_age(time::Duration::days(7))
        .build();

    let jar = jar.add(session_cookie);

    // Redirect to dashboard
    Ok((jar, Redirect::to("/")))
}

/// POST /auth/logout - Handle logout
pub async fn logout_post(
    State(state): State<AppState>,
    jar: CookieJar,
) -> impl IntoResponse {
    // Get session cookie
    if let Some(session_cookie) = jar.get(SESSION_COOKIE_NAME) {
        // Delete session from store
        state.sessions.delete_session(session_cookie.value()).await;
        tracing::info!("User logged out");
    }

    // Remove session cookie
    let jar = jar.remove(
        Cookie::build(SESSION_COOKIE_NAME)
            .path("/")
            .build()
    );

    // Redirect to login
    (jar, Redirect::to("/auth/login"))
}
