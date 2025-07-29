//! Application error types and conversions for unified error handling.
//!
//! This module implements the unified error handling pattern where all errors
//! are categorized into generic types rather than specific error codes. The
//! specific error details are preserved in the error message.

use crate::response::{ApiResponse, ResponseCode};
use thiserror::Error;
/// Application result type alias for convenience.
///
/// This type alias reduces boilerplate when working with fallible operations
/// throughout the application.
pub type AppResult<T> = Result<T, AppError>;

/// Unified application error enum.
///
/// Following the unified error handling pattern, this enum provides generic
/// error categories rather than specific error codes. Detailed error information
/// is preserved in the error message and can be accessed through the error's
/// Display implementation.
///
/// Each variant maps directly to a corresponding [`ResponseCode`] for consistent
/// API responses.
#[derive(Error, Debug)]
pub enum AppError {
    /// Input validation failed
    ///
    /// Used for request validation errors, malformed input, missing required fields, etc.
    /// Maps to [`ResponseCode::ValidationError`].
    #[error("Validation error: {0}")]
    Validation(String),

    /// Authentication failed or missing
    ///
    /// Used for missing or invalid authentication credentials.
    /// Maps to [`ResponseCode::AuthenticationError`].
    #[error("Authentication error: {0}")]
    Authentication(String),

    /// User does not have permission for this operation
    ///
    /// Used for authorization failures when user lacks required permissions.
    /// Maps to [`ResponseCode::AuthorizationError`].
    #[error("Authorization error: {0}")]
    Authorization(String),

    /// Requested resource was not found
    ///
    /// Used when a requested resource (user, item, etc.) does not exist.
    /// Maps to [`ResponseCode::NotFoundError`].
    #[error("Not found: {0}")]
    NotFound(String),

    /// Resource already exists and cannot be created again
    ///
    /// Used for duplicate resource creation attempts.
    /// Maps to [`ResponseCode::ConflictError`].
    #[error("Conflict: {0}")]
    Conflict(String),

    /// Rate limit exceeded
    ///
    /// Used when rate limiting thresholds are exceeded.
    /// Maps to [`ResponseCode::RateLimitError`].
    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),

    /// External service is temporarily unavailable
    ///
    /// Used when external dependencies are unavailable.
    /// Maps to [`ResponseCode::ServiceUnavailableError`].
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),

    /// Internal server error occurred
    ///
    /// Used for unexpected internal errors that should not be exposed to clients.
    /// Maps to [`ResponseCode::InternalError`].
    #[error("Internal error: {0}")]
    Internal(String),

    /// Request timeout
    ///
    /// Used when operations exceed their timeout duration.
    /// Maps to [`ResponseCode::TimeoutError`].
    #[error("Timeout: {0}")]
    Timeout(String),

    /// Database operation failed
    ///
    /// Used for database-specific errors like connection failures, constraint violations, etc.
    /// Maps to [`ResponseCode::DatabaseError`].
    #[error("Database error: {0}")]
    Database(String),

    /// Network or connectivity error
    ///
    /// Used for network-related errors in external service calls.
    /// Maps to [`ResponseCode::NetworkError`].
    #[error("Network error: {0}")]
    Network(String),

    /// IPFS operation failed
    ///
    /// Used for IPFS-related errors like upload/download failures.
    /// Maps to [`ResponseCode::ServiceUnavailableError`].
    #[error("IPFS error: {0}")]
    Ipfs(String),

    /// Blockchain operation failed
    ///
    /// Used for blockchain interaction errors.
    /// Maps to [`ResponseCode::ServiceUnavailableError`].
    #[error("Blockchain error: {0}")]
    Blockchain(String),

    /// Docker operation failed
    ///
    /// Used for container management errors.
    /// Maps to [`ResponseCode::InternalError`].
    #[error("Docker error: {0}")]
    Docker(String),

    /// TEE (Trusted Execution Environment) operation failed
    ///
    /// Used for TEE-related errors.
    /// Maps to [`ResponseCode::InternalError`].
    #[error("TEE error: {0}")]
    Tee(String),

    /// Configuration error
    ///
    /// Used for invalid or missing configuration.
    /// Maps to [`ResponseCode::InternalError`].
    #[error("Configuration error: {0}")]
    Configuration(String),

    /// Bad request error
    ///
    /// Used for malformed requests that don't fit validation errors.
    /// Maps to [`ResponseCode::ValidationError`].
    #[error("Bad request: {0}")]
    BadRequest(String),

    /// Wrapper for other errors
    ///
    /// Used to wrap anyhow::Error and other error types.
    /// Maps to [`ResponseCode::InternalError`].
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl AppError {
    /// Creates a validation error
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation(message.into())
    }

    /// Creates an authentication error
    pub fn authentication(message: impl Into<String>) -> Self {
        Self::Authentication(message.into())
    }

    /// Creates an authorization error
    pub fn authorization(message: impl Into<String>) -> Self {
        Self::Authorization(message.into())
    }

    /// Creates a not found error
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::NotFound(message.into())
    }

    /// Creates a conflict error
    pub fn conflict(message: impl Into<String>) -> Self {
        Self::Conflict(message.into())
    }

    /// Creates an internal error
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal(message.into())
    }

    /// Creates a database error
    pub fn database(message: impl Into<String>) -> Self {
        Self::Database(message.into())
    }

    /// Creates a service unavailable error
    pub fn service_unavailable(message: impl Into<String>) -> Self {
        Self::ServiceUnavailable(message.into())
    }

    /// Creates a timeout error
    pub fn timeout(message: impl Into<String>) -> Self {
        Self::Timeout(message.into())
    }

    /// Creates a rate limit error
    pub fn rate_limit(message: impl Into<String>) -> Self {
        Self::RateLimit(message.into())
    }

    /// Creates a network error
    pub fn network(message: impl Into<String>) -> Self {
        Self::Network(message.into())
    }

    /// Creates an IPFS error
    pub fn ipfs(message: impl Into<String>) -> Self {
        Self::Ipfs(message.into())
    }

    /// Creates a blockchain error
    pub fn blockchain(message: impl Into<String>) -> Self {
        Self::Blockchain(message.into())
    }

    /// Creates a Docker error
    pub fn docker(message: impl Into<String>) -> Self {
        Self::Docker(message.into())
    }

    /// Creates a TEE error
    pub fn tee(message: impl Into<String>) -> Self {
        Self::Tee(message.into())
    }

    /// Creates a configuration error
    pub fn configuration(message: impl Into<String>) -> Self {
        Self::Configuration(message.into())
    }

    /// Creates a bad request error
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::BadRequest(message.into())
    }
}

