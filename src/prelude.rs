//! Prelude module for convenient imports.
//!
//! This module re-exports the most commonly used types, traits, and macros
//! from the avinapi library, allowing users to get started quickly with
//! a single import statement.
//!
//! # Usage
//!
//! Add this to the top of your files to import the most commonly used items:
//!
//! ```rust
//! use avinapi::prelude::*;
//! ```
//!
//! This will import:
//!
//! - **Response Types**: `ApiResponse<T>`, `ResponseCode`
//! - **Error Handling**: `AppError`, `AppResult<T>`
//! - **Response Macros**: `data!()`, `empty!()`, `error!()`, `error_default!()`
//! - **Validation**: `ValidatedJson<T>` (when axum feature is enabled)
//! - **External Traits**: Common serde, validator, and utoipa traits
//!
//! # Examples
//!
//! Basic API handler:
//!
//! ```rust,ignore
//! use avinapi::prelude::*;
//! use axum::Json;
//!
//! #[derive(Deserialize, Validate, ToSchema)]
//! struct CreateUserRequest {
//!     #[validate(email)]
//!     email: String,
//!     #[validate(length(min = 1))]
//!     name: String,
//! }
//!
//! #[derive(Serialize, ToSchema)]
//! struct UserResponse {
//!     id: u32,
//!     email: String,
//!     name: String,
//! }
//!
//! async fn create_user(
//!     ValidatedJson(request): ValidatedJson<CreateUserRequest>,
//! ) -> AppResult<Json<ApiResponse<UserResponse>>> {
//!     let user = UserResponse {
//!         id: 123,
//!         email: request.email,
//!         name: request.name,
//!     };
//!     data!(user)
//! }
//! ```

// Re-export core response types
pub use crate::transport::{ApiResponse, ResponseCode};

// Re-export error types
pub use crate::transport::error::{AppError, AppResult};

// Re-export response macros
pub use crate::transport::{data, empty};

// Re-export validation extractor (when axum feature is enabled)
#[cfg(feature = "axum")]
pub use crate::transport::extractor::{ValidatedJson, ValidatedQuery};

// Re-export custom validation functions
pub use crate::validation::common::{
    validate_age, validate_file_extension, validate_hex_color, validate_phone_number,
    validate_postal_code, validate_url_slug, validate_username,
};
pub use crate::validation::datetime::{
    validate_date_range, validate_datetime_format, validate_datetime_range,
};
pub use crate::validation::pwd::validate_password_strength;

// Re-export query parameter types
pub use crate::query::{
    DateRangeInfo, DateRangeQuery, PaginationQuery, SortField, SortOrder, SortQuery,
};

// Type aliases for common handler return types
#[cfg(feature = "axum")]
pub use crate::transport::{JsonResult, PaginatedResult};

// Re-export commonly used external crate items that users will need

// Serde traits for serialization/deserialization
pub use serde::{Deserialize, Serialize};

// Validator traits for validation
pub use validator::Validate;

// utoipa traits for OpenAPI documentation
pub use utoipa::{IntoParams, ToSchema};

// Common types that are frequently used
pub use chrono::{DateTime, Utc};
pub use uuid::Uuid;

// Framework-specific re-exports (feature-gated)
#[cfg(feature = "axum")]
pub use axum::{
    Router,
    extract::{Json, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, patch, post, put},
};

#[cfg(feature = "axum")]
pub use tower_http::cors::CorsLayer;

// JWT-related re-exports (when jwt feature is enabled)
#[cfg(feature = "jwt")]
pub use jsonwebtoken::{DecodingKey, EncodingKey, Header as JwtHeader, Validation};

// Database-related re-exports (when sqlx feature is enabled)
#[cfg(feature = "sqlx")]
pub use sqlx::{PgPool, Row};

// Future: testcontainers support when feature is added
// #[cfg(feature = "testcontainers")]
// pub use testcontainers::{Container, Docker, Image, clients::Cli};

// Common standard library items that are frequently used in web APIs
pub use std::collections::HashMap;
pub use std::sync::Arc;
pub use std::time::Duration;

// Async/await related items
pub use std::future::Future;
pub use std::pin::Pin;

/// Version information for the library
pub const VERSION: &str = crate::VERSION;

/// Library name
pub const NAME: &str = crate::NAME;
