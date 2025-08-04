//! Pagination query parameters and response structures.
//!
//! This module provides pagination functionality for API endpoints, including
//! query parameter extraction, response formatting, and metadata calculation.

use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

/// Query parameters for pagination.
///
/// Provides page-based pagination with configurable page size limits.
/// Uses 1-based indexing for user-friendly URLs.
///
/// # Query Parameter Format
///
/// - `page`: Page number (1-based, defaults to 1)
/// - `per_page`: Items per page (defaults to 20, max 100)
///
/// # Examples
///
/// ```
/// use avinapi::query::PaginationQuery;
/// use serde_json;
///
/// // Default pagination (page=1, per_page=20)
/// let default = PaginationQuery::default();
/// assert_eq!(default.page, 1);
/// assert_eq!(default.per_page, 20);
///
/// // Custom pagination
/// let custom = PaginationQuery { page: 2, per_page: 50 };
/// assert_eq!(custom.get_offset(), 50); // (page-1) * per_page
/// assert_eq!(custom.get_limit(), 50);
/// ```
///
/// URL examples:
/// - `GET /users` - First page with default size (20 items)
/// - `GET /users?page=2` - Second page with default size
/// - `GET /users?page=1&per_page=50` - First page with 50 items
/// - `GET /users?page=3&per_page=10` - Third page with 10 items
#[derive(Debug, Clone, Deserialize, Validate, IntoParams, ToSchema)]
#[into_params(parameter_in = Query)]
#[serde(default)]
pub struct PaginationQuery {
    /// Page number (1-based indexing)
    #[validate(range(min = 1))]
    #[param(example = 1, minimum = 1)]
    pub page: u32,

    /// Number of items per page
    #[validate(range(min = 1, max = 100))]
    #[param(example = 20, minimum = 1, maximum = 100)]
    pub per_page: u32,
}

impl Default for PaginationQuery {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 20,
        }
    }
}

impl PaginationQuery {
    /// Creates a new PaginationQuery with the given page and per_page values.
    ///
    /// # Example
    ///
    /// ```
    /// use avinapi::query::PaginationQuery;
    ///
    /// let pagination = PaginationQuery::new(2, 50);
    /// assert_eq!(pagination.page, 2);
    /// assert_eq!(pagination.per_page, 50);
    /// ```
    pub fn new(page: u32, per_page: u32) -> Self {
        Self { page, per_page }
    }

    /// Gets the current page number (1-based).
    ///
    /// # Example
    ///
    /// ```
    /// use avinapi::query::PaginationQuery;
    ///
    /// let pagination = PaginationQuery::new(3, 20);
    /// assert_eq!(pagination.get_page(), 3);
    /// ```
    pub fn get_page(&self) -> u32 {
        self.page
    }

    /// Gets the number of items per page.
    ///
    /// # Example
    ///
    /// ```
    /// use avinapi::query::PaginationQuery;
    ///
    /// let pagination = PaginationQuery::new(1, 50);
    /// assert_eq!(pagination.get_per_page(), 50);
    /// ```
    pub fn get_per_page(&self) -> u32 {
        self.per_page
    }

    /// Calculates the offset for database queries (0-based).
    ///
    /// Offset = (page - 1) * per_page
    ///
    /// # Example
    ///
    /// ```
    /// use avinapi::query::PaginationQuery;
    ///
    /// let pagination = PaginationQuery::new(3, 20);
    /// assert_eq!(pagination.get_offset(), 40); // (3-1) * 20
    ///
    /// let first_page = PaginationQuery::new(1, 20);
    /// assert_eq!(first_page.get_offset(), 0); // (1-1) * 20
    /// ```
    pub fn get_offset(&self) -> u32 {
        (self.page - 1) * self.per_page
    }

    /// Gets the limit for database queries (same as per_page).
    ///
    /// This is provided for API consistency and clarity in database queries.
    ///
    /// # Example
    ///
    /// ```
    /// use avinapi::query::PaginationQuery;
    ///
    /// let pagination = PaginationQuery::new(2, 15);
    /// assert_eq!(pagination.get_limit(), 15);
    /// ```
    pub fn get_limit(&self) -> u32 {
        self.per_page
    }

    /// Creates pagination metadata for a given total count.
    ///
    /// # Example
    ///
    /// ```
    /// use avinapi::query::PaginationQuery;
    ///
    /// let pagination = PaginationQuery::new(2, 20);
    /// let meta = pagination.create_meta(85);
    ///
    /// assert_eq!(meta.current_page, 2);
    /// assert_eq!(meta.per_page, 20);
    /// assert_eq!(meta.total_items, 85);
    /// assert_eq!(meta.total_pages, 5); // ceil(85/20) = 5
    /// assert_eq!(meta.has_next, true);
    /// assert_eq!(meta.has_prev, true);
    /// ```
    pub fn create_meta(&self, total_items: u64) -> PaginationMeta {
        PaginationMeta::new(self.page, self.per_page, total_items)
    }
}

