//! Error handling module for unified application error management.
//!
//! This module provides a unified error handling system that categorizes all errors
//! into generic types rather than specific error codes. This approach provides
//! consistent error responses while preserving detailed error information in messages.
//!
//! # Key Components
//!
//! - [`AppError`] - Unified error enum with generic categories
//! - [`AppResult<T>`] - Type alias for `Result<T, AppError>`
//!
//! # Design Philosophy
//!
//! The error handling system follows these principles:
//!
//! 1. **Generic Categories**: Use broad error types (Validation, Authentication, etc.)
//! 2. **Specific Messages**: Include detailed error information in the message field
//! 3. **Consistent Mapping**: 1:1 mapping from `AppError` to `ResponseCode`
//! 4. **Automatic Conversion**: Built-in conversions from common error types
//!
//! # Examples
//!
//! Creating errors manually:
//!
//! ```
//! use avinasi_web::error::AppError;
//!
//! // Using convenience constructors
//! let validation_err = AppError::validation("Email format is invalid");
//! let not_found_err = AppError::not_found("User with ID 123 not found");
//!
//! // Using the enum variants directly
//! let auth_err = AppError::Authentication("Invalid credentials".to_string());
//! ```
//!
//! Using with the `?` operator:
//!
//! ```rust,ignore
//! use avinasi_web::{AppResult, AppError};
//!
//! fn validate_email(email: &str) -> AppResult<()> {
//!     if !email.contains('@') {
//!         return Err(AppError::validation("Email must contain @ symbol"));
//!     }
//!     Ok(())
//! }
//!
//! fn process_user(email: &str) -> AppResult<String> {
//!     validate_email(email)?; // Automatically propagates AppError
//!     Ok(format!("Processing user: {}", email))
//! }
//! ```
//!
//! Automatic conversion from external errors:
//!
//! ```
//! use avinasi_web::{AppResult, AppError};
//!
//! fn parse_json(data: &str) -> AppResult<serde_json::Value> {
//!     let value = serde_json::from_str(data)?; // Automatically converts serde_json::Error
//!     Ok(value)
//! }
//! ```
//!
//! Integration with web frameworks:
//!
//! ```rust,ignore
//! use avinasi_web::{AppResult, AppError, ApiResponse};
//! use axum::Json;
//!
//! async fn handler() -> AppResult<Json<ApiResponse<String>>> {
//!     let result = some_fallible_operation().await?;
//!     data!(result)
//! }
//!
//! // AppError automatically converts to proper HTTP response
//! ```

pub mod app_error;

// Re-export the main types for convenient access
pub use app_error::{AppError, AppResult};
