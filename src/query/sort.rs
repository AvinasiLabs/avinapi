//! Sort query parameters for ordering results.
//!
//! This module provides sorting functionality for API endpoints, allowing
//! clients to specify sort fields and directions through query parameters.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use utoipa::{IntoParams, ToSchema};
use validator::{Validate, ValidationError};

/// Sort order enumeration.
///
/// Specifies the direction of sorting for query results.
///
/// # Examples
///
/// ```
/// use avinasi_web::query::SortOrder;
/// use std::str::FromStr;
///
/// assert_eq!(SortOrder::from_str("asc").unwrap(), SortOrder::Asc);
/// assert_eq!(SortOrder::from_str("desc").unwrap(), SortOrder::Desc);
/// assert_eq!(SortOrder::from_str("ASC").unwrap(), SortOrder::Asc); // Case insensitive
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    /// Ascending order (smallest to largest, A to Z)
    #[schema(rename = "asc")]
    Asc,
    /// Descending order (largest to smallest, Z to A)
    #[schema(rename = "desc")]
    Desc,
}

impl Default for SortOrder {
    fn default() -> Self {
        Self::Asc
    }
}

impl fmt::Display for SortOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SortOrder::Asc => write!(f, "asc"),
            SortOrder::Desc => write!(f, "desc"),
        }
    }
}

impl FromStr for SortOrder {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "asc" | "ascending" => Ok(SortOrder::Asc),
            "desc" | "descending" => Ok(SortOrder::Desc),
            _ => Err(format!("Invalid sort order: {}", s)),
        }
    }
}

impl SortOrder {
    /// Returns true if this is ascending order.
    ///
    /// # Example
    ///
    /// ```
    /// use avinasi_web::query::SortOrder;
    ///
    /// assert!(SortOrder::Asc.is_asc());
    /// assert!(!SortOrder::Desc.is_asc());
    /// ```
    pub fn is_asc(&self) -> bool {
        matches!(self, SortOrder::Asc)
    }

    /// Returns true if this is descending order.
    ///
    /// # Example
    ///
    /// ```
    /// use avinasi_web::query::SortOrder;
    ///
    /// assert!(!SortOrder::Asc.is_desc());
    /// assert!(SortOrder::Desc.is_desc());
    /// ```
    pub fn is_desc(&self) -> bool {
        matches!(self, SortOrder::Desc)
    }

    /// Returns the opposite sort order.
    ///
    /// # Example
    ///
    /// ```
    /// use avinasi_web::query::SortOrder;
    ///
    /// assert_eq!(SortOrder::Asc.reverse(), SortOrder::Desc);
    /// assert_eq!(SortOrder::Desc.reverse(), SortOrder::Asc);
    /// ```
    pub fn reverse(&self) -> SortOrder {
        match self {
            SortOrder::Asc => SortOrder::Desc,
            SortOrder::Desc => SortOrder::Asc,
        }
    }

    /// Converts to SQL ORDER BY clause suffix.
    ///
    /// # Example
    ///
    /// ```
    /// use avinasi_web::query::SortOrder;
    ///
    /// assert_eq!(SortOrder::Asc.to_sql(), "ASC");
    /// assert_eq!(SortOrder::Desc.to_sql(), "DESC");
    /// ```
    pub fn to_sql(&self) -> &'static str {
        match self {
            SortOrder::Asc => "ASC",
            SortOrder::Desc => "DESC",
        }
    }
}

/// A single sort field specification.
///
/// Represents a field name and its sort direction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct SortField {
    /// Field name to sort by
    #[schema(example = "created_at")]
    pub field: String,

    /// Sort direction
    #[serde(default)]
    pub order: SortOrder,
}

