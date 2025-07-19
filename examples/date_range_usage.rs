//! Example showing how to handle different date formats in web handlers
//!
//! This demonstrates how users can handle various date formats from frontend
//! and convert them to use with our DateRangeQuery

use avinapi::prelude::*;
use axum::{Router, extract::Query, routing::get};
use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize)]
struct Order {
    id: u32,
    amount: f64,
    order_date: NaiveDate,
    customer: String,
}

// Sample data
fn get_orders() -> Vec<Order> {
    vec![
        Order {
            id: 1,
            amount: 99.99,
            order_date: NaiveDate::from_ymd_opt(2024, 12, 15).unwrap(),
            customer: "Alice".to_string(),
        },
        Order {
            id: 2,
            amount: 149.50,
            order_date: NaiveDate::from_ymd_opt(2024, 12, 20).unwrap(),
            customer: "Bob".to_string(),
        },
        Order {
            id: 3,
            amount: 75.25,
            order_date: NaiveDate::from_ymd_opt(2024, 12, 25).unwrap(),
            customer: "Charlie".to_string(),
        },
    ]
}

/// Handler that accepts different date formats as strings and converts to DateRangeQuery
#[derive(Debug, Deserialize)]
struct DateParams {
    start: Option<String>, // Can be "2024-12-19", "2024-12-19T10:30:00Z", "2024-12-19T10:30:00+08:00"
    end: Option<String>,
}

/// Parse different date formats to NaiveDate
/// This is what users need to implement to handle various frontend formats
fn parse_date_string(date_str: &str) -> Result<NaiveDate, String> {
    println!("  Parsing: {}", date_str);

    // Format 1: Date only (2024-12-19)
    if let Ok(date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        println!("    ✅ Parsed as date only: {}", date);
        return Ok(date);
    }

    // Format 2: Date with UTC time (2024-12-19T10:30:00Z)
    if let Ok(dt) = DateTime::parse_from_rfc3339(date_str) {
        let date = dt.date_naive();
        println!("    ✅ Parsed as RFC3339, extracted date: {}", date);
        return Ok(date);
    }

    // Format 3: Naive datetime without timezone (2024-12-19T10:30:00)
    if let Ok(naive_dt) = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S") {
        let date = naive_dt.date();
        println!("    ✅ Parsed as naive datetime, extracted date: {}", date);
        return Ok(date);
    }

    Err(format!("Unsupported date format: {}", date_str))
}

/// Example endpoint showing how to handle different date formats
async fn orders_with_flexible_dates(
    Query(params): Query<DateParams>,
) -> JsonResult<HashMap<String, serde_json::Value>> {
    println!("🎯 Handling flexible date formats:");
    println!("  Input: start={:?}, end={:?}", params.start, params.end);

    // Parse start date from string
    let start_date = if let Some(start_str) = &params.start {
        match parse_date_string(start_str) {
            Ok(date) => Some(date),
            Err(e) => return Err(AppError::validation(format!("Invalid start date: {}", e))),
        }
    } else {
        None
    };

    // Parse end date from string
    let end_date = if let Some(end_str) = &params.end {
        match parse_date_string(end_str) {
            Ok(date) => Some(date),
            Err(e) => return Err(AppError::validation(format!("Invalid end date: {}", e))),
        }
    } else {
        None
    };

    // Create our library's DateRangeQuery from parsed dates
    let date_range = DateRangeQuery {
        start_date,
        end_date,
    };

    println!("📋 Created DateRangeQuery: {:?}", date_range);

    // Use library validation
    date_range.validate().map_err(AppError::from)?;
    println!("✅ Validation passed");

    // Now show practical business usage with the NaiveDate fields
    println!("🔧 Practical business usage:");

    // 1. SQL query generation - use the NaiveDate directly
    let sql_query = generate_sql_with_date_range(&date_range);
    println!("  📄 Generated SQL: {}", sql_query);

    // 2. Database parameter binding - NaiveDate works directly with sqlx
    let db_params = extract_db_parameters(&date_range);
    println!("  🗃️  Database parameters: {:?}", db_params);

    // 3. Business logic decisions
    let business_info = analyze_date_range_business_impact(&date_range);
    println!("  💼 Business analysis: {:?}", business_info);

    // 4. Convert to other formats if needed
    let formatted_dates = format_for_external_api(&date_range);
    println!("  🌐 External API format: {:?}", formatted_dates);

    // 5. In-memory filtering (one of many uses)
    let orders = get_orders();
    let filtered: Vec<Order> = orders
        .into_iter()
        .filter(|order| date_range.contains(order.order_date))
        .collect();

    println!("  📊 Found {} orders in date range", filtered.len());

    // Return comprehensive result showing library functionality
    let mut result = HashMap::new();
    result.insert(
        "original_input".to_string(),
        serde_json::json!({
            "start": params.start,
            "end": params.end
        }),
    );
    result.insert(
        "parsed_dates".to_string(),
        serde_json::json!({
            "start_date": start_date,
            "end_date": end_date
        }),
    );
    result.insert(
        "date_range_query".to_string(),
        serde_json::json!(date_range),
    );

    // Show all library methods
    result.insert(
        "library_methods".to_string(),
        serde_json::json!({
            "is_empty": date_range.is_empty(),
            "is_bounded": date_range.is_bounded(),
            "has_start": date_range.has_start(),
            "has_end": date_range.has_end()
        }),
    );

    // Show datetime conversion
    let (start_dt, end_dt) = date_range.to_datetime_range();
    result.insert(
        "datetime_range".to_string(),
        serde_json::json!({
            "start": start_dt,
            "end": end_dt
        }),
    );

    result.insert("filtered_orders".to_string(), serde_json::json!(filtered));
    result.insert("total_found".to_string(), serde_json::json!(filtered.len()));

    data!(result)
}

