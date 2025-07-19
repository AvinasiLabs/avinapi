//! API Response types and codes for consistent response handling across web APIs.
//!
//! This module implements the unified response pattern where all API endpoints return
//! HTTP 200 OK with business status indicated in the response body.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Response codes for API operations.
///
/// Following the unified response pattern, all business logic uses generic categories
/// rather than specific error codes. Detailed error information is provided in the
/// message field of the response.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ResponseCode {
    /// Operation completed successfully
    Success,

    /// Input validation failed
    ValidationError,

    /// Authentication failed or missing
    AuthenticationError,

    /// User does not have permission for this operation
    AuthorizationError,

    /// Requested resource was not found
    NotFoundError,

    /// Resource already exists and cannot be created again
    ConflictError,

    /// Rate limit exceeded
    RateLimitError,

    /// External service is temporarily unavailable
    ServiceUnavailableError,

    /// Internal server error occurred
    InternalError,

    /// Request timeout
    TimeoutError,

    /// Database operation failed
    DatabaseError,

    /// Network or connectivity error
    NetworkError,
}

impl ResponseCode {
    /// Returns true if this response code indicates success
    pub fn is_success(self) -> bool {
        matches!(self, ResponseCode::Success)
    }

    /// Returns true if this response code indicates an error
    pub fn is_error(self) -> bool {
        !self.is_success()
    }
}

impl std::fmt::Display for ResponseCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            ResponseCode::Success => "Operation completed successfully",
            ResponseCode::ValidationError => "Input validation failed",
            ResponseCode::AuthenticationError => "Authentication failed",
            ResponseCode::AuthorizationError => "Permission denied",
            ResponseCode::NotFoundError => "Resource not found",
            ResponseCode::ConflictError => "Resource already exists",
            ResponseCode::RateLimitError => "Rate limit exceeded",
            ResponseCode::ServiceUnavailableError => "Service temporarily unavailable",
            ResponseCode::InternalError => "Internal server error",
            ResponseCode::TimeoutError => "Request timeout",
            ResponseCode::DatabaseError => "Database operation failed",
            ResponseCode::NetworkError => "Network error",
        };
        write!(f, "{}", message)
    }
}