impl SortField {
    /// Creates a new sort field with ascending order.
    ///
    /// # Example
    ///
    /// ```
    /// use avinasi_web::query::{SortField, SortOrder};
    ///
    /// let field = SortField::asc("name");
    /// assert_eq!(field.field, "name");
    /// assert_eq!(field.order, SortOrder::Asc);
    /// ```
    pub fn asc(field: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            order: SortOrder::Asc,
        }
    }

    /// Creates a new sort field with descending order.
    ///
    /// # Example
    ///
    /// ```
    /// use avinasi_web::query::{SortField, SortOrder};
    ///
    /// let field = SortField::desc("created_at");
    /// assert_eq!(field.field, "created_at");
    /// assert_eq!(field.order, SortOrder::Desc);
    /// ```
    pub fn desc(field: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            order: SortOrder::Desc,
        }
    }

    /// Creates a new sort field with the specified order.
    ///
    /// # Example
    ///
    /// ```
    /// use avinasi_web::query::{SortField, SortOrder};
    ///
    /// let field = SortField::new("price", SortOrder::Desc);
    /// assert_eq!(field.field, "price");
    /// assert_eq!(field.order, SortOrder::Desc);
    /// ```
    pub fn new(field: impl Into<String>, order: SortOrder) -> Self {
        Self {
            field: field.into(),
            order,
        }
    }

    /// Converts to SQL ORDER BY clause.
    ///
    /// # Example
    ///
    /// ```
    /// use avinasi_web::query::SortField;
    ///
    /// let field = SortField::desc("created_at");
    /// assert_eq!(field.to_sql(), "created_at DESC");
    /// ```
    pub fn to_sql(&self) -> String {
        format!("{} {}", self.field, self.order.to_sql())
    }

    /// Converts to SQL ORDER BY clause with table prefix.
    ///
    /// # Example
    ///
    /// ```
    /// use avinasi_web::query::SortField;
    ///
    /// let field = SortField::desc("created_at");
    /// assert_eq!(field.to_sql_with_prefix("users"), "users.created_at DESC");
    /// ```
    pub fn to_sql_with_prefix(&self, table_prefix: &str) -> String {
        format!("{}.{} {}", table_prefix, self.field, self.order.to_sql())
    }
}

impl fmt::Display for SortField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.field, self.order)
    }
}

impl FromStr for SortField {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().split_whitespace().collect();
        match parts.len() {
            1 => Ok(SortField::asc(parts[0])),
            2 => {
                let order = SortOrder::from_str(parts[1])
                    .map_err(|_| format!("Invalid sort specification: {}", s))?;
                Ok(SortField::new(parts[0], order))
            }
            _ => Err(format!("Invalid sort specification: {}", s)),
        }
    }
}

/// Query parameters for sorting results.
///
/// Provides flexible sorting with support for multiple fields and directions.
/// Can be used with comma-separated field specifications or individual parameters.
///
/// # Query Parameter Format
///
/// ## Option 1: Single sort parameter (comma-separated)
/// - `sort`: Comma-separated list of field specifications
///   - Format: `field` (ascending) or `field direction`
///   - Example: `sort=name,created_at desc,price asc`
///
/// ## Option 2: Individual parameters
/// - `sort_by`: Field name to sort by
/// - `sort_order`: Sort direction (asc/desc, defaults to asc)
///   - Example: `sort_by=name&sort_order=desc`
///
/// # Examples
///
/// ```
/// use avinasi_web::query::{SortQuery, SortField, SortOrder};
///
/// // Single field
/// let sort = SortQuery::by("name");
/// assert_eq!(sort.fields.len(), 1);
/// assert_eq!(sort.fields[0].field, "name");
/// assert_eq!(sort.fields[0].order, SortOrder::Asc);
///
/// // Multiple fields
/// let sort = SortQuery::new(vec![
///     SortField::asc("name"),
///     SortField::desc("created_at"),
/// ]);
/// assert_eq!(sort.fields.len(), 2);
/// ```
///
/// URL examples:
/// - `GET /users?sort=name` - Sort by name ascending
/// - `GET /users?sort=name desc` - Sort by name descending
/// - `GET /users?sort=name,created_at desc` - Sort by name asc, then created_at desc
/// - `GET /users?sort_by=email&sort_order=asc` - Sort by email ascending
#[derive(Debug, Clone, Serialize, Validate, IntoParams, ToSchema)]
#[into_params(parameter_in = Query)]
#[validate(schema(function = "validate_sort_fields"))]
pub struct SortQuery {
    /// Comma-separated sort specifications
    /// Format: "field1,field2 desc,field3 asc"
    #[param(example = "name,created_at desc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,

