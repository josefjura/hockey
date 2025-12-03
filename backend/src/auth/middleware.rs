use axum::{
    extract::{FromRequestParts, Request},
    http::{header::AUTHORIZATION, request::Parts, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use std::sync::Arc;

use super::jwt::{Claims, JwtManager};

/// Authenticated user information extracted from JWT token
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: i64,
    pub email: String,
    pub name: Option<String>,
}

impl AuthUser {
    /// Create an AuthUser from JWT claims
    pub fn from_claims(claims: Claims) -> Result<Self, AuthError> {
        let user_id = claims
            .sub
            .parse::<i64>()
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(Self {
            user_id,
            email: claims.email,
            name: claims.name,
        })
    }
}

/// Errors that can occur during authentication
#[derive(Debug, Serialize)]
pub enum AuthError {
    #[serde(rename = "missing_token")]
    MissingToken,
    #[serde(rename = "invalid_token")]
    InvalidToken,
    #[serde(rename = "token_expired")]
    TokenExpired,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::MissingToken => (
                StatusCode::UNAUTHORIZED,
                "Missing authorization header".to_string(),
            ),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token".to_string()),
            AuthError::TokenExpired => (StatusCode::UNAUTHORIZED, "Token expired".to_string()),
        };

        (status, Json(message)).into_response()
    }
}

/// Extract and validate Bearer token from Authorization header
fn extract_bearer_token(auth_header: &str) -> Result<&str, AuthError> {
    if !auth_header.starts_with("Bearer ") {
        return Err(AuthError::InvalidToken);
    }

    let token = auth_header.trim_start_matches("Bearer ");
    if token.is_empty() {
        return Err(AuthError::InvalidToken);
    }

    Ok(token)
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            // Extract JWT manager from extensions
            let jwt_manager = parts
                .extensions
                .get::<Arc<JwtManager>>()
                .ok_or(AuthError::InvalidToken)?;

            // Extract Authorization header
            let auth_header = parts
                .headers
                .get(AUTHORIZATION)
                .and_then(|h| h.to_str().ok())
                .ok_or(AuthError::MissingToken)?;

            // Extract Bearer token
            let token = extract_bearer_token(auth_header)?;

            // Validate the token as an access token
            let claims = jwt_manager
                .validate_access_token(token)
                .map_err(|e| match e {
                    super::jwt::JwtManagerError::TokenExpired => AuthError::TokenExpired,
                    _ => AuthError::InvalidToken,
                })?;

            // Convert claims to AuthUser
            AuthUser::from_claims(claims)
        }
    }
}

