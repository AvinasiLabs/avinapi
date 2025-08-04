//! response module docs

use axum::{Json, response::IntoResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Default number of items per page for pagination.
pub const DEFAULT_PER_PAGE: u32 = 20;

/// Business-specific response codes with simplified Error suffix design
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[schema(example = "SUCCESS")]
#[allow(missing_docs)]
pub enum ResponseCode {
    // Success
    Success,

    // Error codes with simplified naming
    ValidationError,
    ParsingError,
    DatabaseError,
    SerializationError,
    AuthenticationError,
    AuthorizationError,
    NotFoundError,
    ConflictError,
    InternalError,
    ConfigError,
    TimeoutError,
    RateLimitError,
}

/// Unified API response structure
/// All API responses use HTTP 200 OK status code
/// The actual business status is indicated in the `code` field
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiResponse<T: Serialize> {
    /// Response code indicating the business status
    #[schema(example = "SUCCESS")]
    pub code: ResponseCode,

    /// Response data (only present on success)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,

    /// Detailed error message (only present on error)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(example = "ok")]
    pub message: Option<String>,

    /// Request ID for tracing and debugging (always present)
    #[schema(example = "req-123e4567-e89b-12d3-a456-426614174000")]
    pub req_id: String,
}

/// Convert ApiResponse to HTTP response
impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

/// Paginated response wrapper
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PaginatedData<T> {
    /// The actual data items
    pub items: Vec<T>,
    /// Total number of items
    #[schema(example = 100)]
    pub total: u64,
    /// Current page number (1-based)
    #[schema(example = 1)]
    pub n_page: u32,
    /// Items per page
    #[schema(example = 30)]
    pub per_page: u32,
}

/// Create a successful response with data
pub fn data<T: Serialize>(data: T) -> ApiResponse<T> {
    ApiResponse {
        code: ResponseCode::Success,
        data: Some(data),
        message: None,
        req_id: String::new(),
    }
}

/// Create an error response with code and message
pub fn fail<T: Serialize>(code: ResponseCode, message: String) -> ApiResponse<T> {
    ApiResponse {
        code,
        data: None,
        message: Some(message),
        req_id: String::new(),
    }
}

/// Create an empty success response
pub fn empty<T: Serialize>() -> ApiResponse<T> {
    ApiResponse {
        code: ResponseCode::Success,
        data: None,
        message: None,
        req_id: String::new(),
    }
}

/// Create a paginated response
pub fn paginated<T: Serialize>(
    items: Vec<T>,
    total: u64,
    n_page: u32,
    per_page: u32,
) -> ApiResponse<PaginatedData<T>> {
    ApiResponse {
        code: ResponseCode::Success,
        data: Some(PaginatedData {
            items,
            total,
            n_page,
            per_page,
        }),
        message: None,
        req_id: String::new(),
    }
}