// Conversion from validator errors
impl From<validator::ValidationErrors> for AppError {
    fn from(errors: validator::ValidationErrors) -> Self {
        let error_messages: Vec<String> = errors
            .field_errors()
            .iter()
            .flat_map(|(field, field_errors)| {
                field_errors.iter().map(move |error| {
                    let message = error
                        .message
                        .as_ref()
                        .map(|m| m.to_string())
                        .unwrap_or_else(|| format!("Invalid value for field '{}'", field));
                    format!("{}: {}", field, message)
                })
            })
            .collect();

        Self::Validation(error_messages.join(", "))
    }
}

// Implement From<AppError> for ResponseCode to enable automatic conversion
impl From<&AppError> for ResponseCode {
    fn from(error: &AppError) -> Self {
        match error {
            AppError::Validation(_) => ResponseCode::ValidationError,
            AppError::Authentication(_) => ResponseCode::AuthenticationError,
            AppError::Authorization(_) => ResponseCode::AuthorizationError,
            AppError::NotFound(_) => ResponseCode::NotFoundError,
            AppError::Conflict(_) => ResponseCode::ConflictError,
            AppError::RateLimit(_) => ResponseCode::RateLimitError,
            AppError::ServiceUnavailable(_) => ResponseCode::ServiceUnavailableError,
            AppError::Internal(_) => ResponseCode::InternalError,
            AppError::Timeout(_) => ResponseCode::TimeoutError,
            AppError::Database(_) => ResponseCode::DatabaseError,
            AppError::Network(_) => ResponseCode::NetworkError,
            AppError::Ipfs(_) => ResponseCode::ServiceUnavailableError,
            AppError::Blockchain(_) => ResponseCode::ServiceUnavailableError,
            AppError::Docker(_) => ResponseCode::InternalError,
            AppError::Tee(_) => ResponseCode::InternalError,
            AppError::Configuration(_) => ResponseCode::InternalError,
            AppError::BadRequest(_) => ResponseCode::ValidationError,
            AppError::Other(_) => ResponseCode::InternalError,
        }
    }
}

impl From<AppError> for ResponseCode {
    fn from(error: AppError) -> Self {
        Self::from(&error)
    }
}

// Conversion from serde_json errors
impl From<serde_json::Error> for AppError {
    fn from(error: serde_json::Error) -> Self {
        Self::Validation(format!("JSON parsing error: {}", error))
    }
}

// Conversion from UUID parsing errors
impl From<uuid::Error> for AppError {
    fn from(error: uuid::Error) -> Self {
        Self::Validation(format!("Invalid UUID format: {}", error))
    }
}

// Conversion from chrono parsing errors
impl From<chrono::format::ParseError> for AppError {
    fn from(error: chrono::format::ParseError) -> Self {
        Self::Validation(format!("Invalid date/time format: {}", error))
    }
}

