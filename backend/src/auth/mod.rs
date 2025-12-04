use aide::{
    axum::{routing::post_with, ApiRouter, IntoApiResponse},
    transform::TransformOperation,
};
use axum::{extract::Extension, http::StatusCode, response::Json};
use chrono::{Duration, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::http::ApiContext;

mod jwt;
mod middleware;
mod service;

#[cfg(test)]
mod e2e_tests;

pub use jwt::*;
pub use middleware::*;
pub use service::*;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RefreshRequest {
    /// Refresh token obtained from login or previous refresh
    pub refresh_token: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct LogoutRequest {
    /// Refresh token to revoke
    pub refresh_token: String,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct LoginResponse {
    /// OAuth2 access token (JWT)
    pub access_token: String,
    /// Token type (always "Bearer")
    pub token_type: String,
    /// Access token expiration time in seconds
    pub expires_in: i64,
    /// Refresh token for obtaining new access tokens
    pub refresh_token: String,
    /// User information
    pub user_id: i64,
    pub email: String,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct RefreshResponse {
    /// New OAuth2 access token (JWT)
    pub access_token: String,
    /// Token type (always "Bearer")
    pub token_type: String,
    /// Access token expiration time in seconds
    pub expires_in: i64,
    /// New refresh token for obtaining future access tokens
    pub refresh_token: String,
}

#[derive(Debug, Serialize, JsonSchema, sqlx::FromRow)]
pub struct User {
    pub id: Option<i64>,
    pub email: String,
    pub name: Option<String>,
}

pub fn auth_routes() -> ApiRouter {
    ApiRouter::new()
        .api_route("/login", post_with(login, login_docs))
        .api_route("/refresh", post_with(refresh, refresh_docs))
        .api_route("/logout", post_with(logout, logout_docs))
}

fn login_docs(op: TransformOperation) -> TransformOperation {
    op.description("User login - Returns OAuth2-compliant JWT tokens")
        .tag("auth")
        .response::<200, Json<LoginResponse>>()
        .response_with::<401, Json<String>, _>(|res| res.description("Invalid credentials"))
}

fn refresh_docs(op: TransformOperation) -> TransformOperation {
    op.description("Token refresh - Exchange refresh token for new access and refresh tokens")
        .tag("auth")
        .response::<200, Json<RefreshResponse>>()
        .response_with::<401, Json<String>, _>(|res| {
            res.description("Invalid or expired refresh token")
        })
}

fn logout_docs(op: TransformOperation) -> TransformOperation {
    op.description("User logout - Revokes the refresh token to prevent future token refreshes")
        .tag("auth")
        .response_with::<200, Json<String>, _>(|res| res.description("Successfully logged out"))
        .response_with::<401, Json<String>, _>(|res| res.description("Invalid refresh token"))
}

async fn login(
    Extension(ctx): Extension<ApiContext>,
    Json(req): Json<LoginRequest>,
) -> impl IntoApiResponse {
    match authenticate_user(&ctx.db, &req.email, &req.password).await {
        Ok(user) => {
            let user_id = user.id.unwrap_or(0);

            // Generate JWT access and refresh tokens
            let access_token =
                match ctx
                    .jwt_manager
                    .generate_access_token(user_id, &user.email, user.name.clone())
                {
                    Ok(token) => token,
                    Err(err) => {
                        return Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(format!("Failed to generate access token: {:?}", err)),
                        ));
                    }
                };

            let refresh_token = match ctx.jwt_manager.generate_refresh_token(
                user_id,
                &user.email,
                user.name.clone(),
            ) {
                Ok(token) => token,
                Err(err) => {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(format!("Failed to generate refresh token: {:?}", err)),
                    ));
                }
            };

            // Store refresh token in database
            let expires_at =
                Utc::now() + Duration::days(ctx.config.jwt_refresh_token_duration_days);
            if let Err(err) =
                store_refresh_token(&ctx.db, &refresh_token, user_id, expires_at).await
            {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(format!("Failed to store refresh token: {:?}", err)),
                ));
            }

            let response = LoginResponse {
                access_token,
                token_type: "Bearer".to_string(),
                expires_in: ctx.config.jwt_access_token_duration_minutes * 60,
                refresh_token,
                user_id,
                email: user.email,
                name: user.name,
            };

            Ok((StatusCode::OK, Json(response)))
        }
        Err(err) => Err((StatusCode::UNAUTHORIZED, Json(format!("{:?}", err)))),
    }
}