/// Middleware layer that validates JWT tokens on all requests
/// This can be applied to route groups to enforce authentication
pub async fn require_auth(mut request: Request, next: Next) -> Response {
    // Extract JWT manager from extensions
    let jwt_manager = match request.extensions().get::<Arc<JwtManager>>() {
        Some(manager) => manager.clone(),
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("JWT manager not configured"),
            )
                .into_response()
        }
    };

    // Extract Authorization header
    let auth_header = match request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
    {
        Some(header) => header,
        None => {
            return AuthError::MissingToken.into_response();
        }
    };

    // Extract Bearer token
    let token = match extract_bearer_token(auth_header) {
        Ok(t) => t,
        Err(e) => {
            return e.into_response();
        }
    };

    // Validate the token as an access token
    let claims = match jwt_manager.validate_access_token(token) {
        Ok(claims) => claims,
        Err(e) => {
            let error = match e {
                super::jwt::JwtManagerError::TokenExpired => AuthError::TokenExpired,
                _ => AuthError::InvalidToken,
            };
            return error.into_response();
        }
    };

    // Convert claims to AuthUser and add to request extensions
    let auth_user = match AuthUser::from_claims(claims) {
        Ok(user) => user,
        Err(e) => {
            return e.into_response();
        }
    };

    // Add the authenticated user to request extensions
    // so handlers can access it if they need it
    request.extensions_mut().insert(auth_user);

    // Continue to the next middleware/handler
    next.run(request).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        routing::get,
        Extension, Router,
    };
    use tower::ServiceExt;

    use crate::auth::jwt::JwtManager;

    fn create_test_jwt_manager() -> JwtManager {
        JwtManager::new("jwt_private.pem", "jwt_public.pem").expect("Failed to create JwtManager")
    }

    async fn protected_handler(user: AuthUser) -> String {
        format!("Hello, {}!", user.email)
    }

    #[test]
    fn test_extract_bearer_token() {
        // Valid bearer token
        assert!(extract_bearer_token("Bearer abc123").is_ok());
        assert_eq!(extract_bearer_token("Bearer abc123").unwrap(), "abc123");

        // Invalid formats
        assert!(extract_bearer_token("abc123").is_err());
        assert!(extract_bearer_token("Bearer").is_err());
        assert!(extract_bearer_token("Bearer ").is_err());
        assert!(extract_bearer_token("Basic abc123").is_err());
    }

    #[tokio::test]
    async fn test_auth_user_from_claims() {
        let claims = Claims {
            sub: "42".to_string(),
            email: "test@example.com".to_string(),
            name: Some("Test User".to_string()),
            exp: 0,
            iat: 0,
            token_type: "access".to_string(),
        };

        let user = AuthUser::from_claims(claims).unwrap();
        assert_eq!(user.user_id, 42);
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.name, Some("Test User".to_string()));
    }

    #[tokio::test]
    async fn test_auth_user_invalid_user_id() {
        let claims = Claims {
            sub: "not-a-number".to_string(),
            email: "test@example.com".to_string(),
            name: None,
            exp: 0,
            iat: 0,
            token_type: "access".to_string(),
        };

        assert!(AuthUser::from_claims(claims).is_err());
    }

    #[tokio::test]
    async fn test_middleware_missing_authorization_header() {
        let jwt_manager = Arc::new(create_test_jwt_manager());
        let app = Router::new()
            .route("/protected", get(protected_handler))
            .layer(Extension(jwt_manager));

        let request = Request::builder()
            .uri("/protected")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_middleware_invalid_bearer_format() {
        let jwt_manager = Arc::new(create_test_jwt_manager());
        let app = Router::new()
            .route("/protected", get(protected_handler))
            .layer(Extension(jwt_manager));

        let request = Request::builder()
            .uri("/protected")
            .header("Authorization", "InvalidFormat token123")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_middleware_invalid_token() {
        let jwt_manager = Arc::new(create_test_jwt_manager());
        let app = Router::new()
            .route("/protected", get(protected_handler))
            .layer(Extension(jwt_manager));

        let request = Request::builder()
            .uri("/protected")
            .header("Authorization", "Bearer invalid.token.here")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_middleware_valid_token() {
        let jwt_manager = create_test_jwt_manager();
        let token = jwt_manager
            .generate_access_token(1, "test@example.com", Some("Test User".to_string()))
            .unwrap();

        let app = Router::new()
            .route("/protected", get(protected_handler))
            .layer(Extension(Arc::new(jwt_manager)));

        let request = Request::builder()
            .uri("/protected")
            .header("Authorization", format!("Bearer {}", token))
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        assert_eq!(body_str, "Hello, test@example.com!");
    }

    #[tokio::test]
    async fn test_middleware_refresh_token_rejected() {
        let jwt_manager = create_test_jwt_manager();
        // Generate a REFRESH token (not access)
        let token = jwt_manager
            .generate_refresh_token(1, "test@example.com", Some("Test User".to_string()))
            .unwrap();

        let app = Router::new()
            .route("/protected", get(protected_handler))
            .layer(Extension(Arc::new(jwt_manager)));

        let request = Request::builder()
            .uri("/protected")
            .header("Authorization", format!("Bearer {}", token))
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        // Should be rejected because refresh tokens can't be used for API access
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_require_auth_layer_rejects_unauthenticated() {
        let jwt_manager = Arc::new(create_test_jwt_manager());

        async fn test_handler() -> String {
            "Success".to_string()
        }

        let app = Router::new()
            .route("/protected", get(test_handler))
            .layer(axum::middleware::from_fn(require_auth))
            .layer(Extension(jwt_manager));

        let request = Request::builder()
            .uri("/protected")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_require_auth_layer_accepts_authenticated() {
        let jwt_manager = create_test_jwt_manager();
        let token = jwt_manager
            .generate_access_token(1, "test@example.com", Some("Test User".to_string()))
            .unwrap();

        async fn test_handler() -> String {
            "Success".to_string()
        }

        let app = Router::new()
            .route("/protected", get(test_handler))
            .layer(axum::middleware::from_fn(require_auth))
            .layer(Extension(Arc::new(jwt_manager)));

        let request = Request::builder()
            .uri("/protected")
            .header("Authorization", format!("Bearer {}", token))
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
