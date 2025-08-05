//! # Avinapi
//!
//! A comprehensive Rust web API foundation library providing reusable patterns and utilities
//! for building consistent, maintainable web APIs.
//!
//! ## Features
//!
//! - **Unified Response Pattern**: All API endpoints return HTTP 200 with business status in response body
//! - **Automatic Validation**: Input validation through `ValidatedJson<T>` extractor
//! - **Error Handling**: Comprehensive error handling with generic categories and specific messages
//! - **Query Parameters**: Composable query parameter types for pagination, sorting, and filtering
//! - **Middleware Collection**: Common middleware for authentication, CORS, rate limiting, and request tracking
//! - **Testing Utilities**: Test context and utilities for isolated testing
//!
//! ## Quick Start
//!
//! Add to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! avinapi = { version = "0.1.0", features = ["axum"] }
//! ```
//!
//! Basic usage:
//!
//! ```rust,ignore
//! use avinapi::prelude::*;
//!
//! #[derive(Deserialize, Validate, ToSchema)]
//! struct CreateUserRequest {
//!     #[validate(email)]
//!     email: String,
//!     #[validate(length(min = 1))]
//!     name: String,
//! }
//!
//! async fn create_user(
//!     ValidatedJson(request): ValidatedJson<CreateUserRequest>,
//! ) -> AppResult<Json<ApiResponse<String>>> {
//!     // Business logic here
//!     data!(format!("User {} created", request.name))
//! }
//! ```
//!
//! ## Feature Flags
//!
//! - `axum` (default): Support for Axum web framework
//! - `actix`: Support for Actix-web framework (future)
//! - `warp`: Support for Warp framework (future)
//! - `jwt`: JWT authentication utilities
//! - `redis`: Redis-backed rate limiting and caching
//! - `testing`: Testing utilities and test context

#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

// Core modules - always available
pub mod transport;

// Validation module with custom validation functions
pub mod validation;

// Query parameter utilities - always available
pub mod query;

// Framework-specific middleware (feature-gated)
#[cfg(feature = "axum")]
#[cfg_attr(docsrs, doc(cfg(feature = "axum")))]
pub mod middleware;

// Prelude module for convenient imports
pub mod prelude;

// Framework-specific re-exports
#[cfg(feature = "axum")]
#[cfg_attr(docsrs, doc(cfg(feature = "axum")))]
pub use crate::transport::extractor::{ValidatedJson, ValidatedQuery};

// Version information
/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Library name
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// Library description
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
