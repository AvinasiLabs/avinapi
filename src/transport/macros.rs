//! Response macros for eliminating boilerplate in API response creation.
//!
//! These macros provide a clean, ergonomic way to create API responses that follow
//! the unified response pattern. They automatically wrap responses in the appropriate
//! JSON and result types for web framework integration.

/// Creates a successful API response with data.
///
/// This macro wraps the provided data in an `ApiResponse::success()` call and
/// returns it as `Ok(Json(response))`, ready for use in axum handlers.
///
/// # Examples
///
/// ```rust,ignore
/// use avinapi::{data, AppResult, ApiResponse};
/// use axum::Json;
///
/// async fn get_user() -> AppResult<Json<ApiResponse<String>>> {
///     let user_name = "John Doe".to_string();
///     data!(user_name)
/// }
/// ```
///
/// # Returns
///
/// Returns `Ok(Json(ApiResponse::success(data)))` where the data is moved into
/// the response structure.
#[macro_export]
macro_rules! data {
    ($data:expr) => {
        Ok(axum::Json($crate::transport::response::data($data)))
    };
}

/// Creates a successful API response without data.
///
/// This macro creates an empty success response, typically used for operations
/// that don't need to return data (like deletions or updates that only need
/// to confirm success).
///
/// # Examples
///
/// ```rust,ignore
/// use avinapi::{empty, AppResult, ApiResponse};
/// use axum::Json;
///
/// async fn delete_user() -> AppResult<Json<ApiResponse<()>>> {
///     // Perform deletion logic...
///     empty!()
/// }
/// ```
///
/// # Returns
///
/// Returns `Ok(Json(ApiResponse::empty()))`.
#[macro_export]
macro_rules! empty {
    () => {
        Ok($crate::axum_json!($crate::transport::response::empty()))
    };
}

// Note: error! and error_default! macros have been removed.
// Use AppError with the ? operator instead, as AppError implements IntoResponse.
// This provides better error handling and automatic conversion to API responses.

/// Internal macro for conditional JSON wrapping based on feature flags.
///
/// This macro provides framework-agnostic JSON wrapping. When the `axum` feature
/// is enabled, it uses `axum::Json`. This allows the same macros to work across
/// different web frameworks.
#[doc(hidden)]
#[macro_export]
macro_rules! axum_json {
    ($expr:expr) => {{
        #[cfg(feature = "axum")]
        {
            axum::Json($expr)
        }
        #[cfg(not(feature = "axum"))]
        {
            $expr
        }
    }};
}

/// Macro for paginated responses
///
/// This macro simplifies creating paginated API responses by providing
/// a simple `paginated!(items, total, page, per_page)` syntax.
///
/// ## Usage
/// ```rust
/// use crate::paginated;
///
/// pub async fn get_users(page: u32) -> AppResult<Json<ApiResponse<PaginatedData<User>>>> {
///     let users = User::find_all(&db, page).await?;
///     let total = User::count(&db).await?;
///     paginated!(users, total, page, 20)
/// }
/// ```
///
/// ## Equivalent to
/// ```rust
/// Ok(Json(paginated(users, total, page, 20)))
/// ```
#[macro_export]
macro_rules! paginated {
    ($items:expr, $total:expr, $page:expr, $per_page:expr) => {
        Ok(axum::Json($crate::transport::response::paginated(
            $items, $total, $page, $per_page,
        )))
    };
}

// Re-export the macros for easier use
pub use data;
pub use empty;
pub use paginated;

#[cfg(test)]
mod tests {
    use crate::prelude::PaginatedResult;

    use super::*;

    #[test]
    fn test_data_macro() {
        let result: Result<_, crate::transport::error::AppError> = data!("test data");
        assert!(result.is_ok());

        #[cfg(feature = "axum")]
        {
            let json_response: axum::Json<crate::transport::ApiResponse<&str>> = result.unwrap();
            assert_eq!(json_response.0.data, Some("test data"));
        }
    }

    #[test]
    fn test_empty_macro() {
        let result: Result<_, crate::transport::error::AppError> = empty!();
        assert!(result.is_ok());

        #[cfg(feature = "axum")]
        {
            let json_response: axum::Json<crate::transport::ApiResponse<()>> = result.unwrap();
            assert_eq!(json_response.0.data, None);
        }
    }

    #[test]
    fn test_paginated_macro() {
        let result: PaginatedResult<i32> = paginated!(vec![1, 2, 3], 100, 1, 10);
        assert!(result.is_ok());

        #[cfg(feature = "axum")]
        {
            let json_response = result.unwrap();
            assert_eq!(json_response.0.data.unwrap().items, vec![1, 2, 3]);
        }
    }

    // Note: error macro tests removed as error! macro has been removed.
    // AppError now implements IntoResponse for direct error handling.
}