/// Simple endpoint using DateRangeQuery directly (for comparison)
async fn orders_direct(Query(date_range): Query<DateRangeQuery>) -> JsonResult<Vec<Order>> {
    println!("📋 Using DateRangeQuery directly: {:?}", date_range);

    let orders = get_orders();
    let filtered: Vec<Order> = orders
        .into_iter()
        .filter(|order| date_range.contains(order.order_date))
        .collect();

    data!(filtered)
}

/// Generate SQL WHERE clause using NaiveDate from DateRangeQuery
/// This shows how NaiveDate formats perfectly for database queries
fn generate_sql_with_date_range(date_range: &DateRangeQuery) -> String {
    let mut conditions = Vec::new();

    if let Some(start) = date_range.start_date {
        // NaiveDate formats as 'YYYY-MM-DD' - perfect for SQL
        conditions.push(format!("order_date >= '{}'", start.format("%Y-%m-%d")));
    }

    if let Some(end) = date_range.end_date {
        conditions.push(format!("order_date <= '{}'", end.format("%Y-%m-%d")));
    }

    if conditions.is_empty() {
        "SELECT * FROM orders".to_string()
    } else {
        format!("SELECT * FROM orders WHERE {}", conditions.join(" AND "))
    }
}

/// Extract database parameters for prepared statements
/// Shows how NaiveDate works directly with sqlx parameter binding
fn extract_db_parameters(date_range: &DateRangeQuery) -> Vec<(&str, Option<NaiveDate>)> {
    vec![
        ("start_date", date_range.start_date), // Ready for sqlx bind
        ("end_date", date_range.end_date),     // Ready for sqlx bind
    ]
}

/// Business logic using date range analysis
/// Shows practical business decisions based on parsed dates
fn analyze_date_range_business_impact(
    date_range: &DateRangeQuery,
) -> HashMap<String, serde_json::Value> {
    let mut analysis = HashMap::new();

    if let (Some(start), Some(end)) = (date_range.start_date, date_range.end_date) {
        // Calculate duration using NaiveDate methods
        let duration = end.signed_duration_since(start);
        let days = duration.num_days();

        analysis.insert("duration_days".to_string(), serde_json::json!(days));

        // Business categorization
        let period_type = match days {
            0 => "single_day",
            1..=7 => "weekly",
            8..=31 => "monthly",
            32..=365 => "yearly",
            _ => "multi_year",
        };
        analysis.insert("period_type".to_string(), serde_json::json!(period_type));

        // Check if includes weekend
        let includes_weekend = (start.weekday().num_days_from_monday()
            ..=end.weekday().num_days_from_monday())
            .any(|day| day >= 5);
        analysis.insert(
            "includes_weekend".to_string(),
            serde_json::json!(includes_weekend),
        );

        // Quarter analysis
        let start_quarter = (start.month() - 1) / 3 + 1;
        let end_quarter = (end.month() - 1) / 3 + 1;
        analysis.insert(
            "spans_quarters".to_string(),
            serde_json::json!(start_quarter != end_quarter),
        );
    } else if let Some(start) = date_range.start_date {
        analysis.insert("type".to_string(), serde_json::json!("open_ended_from"));
        analysis.insert(
            "start_weekday".to_string(),
            serde_json::json!(start.weekday().to_string()),
        );
    } else if let Some(end) = date_range.end_date {
        analysis.insert("type".to_string(), serde_json::json!("open_ended_until"));
        analysis.insert(
            "end_weekday".to_string(),
            serde_json::json!(end.weekday().to_string()),
        );
    } else {
        analysis.insert("type".to_string(), serde_json::json!("no_filter"));
    }

    analysis
}

