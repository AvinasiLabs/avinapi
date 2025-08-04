//! error module docs

use super::response::fail;
use crate::transport::ResponseCode;
use axum::response::{IntoResponse, Response};
use std::error::Error;
use thiserror::Error;

/// Unified application error type with simplified variants
#[derive(Error, Debug)]
pub enum AppError {
    /// Validation errors (password, email, field validation, etc.)
    #[error("Validation error: {0}")]
    Validation(String),

    /// Parsing errors (UUID, datetime, JSON, etc.)
    #[error("Parsing error: {0}")]
    Parsing(String),

    /// DateTime parsing errors
    #[error("DateTime parsing error: {0}")]
    ParsingDatetime(String),

    /// Database operation errors
    #[error("Database operation failed: {0}")]
    Database(#[from] sqlx::Error),

    /// Serialization/deserialization errors
    #[error("Serialization failed: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Authentication errors (invalid credentials, tokens, etc.)
    #[error("Authentication error: {0}")]
    Authentication(String),

    /// Authorization errors (insufficient permissions, access denied)
    #[error("Authorization error: {0}")]
    Authorization(String),

    /// Resource not found errors
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Resource conflict errors (already exists, etc.)
    #[error("Resource conflict: {0}")]
    Conflict(String),

    /// Internal server errors
    #[error("Internal server error: {0}")]
    Internal(String),

    /// Configuration errors
    #[error("Configuration error: {0}")]
    Config(String),

    /// Request timeout errors
    #[error("Request timeout: {0}")]
    Timeout(String),

    /// Rate limiting errors
    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),
}

/// 1:1 mapping from AppError to ResponseCode
impl From<&AppError> for ResponseCode {
    fn from(error: &AppError) -> Self {
        match error {
            AppError::Validation(_) => ResponseCode::ValidationError,
            AppError::Parsing(_) => ResponseCode::ParsingError,
            AppError::ParsingDatetime(_) => ResponseCode::ParsingError,
            AppError::Database(_) => ResponseCode::DatabaseError,
            AppError::Serialization(_) => ResponseCode::SerializationError,
            AppError::Authentication(_) => ResponseCode::AuthenticationError,
            AppError::Authorization(_) => ResponseCode::AuthorizationError,
            AppError::NotFound(_) => ResponseCode::NotFoundError,
            AppError::Conflict(_) => ResponseCode::ConflictError,
            AppError::Internal(_) => ResponseCode::InternalError,
            AppError::Config(_) => ResponseCode::ConfigError,
            AppError::Timeout(_) => ResponseCode::TimeoutError,
            AppError::RateLimit(_) => ResponseCode::RateLimitError,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let error_message = self.to_string();
        let response_code = ResponseCode::from(&self);

        // Enhanced error logging with context
        tracing::error!(
            error = %self,
            resp_msg = %error_message,
            response_code = ?response_code,
            "Request failed",
        );

        // In debug mode, also log the error chain for better debugging
        #[cfg(debug_assertions)]
        {
            let mut source = self.source();
            let mut level = 0;
            while let Some(err) = source {
                level += 1;
                tracing::debug!(
                    level = level,
                    source_error = %err,
                    "Error chain"
                );
                source = err.source();
            }
        }

        // Creates: {code: "ValidationError", message: "Password is too short", data: null}
        fail::<()>(response_code, error_message).into_response()
    }
}

// Automatic conversions from common third-party errors
impl From<uuid::Error> for AppError {
    fn from(err: uuid::Error) -> Self {
        Self::Parsing(format!("Invalid UUID format: {}", err))
    }
}

impl From<chrono::ParseError> for AppError {
    fn from(err: chrono::ParseError) -> Self {
        Self::ParsingDatetime(format!("Invalid datetime format: {}", err))
    }
}

impl From<String> for AppError {
    fn from(msg: String) -> Self {
        Self::Internal(msg)
    }
}

impl From<&str> for AppError {
    fn from(msg: &str) -> Self {
        Self::Internal(msg.to_string())
    }
}

/// Result type alias for application operations
pub type AppResult<T> = Result<T, AppError>;

// Import ValidationErrorsKind for nested error handling
use validator::ValidationErrorsKind;

impl From<validator::ValidationErrors> for AppError {
    fn from(errors: validator::ValidationErrors) -> Self {
        // Simply find the first custom message in the error tree
        if let Some(message) = find_first_custom_message(&errors) {
            return AppError::Validation(message);
        }

        // If no custom message is found (which shouldn't happen if we always provide messages),
        // return a generic error
        AppError::Validation("Invalid request parameters".to_string())
    }
}

/// Recursively find the first custom error message in the validation errors tree
fn find_first_custom_message(errors: &validator::ValidationErrors) -> Option<String> {
    // Check top-level field errors first
    for (_field_name, field_errors) in errors.field_errors() {
        for error in field_errors {
            if let Some(ref message) = error.message {
                return Some(message.to_string());
            }
        }
    }

    // Then check nested errors
    for (_field_name, error_kind) in errors.errors() {
        match error_kind {
            ValidationErrorsKind::List(list_errors) => {
                // For arrays/vectors with nested validation
                for (_index, nested_errors) in list_errors {
                    // nested_errors is a &Box<ValidationErrors>, so we dereference it
                    if let Some(message) = find_first_custom_message(nested_errors) {
                        return Some(message);
                    }
                }
            }
            ValidationErrorsKind::Struct(struct_errors) => {
                // For nested structs
                if let Some(message) = find_first_custom_message(struct_errors) {
                    return Some(message);
                }
            }
            ValidationErrorsKind::Field(field_errors) => {
                // Direct field errors
                for error in field_errors {
                    if let Some(ref message) = error.message {
                        return Some(message.to_string());
                    }
                }
            }
        }
    }

    None
}
