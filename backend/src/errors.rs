use axum::{http::StatusCode, response::IntoResponse};
use schemars::JsonSchema;
use serde::Serialize;
use serde_json::Value;
use thiserror::Error;
use uuid::Uuid;

/// Application-specific errors with proper HTTP status codes
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Team not found with id: {id}")]
    TeamNotFound { id: i64 },

    #[error("Event not found with id: {id}")]
    EventNotFound { id: i64 },

    #[error("Match not found with id: {id}")]
    MatchNotFound { id: i64 },

    #[error("Score event not found with id: {id}")]
    ScoreEventNotFound { id: i64 },

    #[error("Player not found with id: {id}")]
    PlayerNotFound { id: i64 },

    #[error("Season not found with id: {id}")]
    SeasonNotFound { id: i64 },

    #[error("Team participation not found with id: {id}")]
    TeamParticipationNotFound { id: i64 },

    #[error("Player contract not found with id: {id}")]
    PlayerContractNotFound { id: i64 },

    #[error("Invalid input: {message}")]
    InvalidInput { message: String },

    #[error("Database error")]
    Database(#[from] sqlx::Error),

    #[error("Internal server error")]
    Internal(anyhow::Error),

    #[error("Unauthorized")]
    Unauthorized,
}

/// A default error response for API errors.
#[derive(Debug, Serialize, JsonSchema)]
pub struct ErrorResponse {
    /// An error message.
    pub error: String,
    /// A unique error ID.
    pub error_id: Uuid,
    /// Optional Additional error details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_details: Option<Value>,
}

impl AppError {
    pub fn team_not_found(id: i64) -> Self {
        Self::TeamNotFound { id }
    }

    pub fn event_not_found(id: i64) -> Self {
        Self::EventNotFound { id }
    }

    pub fn match_not_found(id: i64) -> Self {
        Self::MatchNotFound { id }
    }

    pub fn score_event_not_found(id: i64) -> Self {
        Self::ScoreEventNotFound { id }
    }

    pub fn player_not_found(id: i64) -> Self {
        Self::PlayerNotFound { id }
    }

    pub fn season_not_found(id: i64) -> Self {
        Self::SeasonNotFound { id }
    }

    pub fn invalid_input(message: impl Into<String>) -> Self {
        Self::InvalidInput {
            message: message.into(),
        }
    }

    pub fn unauthorized() -> Self {
        Self::Unauthorized
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message, details) = match &self {
            AppError::TeamNotFound { id } => (
                StatusCode::NOT_FOUND,
                format!("Team not found with id: {}", id),
                None,
            ),
            AppError::EventNotFound { id } => (
                StatusCode::NOT_FOUND,
                format!("Event not found with id: {}", id),
                None,
            ),
            AppError::MatchNotFound { id } => (
                StatusCode::NOT_FOUND,
                format!("Match not found with id: {}", id),
                None,
            ),
            AppError::ScoreEventNotFound { id } => (
                StatusCode::NOT_FOUND,
                format!("Score event not found with id: {}", id),
                None,
            ),
            AppError::PlayerNotFound { id } => (
                StatusCode::NOT_FOUND,
                format!("Player not found with id: {}", id),
                None,
            ),
            AppError::SeasonNotFound { id } => (
                StatusCode::NOT_FOUND,
                format!("Season not found with id: {}", id),
                None,
            ),
            AppError::TeamParticipationNotFound { id } => (
                StatusCode::NOT_FOUND,
                format!("Team participation not found with id: {}", id),
                None,
            ),
            AppError::PlayerContractNotFound { id } => (
                StatusCode::NOT_FOUND,
                format!("Player contract not found with id: {}", id),
                None,
            ),
            AppError::InvalidInput { message } => (
                StatusCode::BAD_REQUEST,
                "Invalid input".to_string(),
                Some(serde_json::json!({ "details": message })),
            ),
            AppError::Database(e) => {
                tracing::error!("Database error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                    None,
                )
            }
            AppError::Internal(e) => {
                tracing::error!("Internal error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                    None,
                )
            }
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string(), None),
        };

        let error_response = ErrorResponse {
            error: error_message,
            error_id: Uuid::new_v4(),
            error_details: details,
        };

        (status, axum::Json(error_response)).into_response()
    }
}

impl From<bcrypt::BcryptError> for AppError {
    fn from(error: bcrypt::BcryptError) -> Self {
        Self::Internal(error.into())
    }
}

impl From<anyhow::Error> for AppError {
    fn from(error: anyhow::Error) -> Self {
        Self::Internal(error)
    }
}
