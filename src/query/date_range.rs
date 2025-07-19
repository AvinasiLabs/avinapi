//! Date range query parameters for filtering by date ranges.
//!
//! This module provides date range filtering functionality for API endpoints,
//! allowing clients to filter results by date ranges using query parameters.

use crate::validation::validate_date_range;
use chrono::{DateTime, NaiveDate, Utc};
#[cfg(test)]
use chrono::{NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

/// Query parameters for date range filtering.
///
/// Provides flexible date range filtering with optional start and end dates.
/// Supports both date-only and datetime formats through flexible parsing.
///
/// # Query Parameter Format
///
/// - `start_date`: Start date (optional, ISO 8601 format)
/// - `end_date`: End date (optional, ISO 8601 format)
///
/// # Supported Date Formats
///
/// - Date only: `2024-12-19`
/// - Date with time: `2024-12-19T10:30:00Z`
/// - Date with timezone: `2024-12-19T10:30:00+08:00`
///
/// # Examples
///
/// ```
/// use avinapi::query::DateRangeQuery;
/// use chrono::NaiveDate;
///
/// // Open-ended ranges
/// let after_2024 = DateRangeQuery::after(
///     NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()
/// );
/// let before_2025 = DateRangeQuery::before(
///     NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()
/// );
///
/// // Closed range
/// let q1_2024 = DateRangeQuery::between(
///     NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
///     NaiveDate::from_ymd_opt(2024, 3, 31).unwrap()
/// );
///
/// assert!(q1_2024.has_start());
/// assert!(q1_2024.has_end());
/// assert!(q1_2024.is_bounded());
/// ```
///
/// URL examples:
/// - `GET /orders?start_date=2024-01-01` - Orders from January 1st, 2024 onwards
/// - `GET /orders?end_date=2024-12-31` - Orders up to December 31st, 2024
/// - `GET /orders?start_date=2024-01-01&end_date=2024-03-31` - Q1 2024 orders
/// - `GET /orders?start_date=2024-12-19T10:30:00Z` - Orders from specific datetime
#[derive(Debug, Clone, Serialize, Deserialize, Validate, IntoParams, ToSchema)]
#[into_params(parameter_in = Query)]
#[validate(schema(function = "validate_date_range"))]
pub struct DateRangeQuery {
    /// Start date for the range (inclusive)
    ///
    /// Accepts dates in ISO 8601 format (YYYY-MM-DD).
    /// When provided, only records from this date onwards will be included.
    #[param(example = "2024-01-01", default = default_start_date)]
    pub start_date: Option<NaiveDate>,

    /// End date for the range (inclusive)
    ///
    /// Accepts dates in ISO 8601 format (YYYY-MM-DD).
    /// When provided, only records up to and including this date will be included.
    #[param(example = "2024-12-31", default = default_end_date)]
    pub end_date: Option<NaiveDate>,
}

/// Default start date (30 days ago from current date)
fn default_start_date() -> String {
    use chrono::{Days, Local};
    Local::now()
        .date_naive()
        .checked_sub_days(Days::new(30))
        .unwrap_or_else(|| Local::now().date_naive())
        .format("%Y-%m-%d")
        .to_string()
}

/// Default end date (current date)
fn default_end_date() -> String {
    use chrono::Local;
    Local::now().date_naive().format("%Y-%m-%d").to_string()
}

impl Default for DateRangeQuery {
    fn default() -> Self {
        Self {
            start_date: None,
            end_date: None,
        }
    }
}

impl DateRangeQuery {
    /// Creates a new empty date range (no filtering).
    ///
    /// # Example
    ///
    /// ```
    /// use avinapi::query::DateRangeQuery;
    ///
    /// let range = DateRangeQuery::new();
    /// assert!(!range.has_start());
    /// assert!(!range.has_end());
    /// assert!(!range.is_bounded());
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a date range that filters for dates after (and including) the given start date.
    ///
    /// # Example
    ///
    /// ```
    /// use avinapi::query::DateRangeQuery;
    /// use chrono::NaiveDate;
    ///
    /// let date = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
    /// let range = DateRangeQuery::after(date);
    ///
    /// assert_eq!(range.start_date, Some(date));
    /// assert_eq!(range.end_date, None);
    /// assert!(range.has_start());
    /// assert!(!range.has_end());
    /// ```
    pub fn after(start_date: NaiveDate) -> Self {
        Self {
            start_date: Some(start_date),
            end_date: None,
        }
    }

    /// Creates a date range that filters for dates before (and including) the given end date.
    ///
    /// # Example
    ///
    /// ```
    /// use avinapi::query::DateRangeQuery;
    /// use chrono::NaiveDate;
    ///
    /// let date = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
    /// let range = DateRangeQuery::before(date);
    ///
    /// assert_eq!(range.start_date, None);
    /// assert_eq!(range.end_date, Some(date));
    /// assert!(!range.has_start());
    /// assert!(range.has_end());
    /// ```
    pub fn before(end_date: NaiveDate) -> Self {
        Self {
            start_date: None,
            end_date: Some(end_date),
        }
    }

    /// Creates a date range between two dates (both inclusive).
    ///
    /// # Example
    ///
    /// ```
    /// use avinapi::query::DateRangeQuery;
    /// use chrono::NaiveDate;
    ///
    /// let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    /// let end = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
    /// let range = DateRangeQuery::between(start, end);
    ///
    /// assert_eq!(range.start_date, Some(start));
    /// assert_eq!(range.end_date, Some(end));
    /// assert!(range.is_bounded());
    /// ```
    pub fn between(start_date: NaiveDate, end_date: NaiveDate) -> Self {
        Self {
            start_date: Some(start_date),
            end_date: Some(end_date),
        }
    }

    /// Checks if the range has a start date.
    ///
    /// # Example
    ///
    /// ```
    /// use avinapi::query::DateRangeQuery;
    /// use chrono::NaiveDate;
    ///
    /// let with_start = DateRangeQuery::after(
    ///     NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()
    /// );
    /// let without_start = DateRangeQuery::before(
    ///     NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()
    /// );
    ///
    /// assert!(with_start.has_start());
    /// assert!(!without_start.has_start());
    /// ```
    pub fn has_start(&self) -> bool {
        self.start_date.is_some()
    }

    /// Checks if the range has an end date.
    ///
    /// # Example
    ///
    /// ```
    /// use avinapi::query::DateRangeQuery;
    /// use chrono::NaiveDate;
    ///
    /// let with_end = DateRangeQuery::before(
    ///     NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()
    /// );
    /// let without_end = DateRangeQuery::after(
    ///     NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()
    /// );
    ///
    /// assert!(with_end.has_end());
    /// assert!(!without_end.has_end());
    /// ```
    pub fn has_end(&self) -> bool {
        self.end_date.is_some()
    }

    /// Checks if the range is bounded (has both start and end dates).
    ///
    /// # Example
    ///
    /// ```
    /// use avinapi::query::DateRangeQuery;
    /// use chrono::NaiveDate;
    ///
    /// let bounded = DateRangeQuery::between(
    ///     NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
    ///     NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()
    /// );
    /// let unbounded = DateRangeQuery::after(
    ///     NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()
    /// );
    ///
    /// assert!(bounded.is_bounded());
    /// assert!(!unbounded.is_bounded());
    /// ```
    pub fn is_bounded(&self) -> bool {
        self.start_date.is_some() && self.end_date.is_some()
    }

    /// Checks if the range is empty (has no start or end date).
    ///
    /// # Example
    ///
    /// ```
    /// use avinapi::query::DateRangeQuery;
    /// use chrono::NaiveDate;
    ///
    /// let empty = DateRangeQuery::new();
    /// let with_start = DateRangeQuery::after(
    ///     NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()
    /// );
    ///
    /// assert!(empty.is_empty());
    /// assert!(!with_start.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.start_date.is_none() && self.end_date.is_none()
    }

    /// Gets the start date if present.
    ///
    /// # Example
    ///
    /// ```
    /// use avinapi::query::DateRangeQuery;
    /// use chrono::NaiveDate;
    ///
    /// let date = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
    /// let range = DateRangeQuery::after(date);
    ///
    /// assert_eq!(range.get_start(), Some(date));
    /// ```
    pub fn get_start(&self) -> Option<NaiveDate> {
        self.start_date
    }

    /// Gets the end date if present.
    ///
    /// # Example
    ///
    /// ```
    /// use avinapi::query::DateRangeQuery;
    /// use chrono::NaiveDate;
    ///
    /// let date = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
    /// let range = DateRangeQuery::before(date);
    ///
    /// assert_eq!(range.get_end(), Some(date));
    /// ```
    pub fn get_end(&self) -> Option<NaiveDate> {
        self.end_date
    }

    /// Converts the date range to start and end DateTime<Utc> for database queries.
    ///
    /// Start date is converted to the beginning of the day (00:00:00 UTC).
    /// End date is converted to the end of the day (23:59:59.999 UTC).
    ///
    /// # Example
    ///
    /// ```
    /// use avinapi::query::DateRangeQuery;
    /// use chrono::{NaiveDate, TimeZone, Utc};
    ///
    /// let start = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
    /// let end = NaiveDate::from_ymd_opt(2024, 6, 16).unwrap();
    /// let range = DateRangeQuery::between(start, end);
    ///
    /// let (start_dt, end_dt) = range.to_datetime_range();
    ///
    /// assert_eq!(
    ///     start_dt,
    ///     Some(Utc.with_ymd_and_hms(2024, 6, 15, 0, 0, 0).unwrap())
    /// );
    /// assert_eq!(
    ///     end_dt,
    ///     Some(Utc.with_ymd_and_hms(2024, 6, 16, 23, 59, 59).unwrap())
    /// );
    /// ```
    pub fn to_datetime_range(&self) -> (Option<DateTime<Utc>>, Option<DateTime<Utc>>) {
        let start_dt = self.start_date.map(|date| {
            date.and_hms_opt(0, 0, 0)
                .unwrap()
                .and_local_timezone(Utc)
                .unwrap()
        });

        let end_dt = self.end_date.map(|date| {
            date.and_hms_opt(23, 59, 59)
                .unwrap()
                .and_local_timezone(Utc)
                .unwrap()
        });

        (start_dt, end_dt)
    }

    /// Checks if a given date falls within this range.
    ///
    /// # Example
    ///
    /// ```
    /// use avinapi::query::DateRangeQuery;
    /// use chrono::NaiveDate;
    ///
    /// let range = DateRangeQuery::between(
    ///     NaiveDate::from_ymd_opt(2024, 6, 1).unwrap(),
    ///     NaiveDate::from_ymd_opt(2024, 6, 30).unwrap()
    /// );
    ///
    /// let june_15 = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
    /// let july_1 = NaiveDate::from_ymd_opt(2024, 7, 1).unwrap();
    ///
    /// assert!(range.contains(june_15));
    /// assert!(!range.contains(july_1));
    /// ```
    pub fn contains(&self, date: NaiveDate) -> bool {
        let after_start = self.start_date.map_or(true, |start| date >= start);
        let before_end = self.end_date.map_or(true, |end| date <= end);
        after_start && before_end
    }
}

