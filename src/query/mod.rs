//! Query parameter utilities for web API requests.
//!
//! This module provides composable query parameter types for common web API
//! operations like pagination, sorting, and filtering. The query parameters
//! are designed to be used together through multiple Query extractors.
//!
//! # Key Components
//!
//! - **Pagination**: `PaginationQuery` for page-based pagination with metadata
//! - **Date Range**: `DateRangeQuery` for date-based filtering
//! - **Sorting**: `SortQuery` and `SortOrder` for result ordering
//!
//! # Design Philosophy
//!
//! Query parameters follow a composition-over-duplication approach:
//!
//! 1. **Generic Parameters**: Common functionality like pagination is provided as reusable types
//! 2. **Composable**: Multiple query parameter types can be used together in handlers
//! 3. **Validated**: All query parameters include appropriate validation rules
//! 4. **Documented**: Full OpenAPI documentation support with examples
//!
//! # Examples
//!
//! Using multiple query parameter types together:
//!
//! ```rust,ignore
//! use avinapi::query::{PaginationQuery, DateRangeQuery, SortQuery};
//! use axum::{extract::Query, Json};
//!
//! async fn get_items(
//!     Query(pagination): Query<PaginationQuery>,
//!     Query(date_range): Query<DateRangeQuery>,
//!     Query(sort): Query<SortQuery>,
//! ) -> AppResult<Json<ApiResponse<PaginatedData<Item>>>> {
//!     let page = pagination.get_page();
//!     let per_page = pagination.get_per_page();
//!     let (start_date, end_date) = date_range.to_datetime_range();
//!     let order_by = sort.to_sql();
//!
//!     // Use parameters to query database...
//!     todo!("Implementation")
//! }
//! ```
//!
//! Combined usage in a single handler:
//!
//! ```rust,ignore
//! use avinapi::{query::*, JsonResult, data};
//! use axum::extract::Query;
//!
//! async fn list_users(
//!     Query(pagination): Query<PaginationQuery>,
//!     Query(date_filter): Query<DateRangeQuery>,
//!     Query(sort): Query<SortQuery>,
//! ) -> JsonResult<PaginatedData<UserResponse>> {
//!     // Validate sort fields
//!     let allowed_fields = vec!["name", "email", "created_at"];
//!     sort.validate_fields(&allowed_fields)?;
//!
//!     // Build database query with all parameters
//!     let users = query_users_with_filters(
//!         pagination.get_offset(),
//!         pagination.get_limit(),
//!         date_filter.to_datetime_range(),
//!         sort.to_sql(),
//!     ).await?;
//!
//!     let total_count = count_users_with_filters(&date_filter).await?;
//!     let response = PaginatedData::new(users, &pagination, total_count);
//!
//!     data!(response)
//! }
//! ```

// Pagination utilities
pub mod pagination;

// Date range filtering utilities
pub mod date_range;

// DateTime range filtering utilities
pub mod datetime_range;

// Sorting utilities
pub mod sort;

// Re-export main types for convenient access
pub use date_range::{DateRangeInfo, DateRangeQuery};
pub use datetime_range::{DateTimeRangeInfo, DateTimeRangeQuery};
pub use pagination::{PaginatedData, PaginationMeta, PaginationQuery};
pub use sort::{SortField, SortOrder, SortQuery};
