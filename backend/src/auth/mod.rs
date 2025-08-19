use aide::{
    axum::{routing::post_with, ApiRouter, IntoApiResponse},
    transform::TransformOperation,
};
use axum::{extract::Extension, http::StatusCode, response::Json};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::http::ApiContext;

mod service;

pub use service::*;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct LoginResponse {
    pub user_id: i64,
    pub email: String,
    pub name: Option<String>,
    pub token: String,
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
    op.description("User login").tag("auth")
}

async fn login(
    Extension(ctx): Extension<ApiContext>,
    Json(req): Json<LoginRequest>,
) -> impl IntoApiResponse {
    match authenticate_user(&ctx.db, &req.email, &req.password).await {
        Ok(user) => {
            // Generate a simple token (in production, use JWT or proper session management)
            let token = Uuid::new_v4().to_string();

            let response = LoginResponse {
                user_id: user.id.unwrap_or(0),
                email: user.email,
                name: user.name,
                token,
            };

            Ok((StatusCode::OK, Json(response)))
        }
        Err(err) => Err((StatusCode::UNAUTHORIZED, Json(format!("{:?}", err)))),
    }
}
