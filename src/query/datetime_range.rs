//! DateTime range query parameters for precise datetime filtering.
//!
//! This module provides datetime range filtering functionality for API endpoints,
//! allowing clients to filter results by precise datetime ranges with timezone support.
//! Unlike DateRangeQuery which works with dates only, this supports full datetime
//! precision including hours, minutes, seconds, and timezone information.

use crate::prelude::validate_datetime_range;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

/// Query parameters for datetime range filtering with timezone support.
///
/// Provides precise datetime range filtering with full timezone support.
/// Accepts RFC 3339 formatted datetime strings and converts them for database queries.
///
/// # Supported Formats
///
/// - UTC datetime: `2024-12-19T10:30:00Z`
/// - Timezone-aware: `2024-12-19T10:30:00+08:00`
/// - Timezone-aware (negative): `2024-12-19T10:30:00-05:00`
///
/// # Query Parameter Format
///
/// - `start`: Start datetime (optional, RFC 3339 format)
/// - `end`: End datetime (optional, RFC 3339 format)
///
/// # URL Examples
///
/// - `GET /events?start=2024-12-19T10:30:00Z&end=2024-12-19T18:00:00Z`
/// - `GET /events?start=2024-12-19T10:30:00+08:00&end=2024-12-19T18:00:00+08:00`
/// - `GET /events?start=2024-12-19T00:00:00Z` - Events from specific datetime onwards
/// - `GET /events?end=2024-12-19T23:59:59Z` - Events up to specific datetime
#[derive(Debug, Clone, Serialize, Deserialize, Validate, IntoParams, ToSchema)]
#[into_params(parameter_in = Query)]
#[validate(schema(function = "validate_datetime_range"))]
pub struct DateTimeRangeQuery {
    /// Start datetime for the range (inclusive)
    ///
    /// Accepts RFC 3339 format with timezone information.
    /// When provided, only records from this datetime onwards will be included.
    ///
    /// Examples:
    /// - "2024-12-19T10:30:00Z" (UTC)
    /// - "2024-12-19T10:30:00+08:00" (UTC+8)
    /// - "2024-12-19T10:30:00-05:00" (UTC-5)
    #[param(example = "2024-12-19T10:30:00+08:00", default = default_start_datetime)]
    pub start_datetime: Option<String>,

    /// End datetime for the range (exclusive)
    ///
    /// Accepts RFC 3339 format with timezone information.
    /// When provided, only records before this datetime will be included.
    /// Note: Uses exclusive end (< end_datetime) for precise range queries.
    ///
    /// Examples:
    /// - "2024-12-19T18:00:00Z" (UTC)
    /// - "2024-12-19T18:00:00+08:00" (UTC+8)
    /// - "2024-12-19T18:00:00-05:00" (UTC-5)
    #[param(example = "2024-12-19T18:00:00+08:00", default = default_end_datetime)]
    pub end_datetime: Option<String>,
}

