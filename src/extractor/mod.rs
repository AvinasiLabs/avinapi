//! Request extractors with automatic validation.
//!
//! This module provides enhanced versions of axum's extractors that automatically
//! validate the extracted data using the `validator` crate.
//!
//! # Available Extractors
//!
//! - [`ValidatedJson`]: JSON body extractor with validation
//! - [`ValidatedQuery`]: Query parameter extractor with validation
//!
//! # Example
//!
//! ```
//! use avinapi::extractor::{ValidatedJson, ValidatedQuery};
//! use avinapi::response::ApiResponse;
//! use serde::{Deserialize, Serialize};
//! use validator::Validate;
//!
//! #[derive(Debug, Deserialize, Validate)]
//! struct CreateUserRequest {
//!     #[validate(email)]
//!     email: String,
//!
//!     #[validate(length(min = 3, max = 50))]
//!     username: String,
//! }
//!
//! #[derive(Debug, Deserialize, Validate)]
//! struct ListUsersQuery {
//!     #[validate(range(min = 1, max = 100))]
//!     page: Option<u32>,
//!
//!     #[validate(range(min = 1, max = 100))]
//!     per_page: Option<u32>,
//! }
//!
//! async fn create_user(
//!     ValidatedJson(payload): ValidatedJson<CreateUserRequest>
//! ) -> ApiResponse<()> {
//!     // payload is guaranteed to be valid
//!     ApiResponse::success(())
//! }
//!
//! async fn list_users(
//!     ValidatedQuery(params): ValidatedQuery<ListUsersQuery>
//! ) -> ApiResponse<Vec<String>> {
//!     // params are guaranteed to be valid
//!     ApiResponse::success(vec![])
//! }
//! ```

mod validated_json;
mod validated_query;

pub use validated_json::ValidatedJson;
pub use validated_query::ValidatedQuery;