    /// Single field to sort by (alternative to sort parameter)
    #[param(example = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,

    /// Sort direction for sort_by field
    #[param(example = "asc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<SortOrder>,

    /// Parsed sort fields (populated during deserialization)
    #[serde(skip)]
    pub fields: Vec<SortField>,
}

// Helper struct for deserialization
#[derive(Deserialize)]
struct SortQueryHelper {
    sort: Option<String>,
    sort_by: Option<String>,
    sort_order: Option<SortOrder>,
}

impl Default for SortQueryHelper {
    fn default() -> Self {
        Self {
            sort: None,
            sort_by: None,
            sort_order: None,
        }
    }
}

impl<'de> Deserialize<'de> for SortQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let helper = SortQueryHelper::deserialize(deserializer)?;
        let mut query = SortQuery {
            sort: helper.sort,
            sort_by: helper.sort_by,
            sort_order: helper.sort_order,
            fields: Vec::new(),
        };

        // Automatically parse fields during deserialization
        query.parse().map_err(serde::de::Error::custom)?;

        Ok(query)
    }
}

impl Default for SortQuery {
    fn default() -> Self {
        Self {
            sort: None,
            sort_by: None,
            sort_order: None,
            fields: Vec::new(),
        }
    }
}

impl SortQuery {
    /// Creates a new empty sort query.
    ///
    /// # Example
    ///
    /// ```
    /// use avinasi_web::query::SortQuery;
    ///
    /// let sort = SortQuery::new(vec![]);
    /// assert!(sort.is_empty());
    /// ```
    pub fn new(fields: Vec<SortField>) -> Self {
        Self {
            sort: None,
            sort_by: None,
            sort_order: None,
            fields,
        }
    }

    /// Creates a sort query for a single field with ascending order.
    ///
    /// # Example
    ///
    /// ```
    /// use avinasi_web::query::{SortQuery, SortOrder};
    ///
    /// let sort = SortQuery::by("name");
    /// assert_eq!(sort.fields.len(), 1);
    /// assert_eq!(sort.fields[0].field, "name");
    /// assert_eq!(sort.fields[0].order, SortOrder::Asc);
    /// ```
    pub fn by(field: impl Into<String>) -> Self {
        Self::new(vec![SortField::asc(field)])
    }

    /// Creates a sort query for a single field with the specified order.
    ///
    /// # Example
    ///
    /// ```
    /// use avinasi_web::query::{SortQuery, SortOrder};
    ///
    /// let sort = SortQuery::by_order("created_at", SortOrder::Desc);
    /// assert_eq!(sort.fields[0].field, "created_at");
    /// assert_eq!(sort.fields[0].order, SortOrder::Desc);
    /// ```
    pub fn by_order(field: impl Into<String>, order: SortOrder) -> Self {
        Self::new(vec![SortField::new(field, order)])
    }