/// Default start datetime (current time)
fn default_start_datetime() -> String {
    Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

/// Default end datetime (24 hours from now)
fn default_end_datetime() -> String {
    let end_time = Utc::now() + chrono::Duration::hours(24);
    end_time.format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

impl Default for DateTimeRangeQuery {
    fn default() -> Self {
        Self {
            start_datetime: None,
            end_datetime: None,
        }
    }
}

impl DateTimeRangeQuery {
    /// Creates a new empty datetime range query.
    ///
    /// # Example
    ///
    /// ```
    /// use avinapi::query::DateTimeRangeQuery;
    ///
    /// let range = DateTimeRangeQuery::new();
    /// assert!(range.is_empty());
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a datetime range query with only a start datetime.
    ///
    /// # Arguments
    ///
    /// * `start` - RFC 3339 formatted datetime string
    ///
    /// # Example
    ///
    /// ```
    /// use avinapi::query::DateTimeRangeQuery;
    ///
    /// let range = DateTimeRangeQuery::after("2024-12-19T10:30:00Z");
    /// assert!(range.has_start());
    /// assert!(!range.has_end());
    /// ```
    pub fn after(start: impl Into<String>) -> Self {
        Self {
            start_datetime: Some(start.into()),
            end_datetime: None,
        }
    }

    /// Creates a datetime range query with only an end datetime.
    ///
    /// # Arguments
    ///
    /// * `end` - RFC 3339 formatted datetime string
    ///
    /// # Example
    ///
    /// ```
    /// use avinapi::query::DateTimeRangeQuery;
    ///
    /// let range = DateTimeRangeQuery::before("2024-12-19T18:00:00Z");
    /// assert!(!range.has_start());
    /// assert!(range.has_end());
    /// ```
    pub fn before(end: impl Into<String>) -> Self {
        Self {
            start_datetime: None,
            end_datetime: Some(end.into()),
        }
    }

    /// Creates a datetime range query with both start and end datetimes.
    ///
    /// # Arguments
    ///
    /// * `start` - RFC 3339 formatted datetime string
    /// * `end` - RFC 3339 formatted datetime string
    ///
    /// # Example
    ///
    /// ```
    /// use avinapi::query::DateTimeRangeQuery;
    ///
    /// let range = DateTimeRangeQuery::between(
    ///     "2024-12-19T10:30:00Z",
    ///     "2024-12-19T18:00:00Z"
    /// );
    /// assert!(range.is_bounded());
    /// ```
    pub fn between(start: impl Into<String>, end: impl Into<String>) -> Self {
        Self {
            start_datetime: Some(start.into()),
            end_datetime: Some(end.into()),
        }
    }

    /// Returns the start datetime string.
    ///
    /// # Example
    ///
    /// ```
    /// use avinapi::query::DateTimeRangeQuery;
    ///
    /// let range = DateTimeRangeQuery::after("2024-12-19T10:30:00Z");
    /// assert_eq!(range.get_start(), Some("2024-12-19T10:30:00Z"));
    /// ```
    pub fn get_start(&self) -> Option<&str> {
        self.start_datetime.as_deref()
    }

    /// Returns the end datetime string.
    ///
    /// # Example
    ///
    /// ```
    /// use avinapi::query::DateTimeRangeQuery;
    ///
    /// let range = DateTimeRangeQuery::before("2024-12-19T18:00:00Z");
    /// assert_eq!(range.get_end(), Some("2024-12-19T18:00:00Z"));
    /// ```
    pub fn get_end(&self) -> Option<&str> {
        self.end_datetime.as_deref()
    }

    /// Checks if the range has a start datetime.
    ///
    /// # Example
    ///
    /// ```
    /// use avinapi::query::DateTimeRangeQuery;
    ///
    /// let range = DateTimeRangeQuery::after("2024-12-19T10:30:00Z");
    /// assert!(range.has_start());
    /// ```
    pub fn has_start(&self) -> bool {
        self.start_datetime.is_some()
    }

    /// Checks if the range has an end datetime.
    ///
    /// # Example
    ///
    /// ```
    /// use avinapi::query::DateTimeRangeQuery;
    ///
    /// let range = DateTimeRangeQuery::before("2024-12-19T18:00:00Z");
    /// assert!(range.has_end());
    /// ```
    pub fn has_end(&self) -> bool {
        self.end_datetime.is_some()
    }

    /// Checks if the range is empty (no start or end datetime).
    ///
    /// # Example
    ///
    /// ```
    /// use avinapi::query::DateTimeRangeQuery;
    ///
    /// let range = DateTimeRangeQuery::new();
    /// assert!(range.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.start_datetime.is_none() && self.end_datetime.is_none()
    }

    /// Checks if the range is bounded (has both start and end datetimes).
    ///
    /// # Example
    ///
    /// ```
    /// use avinapi::query::DateTimeRangeQuery;
    ///
    /// let range = DateTimeRangeQuery::between(
    ///     "2024-12-19T10:30:00Z",
    ///     "2024-12-19T18:00:00Z"
    /// );
    /// assert!(range.is_bounded());
    /// ```
    pub fn is_bounded(&self) -> bool {
        self.start_datetime.is_some() && self.end_datetime.is_some()
    }

    /// Parses the start datetime to UTC.
    ///
    /// # Returns
    ///
    /// * `Ok(DateTime<Utc>)` - Parsed UTC datetime
    /// * `Err(chrono::ParseError)` - If parsing fails
    ///
    /// # Example
    ///
    /// ```
    /// use avinapi::query::DateTimeRangeQuery;
    /// use chrono::{DateTime, Utc};
    ///
    /// let range = DateTimeRangeQuery::after("2024-12-19T10:30:00+08:00");
    /// let utc_time = range.parse_start_utc().unwrap();
    /// // Converted to UTC: 2024-12-19T02:30:00Z
    /// ```
    pub fn parse_start_utc(&self) -> Result<DateTime<Utc>, chrono::ParseError> {
        match &self.start_datetime {
            Some(start_str) => {
                let parsed = DateTime::parse_from_rfc3339(start_str)?;
                Ok(parsed.with_timezone(&Utc))
            }
            None => {
                // Create a ParseError by attempting to parse invalid input
                DateTime::parse_from_rfc3339("").map(|dt| dt.with_timezone(&Utc))
            }
        }
    }

    /// Parses the end datetime to UTC.
    ///
    /// # Returns
    ///
    /// * `Ok(DateTime<Utc>)` - Parsed UTC datetime
    /// * `Err(chrono::ParseError)` - If parsing fails
    ///
    /// # Example
    ///
    /// ```
    /// use avinapi::query::DateTimeRangeQuery;
    /// use chrono::{DateTime, Utc};
    ///
    /// let range = DateTimeRangeQuery::before("2024-12-19T18:00:00+08:00");
    /// let utc_time = range.parse_end_utc().unwrap();
    /// // Converted to UTC: 2024-12-19T10:00:00Z
    /// ```
    pub fn parse_end_utc(&self) -> Result<DateTime<Utc>, chrono::ParseError> {
        match &self.end_datetime {
            Some(end_str) => {
                let parsed = DateTime::parse_from_rfc3339(end_str)?;
                Ok(parsed.with_timezone(&Utc))
            }
            None => {
                // Create a ParseError by attempting to parse invalid input
                DateTime::parse_from_rfc3339("").map(|dt| dt.with_timezone(&Utc))
            }
        }
    }

    /// Parses both start and end datetimes to UTC.
    ///
    /// # Returns
    ///
    /// * `(Option<DateTime<Utc>>, Option<DateTime<Utc>>)` - Tuple of parsed datetimes
    ///
    /// # Example
    ///
    /// ```
    /// use avinapi::query::DateTimeRangeQuery;
    ///
    /// let range = DateTimeRangeQuery::between(
    ///     "2024-12-19T10:30:00+08:00",
    ///     "2024-12-19T18:00:00+08:00"
    /// );
    /// let (start_utc, end_utc) = range.parse_utc_range();
    /// assert!(start_utc.is_some());
    /// assert!(end_utc.is_some());
    /// ```
    pub fn parse_utc_range(&self) -> (Option<DateTime<Utc>>, Option<DateTime<Utc>>) {
        let start_utc = self.start_datetime.as_ref().and_then(|s| {
            DateTime::parse_from_rfc3339(s)
                .ok()
                .map(|dt| dt.with_timezone(&Utc))
        });

        let end_utc = self.end_datetime.as_ref().and_then(|s| {
            DateTime::parse_from_rfc3339(s)
                .ok()
                .map(|dt| dt.with_timezone(&Utc))
        });

        (start_utc, end_utc)
    }

    /// Checks if a given datetime falls within the range.
    ///
    /// Uses half-open interval [start, end) - includes start, excludes end.
    ///
    /// # Arguments
    ///
    /// * `datetime` - The datetime to check
    ///
    /// # Example
    ///
    /// ```
    /// use avinapi::query::DateTimeRangeQuery;
    /// use chrono::{DateTime, Utc};
    ///
    /// let range = DateTimeRangeQuery::between(
    ///     "2024-12-19T10:00:00Z",
    ///     "2024-12-19T18:00:00Z"
    /// );
    ///
    /// let test_time = "2024-12-19T14:30:00Z".parse::<DateTime<Utc>>().unwrap();
    /// assert!(range.contains(test_time));
    /// ```
    pub fn contains(&self, datetime: DateTime<Utc>) -> bool {
        let (start_utc, end_utc) = self.parse_utc_range();

        let after_start = start_utc.map_or(true, |start| datetime >= start);
        let before_end = end_utc.map_or(true, |end| datetime < end);

        after_start && before_end
    }

    /// Validates that the range is logically correct and parseable.
    ///
    /// Checks:
    /// - Both datetimes can be parsed as RFC 3339
    /// - Start datetime is before end datetime (if both provided)
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the range is valid
    /// * `Err(String)` - If the range is invalid
    ///
    /// # Example
    ///
    /// ```
    /// use avinapi::query::DateTimeRangeQuery;
    ///
    /// let valid_range = DateTimeRangeQuery::between(
    ///     "2024-12-19T10:00:00Z",
    ///     "2024-12-19T18:00:00Z"
    /// );
    /// assert!(valid_range.validate_range().is_ok());
    ///
    /// let invalid_range = DateTimeRangeQuery::between(
    ///     "2024-12-19T18:00:00Z",
    ///     "2024-12-19T10:00:00Z"
    /// );
    /// assert!(invalid_range.validate_range().is_err());
    /// ```
    pub fn validate_range(&self) -> Result<(), String> {
        let (start_utc, end_utc) = self.parse_utc_range();

        // Check if we failed to parse any provided datetime
        if self.start_datetime.is_some() && start_utc.is_none() {
            return Err("Invalid start datetime format".to_string());
        }

        if self.end_datetime.is_some() && end_utc.is_none() {
            return Err("Invalid end datetime format".to_string());
        }

        // Check logical range
        if let (Some(start), Some(end)) = (start_utc, end_utc) {
            if start >= end {
                return Err("Start datetime must be before end datetime".to_string());
            }
        }

        Ok(())
    }

    /// Validates that the datetime range does not exceed a maximum duration.
    ///
    /// # Arguments
    ///
    /// * `max_days` - Maximum allowed duration in days
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the duration is within limits
    /// * `Err(String)` - If the duration exceeds the limit
    ///
    /// # Example
    ///
    /// ```
    /// use avinapi::query::DateTimeRangeQuery;
    ///
    /// let range = DateTimeRangeQuery::between(
    ///     "2024-12-19T10:00:00Z",
    ///     "2024-12-20T10:00:00Z"
    /// );
    /// assert!(range.validate_max_duration(7).is_ok()); // 1 day < 7 days
    /// assert!(range.validate_max_duration(0).is_err()); // 1 day > 0 days
    /// ```
    pub fn validate_max_duration(&self, max_days: u32) -> Result<(), String> {
        if let (Some(start), Some(end)) = self.parse_utc_range() {
            let duration = end.signed_duration_since(start);
            let days = duration.num_days();

            if days > max_days as i64 {
                return Err(format!(
                    "Date range duration ({} days) exceeds maximum allowed duration ({} days)",
                    days, max_days
                ));
            }
        }

        Ok(())
    }

    /// Gets the duration of the datetime range in various units.
    ///
    /// # Returns
    ///
    /// * `Some(chrono::Duration)` - If both start and end are provided and valid
    /// * `None` - If either datetime is missing or invalid
    ///
    /// # Example
    ///
    /// ```
    /// use avinapi::query::DateTimeRangeQuery;
    ///
    /// let range = DateTimeRangeQuery::between(
    ///     "2024-12-19T10:00:00Z",
    ///     "2024-12-19T18:00:00Z"
    /// );
    ///
    /// if let Some(duration) = range.get_duration() {
    ///     println!("Duration: {} hours", duration.num_hours()); // 8 hours
    /// }
    /// ```
    pub fn get_duration(&self) -> Option<chrono::Duration> {
        let (start_utc, end_utc) = self.parse_utc_range();

        if let (Some(start), Some(end)) = (start_utc, end_utc) {
            Some(end.signed_duration_since(start))
        } else {
            None
        }
    }

    /// Generates SQL WHERE clause conditions for the datetime range.
    ///
    /// # Arguments
    ///
    /// * `field_name` - Database field name to filter on
    ///
    /// # Returns
    ///
    /// * `String` - SQL WHERE conditions
    ///
    /// # Example
    ///
    /// ```
    /// use avinapi::query::DateTimeRangeQuery;
    ///
    /// let range = DateTimeRangeQuery::between(
    ///     "2024-12-19T10:00:00Z",
    ///     "2024-12-19T18:00:00Z"
    /// );
    ///
    /// let sql = range.to_sql_conditions("created_at");
    /// // Returns: "created_at >= '2024-12-19 10:00:00+00' AND created_at < '2024-12-19 18:00:00+00'"
    /// ```
    pub fn to_sql_conditions(&self, field_name: &str) -> String {
        let (start_utc, end_utc) = self.parse_utc_range();
        let mut conditions = Vec::new();

        if let Some(start) = start_utc {
            conditions.push(format!(
                "{} >= '{}'",
                field_name,
                start.format("%Y-%m-%d %H:%M:%S%z")
            ));
        }

        if let Some(end) = end_utc {
            conditions.push(format!(
                "{} < '{}'",
                field_name,
                end.format("%Y-%m-%d %H:%M:%S%z")
            ));
        }

        if conditions.is_empty() {
            "TRUE".to_string()
        } else {
            conditions.join(" AND ")
        }
    }
}

impl std::fmt::Display for DateTimeRangeQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.start_datetime, &self.end_datetime) {
            (Some(start), Some(end)) => write!(f, "{} to {}", start, end),
            (Some(start), None) => write!(f, "from {}", start),
            (None, Some(end)) => write!(f, "until {}", end),
            (None, None) => write!(f, "no datetime filter"),
        }
    }
}

