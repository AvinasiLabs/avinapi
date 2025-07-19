# Avinasi Web

A comprehensive Rust web API foundation library providing reusable patterns and utilities for building consistent, maintainable web APIs.

[![Rust](https://img.shields.io/badge/rust-2024%20edition-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/lilhammer111/avinasi-web#license)
[![Tests](https://img.shields.io/badge/tests-79%20passing-brightgreen.svg)](#testing)

<!-- Will be enabled after publishing to crates.io:
[![Crates.io](https://img.shields.io/crates/v/avinasi-web.svg)](https://crates.io/crates/avinasi-web)
[![Documentation](https://docs.rs/avinasi-web/badge.svg)](https://docs.rs/avinasi-web)
-->

## 🚀 Features

### 📋 Query Parameters
- **`DateRangeQuery`** - Simple date-based filtering (YYYY-MM-DD format)
- **`DateTimeRangeQuery`** - Precise datetime filtering with timezone support (RFC 3339)
- **`PaginationQuery`** - Page-based pagination with metadata
- **`SortQuery`** - Flexible multi-field sorting with SQL generation

### 🔄 Response Handling
- **`ApiResponse<T>`** - Standardized JSON response format
- **`PaginatedData<T>`** - Paginated response wrapper with metadata
- **Response macros** - `data!()`, `empty!()` for quick responses

### ❌ Error Management
- **`AppError`** - Comprehensive error types for web applications
- **Unified HTTP 200 responses** - All responses return HTTP 200 with business codes in JSON
- **Business code mapping** - Errors map to semantic business codes (SUCCESS, VALIDATION_ERROR, etc.)
- **Validation integration** - Seamless validator crate integration

### ✅ Validation
- **`ValidatedJson<T>`** - JSON extractor with automatic validation
- **Custom validators** - Password strength, phone numbers, email, etc.
- **Query validation** - Built-in validation for all query parameter types

### 🛠️ Utilities
- **Middleware helpers** - CORS, request ID, tracing integration
- **OpenAPI support** - Full utoipa integration with examples
- **Feature flags** - Optional dependencies (axum, sqlx, jwt)

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
avinasi-web = "0.1.0"

# Optional features
avinasi-web = { version = "0.1.0", features = ["axum", "sqlx", "jwt"] }
```

### Available Features

- **`axum`** (default) - Axum web framework integration
- **`sqlx`** (default) - Database integration utilities
- **`jwt`** (default) - JWT authentication helpers

## 🎯 Quick Start

```rust
use avinasi_web::prelude::*;
use axum::{Router, extract::Query, routing::get};

#[derive(serde::Serialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

// Simple pagination endpoint
async fn list_users(
    Query(pagination): Query<PaginationQuery>,
) -> JsonResult<PaginatedData<User>> {
    let users = get_users_from_db(
        pagination.get_limit(),
        pagination.get_offset(),
    ).await?;
    
    let total_count = count_users().await?;
    let response = PaginatedData::new(users, &pagination, total_count);
    
    data!(response)
}

// Advanced filtering with multiple query parameters
async fn list_orders(
    Query(pagination): Query<PaginationQuery>,
    Query(date_range): Query<DateTimeRangeQuery>,
    Query(sort): Query<SortQuery>,
) -> JsonResult<PaginatedData<Order>> {
    // Validate datetime range (max 30 days)
    date_range.validate_max_duration(30)?;
    
    // Parse to UTC for database queries
    let start_utc = date_range.parse_start_utc()?;
    let end_utc = date_range.parse_end_utc()?;
    
    // Generate SQL ORDER BY clause
    let order_clause = sort.to_sql_with_prefix("orders");
    
    let orders = query_orders(start_utc, end_utc, order_clause, pagination).await?;
    data!(orders)
}

fn main() {
    let app = Router::new()
        .route("/users", get(list_users))
        .route("/orders", get(list_orders));
    
    // ... run server
}
```

## 🗓️ Date Handling Examples

### Simple Date Filtering

```rust
// URL: /reports?start_date=2024-01-01&end_date=2024-12-31
async fn reports(Query(date_range): Query<DateRangeQuery>) -> JsonResult<Vec<Report>> {
    let reports = Report::find_by_date_range(
        date_range.start_date,
        date_range.end_date
    ).await?;
    
    data!(reports)
}
```

### Precise DateTime Filtering with Timezones

```rust
// URL: /events?start_datetime=2024-12-19T10:30:00+08:00&end_datetime=2024-12-19T18:00:00+08:00
async fn events(Query(datetime_range): Query<DateTimeRangeQuery>) -> JsonResult<Vec<Event>> {
    // Validate range
    datetime_range.validate_range()?;
    datetime_range.validate_max_duration(7)?; // Max 7 days
    
    // Parse to UTC (handles timezone conversion automatically)
    let start_utc = datetime_range.parse_start_utc()?;
    let end_utc = datetime_range.parse_end_utc()?;
    
    let events = Event::find_by_datetime_range(start_utc, end_utc).await?;
    data!(events)
}
```

## 📊 Supported Date Formats

### DateRangeQuery (Simple dates)
- `start_date=2024-12-19` - Date only format
- `end_date=2024-12-31` - Perfect for daily/monthly reports

### DateTimeRangeQuery (Precise times)
- `start_datetime=2024-12-19T10:30:00Z` - UTC format
- `start_datetime=2024-12-19T10:30:00+08:00` - Timezone aware
- `start_datetime=2024-12-19T10:30:00-05:00` - Negative timezone

## ✅ Validation Example

```rust
use avinasi_web::{ValidatedJson, validation::*};

#[derive(Deserialize, Validate, ToSchema)]
struct CreateUserRequest {
    #[validate(length(min = 1, max = 100))]
    name: String,
    
    #[validate(email)]
    email: String,
    
    #[validate(custom = "validate_password_strength")]
    password: String,
}

async fn create_user(
    ValidatedJson(request): ValidatedJson<CreateUserRequest>
) -> JsonResult<User> {
    // Request is guaranteed to be valid here
    let user = User::create(request.name, request.email, request.password).await?;
    data!(user)
}
```

## 🎨 Response Format

**Important**: This library follows a unified response pattern where all endpoints return **HTTP 200** status codes, with business logic status indicated by the `code` field in the JSON response body.

All endpoints return a consistent JSON structure:

```json
{
  "code": "SUCCESS",           // Business status code
  "data": { ... },            // Response data
  "message": null             // Optional message
}
```

Error responses (still HTTP 200):

```json
{
  "code": "VALIDATION_ERROR", // Business error code
  "data": null,
  "message": "name: Name is required, email: Invalid email format"
}
```

### Supported Business Codes

- `SUCCESS` - Operation completed successfully
- `VALIDATION_ERROR` - Input validation failed
- `AUTHENTICATION_ERROR` - Authentication required or failed
- `AUTHORIZATION_ERROR` - Insufficient permissions
- `NOT_FOUND` - Requested resource not found
- `CONFLICT` - Resource conflict (e.g., duplicate email)
- `INTERNAL_ERROR` - Server-side error
- `DATABASE_ERROR` - Database operation failed
- `NETWORK_ERROR` - External service call failed

Paginated responses:

```json
{
  "code": "SUCCESS",
  "data": {
    "data": [...],
    "meta": {
      "current_page": 1,
      "per_page": 20,
      "total_items": 150,
      "total_pages": 8,
      "has_next_page": true,
      "has_prev_page": false
    }
  },
  "message": null
}
```

## 🔧 Configuration

### Cargo.toml features

```toml
# Minimal setup
avinasi-web = { version = "0.1.0", default-features = false }

# With specific features
avinasi-web = { version = "0.1.0", features = ["axum", "sqlx"] }

# Full features
avinasi-web = { version = "0.1.0", features = ["axum", "sqlx", "jwt"] }
```

### Environment variables

The library respects these environment variables:

- `RUST_LOG` - Logging level (uses tracing)
- `DATABASE_URL` - Database connection (when using sqlx feature)

## 🏗️ Architecture

```
avinasi-web/
├── src/
│   ├── error/          # Error handling and HTTP status mapping
│   ├── middleware/     # Common middleware utilities
│   ├── query/          # Query parameter types and validation
│   │   ├── date_range.rs      # Simple date filtering
│   │   ├── datetime_range.rs  # Precise datetime filtering
│   │   ├── pagination.rs     # Page-based pagination
│   │   └── sort.rs           # Multi-field sorting
│   ├── response/       # Standardized response types
│   ├── validation/     # JSON validation and custom validators
│   └── prelude.rs     # Common imports
├── examples/          # Usage examples
└── tests/            # Integration tests
```

## 📚 Examples

Run the included examples:

```bash
# Basic query parameter usage
cargo run --example query_parameters

# DateTime range handling with different formats
cargo run --example datetime_range_usage
```

Then visit:
- `http://localhost:3000/users?page=1&per_page=10`
- `http://localhost:3000/calendar?start_datetime=2024-12-19T10:30:00+08:00`

## 🤝 Integration with Other Crates

### Axum Integration

```rust
use avinasi_web::prelude::*;
use axum::{Router, middleware};

let app = Router::new()
    .route("/api/users", get(list_users))
    .layer(middleware::from_fn(request_id_middleware))
    .layer(cors_layer());
```

### SQLx Integration

```rust
use avinasi_web::query::*;
use sqlx::{PgPool, query_as};

async fn get_orders(
    pool: &PgPool,
    datetime_range: &DateTimeRangeQuery,
    pagination: &PaginationQuery,
) -> Result<Vec<Order>, sqlx::Error> {
    let (start_utc, end_utc) = datetime_range.parse_utc_range();
    
    query_as!(
        Order,
        r#"
        SELECT * FROM orders 
        WHERE ($1::timestamptz IS NULL OR created_at >= $1)
          AND ($2::timestamptz IS NULL OR created_at < $2)
        ORDER BY created_at DESC
        LIMIT $3 OFFSET $4
        "#,
        start_utc,
        end_utc,
        pagination.get_limit() as i64,
        pagination.get_offset() as i64
    )
    .fetch_all(pool)
    .await
}
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run with all features
cargo test --all-features

# Run examples
cargo test --examples

# Run doc tests
cargo test --doc
```

## 📖 Documentation

- [API Documentation](https://docs.rs/avinasi-web)
- [Examples](./examples/)
- [Changelog](./CHANGELOG.md)

## 🚢 Publishing to Other Projects

### Option 1: Publish to crates.io (Recommended)

1. **Prepare for publishing:**
   ```bash
   cargo publish --dry-run  # Test the publish
   cargo publish           # Publish to crates.io
   ```

2. **Use in other projects:**
   ```toml
   [dependencies]
   avinasi-web = "0.1.0"
   ```

### Option 2: Git dependencies

```toml
[dependencies]
avinasi-web = { git = "https://github.com/your-username/avinasi-web", tag = "v0.1.0" }
```

### Option 3: Local development

```toml
[dependencies]
avinasi-web = { path = "../avinasi-web" }
```

## 🔄 Versioning

This project follows [Semantic Versioning](https://semver.org/):
- **MAJOR** version for incompatible API changes
- **MINOR** version for backwards-compatible functionality additions  
- **PATCH** version for backwards-compatible bug fixes

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development setup

```bash
git clone https://github.com/your-username/avinasi-web
cd avinasi-web
cargo build --all-features
cargo test --all-features
```

## 📄 License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## 🙏 Acknowledgments

- [Axum](https://github.com/tokio-rs/axum) - Modern web framework
- [Chrono](https://github.com/chronotope/chrono) - Date and time library
- [Serde](https://github.com/serde-rs/serde) - Serialization framework
- [Validator](https://github.com/Keats/validator) - Data validation
- [utoipa](https://github.com/juhaku/utoipa) - OpenAPI documentation

---

## 👨‍💻 Author

Created by [@lilhammer111](https://github.com/lilhammer111) | **Latte Team**, **Avinasi Labs**