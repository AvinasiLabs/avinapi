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

/// Unified API response structure using Either for type-safe mutual exclusivity
/// All API responses use HTTP 200 OK status code
/// The actual business status is indicated in the `code` field
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T: Serialize> {
    /// Response code indicating the business status
    pub code: ResponseCode,

    /// Either data or message (mutually exclusive)
    #[serde(flatten)]
    pub payload: Payload<T>,

    /// Request ID for tracing and debugging (always present)
    pub req_id: String,
}

/// Payload that ensures mutual exclusivity between data and message
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Payload<T> {
    /// Success payload with data
    Data {
        /// The actual response data
        data: T,
    },
    /// Error payload with message
    Message {
        /// The error message
        message: String,
    },
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
        payload: Payload::Data { data },
        req_id: String::new(),
    }
}

/// Create an error response with code and message
pub fn fail<T: Serialize>(code: ResponseCode, message: impl Into<String>) -> ApiResponse<T> {
    ApiResponse {
        code,
        payload: Payload::Message {
            message: message.into(),
        },
        req_id: String::new(),
    }
}

/// Create an empty success response (using unit type)
pub fn empty() -> ApiResponse<()> {
    data(())
}

/// Create a paginated response
pub fn paginated<T: Serialize>(
    items: Vec<T>,
    total: u64,
    n_page: u32,
    per_page: u32,
) -> ApiResponse<PaginatedData<T>> {
    data(PaginatedData {
        items,
        total,
        n_page,
        per_page,
    })
}