/// Date range information for API responses.
///
/// Provides metadata about the date range used in a query,
/// useful for API responses and client-side display.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DateRangeInfo {
    /// Start date of the range (if specified)
    #[schema(example = "2024-01-01")]
    pub start_date: Option<NaiveDate>,

    /// End date of the range (if specified)
    #[schema(example = "2024-12-31")]
    pub end_date: Option<NaiveDate>,

    /// Whether the range is bounded (has both start and end)
    #[schema(example = true)]
    pub is_bounded: bool,

    /// Human-readable description of the range
    #[schema(example = "January 1, 2024 to December 31, 2024")]
    pub description: String,
}

impl From<&DateRangeQuery> for DateRangeInfo {
    fn from(query: &DateRangeQuery) -> Self {
        let description = match (query.start_date, query.end_date) {
            (Some(start), Some(end)) => {
                if start == end {
                    format!("{}", start.format("%B %-d, %Y"))
                } else {
                    format!(
                        "{} to {}",
                        start.format("%B %-d, %Y"),
                        end.format("%B %-d, %Y")
                    )
                }
            }
            (Some(start), None) => format!("From {}", start.format("%B %-d, %Y")),
            (None, Some(end)) => format!("Until {}", end.format("%B %-d, %Y")),
            (None, None) => "No date filter".to_string(),
        };

        Self {
            start_date: query.start_date,
            end_date: query.end_date,
            is_bounded: query.is_bounded(),
            description,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_date_range_query_default() {
        let range = DateRangeQuery::default();
        assert_eq!(range.start_date, None);
        assert_eq!(range.end_date, None);
        assert!(!range.has_start());
        assert!(!range.has_end());
        assert!(!range.is_bounded());
        assert!(range.is_empty());
    }

    #[test]
    fn test_date_range_query_new() {
        let range = DateRangeQuery::new();
        assert_eq!(range.start_date, None);
        assert_eq!(range.end_date, None);
    }

    #[test]
    fn test_date_range_after() {
        let date = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
        let range = DateRangeQuery::after(date);

        assert_eq!(range.start_date, Some(date));
        assert_eq!(range.end_date, None);
        assert!(range.has_start());
        assert!(!range.has_end());
        assert!(!range.is_bounded());
        assert!(!range.is_empty());
    }

    #[test]
    fn test_date_range_before() {
        let date = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
        let range = DateRangeQuery::before(date);

        assert_eq!(range.start_date, None);
        assert_eq!(range.end_date, Some(date));
        assert!(!range.has_start());
        assert!(range.has_end());
        assert!(!range.is_bounded());
        assert!(!range.is_empty());
    }

    #[test]
    fn test_date_range_between() {
        let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
        let range = DateRangeQuery::between(start, end);

        assert_eq!(range.start_date, Some(start));
        assert_eq!(range.end_date, Some(end));
        assert!(range.has_start());
        assert!(range.has_end());
        assert!(range.is_bounded());
        assert!(!range.is_empty());
    }

    #[test]
    fn test_getters() {
        let start = NaiveDate::from_ymd_opt(2024, 6, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 6, 30).unwrap();
        let range = DateRangeQuery::between(start, end);

        assert_eq!(range.get_start(), Some(start));
        assert_eq!(range.get_end(), Some(end));
    }

    #[test]
    fn test_to_datetime_range() {
        let start = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 6, 16).unwrap();
        let range = DateRangeQuery::between(start, end);

        let (start_dt, end_dt) = range.to_datetime_range();

        assert_eq!(
            start_dt,
            Some(DateTime::<Utc>::from_naive_utc_and_offset(
                NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(2024, 6, 15).unwrap(),
                    NaiveTime::from_hms_opt(0, 0, 0).unwrap()
                ),
                Utc
            ))
        );
        assert_eq!(
            end_dt,
            Some(DateTime::<Utc>::from_naive_utc_and_offset(
                NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(2024, 6, 16).unwrap(),
                    NaiveTime::from_hms_opt(23, 59, 59).unwrap()
                ),
                Utc
            ))
        );
    }

    #[test]
    fn test_to_datetime_range_partial() {
        // Only start date
        let start_only = DateRangeQuery::after(NaiveDate::from_ymd_opt(2024, 6, 15).unwrap());
        let (start_dt, end_dt) = start_only.to_datetime_range();
        assert!(start_dt.is_some());
        assert!(end_dt.is_none());

        // Only end date
        let end_only = DateRangeQuery::before(NaiveDate::from_ymd_opt(2024, 6, 16).unwrap());
        let (start_dt, end_dt) = end_only.to_datetime_range();
        assert!(start_dt.is_none());
        assert!(end_dt.is_some());
    }

    #[test]
    fn test_contains() {
        let range = DateRangeQuery::between(
            NaiveDate::from_ymd_opt(2024, 6, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 6, 30).unwrap(),
        );

        // Within range
        assert!(range.contains(NaiveDate::from_ymd_opt(2024, 6, 15).unwrap()));
        assert!(range.contains(NaiveDate::from_ymd_opt(2024, 6, 1).unwrap())); // Start boundary
        assert!(range.contains(NaiveDate::from_ymd_opt(2024, 6, 30).unwrap())); // End boundary

        // Outside range
        assert!(!range.contains(NaiveDate::from_ymd_opt(2024, 5, 31).unwrap()));
        assert!(!range.contains(NaiveDate::from_ymd_opt(2024, 7, 1).unwrap()));
    }

    #[test]
    fn test_contains_open_ranges() {
        // After start date only
        let after_range = DateRangeQuery::after(NaiveDate::from_ymd_opt(2024, 6, 1).unwrap());
        assert!(after_range.contains(NaiveDate::from_ymd_opt(2024, 6, 15).unwrap()));
        assert!(after_range.contains(NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()));
        assert!(!after_range.contains(NaiveDate::from_ymd_opt(2024, 5, 31).unwrap()));

        // Before end date only
        let before_range = DateRangeQuery::before(NaiveDate::from_ymd_opt(2024, 6, 30).unwrap());
        assert!(before_range.contains(NaiveDate::from_ymd_opt(2024, 6, 15).unwrap()));
        assert!(before_range.contains(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()));
        assert!(!before_range.contains(NaiveDate::from_ymd_opt(2024, 7, 1).unwrap()));
    }

    #[test]
    fn test_validation_valid_ranges() {
        // Valid ranges should pass validation
        let same_day = DateRangeQuery::between(
            NaiveDate::from_ymd_opt(2024, 6, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 6, 15).unwrap(),
        );
        assert!(validate_date_range(&same_day).is_ok());

        let normal_range = DateRangeQuery::between(
            NaiveDate::from_ymd_opt(2024, 6, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 6, 30).unwrap(),
        );
        assert!(validate_date_range(&normal_range).is_ok());

        let partial_ranges = vec![
            DateRangeQuery::after(NaiveDate::from_ymd_opt(2024, 6, 1).unwrap()),
            DateRangeQuery::before(NaiveDate::from_ymd_opt(2024, 6, 30).unwrap()),
            DateRangeQuery::new(),
        ];

        for range in partial_ranges {
            assert!(validate_date_range(&range).is_ok());
        }
    }

    #[test]
    fn test_validation_invalid_range() {
        // Invalid range (start after end) should fail validation
        let invalid_range = DateRangeQuery::between(
            NaiveDate::from_ymd_opt(2024, 6, 30).unwrap(),
            NaiveDate::from_ymd_opt(2024, 6, 1).unwrap(),
        );

        let result = validate_date_range(&invalid_range);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code, "start_date_after_end_date");
    }

    #[test]
    fn test_date_range_info_conversion() {
        // Bounded range
        let bounded = DateRangeQuery::between(
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
        );
        let info = DateRangeInfo::from(&bounded);
        assert!(info.is_bounded);
        assert!(
            info.description
                .contains("January 1, 2024 to December 31, 2024")
        );

        // Same day range
        let same_day = DateRangeQuery::between(
            NaiveDate::from_ymd_opt(2024, 6, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 6, 15).unwrap(),
        );
        let info = DateRangeInfo::from(&same_day);
        assert!(info.description.contains("June 15, 2024"));
        assert!(!info.description.contains("to"));

        // Start only
        let start_only = DateRangeQuery::after(NaiveDate::from_ymd_opt(2024, 6, 1).unwrap());
        let info = DateRangeInfo::from(&start_only);
        assert!(!info.is_bounded);
        assert!(info.description.starts_with("From"));

        // End only
        let end_only = DateRangeQuery::before(NaiveDate::from_ymd_opt(2024, 6, 30).unwrap());
        let info = DateRangeInfo::from(&end_only);
        assert!(info.description.starts_with("Until"));

        // Empty range
        let empty = DateRangeQuery::new();
        let info = DateRangeInfo::from(&empty);
        assert_eq!(info.description, "No date filter");
    }

    #[test]
    fn test_serialization_deserialization() {
        let range = DateRangeQuery::between(
            NaiveDate::from_ymd_opt(2024, 6, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 6, 30).unwrap(),
        );

        let info = DateRangeInfo::from(&range);

        // Test that structures can be serialized
        let _range_serialized =
            serde_json::to_string(&range).expect("Should serialize DateRangeQuery");
        let _info_serialized =
            serde_json::to_string(&info).expect("Should serialize DateRangeInfo");
    }

    #[test]
    fn test_default_value_functions() {
        // Test that default functions return valid date strings
        let start_default = default_start_date();
        let end_default = default_end_date();

        // Should be valid ISO 8601 date format
        assert_eq!(start_default.len(), 10); // YYYY-MM-DD format
        assert_eq!(end_default.len(), 10);

        // Should be parseable as NaiveDate
        let start_parsed = NaiveDate::parse_from_str(&start_default, "%Y-%m-%d");
        let end_parsed = NaiveDate::parse_from_str(&end_default, "%Y-%m-%d");

        assert!(start_parsed.is_ok());
        assert!(end_parsed.is_ok());

        // Start should be before or equal to end
        let start_date = start_parsed.unwrap();
        let end_date = end_parsed.unwrap();
        assert!(start_date <= end_date);
    }
}