/// Unified API response structure.
///
/// All API endpoints return this structure with:
/// - `code`: Business status (SUCCESS, VALIDATION_ERROR, etc.)
/// - `data`: Present only on successful operations
/// - `message`: Present only on errors with detailed information
///
/// # Examples
///
/// Successful response with data:
/// ```json
/// {
///   "code": "SUCCESS",
///   "data": { "id": 123, "name": "John Doe" },
///   "message": null
/// }
/// ```
///
/// Error response:
/// ```json
/// {
///   "code": "VALIDATION_ERROR",
///   "data": null,
///   "message": "Email field is required and must be valid"
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct ApiResponse<T> {
    /// Business operation status code
    pub code: ResponseCode,

    /// Response data - present only on successful operations

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,

    /// Error message - present only when code indicates an error

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl<T> ApiResponse<T> {
    /// Creates a successful response with data
    ///
    /// # Examples
    ///
    /// ```
    /// # use avinasi_web::response::ApiResponse;
    /// let response = ApiResponse::success("Hello World");
    /// assert!(response.code.is_success());
    /// assert_eq!(response.data, Some("Hello World"));
    /// assert_eq!(response.message, None);
    /// ```
    pub fn success(data: T) -> Self {
        Self {
            code: ResponseCode::Success,
            data: Some(data),
            message: None,
        }
    }

    /// Creates an error response with a specific code and message
    ///
    /// # Examples
    ///
    /// ```
    /// # use avinasi_web::response::{ApiResponse, ResponseCode};
    /// let response: ApiResponse<()> = ApiResponse::error(
    ///     ResponseCode::ValidationError,
    ///     "Email is required"
    /// );
    /// assert!(response.code.is_error());
    /// assert_eq!(response.data, None);
    /// assert_eq!(response.message, Some("Email is required".to_string()));
    /// ```
    pub fn error(code: ResponseCode, message: impl Into<String>) -> Self {
        Self {
            code,
            data: None,
            message: Some(message.into()),
        }
    }

    /// Creates an error response using the default message for the code
    ///
    /// # Examples
    ///
    /// ```
    /// # use avinasi_web::response::{ApiResponse, ResponseCode};
    /// let response: ApiResponse<()> = ApiResponse::error_with_default(
    ///     ResponseCode::NotFoundError
    /// );
    /// assert_eq!(response.message, Some("Resource not found".to_string()));
    /// ```
    pub fn error_with_default(code: ResponseCode) -> Self {
        Self {
            code,
            data: None,
            message: Some(code.to_string()),
        }
    }

    /// Returns true if this response indicates success
    pub fn is_success(&self) -> bool {
        self.code.is_success()
    }

    /// Returns true if this response indicates an error
    pub fn is_error(&self) -> bool {
        self.code.is_error()
    }

    /// Maps the data field while preserving the response structure
    ///
    /// This is useful for transforming successful responses while maintaining
    /// error responses unchanged.
    pub fn map<U, F>(self, f: F) -> ApiResponse<U>
    where
        F: FnOnce(T) -> U,
    {
        match self.data {
            Some(data) => ApiResponse {
                code: self.code,
                data: Some(f(data)),
                message: self.message,
            },
            None => ApiResponse {
                code: self.code,
                data: None,
                message: self.message,
            },
        }
    }
}

impl ApiResponse<()> {
    /// Creates a successful response without data
    ///
    /// # Examples
    ///
    /// ```
    /// # use avinasi_web::response::ApiResponse;
    /// let response = ApiResponse::empty();
    /// assert!(response.code.is_success());
    /// assert_eq!(response.data, None);
    /// assert_eq!(response.message, None);
    /// ```
    pub fn empty() -> Self {
        Self {
            code: ResponseCode::Success,
            data: None,
            message: None,
        }
    }
}

// Implement Default for empty successful response
impl Default for ApiResponse<()> {
    fn default() -> Self {
        Self::empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_code_is_success() {
        assert!(ResponseCode::Success.is_success());
        assert!(!ResponseCode::ValidationError.is_success());
        assert!(!ResponseCode::InternalError.is_success());
    }

    #[test]
    fn test_response_code_is_error() {
        assert!(!ResponseCode::Success.is_error());
        assert!(ResponseCode::ValidationError.is_error());
        assert!(ResponseCode::InternalError.is_error());
    }

    #[test]
    fn test_api_response_success() {
        let response = ApiResponse::success("test data");
        assert!(response.is_success());
        assert_eq!(response.data, Some("test data"));
        assert_eq!(response.message, None);
    }

    #[test]
    fn test_api_response_empty() {
        let response = ApiResponse::empty();
        assert!(response.is_success());
        assert_eq!(response.data, None);
        assert_eq!(response.message, None);
    }

    #[test]
    fn test_api_response_error() {
        let response: ApiResponse<()> =
            ApiResponse::error(ResponseCode::ValidationError, "Test error message");
        assert!(response.is_error());
        assert_eq!(response.data, None);
        assert_eq!(response.message, Some("Test error message".to_string()));
    }

    #[test]
    fn test_api_response_error_with_default() {
        let response: ApiResponse<()> =
            ApiResponse::error_with_default(ResponseCode::NotFoundError);
        assert!(response.is_error());
        assert_eq!(response.message, Some("Resource not found".to_string()));
    }

    #[test]
    fn test_api_response_map() {
        let response = ApiResponse::success(42);
        let mapped = response.map(|n| n.to_string());
        assert_eq!(mapped.data, Some("42".to_string()));

        let error_response: ApiResponse<i32> =
            ApiResponse::error(ResponseCode::ValidationError, "Error");
        let mapped_error = error_response.map(|n| n.to_string());
        assert_eq!(mapped_error.data, None);
        assert_eq!(mapped_error.message, Some("Error".to_string()));
    }

    #[test]
    fn test_serialization() {
        let response = ApiResponse::success("test");
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"code\":\"SUCCESS\""));
        assert!(json.contains("\"data\":\"test\""));
        assert!(!json.contains("\"message\""));

        let error_response: ApiResponse<()> =
            ApiResponse::error(ResponseCode::ValidationError, "Error message");
        let error_json = serde_json::to_string(&error_response).unwrap();
        assert!(error_json.contains("\"code\":\"VALIDATION_ERROR\""));
        assert!(error_json.contains("\"message\":\"Error message\""));
        assert!(!error_json.contains("\"data\""));
    }
}
