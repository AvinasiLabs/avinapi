//! Validation module for automatic request validation.
//!
//! This module provides automatic validation utilities for web API requests,
//! following the unified response pattern. The primary component is the
//! `ValidatedJson<T>` extractor that combines JSON parsing with validation.
//!
//! # Key Components
//!
//! - [`ValidatedJson<T>`] - JSON extractor with automatic validation
//!
//! # Design Philosophy
//!
//! The validation system follows these principles:
//!
//! 1. **Automatic Validation**: Validation happens at the transport layer before reaching business logic
//! 2. **Comprehensive Error Messages**: Field-level validation errors are collected and formatted
//! 3. **Unified Error Responses**: All validation errors follow the same response pattern
//! 4. **Framework Integration**: Seamlessly integrates with web framework extractors
//!
//! # Examples
//!
//! Basic usage with validation attributes:
//!
//! ```rust,ignore
//! use avinasi_web::{ValidatedJson, AppResult, ApiResponse, data};
//! use axum::Json;
//! use serde::Deserialize;
//! use validator::Validate;
//! use utoipa::ToSchema;
//!
//! #[derive(Deserialize, Validate, ToSchema)]
//! struct CreateUserRequest {
//!     #[validate(email)]
//!     #[schema(example = "user@example.com")]
//!     email: String,
//!
//!     #[validate(length(min = 1, max = 100))]
//!     #[schema(example = "John Doe")]
//!     name: String,
//!
//!     #[validate(length(min = 8))]
//!     #[schema(example = "securepassword")]
//!     password: String,
//! }
//!
//! // Handler automatically validates input
//! async fn create_user(
//!     ValidatedJson(request): ValidatedJson<CreateUserRequest>,
//! ) -> AppResult<Json<ApiResponse<String>>> {
//!     // If we reach this point, the request is guaranteed to be valid:
//!     // - Valid JSON structure
//!     // - Valid email format
//!     // - Name between 1-100 characters
//!     // - Password at least 8 characters
//!
//!     let user_id = create_user_in_db(&request.email, &request.name, &request.password).await?;
//!     data!(format!("User created with ID: {}", user_id))
//! }
//! ```
//!
//! Advanced validation with custom validation functions:
//!
//! ```rust,ignore
//! use validator::{Validate, ValidationError};
//!
//! fn validate_username(username: &str) -> Result<(), ValidationError> {
//!     if username.chars().any(|c| !c.is_alphanumeric() && c != '_') {
//!         return Err(ValidationError::new("invalid_characters"));
//!     }
//!     if username.starts_with('_') || username.ends_with('_') {
//!         return Err(ValidationError::new("invalid_underscore_placement"));
//!     }
//!     Ok(())
//! }
//!
//! #[derive(Deserialize, Validate)]
//! struct AdvancedRequest {
//!     #[validate(length(min = 3, max = 20), custom = "validate_username")]
//!     username: String,
//!
//!     #[validate(range(min = 13, max = 120))]
//!     age: u32,
//!
//!     #[validate(contains = "@")]
//!     email: String,
//! }
//! ```
//!
//! Nested validation with collections:
//!
//! ```rust,ignore
//! #[derive(Deserialize, Validate)]
//! struct Address {
//!     #[validate(length(min = 1))]
//!     street: String,
//!
//!     #[validate(length(min = 1))]
//!     city: String,
//!
//!     #[validate(regex = "ZIP_CODE_REGEX")]
//!     postal_code: String,
//! }
//!
//! #[derive(Deserialize, Validate)]
//! struct UserProfile {
//!     #[validate(length(min = 1))]
//!     name: String,
//!
//!     #[validate(email)]
//!     email: String,
//!
//!     #[validate]
//!     address: Address,
//!
//!     #[validate(length(min = 1))]
//!     tags: Vec<String>,
//! }
//! ```
//!
//! Error handling with detailed field information:
//!
//! When validation fails, the error response includes detailed field-level information:
//!
//! ```json
//! {
//!   "code": "VALIDATION_ERROR",
//!   "data": null,
//!   "message": "email: Invalid email format, name: Name must be between 1 and 100 characters"
//! }
//! ```

#[cfg(feature = "axum")]
pub mod validated_json;

// Custom validation functions module
pub mod validation_functions;

// Re-export the main types for convenient access
#[cfg(feature = "axum")]
pub use validated_json::ValidatedJson;

// Re-export validation functions for easy use
pub use validation_functions::*;
