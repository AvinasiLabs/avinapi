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
/// use avinasi_web::query::PaginationQuery;
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
    /// use avinasi_web::query::PaginationQuery;
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
    /// use avinasi_web::query::PaginationQuery;
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
    /// use avinasi_web::query::PaginationQuery;
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
    /// use avinasi_web::query::PaginationQuery;
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
    /// use avinasi_web::query::PaginationQuery;
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
    /// use avinasi_web::query::PaginationQuery;
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
    /// use avinasi_web::query::PaginationMeta;
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

/// Paginated data response wrapper.
///
/// Combines the actual data with pagination metadata for API responses.
/// This follows the unified response pattern with consistent structure.
///
/// # Generic Parameters
///
/// - `T`: The type of items in the data array
///
/// # JSON Structure
///
/// ```json
/// {
///   "data": [
///     {"id": 1, "name": "Item 1"},
///     {"id": 2, "name": "Item 2"}
///   ],
///   "meta": {
///     "current_page": 1,
///     "per_page": 20,
///     "total_items": 100,
///     "total_pages": 5,
///     "has_next": true,
///     "has_prev": false,
///     "from": 1,
///     "to": 20
///   }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaginatedData<T> {
    /// The paginated items
    pub data: Vec<T>,

    /// Pagination metadata
    pub meta: PaginationMeta,
}

impl<T> PaginatedData<T> {
    /// Creates a new paginated data response.
    ///
    /// # Arguments
    ///
    /// * `data` - The items for the current page
    /// * `pagination` - The pagination query parameters
    /// * `total_items` - Total number of items across all pages
    ///
    /// # Example
    ///
    /// ```
    /// use avinasi_web::query::{PaginatedData, PaginationQuery};
    ///
    /// let items = vec!["item1", "item2", "item3"];
    /// let pagination = PaginationQuery::new(1, 10);
    /// let response = PaginatedData::new(items, &pagination, 25);
    ///
    /// assert_eq!(response.data.len(), 3);
    /// assert_eq!(response.meta.total_items, 25);
    /// assert_eq!(response.meta.current_page, 1);
    /// ```
    pub fn new(data: Vec<T>, pagination: &PaginationQuery, total_items: u64) -> Self {
        let meta = pagination.create_meta(total_items);
        Self { data, meta }
    }

    /// Creates a new paginated data response with explicit metadata.
    ///
    /// Use this when you already have computed pagination metadata.
    ///
    /// # Example
    ///
    /// ```
    /// use avinasi_web::query::{PaginatedData, PaginationMeta};
    ///
    /// let items = vec![1, 2, 3];
    /// let meta = PaginationMeta::new(2, 10, 50);
    /// let response = PaginatedData::with_meta(items, meta);
    ///
    /// assert_eq!(response.data.len(), 3);
    /// assert_eq!(response.meta.current_page, 2);
    /// ```
    pub fn with_meta(data: Vec<T>, meta: PaginationMeta) -> Self {
        Self { data, meta }
    }

    /// Gets the number of items in the current page.
    ///
    /// # Example
    ///
    /// ```
    /// use avinasi_web::query::{PaginatedData, PaginationQuery};
    ///
    /// let items = vec!["a", "b", "c"];
    /// let pagination = PaginationQuery::new(1, 10);
    /// let response = PaginatedData::new(items, &pagination, 100);
    ///
    /// assert_eq!(response.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Checks if the current page has no items.
    ///
    /// # Example
    ///
    /// ```
    /// use avinasi_web::query::{PaginatedData, PaginationQuery};
    ///
    /// let empty_items: Vec<String> = vec![];
    /// let pagination = PaginationQuery::new(1, 10);
    /// let response = PaginatedData::new(empty_items, &pagination, 0);
    ///
    /// assert!(response.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Maps the data items to a different type while preserving pagination metadata.
    ///
    /// # Example
    ///
    /// ```
    /// use avinasi_web::query::{PaginatedData, PaginationQuery};
    ///
    /// let numbers = vec![1, 2, 3];
    /// let pagination = PaginationQuery::new(1, 10);
    /// let response = PaginatedData::new(numbers, &pagination, 100);
    ///
    /// let strings = response.map(|n| format!("number_{}", n));
    /// assert_eq!(strings.data, vec!["number_1", "number_2", "number_3"]);
    /// assert_eq!(strings.meta.total_items, 100); // Metadata preserved
    /// ```
    pub fn map<U, F>(self, f: F) -> PaginatedData<U>
    where
        F: FnMut(T) -> U,
    {
        PaginatedData {
            data: self.data.into_iter().map(f).collect(),
            meta: self.meta,
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
    fn test_paginated_data_creation() {
        let items = vec!["item1", "item2", "item3"];
        let pagination = PaginationQuery::new(1, 10);
        let response = PaginatedData::new(items, &pagination, 25);

        assert_eq!(response.data.len(), 3);
        assert_eq!(response.meta.total_items, 25);
        assert_eq!(response.meta.current_page, 1);
        assert_eq!(response.len(), 3);
        assert!(!response.is_empty());
    }

    #[test]
    fn test_paginated_data_with_meta() {
        let items = vec![1, 2, 3];
        let meta = PaginationMeta::new(2, 10, 50);
        let response = PaginatedData::with_meta(items, meta);

        assert_eq!(response.data.len(), 3);
        assert_eq!(response.meta.current_page, 2);
        assert_eq!(response.meta.total_items, 50);
    }

    #[test]
    fn test_paginated_data_empty() {
        let empty_items: Vec<String> = vec![];
        let pagination = PaginationQuery::new(1, 10);
        let response = PaginatedData::new(empty_items, &pagination, 0);

        assert!(response.is_empty());
        assert_eq!(response.len(), 0);
        assert_eq!(response.meta.total_items, 0);
    }

    #[test]
    fn test_paginated_data_map() {
        let numbers = vec![1, 2, 3];
        let pagination = PaginationQuery::new(1, 10);
        let response = PaginatedData::new(numbers, &pagination, 100);

        let strings = response.map(|n| format!("number_{}", n));
        assert_eq!(strings.data, vec!["number_1", "number_2", "number_3"]);
        assert_eq!(strings.meta.total_items, 100);
        assert_eq!(strings.meta.current_page, 1);
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

    #[test]
    fn test_serialization_deserialization() {
        let pagination = PaginationQuery::new(2, 15);
        let meta = pagination.create_meta(100);
        let data = vec!["test1", "test2"];
        let paginated = PaginatedData::new(data, &pagination, 100);

        // Test that structures can be serialized (this verifies serde annotations)
        let _serialized = serde_json::to_string(&paginated).expect("Should serialize");
        let _meta_serialized = serde_json::to_string(&meta).expect("Should serialize");
    }
}
