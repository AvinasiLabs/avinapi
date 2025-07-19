//! Response module for unified API response patterns.
//!
//! This module provides the core response types and utilities for implementing
//! the unified response pattern where all API endpoints return HTTP 200 with
//! business status indicated in the response body.
//!
//! # Key Components
//!
//! - [`ApiResponse<T>`] - Unified response structure with code, data, and message fields
//! - [`ResponseCode`] - Enum of business status codes (SUCCESS, VALIDATION_ERROR, etc.)
//! - Response macros - [`data!`], [`empty!`], [`error!`] for eliminating boilerplate
//!
//! # Examples
//!
//! Using the response types directly:
//!
//! ```
//! use avinapi::response::{ApiResponse, ResponseCode};
//!
//! // Success response with data
//! let response = ApiResponse::success("Hello World");
//! assert!(response.is_success());
//!
//! // Error response
//! let error_response: ApiResponse<()> = ApiResponse::error(
//!     ResponseCode::ValidationError,
//!     "Email is required"
//! );
//! assert!(error_response.is_error());
//! ```
//!
//! Using the convenience macros (requires axum feature):
//!
//! ```rust,ignore
//! use avinapi::{data, empty, error, AppResult, ApiResponse, ResponseCode};
//! use axum::Json;
//!
//! async fn create_user() -> AppResult<Json<ApiResponse<String>>> {
//!     // Business logic...
//!     data!("User created successfully".to_string())
//! }
//!
//! async fn delete_user() -> AppResult<Json<ApiResponse<()>>> {
//!     // Business logic...
//!     empty!()
//! }
//!
//! async fn validate_input() -> AppResult<Json<ApiResponse<()>>> {
//!     if input_invalid {
//!         error!(ResponseCode::ValidationError, "Invalid input format")
//!     } else {
//!         empty!()
//!     }
//! }
//! ```

pub mod api_response;
pub mod macros;

// Re-export the main types and macros for convenient access
pub use api_response::{ApiResponse, ResponseCode};
pub use macros::{JsonResult, PaginatedResult, data, empty};
