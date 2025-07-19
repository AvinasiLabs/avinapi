//! Example demonstrating DateTimeRangeQuery usage for precise datetime filtering
//!
//! This shows how to use DateTimeRangeQuery for applications that need precise
//! time and timezone handling, like calendar events, appointments, logs, etc.
//!
//! Compare this with DateRangeQuery which is for simple date-only filtering.

use avinasi_web::prelude::*;
use avinasi_web::query::DateTimeRangeQuery;
use axum::{Router, extract::Query, routing::get};
use chrono::{DateTime, Timelike, Utc};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize)]
struct CalendarEvent {
    id: u32,
    title: String,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    location: String,
}

#[derive(Debug, Clone, Serialize)]
struct LogEntry {
    id: u32,
    message: String,
    timestamp: DateTime<Utc>,
    level: String,
}

// Sample calendar events
fn get_calendar_events() -> Vec<CalendarEvent> {
    vec![
        CalendarEvent {
            id: 1,
            title: "Morning Meeting".to_string(),
            start_time: "2024-12-19T09:00:00Z".parse().unwrap(),
            end_time: "2024-12-19T10:00:00Z".parse().unwrap(),
            location: "Conference Room A".to_string(),
        },
        CalendarEvent {
            id: 2,
            title: "Lunch Break".to_string(),
            start_time: "2024-12-19T12:00:00Z".parse().unwrap(),
            end_time: "2024-12-19T13:00:00Z".parse().unwrap(),
            location: "Cafeteria".to_string(),
        },
        CalendarEvent {
            id: 3,
            title: "Project Review".to_string(),
            start_time: "2024-12-19T15:30:00Z".parse().unwrap(),
            end_time: "2024-12-19T17:00:00Z".parse().unwrap(),
            location: "Conference Room B".to_string(),
        },
        CalendarEvent {
            id: 4,
            title: "Team Building".to_string(),
            start_time: "2024-12-20T14:00:00Z".parse().unwrap(),
            end_time: "2024-12-20T16:00:00Z".parse().unwrap(),
            location: "Outdoor Area".to_string(),
        },
    ]
}

// Sample log entries
fn get_log_entries() -> Vec<LogEntry> {
    vec![
        LogEntry {
            id: 1,
            message: "Application started".to_string(),
            timestamp: "2024-12-19T08:30:00Z".parse().unwrap(),
            level: "INFO".to_string(),
        },
        LogEntry {
            id: 2,
            message: "User logged in".to_string(),
            timestamp: "2024-12-19T09:15:00Z".parse().unwrap(),
            level: "INFO".to_string(),
        },
        LogEntry {
            id: 3,
            message: "Database connection error".to_string(),
            timestamp: "2024-12-19T14:22:00Z".parse().unwrap(),
            level: "ERROR".to_string(),
        },
        LogEntry {
            id: 4,
            message: "Task completed successfully".to_string(),
            timestamp: "2024-12-19T16:45:00Z".parse().unwrap(),
            level: "INFO".to_string(),
        },
    ]
}

/// Example 1: Calendar events with precise datetime filtering
/// This mimics real calendar applications that need exact time ranges
async fn calendar_events(
    Query(datetime_range): Query<DateTimeRangeQuery>,
) -> JsonResult<HashMap<String, serde_json::Value>> {
    println!("📅 Calendar Events Query: {}", datetime_range);

    // Validate the datetime range - this is what you'd do in real apps
    datetime_range
        .validate_range()
        .map_err(|e| AppError::validation(format!("Invalid datetime range: {}", e)))?;

    // Limit range to prevent excessive queries (like in your real project)
    datetime_range
        .validate_max_duration(30)
        .map_err(|e| AppError::validation(e))?;

    let events = get_calendar_events();

    // Filter events by start_time using precise datetime range
    let filtered_events: Vec<CalendarEvent> = events
        .into_iter()
        .filter(|event| datetime_range.contains(event.start_time))
        .collect();

    println!("  Found {} events in range", filtered_events.len());

    // Show practical usage similar to your project
    let mut result = HashMap::new();

    // Original query parameters
    result.insert(
        "query_params".to_string(),
        serde_json::json!({
            "start": datetime_range.get_start(),
            "end": datetime_range.get_end(),
            "display": datetime_range.to_string()
        }),
    );

    // Parsed UTC times for database queries
    let (start_utc, end_utc) = datetime_range.parse_utc_range();
    result.insert(
        "parsed_utc".to_string(),
        serde_json::json!({
            "start_utc": start_utc,
            "end_utc": end_utc
        }),
    );

    // Business analysis
    if let Some(duration) = datetime_range.get_duration() {
        result.insert(
            "duration_analysis".to_string(),
            serde_json::json!({
                "total_hours": duration.num_hours(),
                "total_minutes": duration.num_minutes(),
                "duration_type": match duration.num_hours() {
                    0..=1 => "short_meeting",
                    2..=4 => "half_day_session",
                    5..=8 => "full_day_event",
                    _ => "multi_day_event"
                }
            }),
        );
    }

    // SQL query generation for database
    let sql_condition = datetime_range.to_sql_conditions("start_time");
    result.insert(
        "sql_query".to_string(),
        serde_json::json!({
            "where_clause": sql_condition,
            "full_query": format!("SELECT * FROM calendar_events WHERE {}", sql_condition)
        }),
    );

    result.insert("events".to_string(), serde_json::json!(filtered_events));
    result.insert(
        "event_count".to_string(),
        serde_json::json!(filtered_events.len()),
    );

    data!(result)
}