// Conversion from sqlx errors (when sqlx feature is enabled)
#[cfg(feature = "sqlx")]
impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => {
                Self::NotFound("Resource not found in database".to_string())
            }
            sqlx::Error::Database(db_err) => {
                if let Some(constraint) = db_err.constraint() {
                    Self::Conflict(format!("Database constraint violation: {}", constraint))
                } else {
                    Self::Database(format!("Database operation failed: {}", db_err.message()))
                }
            }
            sqlx::Error::PoolTimedOut => Self::Timeout("Database connection timeout".to_string()),
            _ => Self::Database(format!("Database error: {}", error)),
        }
    }
}

// JWT errors (when jwt feature is enabled)
#[cfg(feature = "jwt")]
impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        use jsonwebtoken::errors::ErrorKind;

        match error.kind() {
            ErrorKind::InvalidToken
            | ErrorKind::InvalidSignature
            | ErrorKind::InvalidEcdsaKey
            | ErrorKind::InvalidRsaKey(_)
            | ErrorKind::RsaFailedSigning
            | ErrorKind::InvalidAlgorithm => {
                Self::Authentication("Invalid or malformed token".to_string())
            }
            ErrorKind::ExpiredSignature => Self::Authentication("Token has expired".to_string()),
            ErrorKind::InvalidIssuer
            | ErrorKind::InvalidAudience
            | ErrorKind::InvalidSubject
            | ErrorKind::ImmatureSignature
            | ErrorKind::InvalidKeyFormat => {
                Self::Authentication("Token validation failed".to_string())
            }
            _ => Self::Authentication(format!("Authentication error: {}", error)),
        }
    }
}

// Redis support will be added in future versions

// Axum integration (when axum feature is enabled)
#[cfg(feature = "axum")]
impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        use axum::Json;

        let response_code = ResponseCode::from(&self);
        let error_message = self.to_string();

        // Enhanced error logging with context
        tracing::error!(
            error = %self,
            response_message = %error_message,
            response_code = ?response_code,
            "Request failed"
        );

        // In debug mode, also log the error chain for better debugging
        #[cfg(debug_assertions)]
        {
            use std::error::Error;
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

        let api_response: ApiResponse<()> = ApiResponse::error(response_code, error_message);
        Json(api_response).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_to_response_code_mapping() {
        assert_eq!(
            ResponseCode::from(AppError::Validation("test".to_string())),
            ResponseCode::ValidationError
        );
        assert_eq!(
            ResponseCode::from(AppError::Authentication("test".to_string())),
            ResponseCode::AuthenticationError
        );
        assert_eq!(
            ResponseCode::from(AppError::NotFound("test".to_string())),
            ResponseCode::NotFoundError
        );
    }

    #[test]
    fn test_error_to_api_response() {
        let error = AppError::Validation("Test error".to_string());
        let response_code = ResponseCode::from(&error);
        let response: ApiResponse<()> = ApiResponse::error(response_code, error.to_string());

        assert_eq!(response.code, ResponseCode::ValidationError);
        assert_eq!(response.data, None);
        assert_eq!(
            response.message,
            Some("Validation error: Test error".to_string())
        );
    }

    #[test]
    fn test_convenience_constructors() {
        let error = AppError::validation("test message");
        assert!(matches!(error, AppError::Validation(_)));

        let error = AppError::not_found("resource not found");
        assert!(matches!(error, AppError::NotFound(_)));

        let error = AppError::internal("internal error");
        assert!(matches!(error, AppError::Internal(_)));
    }

    #[test]
    fn test_json_error_conversion() {
        let json_error = serde_json::from_str::<serde_json::Value>("invalid json");
        assert!(json_error.is_err());

        let app_error = AppError::from(json_error.unwrap_err());
        assert!(matches!(app_error, AppError::Validation(_)));
    }

    #[test]
    fn test_uuid_error_conversion() {
        let uuid_error = uuid::Uuid::parse_str("invalid-uuid");
        assert!(uuid_error.is_err());

        let app_error = AppError::from(uuid_error.unwrap_err());
        assert!(matches!(app_error, AppError::Validation(_)));
    }

    #[test]
    fn test_validation_errors_conversion() {
        use validator::Validate;

        #[derive(Validate)]
        struct TestStruct {
            #[validate(email)]
            email: String,
        }

        let test_data = TestStruct {
            email: "invalid-email".to_string(),
        };

        let validation_result = test_data.validate();
        assert!(validation_result.is_err());

        let app_error = AppError::from(validation_result.unwrap_err());
        assert!(matches!(app_error, AppError::Validation(_)));
    }

    #[test]
    fn test_error_display() {
        let error = AppError::Validation("Test message".to_string());
        assert_eq!(format!("{}", error), "Validation error: Test message");
    }
}
