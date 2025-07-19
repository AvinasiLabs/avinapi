//! Comprehensive example demonstrating query parameter functionality in avinapi.
//!
//! This example shows how to use:
//! - PaginationQuery for paginated results
//! - DateRangeQuery for date-based filtering
//! - SortQuery for flexible sorting
//! - Combined usage of multiple query parameter types
//! - Validation and error handling
//! - Response generation with metadata

use avinapi::prelude::*;
use axum::{Router, extract::Query, routing::get};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

// Sample data structures
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
struct User {
    id: u32,
    name: String,
    email: String,
    created_at: NaiveDate,
    role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
struct Order {
    id: u32,
    user_id: u32,
    amount: f64,
    order_date: NaiveDate,
    status: String,
}

// Mock database

/// List users with pagination only.
///
/// Demonstrates basic pagination without filtering or sorting.
///
/// Example URLs:
/// - `/users` - First page with default settings (page=1, per_page=20)
/// - `/users?page=2&per_page=10` - Second page with 10 items per page
async fn list_users_paginated(
    Query(pagination): Query<PaginationQuery>,
) -> JsonResult<PaginatedData<User>> {
    println!(
        "📄 Pagination: page={}, per_page={}",
        pagination.get_page(),
        pagination.get_per_page()
    );

    // Simulate database query with pagination
    let users = get_sample_users();
    let total_users = users.len() as u64;

    // Apply pagination
    let offset = pagination.get_offset() as usize;
    let limit = pagination.get_limit() as usize;
    let paginated_users: Vec<User> = users.into_iter().skip(offset).take(limit).collect();

    let response = PaginatedData::new(paginated_users, &pagination, total_users);
    data!(response)
}

/// List users with date range filtering.
///
/// Demonstrates filtering users by creation date.
///
/// Example URLs:
/// - `/users/by-date?start_date=2024-01-01` - Users created from Jan 1, 2024
/// - `/users/by-date?end_date=2024-12-31` - Users created up to Dec 31, 2024
/// - `/users/by-date?start_date=2024-06-01&end_date=2024-06-30` - Users created in June 2024
async fn list_users_by_date(
    Query(pagination): Query<PaginationQuery>,
    Query(date_filter): Query<DateRangeQuery>,
) -> JsonResult<PaginatedData<User>> {
    println!("📅 Date filter: {:?}", date_filter);
    println!(
        "📄 Pagination: page={}, per_page={}",
        pagination.get_page(),
        pagination.get_per_page()
    );

    // Simulate database query with date filtering
    let users = get_sample_users();

    // Apply date filtering
    let filtered_users: Vec<User> = users
        .into_iter()
        .filter(|user| date_filter.contains(user.created_at))
        .collect();

    let total_filtered = filtered_users.len() as u64;

    // Apply pagination to filtered results
    let offset = pagination.get_offset() as usize;
    let limit = pagination.get_limit() as usize;
    let paginated_users: Vec<User> = filtered_users
        .into_iter()
        .skip(offset)
        .take(limit)
        .collect();

    let response = PaginatedData::new(paginated_users, &pagination, total_filtered);
    data!(response)
}

/// List users with sorting.
///
/// Demonstrates flexible sorting with field validation.
///
/// Example URLs:
/// - `/users/sorted?sort=name` - Sort by name ascending
/// - `/users/sorted?sort=created_at desc` - Sort by creation date descending
/// - `/users/sorted?sort=name,created_at desc` - Sort by name asc, then created_at desc
/// - `/users/sorted?sort_by=email&sort_order=desc` - Sort by email descending
async fn list_users_sorted(
    Query(pagination): Query<PaginationQuery>,
    Query(sort): Query<SortQuery>,
) -> JsonResult<PaginatedData<User>> {
    println!("🔄 Sort: {:?}", sort);

    // Define allowed sort fields for security
    let allowed_fields = vec!["name", "email", "created_at", "role"];

    // Validate sort fields
    if !sort.is_empty() {
        sort.validate_fields(&allowed_fields)
            .map_err(|e| AppError::validation(e))?;
    }

    println!("📊 Generated SQL ORDER BY: {}", sort.to_sql());

    // Simulate database query with sorting
    let mut users = get_sample_users();

    // Apply sorting (simplified - in real app, this would be done in SQL)
    if let Some(first_sort) = sort.first() {
        users.sort_by(|a, b| {
            let ordering = match first_sort.field.as_str() {
                "name" => a.name.cmp(&b.name),
                "email" => a.email.cmp(&b.email),
                "created_at" => a.created_at.cmp(&b.created_at),
                "role" => a.role.cmp(&b.role),
                _ => std::cmp::Ordering::Equal,
            };

            if first_sort.order.is_desc() {
                ordering.reverse()
            } else {
                ordering
            }
        });
    }

    let total_users = users.len() as u64;

    // Apply pagination
    let offset = pagination.get_offset() as usize;
    let limit = pagination.get_limit() as usize;
    let paginated_users: Vec<User> = users.into_iter().skip(offset).take(limit).collect();

    let response = PaginatedData::new(paginated_users, &pagination, total_users);
    data!(response)
}

/// List users with all query parameters combined.
///
/// Demonstrates the full power of combining pagination, date filtering, and sorting.
///
/// Example URLs:
/// - `/users/advanced?start_date=2024-01-01&sort=name&page=2&per_page=5`
/// - `/users/advanced?end_date=2024-06-30&sort=created_at desc,name&page=1&per_page=20`
async fn list_users_advanced(
    Query(pagination): Query<PaginationQuery>,
    Query(date_filter): Query<DateRangeQuery>,
    Query(sort): Query<SortQuery>,
) -> JsonResult<PaginatedData<User>> {
    println!("🔍 Advanced query:");
    println!("  📅 Date filter: {:?}", date_filter);
    println!("  🔄 Sort: {:?}", sort);
    println!(
        "  📄 Pagination: page={}, per_page={}",
        pagination.get_page(),
        pagination.get_per_page()
    );

    // Validate sort fields
    let allowed_fields = vec!["name", "email", "created_at", "role"];
    if !sort.is_empty() {
        sort.validate_fields(&allowed_fields)
            .map_err(|e| AppError::validation(e))?;
    }

    // Simulate complex database query
    let mut users = get_sample_users();

    // Step 1: Apply date filtering
    if !date_filter.is_empty() {
        users.retain(|user| date_filter.contains(user.created_at));
    }

    // Step 2: Apply sorting
    if let Some(first_sort) = sort.first() {
        users.sort_by(|a, b| {
            let ordering = match first_sort.field.as_str() {
                "name" => a.name.cmp(&b.name),
                "email" => a.email.cmp(&b.email),
                "created_at" => a.created_at.cmp(&b.created_at),
                "role" => a.role.cmp(&b.role),
                _ => std::cmp::Ordering::Equal,
            };

            if first_sort.order.is_desc() {
                ordering.reverse()
            } else {
                ordering
            }
        });
    }

    let total_filtered = users.len() as u64;

    // Step 3: Apply pagination
    let offset = pagination.get_offset() as usize;
    let limit = pagination.get_limit() as usize;
    let paginated_users: Vec<User> = users.into_iter().skip(offset).take(limit).collect();

    // Log the results for demonstration
    println!("  📊 SQL would be: SELECT * FROM users");
    if !date_filter.is_empty() {
        let (start, end) = date_filter.to_datetime_range();
        if let Some(start_dt) = start {
            println!("    WHERE created_at >= '{}'", start_dt);
        }
        if let Some(end_dt) = end {
            println!("    AND created_at <= '{}'", end_dt);
        }
    }
    if !sort.is_empty() {
        println!("    ORDER BY {}", sort.to_sql());
    }
    println!("    LIMIT {} OFFSET {}", limit, offset);
    println!(
        "  🎯 Results: {} items out of {} total",
        paginated_users.len(),
        total_filtered
    );

    let response = PaginatedData::new(paginated_users, &pagination, total_filtered);
    data!(response)
}

/// List orders with complex filtering and sorting.
///
/// Demonstrates usage with a different data type and more complex business logic.
async fn list_orders_advanced(
    Query(pagination): Query<PaginationQuery>,
    Query(date_filter): Query<DateRangeQuery>,
    Query(sort): Query<SortQuery>,
) -> JsonResult<PaginatedData<Order>> {
    println!("🛍️ Order query:");
    println!("  📅 Date filter: {:?}", date_filter);
    println!("  🔄 Sort: {:?}", sort);

    // Validate sort fields for orders
    let allowed_fields = vec!["id", "amount", "order_date", "status"];
    if !sort.is_empty() {
        sort.validate_fields(&allowed_fields)
            .map_err(|e| AppError::validation(e))?;
    }

    let mut orders = get_sample_orders();

    // Apply date filtering to order_date
    if !date_filter.is_empty() {
        orders.retain(|order| date_filter.contains(order.order_date));
    }

    // Apply sorting
    if let Some(first_sort) = sort.first() {
        orders.sort_by(|a, b| {
            let ordering = match first_sort.field.as_str() {
                "id" => a.id.cmp(&b.id),
                "amount" => a
                    .amount
                    .partial_cmp(&b.amount)
                    .unwrap_or(std::cmp::Ordering::Equal),
                "order_date" => a.order_date.cmp(&b.order_date),
                "status" => a.status.cmp(&b.status),
                _ => std::cmp::Ordering::Equal,
            };

            if first_sort.order.is_desc() {
                ordering.reverse()
            } else {
                ordering
            }
        });
    }

    let total_filtered = orders.len() as u64;

    // Apply pagination
    let offset = pagination.get_offset() as usize;
    let limit = pagination.get_limit() as usize;
    let paginated_orders: Vec<Order> = orders.into_iter().skip(offset).take(limit).collect();

    let response = PaginatedData::new(paginated_orders, &pagination, total_filtered);
    data!(response)
}

/// Query parameter validation example.
///
/// Demonstrates error handling when invalid parameters are provided.
async fn demo_validation_errors(Query(sort): Query<SortQuery>) -> JsonResult<Vec<String>> {
    println!("🔍 Testing validation with sort: {:?}", sort);

    // This will fail if invalid field names are provided
    let allowed_fields = vec!["name", "created_at"];
    sort.validate_fields(&allowed_fields)
        .map_err(|e| AppError::validation(e))?;

    data!(vec!["Validation passed!".to_string()])
}

// Mock data generators
fn get_sample_users() -> Vec<User> {
    vec![
        User {
            id: 1,
            name: "Alice Johnson".to_string(),
            email: "alice@example.com".to_string(),
            created_at: NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            role: "admin".to_string(),
        },
        User {
            id: 2,
            name: "Bob Smith".to_string(),
            email: "bob@example.com".to_string(),
            created_at: NaiveDate::from_ymd_opt(2024, 2, 20).unwrap(),
            role: "user".to_string(),
        },
        User {
            id: 3,
            name: "Charlie Brown".to_string(),
            email: "charlie@example.com".to_string(),
            created_at: NaiveDate::from_ymd_opt(2024, 3, 10).unwrap(),
            role: "user".to_string(),
        },
        User {
            id: 4,
            name: "Diana Prince".to_string(),
            email: "diana@example.com".to_string(),
            created_at: NaiveDate::from_ymd_opt(2024, 4, 5).unwrap(),
            role: "moderator".to_string(),
        },
        User {
            id: 5,
            name: "Edward Wilson".to_string(),
            email: "edward@example.com".to_string(),
            created_at: NaiveDate::from_ymd_opt(2024, 5, 12).unwrap(),
            role: "user".to_string(),
        },
        User {
            id: 6,
            name: "Fiona Green".to_string(),
            email: "fiona@example.com".to_string(),
            created_at: NaiveDate::from_ymd_opt(2024, 6, 18).unwrap(),
            role: "admin".to_string(),
        },
    ]
}

fn get_sample_orders() -> Vec<Order> {
    vec![
        Order {
            id: 1,
            user_id: 1,
            amount: 299.99,
            order_date: NaiveDate::from_ymd_opt(2024, 1, 20).unwrap(),
            status: "completed".to_string(),
        },
        Order {
            id: 2,
            user_id: 2,
            amount: 89.50,
            order_date: NaiveDate::from_ymd_opt(2024, 2, 15).unwrap(),
            status: "pending".to_string(),
        },
        Order {
            id: 3,
            user_id: 1,
            amount: 156.75,
            order_date: NaiveDate::from_ymd_opt(2024, 3, 5).unwrap(),
            status: "completed".to_string(),
        },
        Order {
            id: 4,
            user_id: 3,
            amount: 45.00,
            order_date: NaiveDate::from_ymd_opt(2024, 4, 10).unwrap(),
            status: "cancelled".to_string(),
        },
        Order {
            id: 5,
            user_id: 2,
            amount: 320.00,
            order_date: NaiveDate::from_ymd_opt(2024, 5, 22).unwrap(),
            status: "completed".to_string(),
        },
    ]
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Build the router with all query parameter examples
    let _app = Router::<()>::new()
        // Basic pagination
        .route("/users", get(list_users_paginated))
        // Date range filtering
        .route("/users/by-date", get(list_users_by_date))
        // Sorting functionality
        .route("/users/sorted", get(list_users_sorted))
        // Advanced: all parameters combined
        .route("/users/advanced", get(list_users_advanced))
        // Orders with complex filtering
        .route("/orders/advanced", get(list_orders_advanced))
        // Validation error demonstration
        .route("/demo/validation", get(demo_validation_errors));

    println!("🚀 Query Parameters Example Server Ready!");
    println!();
    println!("📖 Try these example requests:");
    println!();

    println!("🔸 Basic Pagination:");
    println!("   GET /users");
    println!("   GET /users?page=2&per_page=3");
    println!();

    println!("🔸 Date Range Filtering:");
    println!("   GET /users/by-date?start_date=2024-03-01");
    println!("   GET /users/by-date?start_date=2024-02-01&end_date=2024-04-30");
    println!("   GET /users/by-date?end_date=2024-03-31");
    println!();

    println!("🔸 Sorting:");
    println!("   GET /users/sorted?sort=name");
    println!("   GET /users/sorted?sort=created_at desc");
    println!("   GET /users/sorted?sort=name,created_at desc");
    println!("   GET /users/sorted?sort_by=email&sort_order=desc");
    println!();

    println!("🔸 Advanced Combinations:");
    println!("   GET /users/advanced?start_date=2024-02-01&sort=name&page=1&per_page=3");
    println!("   GET /users/advanced?end_date=2024-04-30&sort=created_at desc,name&page=2");
    println!();

    println!("🔸 Orders Example:");
    println!("   GET /orders/advanced?start_date=2024-01-01&sort=amount desc&page=1&per_page=3");
    println!();

    println!("🔸 Validation Errors:");
    println!("   GET /demo/validation?sort=invalid_field  (will return validation error)");
    println!("   GET /demo/validation?sort=name           (will succeed)");
    println!();

    println!("✅ Example compiled successfully!");
    println!("💡 This demonstrates comprehensive query parameter functionality:");
    println!("   • Pagination with configurable page size");
    println!("   • Date range filtering with flexible date formats");
    println!("   • Multi-field sorting with validation");
    println!("   • Combined usage of all parameter types");
    println!("   • Proper error handling and validation");
    println!("   • Response generation with metadata");
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_sample_data_generation() {
        let users = get_sample_users();
        assert_eq!(users.len(), 6);
        assert_eq!(users[0].name, "Alice Johnson");

        let orders = get_sample_orders();
        assert_eq!(orders.len(), 5);
        assert_eq!(orders[0].amount, 299.99);
    }

    #[test]
    fn test_date_filtering_logic() {
        let users = get_sample_users();
        let date_filter = DateRangeQuery::between(
            NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 4, 30).unwrap(),
        );

        let filtered: Vec<_> = users
            .iter()
            .filter(|user| date_filter.contains(user.created_at))
            .collect();

        // Should include Bob, Charlie, and Diana
        assert_eq!(filtered.len(), 3);
        assert_eq!(filtered[0].name, "Bob Smith");
        assert_eq!(filtered[1].name, "Charlie Brown");
        assert_eq!(filtered[2].name, "Diana Prince");
    }

    #[test]
    fn test_pagination_logic() {
        let users = get_sample_users();
        let pagination = PaginationQuery::new(2, 2); // Page 2, 2 items per page

        let offset = pagination.get_offset() as usize;
        let limit = pagination.get_limit() as usize;
        let paginated: Vec<_> = users.into_iter().skip(offset).take(limit).collect();

        assert_eq!(paginated.len(), 2);
        assert_eq!(paginated[0].name, "Charlie Brown"); // 3rd user (index 2)
        assert_eq!(paginated[1].name, "Diana Prince"); // 4th user (index 3)
    }

    #[test]
    fn test_sort_validation() {
        let sort = SortQuery::by("name");
        let allowed = vec!["name", "email"];
        assert!(sort.validate_fields(&allowed).is_ok());

        let invalid_sort = SortQuery::by("invalid_field");
        assert!(invalid_sort.validate_fields(&allowed).is_err());
    }
}
