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
mod service;

pub use jwt::*;
pub use service::*;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
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

#[derive(Debug, Serialize, JsonSchema, sqlx::FromRow)]
pub struct User {
    pub id: Option<i64>,
    pub email: String,
    pub name: Option<String>,
}

pub fn auth_routes() -> ApiRouter {
    ApiRouter::new().api_route("/login", post_with(login, login_docs))
}

fn login_docs(op: TransformOperation) -> TransformOperation {
    op.description("User login - Returns OAuth2-compliant JWT tokens")
        .tag("auth")
        .response::<200, Json<LoginResponse>>()
        .response_with::<401, Json<String>, _>(|res| {
            res.description("Invalid credentials")
        })
}

async fn login(
    Extension(ctx): Extension<ApiContext>,
    Json(req): Json<LoginRequest>,
) -> impl IntoApiResponse {
    match authenticate_user(&ctx.db, &req.email, &req.password).await {
        Ok(user) => {
            let user_id = user.id.unwrap_or(0);

            // Generate JWT access and refresh tokens
            let access_token = match ctx.jwt_manager.generate_access_token(
                user_id,
                &user.email,
                user.name.clone(),
            ) {
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

            // Store refresh token in database (expires in 7 days)
            let expires_at = Utc::now() + Duration::days(7);
            if let Err(err) = store_refresh_token(&ctx.db, &refresh_token, user_id, expires_at).await {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(format!("Failed to store refresh token: {:?}", err)),
                ));
            }

            let response = LoginResponse {
                access_token,
                token_type: "Bearer".to_string(),
                expires_in: 900, // 15 minutes in seconds
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
