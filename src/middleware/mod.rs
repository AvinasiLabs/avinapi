//! Middleware for transforming responses by injecting request context.
//!
//! This middleware automatically injects request context into API responses.
//! It extracts the request ID from the request extensions and injects it into the response.
//! It also extracts the client IP address from the request headers and stores it in the request extensions.
//!
//! # Example
//!
//! ```rust
//! use axum::{routing::get, Router};
//! use avinapi::middleware::{request_id, ClientIp};
//!
//! let app = Router::new()
//!     .route("/", get(|| async { "Hello, World!" }))
//!     .layer(request_id::request_id_middleware());
//! ```
//!
//! # Headers
//!
//! - `x-request-id`: Unique request ID generated using UUID.
//! - `x-client-ip`: Client IP address extracted from request headers.

pub mod request_id;
pub mod response_transformer;

pub use request_id::{ClientIp, REQUEST_ID_HEADER, RequestId, request_id_middleware};
pub use response_transformer::response_transformer;