/// Example 2: System logs with timezone-aware filtering
async fn system_logs(
    Query(datetime_range): Query<DateTimeRangeQuery>,
    Query(pagination): Query<PaginationQuery>,
) -> JsonResult<HashMap<String, serde_json::Value>> {
    println!("📊 System Logs Query: {}", datetime_range);
    println!(
        "  Pagination: page={}, per_page={}",
        pagination.get_page(),
        pagination.get_per_page()
    );

    // Validate datetime range
    datetime_range
        .validate_range()
        .map_err(|e| AppError::validation(format!("Invalid datetime range: {}", e)))?;

    let logs = get_log_entries();

    // Filter logs by timestamp
    let date_filtered: Vec<LogEntry> = logs
        .into_iter()
        .filter(|log| datetime_range.contains(log.timestamp))
        .collect();

    println!("  After datetime filtering: {} logs", date_filtered.len());

    // Apply pagination
    let total = date_filtered.len() as u64;
    let offset = pagination.get_offset() as usize;
    let limit = pagination.get_limit() as usize;

    let paginated_logs: Vec<LogEntry> =
        date_filtered.into_iter().skip(offset).take(limit).collect();

    let mut result = HashMap::new();

    // Query information
    result.insert(
        "datetime_filter".to_string(),
        serde_json::json!({
            "original_start": datetime_range.get_start(),
            "original_end": datetime_range.get_end(),
            "parsed_range": datetime_range.parse_utc_range(),
            "is_bounded": datetime_range.is_bounded(),
            "duration_hours": datetime_range.get_duration().map(|d| d.num_hours())
        }),
    );

    // Pagination info
    let meta = pagination.create_meta(total);
    result.insert("pagination".to_string(), serde_json::json!(meta));

    // Results
    result.insert("logs".to_string(), serde_json::json!(paginated_logs));
    result.insert("total_found".to_string(), serde_json::json!(total));

    // Database query that would be used
    let db_query_example = format!(
        "SELECT * FROM system_logs WHERE {} ORDER BY timestamp DESC LIMIT {} OFFSET {}",
        datetime_range.to_sql_conditions("timestamp"),
        pagination.get_limit(),
        pagination.get_offset()
    );
    result.insert(
        "database_query".to_string(),
        serde_json::json!(db_query_example),
    );

    data!(result)
}

/// Example 3: Demonstrate timezone handling and conversion
async fn timezone_demo(
    Query(datetime_range): Query<DateTimeRangeQuery>,
) -> JsonResult<HashMap<String, serde_json::Value>> {
    println!("🌍 Timezone Demo: {}", datetime_range);

    let mut result = HashMap::new();

    // Show original input
    result.insert(
        "original_input".to_string(),
        serde_json::json!({
            "start_datetime": datetime_range.get_start(),
            "end_datetime": datetime_range.get_end()
        }),
    );

    // Parse and convert to UTC - this is the key functionality
    if let Some(start_str) = datetime_range.get_start() {
        match datetime_range.parse_start_utc() {
            Ok(utc_time) => {
                result.insert(
                    "start_conversion".to_string(),
                    serde_json::json!({
                        "original": start_str,
                        "utc_time": utc_time,
                        "hour_utc": utc_time.hour(),
                        "iso_string": utc_time.to_rfc3339(),
                        "unix_timestamp": utc_time.timestamp()
                    }),
                );
            }
            Err(e) => {
                result.insert(
                    "start_error".to_string(),
                    serde_json::json!(format!("{:?}", e)),
                );
            }
        }
    }

    if let Some(end_str) = datetime_range.get_end() {
        match datetime_range.parse_end_utc() {
            Ok(utc_time) => {
                result.insert(
                    "end_conversion".to_string(),
                    serde_json::json!({
                        "original": end_str,
                        "utc_time": utc_time,
                        "hour_utc": utc_time.hour(),
                        "iso_string": utc_time.to_rfc3339(),
                        "unix_timestamp": utc_time.timestamp()
                    }),
                );
            }
            Err(e) => {
                result.insert(
                    "end_error".to_string(),
                    serde_json::json!(format!("{:?}", e)),
                );
            }
        }
    }

    // Show validation results
    match datetime_range.validate_range() {
        Ok(_) => result.insert("validation".to_string(), serde_json::json!("✅ Valid")),
        Err(e) => result.insert(
            "validation".to_string(),
            serde_json::json!(format!("❌ Invalid: {}", e)),
        ),
    };

    // Business logic examples
    let events = get_calendar_events();
    let matching_events: Vec<_> = events
        .into_iter()
        .filter(|event| datetime_range.contains(event.start_time))
        .collect();

    result.insert(
        "matching_events".to_string(),
        serde_json::json!(matching_events),
    );

    data!(result)
}