/// Format dates for external API calls
/// Shows converting NaiveDate to various external formats
fn format_for_external_api(date_range: &DateRangeQuery) -> HashMap<String, Option<String>> {
    let mut formatted = HashMap::new();

    // ISO 8601 format (most common for APIs)
    formatted.insert(
        "start_iso".to_string(),
        date_range
            .start_date
            .map(|d| d.format("%Y-%m-%d").to_string()),
    );
    formatted.insert(
        "end_iso".to_string(),
        date_range
            .end_date
            .map(|d| d.format("%Y-%m-%d").to_string()),
    );

    // US format (MM/DD/YYYY)
    formatted.insert(
        "start_us".to_string(),
        date_range
            .start_date
            .map(|d| d.format("%m/%d/%Y").to_string()),
    );
    formatted.insert(
        "end_us".to_string(),
        date_range
            .end_date
            .map(|d| d.format("%m/%d/%Y").to_string()),
    );

    // European format (DD.MM.YYYY)
    formatted.insert(
        "start_eu".to_string(),
        date_range
            .start_date
            .map(|d| d.format("%d.%m.%Y").to_string()),
    );
    formatted.insert(
        "end_eu".to_string(),
        date_range
            .end_date
            .map(|d| d.format("%d.%m.%Y").to_string()),
    );

    // Unix timestamp (seconds since epoch) - convert via DateTime
    formatted.insert(
        "start_timestamp".to_string(),
        date_range.start_date.map(|d| {
            d.and_hms_opt(0, 0, 0)
                .unwrap()
                .and_utc()
                .timestamp()
                .to_string()
        }),
    );
    formatted.insert(
        "end_timestamp".to_string(),
        date_range.end_date.map(|d| {
            d.and_hms_opt(23, 59, 59)
                .unwrap()
                .and_utc()
                .timestamp()
                .to_string()
        }),
    );

    formatted
}