async fn refresh(
    Extension(ctx): Extension<ApiContext>,
    Json(req): Json<RefreshRequest>,
) -> impl IntoApiResponse {
    // Validate the JWT refresh token format first
    let claims = match ctx.jwt_manager.validate_refresh_token(&req.refresh_token) {
        Ok(claims) => claims,
        Err(_) => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json("Invalid refresh token".to_string()),
            ));
        }
    };

    // Validate the refresh token exists in database and is not revoked/expired
    let user_id = match validate_refresh_token(&ctx.db, &req.refresh_token).await {
        Ok(user_id) => user_id,
        Err(_) => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json("Refresh token not found or revoked".to_string()),
            ));
        }
    };

    // Revoke the old refresh token (token rotation)
    if let Err(err) = revoke_refresh_token(&ctx.db, &req.refresh_token).await {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(format!("Failed to revoke old token: {:?}", err)),
        ));
    }

    // Generate new access token
    let access_token =
        match ctx
            .jwt_manager
            .generate_access_token(user_id, &claims.email, claims.name.clone())
        {
            Ok(token) => token,
            Err(err) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(format!("Failed to generate access token: {:?}", err)),
                ));
            }
        };

    // Generate new refresh token
    let new_refresh_token =
        match ctx
            .jwt_manager
            .generate_refresh_token(user_id, &claims.email, claims.name.clone())
        {
            Ok(token) => token,
            Err(err) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(format!("Failed to generate refresh token: {:?}", err)),
                ));
            }
        };

    // Store new refresh token in database
    let expires_at = Utc::now() + Duration::days(ctx.config.jwt_refresh_token_duration_days);
    if let Err(err) = store_refresh_token(&ctx.db, &new_refresh_token, user_id, expires_at).await {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(format!("Failed to store refresh token: {:?}", err)),
        ));
    }

    let response = RefreshResponse {
        access_token,
        token_type: "Bearer".to_string(),
        expires_in: ctx.config.jwt_access_token_duration_minutes * 60,
        refresh_token: new_refresh_token,
    };

    Ok((StatusCode::OK, Json(response)))
}

async fn logout(
    Extension(ctx): Extension<ApiContext>,
    Json(req): Json<LogoutRequest>,
) -> impl IntoApiResponse {
    // Revoke the refresh token
    match revoke_refresh_token(&ctx.db, &req.refresh_token).await {
        Ok(_) => Ok((StatusCode::OK, Json("Successfully logged out".to_string()))),
        Err(_) => Err((
            StatusCode::UNAUTHORIZED,
            Json("Invalid refresh token".to_string()),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use chrono::Duration;
    use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
    use std::sync::Arc;

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
        let password_hash = service::hash_password("testpass").unwrap();
        sqlx::query("INSERT INTO users (email, name, password_hash) VALUES (?, ?, ?)")
            .bind("test@example.com")
            .bind("Test User")
            .bind(password_hash)
            .execute(&pool)
            .await
            .expect("Failed to insert test user");

        pool
    }

    fn create_test_ctx(db: SqlitePool, jwt_manager: JwtManager) -> crate::http::ApiContext {
        crate::http::ApiContext {
            config: Arc::new(Config::test_config()),
            db,
            jwt_manager: Arc::new(jwt_manager),
        }
    }

    fn create_test_jwt_manager() -> JwtManager {
        JwtManager::new("jwt_private.pem", "jwt_public.pem").expect("Failed to create JwtManager")
    }

    #[tokio::test]
    async fn test_refresh_token_rotation() {
        let db = setup_test_db().await;
        let jwt_manager = create_test_jwt_manager();
        let user_id = 1;
        let email = "test@example.com";
        let name = Some("Test User".to_string());

        // Generate and store a refresh token
        let refresh_token = jwt_manager
            .generate_refresh_token(user_id, email, name.clone())
            .unwrap();
        let expires_at =
            Utc::now() + Duration::days(Config::test_config().jwt_refresh_token_duration_days);
        store_refresh_token(&db, &refresh_token, user_id, expires_at)
            .await
            .unwrap();

        // Verify token is valid before refresh
        let validation_before = validate_refresh_token(&db, &refresh_token).await;
        assert!(
            validation_before.is_ok(),
            "Token should be valid before refresh"
        );

        // Revoke the token (simulating what refresh endpoint does)
        revoke_refresh_token(&db, &refresh_token).await.unwrap();

        // Verify old token is revoked
        let validation_after = validate_refresh_token(&db, &refresh_token).await;
        assert!(
            validation_after.is_err(),
            "Token should be revoked after use"
        );
    }

    #[tokio::test]
    async fn test_refresh_validates_token_type() {
        let jwt_manager = create_test_jwt_manager();
        let user_id = 1;
        let email = "test@example.com";
        let name = Some("Test User".to_string());

        // Generate an ACCESS token (not refresh)
        let access_token = jwt_manager
            .generate_access_token(user_id, email, name.clone())
            .unwrap();

        // Verify JWT manager rejects access token as refresh token
        assert!(jwt_manager.validate_refresh_token(&access_token).is_err());
    }

    #[tokio::test]
    async fn test_refresh_validates_database_token() {
        let db = setup_test_db().await;
        let jwt_manager = create_test_jwt_manager();

        // Try to validate a token that doesn't exist in database
        let fake_token = jwt_manager
            .generate_refresh_token(999, "fake@example.com", None)
            .unwrap();

        assert!(validate_refresh_token(&db, &fake_token).await.is_err());
    }
}