/// Example 4: Compare DateRangeQuery vs DateTimeRangeQuery
async fn comparison_demo(
    Query(date_range): Query<DateRangeQuery>,
    Query(datetime_range): Query<DateTimeRangeQuery>,
) -> JsonResult<HashMap<String, serde_json::Value>> {
    println!("⚖️  Comparing DateRangeQuery vs DateTimeRangeQuery");

    let mut result = HashMap::new();

    // DateRangeQuery - simple date filtering
    result.insert(
        "date_range_query".to_string(),
        serde_json::json!({
            "type": "DateRangeQuery",
            "fields": {
                "start_date": date_range.start_date,
                "end_date": date_range.end_date
            },
            "use_case": "Simple date filtering (day, week, month ranges)",
            "precision": "Date only (YYYY-MM-DD)",
            "timezone": "Not applicable",
            "sql_example": format!(
                "WHERE order_date >= '{}' AND order_date <= '{}'",
                date_range.start_date.map_or("NULL".to_string(), |d| d.to_string()),
                date_range.end_date.map_or("NULL".to_string(), |d| d.to_string())
            )
        }),
    );

    // DateTimeRangeQuery - precise datetime filtering
    result.insert(
        "datetime_range_query".to_string(),
        serde_json::json!({
            "type": "DateTimeRangeQuery",
            "fields": {
                "start_datetime": datetime_range.get_start(),
                "end_datetime": datetime_range.get_end()
            },
            "use_case": "Precise datetime filtering (appointments, logs, events)",
            "precision": "Full datetime with timezone (RFC 3339)",
            "timezone": "Full timezone support with automatic UTC conversion",
            "sql_example": datetime_range.to_sql_conditions("created_at"),
            "parsed_utc": datetime_range.parse_utc_range()
        }),
    );

    // Show when to use which
    result.insert(
        "usage_guidelines".to_string(),
        serde_json::json!({
            "use_date_range_when": [
                "Filtering by calendar dates (daily/weekly/monthly reports)",
                "Simple date-based queries (orders by date, user registration by date)",
                "When time of day doesn't matter",
                "When timezone is not a concern"
            ],
            "use_datetime_range_when": [
                "Calendar events with specific times",
                "Log entries with precise timestamps",
                "Appointments and scheduling",
                "When timezone matters",
                "When you need hour/minute precision",
                "Real-time data filtering"
            ]
        }),
    );

    data!(result)
}