/// Pagination metadata for API responses.
///
/// Contains all information needed for clients to understand and navigate
/// through paginated results.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaginationMeta {
    /// Current page number (1-based)
    #[schema(example = 2, minimum = 1)]
    pub current_page: u32,

    /// Number of items per page
    #[schema(example = 20, minimum = 1, maximum = 100)]
    pub per_page: u32,

    /// Total number of items across all pages
    #[schema(example = 85)]
    pub total_items: u64,

    /// Total number of pages
    #[schema(example = 5)]
    pub total_pages: u32,

    /// Whether there is a next page
    #[schema(example = true)]
    pub has_next: bool,

    /// Whether there is a previous page
    #[schema(example = true)]
    pub has_prev: bool,

    /// First item index on current page (1-based)
    #[schema(example = 21)]
    pub from: u64,

    /// Last item index on current page (1-based)
    #[schema(example = 40)]
    pub to: u64,
}

impl PaginationMeta {
    /// Creates pagination metadata from pagination parameters and total count.
    ///
    /// # Example
    ///
    /// ```
    /// use avinapi::query::PaginationMeta;
    ///
    /// let meta = PaginationMeta::new(3, 10, 25);
    /// assert_eq!(meta.current_page, 3);
    /// assert_eq!(meta.total_pages, 3); // ceil(25/10)
    /// assert_eq!(meta.from, 21); // (3-1)*10 + 1
    /// assert_eq!(meta.to, 25);   // min(3*10, 25)
    /// assert_eq!(meta.has_next, false); // page 3 is last
    /// assert_eq!(meta.has_prev, true);  // page 3 has previous
    /// ```
    pub fn new(current_page: u32, per_page: u32, total_items: u64) -> Self {
        let total_pages = ((total_items as f64) / (per_page as f64)).ceil() as u32;
        let has_next = current_page < total_pages;
        let has_prev = current_page > 1;

        let offset = (current_page - 1) as u64 * per_page as u64;
        let from = if total_items > 0 { offset + 1 } else { 0 };
        let to = std::cmp::min(offset + per_page as u64, total_items);

        Self {
            current_page,
            per_page,
            total_items,
            total_pages,
            has_next,
            has_prev,
            from,
            to,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_query_default() {
        let pagination = PaginationQuery::default();
        assert_eq!(pagination.page, 1);
        assert_eq!(pagination.per_page, 20);
    }

    #[test]
    fn test_pagination_query_new() {
        let pagination = PaginationQuery::new(3, 50);
        assert_eq!(pagination.page, 3);
        assert_eq!(pagination.per_page, 50);
    }

    #[test]
    fn test_pagination_query_getters() {
        let pagination = PaginationQuery::new(2, 15);
        assert_eq!(pagination.get_page(), 2);
        assert_eq!(pagination.get_per_page(), 15);
    }

    #[test]
    fn test_pagination_query_offset_limit() {
        let pagination = PaginationQuery::new(3, 20);
        assert_eq!(pagination.get_offset(), 40); // (3-1) * 20
        assert_eq!(pagination.get_limit(), 20);

        let first_page = PaginationQuery::new(1, 20);
        assert_eq!(first_page.get_offset(), 0); // (1-1) * 20
    }

    #[test]
    fn test_pagination_meta_creation() {
        let meta = PaginationMeta::new(2, 10, 25);
        assert_eq!(meta.current_page, 2);
        assert_eq!(meta.per_page, 10);
        assert_eq!(meta.total_items, 25);
        assert_eq!(meta.total_pages, 3); // ceil(25/10)
        assert_eq!(meta.has_next, true); // page 2 of 3
        assert_eq!(meta.has_prev, true); // page 2
        assert_eq!(meta.from, 11); // (2-1)*10 + 1
        assert_eq!(meta.to, 20); // min(2*10, 25)
    }

    #[test]
    fn test_pagination_meta_edge_cases() {
        // First page
        let first = PaginationMeta::new(1, 10, 25);
        assert_eq!(first.has_prev, false);
        assert_eq!(first.has_next, true);
        assert_eq!(first.from, 1);
        assert_eq!(first.to, 10);

        // Last page
        let last = PaginationMeta::new(3, 10, 25);
        assert_eq!(last.has_prev, true);
        assert_eq!(last.has_next, false);
        assert_eq!(last.from, 21);
        assert_eq!(last.to, 25);

        // Empty result
        let empty = PaginationMeta::new(1, 10, 0);
        assert_eq!(empty.total_pages, 0);
        assert_eq!(empty.has_prev, false);
        assert_eq!(empty.has_next, false);
        assert_eq!(empty.from, 0);
        assert_eq!(empty.to, 0);
    }

    #[test]
    fn test_pagination_meta_exact_page_boundaries() {
        // Exactly divisible
        let exact = PaginationMeta::new(2, 10, 20);
        assert_eq!(exact.total_pages, 2);
        assert_eq!(exact.has_next, false);
        assert_eq!(exact.to, 20);
    }

    #[test]
    fn test_create_meta_from_pagination_query() {
        let pagination = PaginationQuery::new(2, 20);
        let meta = pagination.create_meta(85);

        assert_eq!(meta.current_page, 2);
        assert_eq!(meta.per_page, 20);
        assert_eq!(meta.total_items, 85);
        assert_eq!(meta.total_pages, 5); // ceil(85/20) = 5
        assert!(meta.has_next);
        assert!(meta.has_prev);
    }

    #[test]
    fn test_pagination_validation_requirements() {
        // Test that the struct has the required validation attributes
        // This is verified at compile time, but we can test edge cases
        let pagination = PaginationQuery::new(1, 1);
        assert_eq!(pagination.get_offset(), 0);
        assert_eq!(pagination.get_limit(), 1);

        let max_pagination = PaginationQuery::new(1, 100);
        assert_eq!(max_pagination.get_limit(), 100);
    }
}
