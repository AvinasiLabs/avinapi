//! Common result type aliases for simplified handler return types
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
pub type JsonResult<T> = super::error::AppResult<axum::Json<super::response::ApiResponse<T>>>;

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
pub type PaginatedResult<T> = super::error::AppResult<
    axum::Json<super::response::ApiResponse<super::response::PaginatedData<T>>>,
>;