    /// Checks if the sort query is empty (no sort fields).
    ///
    /// # Example
    ///
    /// ```
    /// use avinasi_web::query::SortQuery;
    ///
    /// let empty = SortQuery::new(vec![]);
    /// let non_empty = SortQuery::by("name");
    ///
    /// assert!(empty.is_empty());
    /// assert!(!non_empty.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    /// Gets the number of sort fields.
    ///
    /// # Example
    ///
    /// ```
    /// use avinasi_web::query::{SortQuery, SortField};
    ///
    /// let sort = SortQuery::new(vec![
    ///     SortField::asc("name"),
    ///     SortField::desc("created_at"),
    /// ]);
    /// assert_eq!(sort.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.fields.len()
    }

    /// Gets the first sort field, if any.
    ///
    /// # Example
    ///
    /// ```
    /// use avinasi_web::query::SortQuery;
    ///
    /// let sort = SortQuery::by("name");
    /// let first = sort.first().unwrap();
    /// assert_eq!(first.field, "name");
    /// ```
    pub fn first(&self) -> Option<&SortField> {
        self.fields.first()
    }

    /// Parses sort specifications from query parameters.
    ///
    /// This is called automatically during deserialization.
    pub fn parse(&mut self) -> Result<(), String> {
        self.fields.clear();

        // Parse from sort parameter (comma-separated)
        if let Some(sort_str) = &self.sort {
            for spec in sort_str.split(',') {
                let field = SortField::from_str(spec.trim())
                    .map_err(|e| format!("Invalid sort specification '{}': {}", spec.trim(), e))?;
                self.fields.push(field);
            }
        }
        // Parse from individual parameters
        else if let Some(sort_by) = &self.sort_by {
            let order = self.sort_order.unwrap_or_default();
            self.fields.push(SortField::new(sort_by, order));
        }

        Ok(())
    }

    /// Converts to SQL ORDER BY clause.
    ///
    /// # Example
    ///
    /// ```
    /// use avinasi_web::query::{SortQuery, SortField};
    ///
    /// let sort = SortQuery::new(vec![
    ///     SortField::asc("name"),
    ///     SortField::desc("created_at"),
    /// ]);
    /// assert_eq!(sort.to_sql(), "name ASC, created_at DESC");
    /// ```
    pub fn to_sql(&self) -> String {
        self.fields
            .iter()
            .map(|f| f.to_sql())
            .collect::<Vec<_>>()
            .join(", ")
    }

    /// Converts to SQL ORDER BY clause with table prefix.
    ///
    /// # Example
    ///
    /// ```
    /// use avinasi_web::query::{SortQuery, SortField};
    ///
    /// let sort = SortQuery::new(vec![
    ///     SortField::asc("name"),
    ///     SortField::desc("created_at"),
    /// ]);
    /// assert_eq!(
    ///     sort.to_sql_with_prefix("users"),
    ///     "users.name ASC, users.created_at DESC"
    /// );
    /// ```
    pub fn to_sql_with_prefix(&self, table_prefix: &str) -> String {
        self.fields
            .iter()
            .map(|f| f.to_sql_with_prefix(table_prefix))
            .collect::<Vec<_>>()
            .join(", ")
    }

    /// Validates field names against allowed fields.
    ///
    /// # Example
    ///
    /// ```
    /// use avinasi_web::query::SortQuery;
    ///
    /// let sort = SortQuery::by("name");
    /// let allowed = vec!["name", "email", "created_at"];
    /// assert!(sort.validate_fields(&allowed).is_ok());
    ///
    /// let invalid_sort = SortQuery::by("invalid_field");
    /// assert!(invalid_sort.validate_fields(&allowed).is_err());
    /// ```
    pub fn validate_fields(&self, allowed_fields: &[&str]) -> Result<(), String> {
        for field in &self.fields {
            if !allowed_fields.contains(&field.field.as_str()) {
                return Err(format!(
                    "Invalid sort field '{}'. Allowed fields: {}",
                    field.field,
                    allowed_fields.join(", ")
                ));
            }
        }
        Ok(())
    }
}