fn create_router() -> Router {
    Router::new()
        .route("/calendar", get(calendar_events))
        .route("/logs", get(system_logs))
        .route("/timezone-demo", get(timezone_demo))
        .route("/comparison", get(comparison_demo))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    println!("🚀 DateTimeRangeQuery Usage Examples");
    println!("=====================================");
    println!();
    println!("This demonstrates precise datetime range filtering with timezone support.");
    println!("Perfect for calendar events, logs, appointments, and real-time data.");
    println!();
    println!("🎯 Supported formats (all RFC 3339):");
    println!("  • UTC: 2024-12-19T10:30:00Z");
    println!("  • Timezone aware: 2024-12-19T10:30:00+08:00");
    println!("  • Timezone aware (negative): 2024-12-19T10:30:00-05:00");
    println!();
    println!("🌐 Available endpoints:");
    println!();
    println!("📅 Calendar Events (like your real calendar app):");
    println!(
        "  GET /calendar?start_datetime=2024-12-19T09:00:00Z&end_datetime=2024-12-19T17:00:00Z"
    );
    println!(
        "  GET /calendar?start_datetime=2024-12-19T18:00:00+09:00&end_datetime=2024-12-19T22:00:00+09:00"
    );
    println!();
    println!("📊 System Logs with pagination:");
    println!(
        "  GET /logs?start_datetime=2024-12-19T08:00:00Z&end_datetime=2024-12-19T18:00:00Z&page=1&per_page=10"
    );
    println!();
    println!("🌍 Timezone conversion demo:");
    println!(
        "  GET /timezone-demo?start_datetime=2024-12-19T10:30:00+08:00&end_datetime=2024-12-19T18:00:00+08:00"
    );
    println!(
        "  GET /timezone-demo?start_datetime=2024-12-19T02:30:00Z&end_datetime=2024-12-19T10:00:00Z"
    );
    println!();
    println!("⚖️  Compare DateRangeQuery vs DateTimeRangeQuery:");
    println!(
        "  GET /comparison?start_date=2024-12-19&end_date=2024-12-20&start_datetime=2024-12-19T10:00:00Z&end_datetime=2024-12-20T10:00:00Z"
    );
    println!();

    let app = create_router();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    println!("🌐 Server running on http://127.0.0.1:3000");
    println!("   Try the different timezone formats above!");

    axum::serve(listener, app).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_datetime_range_with_calendar_events() {
        let events = get_calendar_events();

        // Test morning range (UTC)
        let morning_range =
            DateTimeRangeQuery::between("2024-12-19T08:00:00Z", "2024-12-19T11:00:00Z");

        let morning_events: Vec<_> = events
            .iter()
            .filter(|event| morning_range.contains(event.start_time))
            .collect();

        assert_eq!(morning_events.len(), 1); // Only morning meeting
        assert_eq!(morning_events[0].title, "Morning Meeting");
    }

    #[test]
    fn test_timezone_conversion_accuracy() {
        // Same time in different timezone formats should filter same events
        let utc_range = DateTimeRangeQuery::between(
            "2024-12-19T02:00:00Z", // 2 AM UTC
            "2024-12-19T06:00:00Z", // 6 AM UTC
        );

        let plus_eight_range = DateTimeRangeQuery::between(
            "2024-12-19T10:00:00+08:00", // 10 AM +8 = 2 AM UTC
            "2024-12-19T14:00:00+08:00", // 2 PM +8 = 6 AM UTC
        );

        let events = get_calendar_events();

        let utc_filtered: Vec<_> = events
            .iter()
            .filter(|event| utc_range.contains(event.start_time))
            .collect();

        let timezone_filtered: Vec<_> = events
            .iter()
            .filter(|event| plus_eight_range.contains(event.start_time))
            .collect();

        // Should filter exactly the same events
        assert_eq!(utc_filtered.len(), timezone_filtered.len());
    }

    #[test]
    fn test_sql_generation_for_database() {
        let range =
            DateTimeRangeQuery::between("2024-12-19T10:00:00+08:00", "2024-12-19T18:00:00+08:00");

        let sql = range.to_sql_conditions("created_at");

        // Should convert timezone to UTC in SQL
        assert!(sql.contains("created_at >= '2024-12-19 02:00:00")); // 10:00 +8 = 02:00 UTC
        assert!(sql.contains("created_at < '2024-12-19 10:00:00")); // 18:00 +8 = 10:00 UTC
    }

    #[test]
    fn test_real_world_validation() {
        // Test validation like in real projects
        let valid_range =
            DateTimeRangeQuery::between("2024-12-19T10:00:00Z", "2024-12-19T18:00:00Z");

        // Should pass basic validation
        assert!(valid_range.validate_range().is_ok());

        // Should pass duration limit (8 hours < 30 days)
        assert!(valid_range.validate_max_duration(30).is_ok());

        // Test with a longer range that should fail duration validation
        let long_range =
            DateTimeRangeQuery::between("2024-12-19T10:00:00Z", "2024-12-21T18:00:00Z");
        assert!(long_range.validate_max_duration(1).is_err()); // 2+ days > 1 day limit
    }

    #[test]
    fn test_business_logic_with_parsed_datetimes() {
        let range =
            DateTimeRangeQuery::between("2024-12-19T10:00:00+08:00", "2024-12-19T18:00:00+08:00");

        // Parse to UTC for database usage
        let start_utc = range.parse_start_utc().unwrap();
        let end_utc = range.parse_end_utc().unwrap();

        // Verify timezone conversion
        assert_eq!(start_utc.hour(), 2); // 10:00 +8 = 02:00 UTC
        assert_eq!(end_utc.hour(), 10); // 18:00 +8 = 10:00 UTC

        // Calculate duration
        let duration = range.get_duration().unwrap();
        assert_eq!(duration.num_hours(), 8);
    }
}