/// DateTime range information for API responses.
///
/// Provides human-readable information about datetime ranges for client consumption.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DateTimeRangeInfo {
    /// Human-readable description of the range
    pub description: String,
    /// Whether the range is bounded (has both start and end)
    pub is_bounded: bool,
    /// Parsed start datetime in UTC (if available)
    pub start_utc: Option<DateTime<Utc>>,
    /// Parsed end datetime in UTC (if available)
    pub end_utc: Option<DateTime<Utc>>,
    /// Duration of the range (if bounded)
    pub duration_hours: Option<i64>,
}

impl From<&DateTimeRangeQuery> for DateTimeRangeInfo {
    fn from(query: &DateTimeRangeQuery) -> Self {
        let (start_utc, end_utc) = query.parse_utc_range();
        let duration_hours = query.get_duration().map(|d| d.num_hours());

        Self {
            description: query.to_string(),
            is_bounded: query.is_bounded(),
            start_utc,
            end_utc,
            duration_hours,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Timelike, Utc};

    #[test]
    fn test_datetime_range_query_creation() {
        let range = DateTimeRangeQuery::new();
        assert!(range.is_empty());
        assert!(!range.is_bounded());

        let after_range = DateTimeRangeQuery::after("2024-12-19T10:30:00Z");
        assert!(after_range.has_start());
        assert!(!after_range.has_end());

        let before_range = DateTimeRangeQuery::before("2024-12-19T18:00:00Z");
        assert!(!before_range.has_start());
        assert!(before_range.has_end());

        let bounded_range =
            DateTimeRangeQuery::between("2024-12-19T10:30:00Z", "2024-12-19T18:00:00Z");
        assert!(bounded_range.is_bounded());
    }

    #[test]
    fn test_datetime_parsing() {
        let range =
            DateTimeRangeQuery::between("2024-12-19T10:30:00+08:00", "2024-12-19T18:00:00+08:00");

        let start_utc = range.parse_start_utc().unwrap();
        let end_utc = range.parse_end_utc().unwrap();

        // +08:00 should convert to UTC correctly
        assert_eq!(start_utc.hour(), 2); // 10:30 +08:00 = 02:30 UTC
        assert_eq!(end_utc.hour(), 10); // 18:00 +08:00 = 10:00 UTC
    }

    #[test]
    fn test_datetime_contains() {
        let range = DateTimeRangeQuery::between("2024-12-19T10:00:00Z", "2024-12-19T18:00:00Z");

        let inside_time = "2024-12-19T14:30:00Z".parse::<DateTime<Utc>>().unwrap();
        let outside_early = "2024-12-19T08:00:00Z".parse::<DateTime<Utc>>().unwrap();
        let outside_late = "2024-12-19T20:00:00Z".parse::<DateTime<Utc>>().unwrap();
        let boundary_end = "2024-12-19T18:00:00Z".parse::<DateTime<Utc>>().unwrap();

        assert!(range.contains(inside_time));
        assert!(!range.contains(outside_early));
        assert!(!range.contains(outside_late));
        assert!(!range.contains(boundary_end)); // Exclusive end
    }

    #[test]
    fn test_datetime_validation() {
        let valid_range =
            DateTimeRangeQuery::between("2024-12-19T10:00:00Z", "2024-12-19T18:00:00Z");
        assert!(valid_range.validate_range().is_ok());

        let invalid_range =
            DateTimeRangeQuery::between("2024-12-19T18:00:00Z", "2024-12-19T10:00:00Z");
        assert!(invalid_range.validate_range().is_err());

        let invalid_format =
            DateTimeRangeQuery::between("invalid-datetime", "2024-12-19T18:00:00Z");
        assert!(invalid_format.validate_range().is_err());
    }

    #[test]
    fn test_max_duration_validation() {
        let one_day_range =
            DateTimeRangeQuery::between("2024-12-19T10:00:00Z", "2024-12-20T10:00:00Z");

        assert!(one_day_range.validate_max_duration(7).is_ok());
        assert!(one_day_range.validate_max_duration(1).is_ok());
        assert!(one_day_range.validate_max_duration(0).is_err());

        let one_week_range =
            DateTimeRangeQuery::between("2024-12-19T10:00:00Z", "2024-12-26T10:00:00Z");

        assert!(one_week_range.validate_max_duration(10).is_ok());
        assert!(one_week_range.validate_max_duration(7).is_ok());
        assert!(one_week_range.validate_max_duration(5).is_err());
    }

    #[test]
    fn test_duration_calculation() {
        let eight_hour_range =
            DateTimeRangeQuery::between("2024-12-19T10:00:00Z", "2024-12-19T18:00:00Z");

        let duration = eight_hour_range.get_duration().unwrap();
        assert_eq!(duration.num_hours(), 8);
        assert_eq!(duration.num_minutes(), 8 * 60);

        let empty_range = DateTimeRangeQuery::new();
        assert!(empty_range.get_duration().is_none());
    }

    #[test]
    fn test_sql_generation() {
        let range = DateTimeRangeQuery::between("2024-12-19T10:00:00Z", "2024-12-19T18:00:00Z");

        let sql = range.to_sql_conditions("created_at");
        assert!(sql.contains("created_at >= '2024-12-19 10:00:00"));
        assert!(sql.contains("created_at < '2024-12-19 18:00:00"));
        assert!(sql.contains(" AND "));

        let start_only = DateTimeRangeQuery::after("2024-12-19T10:00:00Z");
        let start_sql = start_only.to_sql_conditions("created_at");
        assert!(start_sql.contains(">="));
        assert!(!start_sql.contains("<"));

        let empty_range = DateTimeRangeQuery::new();
        let empty_sql = empty_range.to_sql_conditions("created_at");
        assert_eq!(empty_sql, "TRUE");
    }

    #[test]
    fn test_timezone_handling() {
        let utc_range = DateTimeRangeQuery::after("2024-12-19T10:30:00Z");
        let plus_eight_range = DateTimeRangeQuery::after("2024-12-19T18:30:00+08:00");
        let minus_five_range = DateTimeRangeQuery::after("2024-12-19T05:30:00-05:00");

        let utc_time = utc_range.parse_start_utc().unwrap();
        let plus_eight_time = plus_eight_range.parse_start_utc().unwrap();
        let minus_five_time = minus_five_range.parse_start_utc().unwrap();

        // All three should represent the same UTC time: 10:30 UTC
        assert_eq!(utc_time, plus_eight_time);
        assert_eq!(utc_time, minus_five_time);
    }

    #[test]
    fn test_display_formatting() {
        let bounded = DateTimeRangeQuery::between("2024-12-19T10:30:00Z", "2024-12-19T18:00:00Z");
        assert_eq!(
            bounded.to_string(),
            "2024-12-19T10:30:00Z to 2024-12-19T18:00:00Z"
        );

        let start_only = DateTimeRangeQuery::after("2024-12-19T10:30:00Z");
        assert_eq!(start_only.to_string(), "from 2024-12-19T10:30:00Z");

        let end_only = DateTimeRangeQuery::before("2024-12-19T18:00:00Z");
        assert_eq!(end_only.to_string(), "until 2024-12-19T18:00:00Z");

        let empty = DateTimeRangeQuery::new();
        assert_eq!(empty.to_string(), "no datetime filter");
    }

    #[test]
    fn test_datetime_range_info_conversion() {
        let range = DateTimeRangeQuery::between("2024-12-19T10:00:00Z", "2024-12-19T18:00:00Z");

        let info = DateTimeRangeInfo::from(&range);
        assert!(info.is_bounded);
        assert_eq!(info.duration_hours, Some(8));
        assert!(info.start_utc.is_some());
        assert!(info.end_utc.is_some());
    }
}