/// Custom validation function for sort query fields.
fn validate_sort_fields(sort_query: &SortQuery) -> Result<(), ValidationError> {
    // Create a mutable copy to parse
    let mut temp_query = sort_query.clone();

    if let Err(e) = temp_query.parse() {
        let mut error = ValidationError::new("invalid_sort_format");
        error.message = Some(e.into());
        return Err(error);
    }

    // Basic validation: ensure field names are not empty
    for field in &temp_query.fields {
        if field.field.trim().is_empty() {
            let mut error = ValidationError::new("empty_sort_field");
            error.message = Some("Sort field name cannot be empty".into());
            return Err(error);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_order_default() {
        assert_eq!(SortOrder::default(), SortOrder::Asc);
    }

    #[test]
    fn test_sort_order_display() {
        assert_eq!(SortOrder::Asc.to_string(), "asc");
        assert_eq!(SortOrder::Desc.to_string(), "desc");
    }

    #[test]
    fn test_sort_order_from_str() {
        assert_eq!(SortOrder::from_str("asc").unwrap(), SortOrder::Asc);
        assert_eq!(SortOrder::from_str("desc").unwrap(), SortOrder::Desc);
        assert_eq!(SortOrder::from_str("ASC").unwrap(), SortOrder::Asc);
        assert_eq!(SortOrder::from_str("DESC").unwrap(), SortOrder::Desc);
        assert_eq!(SortOrder::from_str("ascending").unwrap(), SortOrder::Asc);
        assert_eq!(SortOrder::from_str("descending").unwrap(), SortOrder::Desc);

        assert!(SortOrder::from_str("invalid").is_err());
    }

    #[test]
    fn test_sort_order_methods() {
        assert!(SortOrder::Asc.is_asc());
        assert!(!SortOrder::Asc.is_desc());
        assert!(!SortOrder::Desc.is_asc());
        assert!(SortOrder::Desc.is_desc());

        assert_eq!(SortOrder::Asc.reverse(), SortOrder::Desc);
        assert_eq!(SortOrder::Desc.reverse(), SortOrder::Asc);

        assert_eq!(SortOrder::Asc.to_sql(), "ASC");
        assert_eq!(SortOrder::Desc.to_sql(), "DESC");
    }

    #[test]
    fn test_sort_field_creation() {
        let asc_field = SortField::asc("name");
        assert_eq!(asc_field.field, "name");
        assert_eq!(asc_field.order, SortOrder::Asc);

        let desc_field = SortField::desc("created_at");
        assert_eq!(desc_field.field, "created_at");
        assert_eq!(desc_field.order, SortOrder::Desc);

        let custom_field = SortField::new("price", SortOrder::Desc);
        assert_eq!(custom_field.field, "price");
        assert_eq!(custom_field.order, SortOrder::Desc);
    }

    #[test]
    fn test_sort_field_sql() {
        let field = SortField::desc("created_at");
        assert_eq!(field.to_sql(), "created_at DESC");

        let field_with_prefix = SortField::asc("name");
        assert_eq!(
            field_with_prefix.to_sql_with_prefix("users"),
            "users.name ASC"
        );
    }

    #[test]
    fn test_sort_field_display() {
        let field = SortField::desc("created_at");
        assert_eq!(field.to_string(), "created_at desc");
    }

    #[test]
    fn test_sort_field_from_str() {
        let field1 = SortField::from_str("name").unwrap();
        assert_eq!(field1.field, "name");
        assert_eq!(field1.order, SortOrder::Asc);

        let field2 = SortField::from_str("created_at desc").unwrap();
        assert_eq!(field2.field, "created_at");
        assert_eq!(field2.order, SortOrder::Desc);

        let field3 = SortField::from_str("  price  asc  ").unwrap();
        assert_eq!(field3.field, "price");
        assert_eq!(field3.order, SortOrder::Asc);

        assert!(SortField::from_str("field invalid_order").is_err());
        assert!(SortField::from_str("too many words here").is_err());
    }

    #[test]
    fn test_sort_query_creation() {
        let sort = SortQuery::by("name");
        assert_eq!(sort.fields.len(), 1);
        assert_eq!(sort.fields[0].field, "name");
        assert_eq!(sort.fields[0].order, SortOrder::Asc);

        let sort_desc = SortQuery::by_order("created_at", SortOrder::Desc);
        assert_eq!(sort_desc.fields[0].field, "created_at");
        assert_eq!(sort_desc.fields[0].order, SortOrder::Desc);

        let multi_sort =
            SortQuery::new(vec![SortField::asc("name"), SortField::desc("created_at")]);
        assert_eq!(multi_sort.fields.len(), 2);
    }

    #[test]
    fn test_sort_query_methods() {
        let empty = SortQuery::new(vec![]);
        assert!(empty.is_empty());
        assert_eq!(empty.len(), 0);
        assert!(empty.first().is_none());

        let non_empty = SortQuery::by("name");
        assert!(!non_empty.is_empty());
        assert_eq!(non_empty.len(), 1);
        assert!(non_empty.first().is_some());
    }

    #[test]
    fn test_sort_query_parse_comma_separated() {
        let mut sort = SortQuery {
            sort: Some("name,created_at desc,price asc".to_string()),
            ..Default::default()
        };

        sort.parse().unwrap();

        assert_eq!(sort.fields.len(), 3);
        assert_eq!(sort.fields[0].field, "name");
        assert_eq!(sort.fields[0].order, SortOrder::Asc);
        assert_eq!(sort.fields[1].field, "created_at");
        assert_eq!(sort.fields[1].order, SortOrder::Desc);
        assert_eq!(sort.fields[2].field, "price");
        assert_eq!(sort.fields[2].order, SortOrder::Asc);
    }

    #[test]
    fn test_sort_query_parse_individual() {
        let mut sort = SortQuery {
            sort_by: Some("email".to_string()),
            sort_order: Some(SortOrder::Desc),
            ..Default::default()
        };

        sort.parse().unwrap();

        assert_eq!(sort.fields.len(), 1);
        assert_eq!(sort.fields[0].field, "email");
        assert_eq!(sort.fields[0].order, SortOrder::Desc);
    }

    #[test]
    fn test_sort_query_parse_individual_default_order() {
        let mut sort = SortQuery {
            sort_by: Some("email".to_string()),
            sort_order: None,
            ..Default::default()
        };

        sort.parse().unwrap();

        assert_eq!(sort.fields.len(), 1);
        assert_eq!(sort.fields[0].field, "email");
        assert_eq!(sort.fields[0].order, SortOrder::Asc);
    }

    #[test]
    fn test_sort_query_parse_invalid() {
        let mut sort = SortQuery {
            sort: Some("field invalid_order".to_string()),
            ..Default::default()
        };

        assert!(sort.parse().is_err());
    }

    #[test]
    fn test_sort_query_to_sql() {
        let sort = SortQuery::new(vec![SortField::asc("name"), SortField::desc("created_at")]);

        assert_eq!(sort.to_sql(), "name ASC, created_at DESC");
        assert_eq!(
            sort.to_sql_with_prefix("users"),
            "users.name ASC, users.created_at DESC"
        );

        let empty_sort = SortQuery::new(vec![]);
        assert_eq!(empty_sort.to_sql(), "");
    }

    #[test]
    fn test_sort_query_validate_fields() {
        let allowed = vec!["name", "email", "created_at"];

        let valid_sort = SortQuery::by("name");
        assert!(valid_sort.validate_fields(&allowed).is_ok());

        let invalid_sort = SortQuery::by("invalid_field");
        let result = invalid_sort.validate_fields(&allowed);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("invalid_field"));
    }

    #[test]
    fn test_validation_function() {
        // Valid sort query
        let mut valid = SortQuery {
            sort: Some("name,created_at desc".to_string()),
            ..Default::default()
        };
        valid.parse().unwrap(); // Populate fields
        assert!(validate_sort_fields(&valid).is_ok());

        // Invalid sort format
        let invalid = SortQuery {
            sort: Some("field invalid_order".to_string()),
            ..Default::default()
        };
        assert!(validate_sort_fields(&invalid).is_err());
    }

    #[test]
    fn test_serialization_deserialization() {
        let sort = SortQuery::new(vec![SortField::asc("name"), SortField::desc("created_at")]);

        let field = SortField::desc("email");

        // Test that structures can be serialized
        let _sort_serialized = serde_json::to_string(&sort).expect("Should serialize SortQuery");
        let _field_serialized = serde_json::to_string(&field).expect("Should serialize SortField");
        let _order_serialized =
            serde_json::to_string(&SortOrder::Desc).expect("Should serialize SortOrder");
    }
}