fn create_router() -> Router {
    Router::new()
        .route("/orders/flexible", get(orders_with_flexible_dates))
        .route("/orders/direct", get(orders_direct))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    println!("🚀 DateRangeQuery - Handling Different Date Formats");
    println!("===================================================");
    println!();
    println!("This example shows how to handle different date formats from frontend");
    println!("and convert them to use with avinapi's DateRangeQuery.");
    println!();
    println!("📅 Supported formats:");
    println!("  1. Date only: 2024-12-19");
    println!("  2. UTC datetime: 2024-12-19T10:30:00Z");
    println!("  3. Timezone-aware: 2024-12-19T10:30:00+08:00");
    println!("  4. Naive datetime: 2024-12-19T10:30:00");
    println!();
    println!("🌐 Available endpoints:");
    println!();
    println!("📋 Flexible date handling:");
    println!("  GET /orders/flexible?start=2024-12-19&end=2024-12-25");
    println!("  GET /orders/flexible?start=2024-12-19T10:30:00Z&end=2024-12-25T23:59:59Z");
    println!(
        "  GET /orders/flexible?start=2024-12-19T10:30:00+08:00&end=2024-12-25T18:00:00-05:00"
    );
    println!("  GET /orders/flexible?start=2024-12-19T10:30:00&end=2024-12-25T23:59:59");
    println!();
    println!("🎯 Direct DateRangeQuery (for comparison):");
    println!("  GET /orders/direct?start_date=2024-12-19&end_date=2024-12-25");
    println!();

    let app = create_router();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    println!("🌐 Server running on http://127.0.0.1:3000");
    println!("   Try the different date formats above!");

    axum::serve(listener, app).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_different_date_formats() {
        // Format 1: Date only
        assert_eq!(
            parse_date_string("2024-12-19").unwrap(),
            NaiveDate::from_ymd_opt(2024, 12, 19).unwrap()
        );

        // Format 2: UTC datetime
        assert_eq!(
            parse_date_string("2024-12-19T10:30:00Z").unwrap(),
            NaiveDate::from_ymd_opt(2024, 12, 19).unwrap()
        );

        // Format 3: Timezone-aware datetime
        assert_eq!(
            parse_date_string("2024-12-19T10:30:00+08:00").unwrap(),
            NaiveDate::from_ymd_opt(2024, 12, 19).unwrap()
        );

        // Format 4: Naive datetime
        assert_eq!(
            parse_date_string("2024-12-19T10:30:00").unwrap(),
            NaiveDate::from_ymd_opt(2024, 12, 19).unwrap()
        );

        // Invalid format
        assert!(parse_date_string("invalid-date").is_err());
    }

    #[test]
    fn test_date_range_creation_and_usage() {
        // Create DateRangeQuery from parsed strings
        let start = parse_date_string("2024-12-19T10:30:00+08:00").unwrap();
        let end = parse_date_string("2024-12-25").unwrap();

        let date_range = DateRangeQuery {
            start_date: Some(start),
            end_date: Some(end),
        };

        // Test library methods
        assert!(!date_range.is_empty());
        assert!(date_range.is_bounded());
        assert!(date_range.has_start());
        assert!(date_range.has_end());

        // Test date containment
        assert!(date_range.contains(NaiveDate::from_ymd_opt(2024, 12, 20).unwrap()));
        assert!(!date_range.contains(NaiveDate::from_ymd_opt(2024, 12, 30).unwrap()));

        // Test validation
        assert!(date_range.validate().is_ok());
    }

    #[test]
    fn test_practical_business_usage() {
        // Parse different date formats
        let start = parse_date_string("2024-12-18T10:30:00+08:00").unwrap();
        let end = parse_date_string("2024-12-22").unwrap();

        let date_range = DateRangeQuery {
            start_date: Some(start),
            end_date: Some(end),
        };

        // Test SQL generation - NaiveDate formats perfectly
        let sql = generate_sql_with_date_range(&date_range);
        assert!(sql.contains("order_date >= '2024-12-18'"));
        assert!(sql.contains("order_date <= '2024-12-22'"));

        // Test database parameters - NaiveDate is ready for sqlx
        let params = extract_db_parameters(&date_range);
        assert_eq!(
            params[0].1,
            Some(NaiveDate::from_ymd_opt(2024, 12, 18).unwrap())
        );
        assert_eq!(
            params[1].1,
            Some(NaiveDate::from_ymd_opt(2024, 12, 22).unwrap())
        );

        // Test business logic
        let analysis = analyze_date_range_business_impact(&date_range);
        assert_eq!(
            analysis.get("duration_days").unwrap(),
            &serde_json::json!(4)
        );
        assert_eq!(
            analysis.get("period_type").unwrap(),
            &serde_json::json!("weekly")
        );

        // Test external API formatting
        let formatted = format_for_external_api(&date_range);
        assert_eq!(
            formatted.get("start_iso").unwrap(),
            &Some("2024-12-18".to_string())
        );
        assert_eq!(
            formatted.get("end_iso").unwrap(),
            &Some("2024-12-22".to_string())
        );
    }

    #[test]
    fn test_sql_generation_with_different_scenarios() {
        // Only start date
        let start_only = DateRangeQuery {
            start_date: Some(NaiveDate::from_ymd_opt(2024, 12, 18).unwrap()),
            end_date: None,
        };
        let sql = generate_sql_with_date_range(&start_only);
        assert!(sql.contains("order_date >= '2024-12-18'"));
        assert!(!sql.contains("order_date <="));

        // Only end date
        let end_only = DateRangeQuery {
            start_date: None,
            end_date: Some(NaiveDate::from_ymd_opt(2024, 12, 22).unwrap()),
        };
        let sql = generate_sql_with_date_range(&end_only);
        assert!(sql.contains("order_date <= '2024-12-22'"));
        assert!(!sql.contains("order_date >="));
    }
}
