use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use maud::Markup;

/// Application error types
///
/// This enum represents all possible error conditions in the application.
/// It implements IntoResponse to provide consistent error handling across routes.
#[derive(Debug)]
#[allow(dead_code)]
pub enum AppError {
    /// Entity not found (404)
    NotFound { entity: String, id: Option<i64> },
    /// Database error (500)
    Database {
        source: sqlx::Error,
        context: String,
    },
    /// Validation error (400)
    Validation { message: String },
    /// Unauthorized access (401)
    Unauthorized,
    /// CSRF token validation failed (403)
    Csrf,
    /// Business logic error (400)
    BusinessLogic { message: String },
}

#[allow(dead_code)]
impl AppError {
    /// Create a not found error
    pub fn not_found(entity: impl Into<String>) -> Self {
        Self::NotFound {
            entity: entity.into(),
            id: None,
        }
    }

    /// Create a not found error with ID
    pub fn not_found_with_id(entity: impl Into<String>, id: i64) -> Self {
        Self::NotFound {
            entity: entity.into(),
            id: Some(id),
        }
    }

    /// Create a database error with context
    pub fn database(source: sqlx::Error, context: impl Into<String>) -> Self {
        Self::Database {
            source,
            context: context.into(),
        }
    }

    /// Create a validation error
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation {
            message: message.into(),
        }
    }

    /// Create a business logic error
    pub fn business_logic(message: impl Into<String>) -> Self {
        Self::BusinessLogic {
            message: message.into(),
        }
    }

    /// Get the HTTP status code for this error
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound { .. } => StatusCode::NOT_FOUND,
            AppError::Database { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Validation { .. } => StatusCode::BAD_REQUEST,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::Csrf => StatusCode::FORBIDDEN,
            AppError::BusinessLogic { .. } => StatusCode::BAD_REQUEST,
        }
    }

    /// Get user-facing error message
    fn user_message(&self) -> String {
        match self {
            AppError::NotFound { entity, id } => {
                if let Some(id) = id {
                    format!("{} with id {} not found", entity, id)
                } else {
                    format!("{} not found", entity)
                }
            }
            AppError::Database { context, .. } => {
                format!("Database error while {}", context)
            }
            AppError::Validation { message } => message.clone(),
            AppError::Unauthorized => "Unauthorized access".to_string(),
            AppError::Csrf => "CSRF token validation failed".to_string(),
            AppError::BusinessLogic { message } => message.clone(),
        }
    }

    /// Get error HTML markup
    fn error_markup(&self) -> Markup {
        use maud::html;

        // Return simple error div
        // Routes can use translation-aware error components if needed
        html! {
            div class="error" style="padding: 1rem; margin: 1rem 0; color: #dc2626;" {
                (self.user_message())
            }
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();

        // Log error details (but not validation errors, they're expected)
        match &self {
            AppError::Database { source, context } => {
                tracing::error!("Database error while {}: {}", context, source);
            }
            AppError::NotFound { entity, id } => {
                tracing::debug!("Not found: {} {:?}", entity, id);
            }
            AppError::Unauthorized => {
                tracing::warn!("Unauthorized access attempt");
            }
            AppError::Csrf => {
                tracing::warn!("CSRF token validation failed");
            }
            AppError::BusinessLogic { message } => {
                tracing::debug!("Business logic error: {}", message);
            }
            AppError::Validation { message } => {
                tracing::debug!("Validation error: {}", message);
            }
        }

        // Return HTML error response
        let error_html = self.error_markup();
        (status, Html(error_html.into_string())).into_response()
    }
}

// Implement From conversions for common error types

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database {
            source: err,
            context: "database operation".to_string(),
        }
    }
}

impl From<&'static str> for AppError {
    fn from(msg: &'static str) -> Self {
        AppError::Validation {
            message: msg.to_string(),
        }
    }
}

impl From<String> for AppError {
    fn from(msg: String) -> Self {
        AppError::Validation { message: msg }
    }
}

// Helper for converting nested Result types from business layer
#[allow(dead_code)]
impl AppError {
    /// Convert nested Result<T, Result<ValidationError, sqlx::Error>> to Result<T, AppError>
    ///
    /// This is useful for business layer functions that return nested Results
    pub fn from_business_result<T, E>(result: Result<T, Result<E, sqlx::Error>>) -> Result<T, Self>
    where
        E: std::fmt::Display,
    {
        match result {
            Ok(value) => Ok(value),
            Err(Ok(validation_err)) => Err(AppError::Validation {
                message: validation_err.to_string(),
            }),
            Err(Err(db_err)) => Err(AppError::Database {
                source: db_err,
                context: "business logic operation".to_string(),
            }),
        }
    }
}
