//! Custom validation functions for common business scenarios.
//!
//! This module provides validation functions that are commonly needed in business
//! applications but are not available in the validator crate. These functions
//! follow the validator crate's function signature pattern.

use crate::query::date_range::DateRangeQuery;
use crate::query::datetime_range::DateTimeRangeQuery;
use validator::ValidationError;

/// Custom datetime validation function for RFC 3339 format
pub fn validate_datetime_format(datetime: &str) -> Result<(), ValidationError> {
    match chrono::DateTime::parse_from_rfc3339(datetime) {
        Ok(_) => Ok(()),
        Err(_) => {
            let mut error = ValidationError::new("invalid_datetime_format");
            error.message = Some(
                "DateTime must be in RFC 3339 format (e.g., '2025-07-15T00:00:00+08:00')".into(),
            );
            Err(error)
        }
    }
}

/// Custom validation function for date range consistency.
///
/// Ensures that if both start_date and end_date are provided,
/// start_date is not after end_date.
///
/// # Arguments
///
/// * `date_range` - The DateRangeQuery to validate
///
/// # Returns
///
/// * `Ok(())` if the date range is valid
/// * `Err(ValidationError)` if start_date is after end_date
///
/// # Examples
///
/// ```
/// use avinapi::validation::validate_date_range;
/// use avinapi::query::DateRangeQuery;
/// use chrono::NaiveDate;
///
/// let valid_range = DateRangeQuery::between(
///     NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
///     NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
/// );
/// assert!(validate_date_range(&valid_range).is_ok());
///
/// let invalid_range = DateRangeQuery::between(
///     NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
///     NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
/// );
/// assert!(validate_date_range(&invalid_range).is_err());
/// ```
pub fn validate_date_range(date_range: &DateRangeQuery) -> Result<(), ValidationError> {
    if let (Some(start), Some(end)) = (date_range.start_date, date_range.end_date) {
        if start > end {
            return Err(ValidationError::new("start_date_after_end_date"));
        }
    }
    Ok(())
}

/// Custom validation function for datetime range consistency.
///
/// Ensures that datetime strings are valid RFC 3339 format and that
/// start datetime is before end datetime (if both provided).
///
/// # Arguments
///
/// * `datetime_range` - The DateTimeRangeQuery to validate
///
/// # Returns
///
/// * `Ok(())` if the datetime range is valid
/// * `Err(ValidationError)` if datetimes are invalid format or start is after end
///
/// # Examples
///
/// ```
/// use avinapi::validation::validate_datetime_range;
/// use avinapi::query::DateTimeRangeQuery;
///
/// let valid_range = DateTimeRangeQuery::between(
///     "2024-12-19T10:00:00Z",
///     "2024-12-19T18:00:00Z",
/// );
/// assert!(validate_datetime_range(&valid_range).is_ok());
///
/// let invalid_range = DateTimeRangeQuery::between(
///     "2024-12-19T18:00:00Z",
///     "2024-12-19T10:00:00Z",
/// );
/// assert!(validate_datetime_range(&invalid_range).is_err());
/// ```
pub fn validate_datetime_range(datetime_range: &DateTimeRangeQuery) -> Result<(), ValidationError> {
    // Validate using the struct's own validation method
    datetime_range
        .validate_range()
        .map_err(|_| ValidationError::new("invalid_datetime_range"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_range_validation() {
        use crate::query::DateRangeQuery;
        use chrono::NaiveDate;

        // Valid range
        let valid_range = DateRangeQuery::between(
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
        );
        assert!(validate_date_range(&valid_range).is_ok());

        // Invalid range (start after end)
        let invalid_range = DateRangeQuery::between(
            NaiveDate::from_ymd_opt(2024, 12, 31).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        );
        assert!(validate_date_range(&invalid_range).is_err());

        // Same date (should be valid)
        let same_date = DateRangeQuery::between(
            NaiveDate::from_ymd_opt(2024, 6, 15).unwrap(),
            NaiveDate::from_ymd_opt(2024, 6, 15).unwrap(),
        );
        assert!(validate_date_range(&same_date).is_ok());

        // Only start date (should be valid)
        let start_only = DateRangeQuery::after(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());
        assert!(validate_date_range(&start_only).is_ok());

        // Only end date (should be valid)
        let end_only = DateRangeQuery::before(NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());
        assert!(validate_date_range(&end_only).is_ok());

        // Empty range (should be valid)
        let empty = DateRangeQuery::new();
        assert!(validate_date_range(&empty).is_ok());
    }

    #[test]
    fn test_datetime_range_validation() {
        use crate::query::DateTimeRangeQuery;

        // Valid range
        let valid_range =
            DateTimeRangeQuery::between("2024-12-19T10:00:00Z", "2024-12-19T18:00:00Z");
        assert!(validate_datetime_range(&valid_range).is_ok());

        // Invalid range (start after end)
        let invalid_range =
            DateTimeRangeQuery::between("2024-12-19T18:00:00Z", "2024-12-19T10:00:00Z");
        assert!(validate_datetime_range(&invalid_range).is_err());

        // Invalid format
        let invalid_format =
            DateTimeRangeQuery::between("invalid-datetime", "2024-12-19T18:00:00Z");
        assert!(validate_datetime_range(&invalid_format).is_err());

        // Only start datetime (should be valid)
        let start_only = DateTimeRangeQuery::after("2024-12-19T10:00:00Z");
        assert!(validate_datetime_range(&start_only).is_ok());

        // Only end datetime (should be valid)
        let end_only = DateTimeRangeQuery::before("2024-12-19T18:00:00Z");
        assert!(validate_datetime_range(&end_only).is_ok());

        // Empty range (should be valid)
        let empty = DateTimeRangeQuery::new();
        assert!(validate_datetime_range(&empty).is_ok());

        // Different timezone formats
        let timezone_range =
            DateTimeRangeQuery::between("2024-12-19T10:00:00+08:00", "2024-12-19T20:00:00+08:00");
        assert!(validate_datetime_range(&timezone_range).is_ok());
    }
}
