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
        Ok($crate::axum_json!($crate::response::ApiResponse::success(
            $data
        )))
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
        Ok($crate::axum_json!($crate::response::ApiResponse::empty()))
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

// Re-export the macros for easier use
pub use data;
pub use empty;

// Common result type aliases for simplified handler return types

/// Type alias for standard JSON API responses.
///
/// This is a convenience type for handlers that return JSON responses wrapped in `ApiResponse<T>`.
/// It combines `AppResult` error handling with axum's `Json` extractor.
///
/// # Example
///
/// ```rust,ignore
/// use avinapi::response::JsonResult;
///
/// async fn get_user() -> JsonResult<UserResponse> {
///     let user = UserResponse { id: 1, name: "John".to_string() };
///     data!(user)
/// }
/// ```
#[cfg(feature = "axum")]
pub type JsonResult<T> = crate::error::AppResult<axum::Json<crate::response::ApiResponse<T>>>;

/// Type alias for paginated JSON API responses.
///
/// This is a convenience type for handlers that return paginated data wrapped in `ApiResponse<PaginatedData<T>>`.
/// It combines `AppResult` error handling with axum's `Json` extractor and pagination metadata.
///
/// # Example
///
/// ```rust,ignore
/// use avinapi::response::PaginatedResult;
///
/// async fn list_users() -> PaginatedResult<UserResponse> {
///     let users = vec![UserResponse { id: 1, name: "John".to_string() }];
///     let paginated_data = PaginatedData::new(users, pagination_meta);
///     data!(paginated_data)
/// }
/// ```
#[cfg(feature = "axum")]
pub type PaginatedResult<T> = crate::error::AppResult<
    axum::Json<crate::response::ApiResponse<crate::query::PaginatedData<T>>>,
>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_macro() {
        let result: Result<_, crate::error::AppError> = data!("test data");
        assert!(result.is_ok());

        #[cfg(feature = "axum")]
        {
            let json_response: axum::Json<crate::response::ApiResponse<&str>> = result.unwrap();
            assert!(json_response.0.is_success());
            assert_eq!(json_response.0.data, Some("test data"));
        }
    }

    #[test]
    fn test_empty_macro() {
        let result: Result<_, crate::error::AppError> = empty!();
        assert!(result.is_ok());

        #[cfg(feature = "axum")]
        {
            let json_response: axum::Json<crate::response::ApiResponse<()>> = result.unwrap();
            assert!(json_response.0.is_success());
            assert_eq!(json_response.0.data, None);
        }
    }

    // Note: error macro tests removed as error! macro has been removed.
    // AppError now implements IntoResponse for direct error handling.
}
