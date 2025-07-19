# Avinapi Library Development Prompt

## Project Overview

You are tasked with creating `avinapi`, a comprehensive Rust web API foundation library that provides reusable patterns and utilities for building consistent, maintainable web APIs. This library extracts and packages the proven design patterns from the `rust-web-api-patterns.md` document.

## Project Goals

1. **Consistency**: Provide uniform response patterns and error handling across all web projects
2. **Reusability**: Enable rapid bootstrapping of new web API projects
3. **Maintainability**: Centralize common patterns for easier updates and improvements
4. **Framework Flexibility**: Support multiple web frameworks through feature flags
5. **Developer Experience**: Reduce boilerplate and provide intuitive APIs

## Technical Requirements

### Project Structure
```
avinapi/
├── Cargo.toml
├── README.md
├── LICENSE
├── src/
│   ├── lib.rs
│   ├── response/
│   │   ├── mod.rs
│   │   ├── api_response.rs
│   │   └── macros.rs
│   ├── error/
│   │   ├── mod.rs
│   │   └── app_error.rs
│   ├── validation/
│   │   ├── mod.rs
│   │   └── validated_json.rs
│   ├── query/
│   │   ├── mod.rs
│   │   ├── pagination.rs
│   │   ├── date_range.rs
│   │   └── sort.rs
│   ├── middleware/
│   │   ├── mod.rs
│   │   ├── auth.rs
│   │   ├── cors.rs
│   │   ├── rate_limit.rs
│   │   └── request_id.rs
│   ├── testing/
│   │   ├── mod.rs
│   │   └── test_context.rs
│   └── prelude.rs
├── examples/
│   ├── basic_api/
│   ├── auth_api/
│   └── full_featured/
├── tests/
│   ├── integration/
│   └── unit/
└── docs/
    ├── getting_started.md
    ├── patterns.md
    └── migration_guide.md
```

### Feature Flags Design
```toml
[features]
default = ["axum"]
axum = ["dep:axum", "dep:tower", "dep:tower-http"]
actix = ["dep:actix-web"]  # Future support
warp = ["dep:warp"]        # Future support
testing = ["dep:testcontainers", "dep:sqlx"]
redis = ["dep:redis"]
jwt = ["dep:jsonwebtoken"]
```

### Core Dependencies
```toml
[dependencies]
# Core
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
tracing = "0.1"
uuid = { version = "1.0", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }

# Validation
validator = { version = "0.16", features = ["derive"] }

# OpenAPI Documentation
utoipa = { version = "4.0", features = ["axum_extras", "chrono", "uuid"] }

# Framework specific (feature-gated)
axum = { version = "0.7", optional = true, features = ["macros"] }
tower = { version = "0.4", optional = true }
tower-http = { version = "0.5", optional = true, features = ["cors", "trace", "request-id"] }

# Testing (feature-gated)
testcontainers = { version = "0.15", optional = true }
sqlx = { version = "0.7", optional = true, features = ["postgres", "runtime-tokio-rustls", "uuid", "chrono"] }

# Optional features
redis = { version = "0.24", optional = true }
jsonwebtoken = { version = "9.0", optional = true }
```

## Module Implementation Requirements

### 1. Response Module (`src/response/`)

**File: `api_response.rs`**
- Implement `ApiResponse<T>` struct exactly as defined in patterns document
- Implement `ResponseCode` enum with all error categories
- Provide helper methods: `success()`, `empty()`, `error()`
- Add comprehensive utoipa schema annotations
- Include serialization attributes for clean JSON output

**File: `macros.rs`**
- Implement `data!`, `empty!`, and `error!` macros
- Ensure macros work with axum's `Json` type
- Add comprehensive documentation with usage examples
- Include macro export declarations

### 2. Error Module (`src/error/`)

**File: `app_error.rs`**
- Implement `AppError` enum with all categories from patterns document
- Implement `AppResult<T>` type alias
- Add `into_response_parts()` method for HTTP conversion
- Implement `From` traits for common error types (sqlx, validation, etc.)
- Add `IntoResponse` implementation for axum integration
- Include comprehensive error context preservation

### 3. Validation Module (`src/validation/`)

**File: `validated_json.rs`**
- Implement `ValidatedJson<T>` extractor exactly as shown in patterns
- Handle JSON parsing errors with proper ApiResponse format
- Handle validation errors with field-level error messages
- Provide clear error messages for debugging
- Support async extraction with proper error propagation

### 4. Query Module (`src/query/`)

**File: `pagination.rs`**
- Implement `PaginationQuery` with validation
- Implement `PaginatedData<T>` response structure
- Implement `PaginationMeta` with all helper methods
- Add comprehensive utoipa annotations
- Include reasonable defaults and validation rules

**File: `date_range.rs`**
- Implement `DateRangeQuery` for date filtering
- Support optional start/end dates
- Handle timezone considerations
- Add validation for date range logic

**File: `sort.rs`**
- Implement `SortQuery` with field and order
- Implement `SortOrder` enum
- Add validation for allowed sort fields (extensible)
- Support multiple sort criteria

### 5. Middleware Module (`src/middleware/`)

**File: `auth.rs`**
- Implement JWT-based `AuthUser` extractor (feature-gated)
- Support Bearer token extraction from headers
- Provide flexible token validation
- Include comprehensive error handling
- Support custom claims structure

**File: `cors.rs`**
- Provide pre-configured CORS middleware for common use cases
- Support development and production configurations
- Allow customization of allowed origins, methods, headers

**File: `rate_limit.rs`**
- Implement configurable rate limiting middleware
- Support in-memory and Redis-backed rate limiting
- Provide reasonable defaults for different use cases
- Include proper error responses

**File: `request_id.rs`**
- Generate and propagate request IDs
- Integrate with tracing infrastructure
- Support custom ID generation strategies
- Add response headers for request tracking

### 6. Testing Module (`src/testing/`)

**File: `test_context.rs`**
- Implement `TestContext` with database setup
- Support testcontainers for isolated testing
- Provide migration running capabilities
- Include cleanup and teardown logic
- Support multiple database types

### 7. Core Files

**File: `lib.rs`**
- Define feature-gated exports
- Provide clear module organization
- Include comprehensive library documentation
- Define version and metadata

**File: `prelude.rs`**
- Export commonly used types and traits
- Organize exports by functionality
- Provide one-import convenience for users
- Include documentation for common usage patterns

## Code Standards

### 1. Error Handling
- All public functions return `AppResult<T>` where appropriate
- Use `?` operator for error propagation
- Provide meaningful error messages with context
- Map external errors to `AppError` variants

### 2. Documentation
- Every public item must have comprehensive docs
- Include usage examples in doc comments
- Provide links to patterns document where relevant
- Use `#[doc = include_str!("../docs/...")]` for complex docs

### 3. Testing
- Unit tests for all core functionality
- Integration tests demonstrating end-to-end usage
- Property-based tests where appropriate
- Benchmark tests for performance-critical code

### 4. Validation
- Use `validator` crate consistently
- Provide clear validation error messages
- Support custom validation functions
- Document validation rules in schema annotations

### 5. Tracing Integration
- All public functions should support tracing
- Use `#[tracing::instrument]` appropriately
- Skip sensitive data in traces
- Provide structured logging fields

## Implementation Guidelines

### Phase 1: Core Infrastructure
1. Set up project structure and Cargo.toml
2. Implement response types and macros
3. Implement error handling system
4. Create basic validation extractor
5. Write comprehensive tests

### Phase 2: Query and Pagination
1. Implement query parameter types
2. Add pagination support with metadata
3. Create composable query system
4. Add comprehensive query validation
5. Document query patterns

### Phase 3: Middleware Collection
1. Implement authentication middleware
2. Add CORS and security middleware
3. Create rate limiting middleware
4. Add request ID and tracing middleware
5. Provide configuration options

### Phase 4: Testing and Documentation
1. Create testing utilities and context
2. Write comprehensive examples
3. Add migration guides
4. Create getting started documentation
5. Prepare for publication

### API Design Principles

1. **Ergonomic**: Easy to use with minimal boilerplate
2. **Flexible**: Support customization without breaking changes
3. **Type-Safe**: Leverage Rust's type system for correctness
4. **Well-Documented**: Clear examples and comprehensive docs
5. **Backward-Compatible**: Semantic versioning with clear migration paths

## Example Usage Target

The library should enable this level of simplicity:

```rust
// Cargo.toml
[dependencies]
avinapi = { version = "0.1.0", features = ["axum", "jwt", "testing"] }

// main.rs
use avinapi::prelude::*;

#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUserRequest,
    responses((status = 200, body = ApiResponse<UserResponse>))
)]
async fn create_user(
    ValidatedJson(request): ValidatedJson<CreateUserRequest>,
) -> AppResult<Json<ApiResponse<UserResponse>>> {
    let user = User::create(request).await?;
    data!(user.to_response())
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/users", post(create_user))
        .layer(cors_middleware())
        .layer(request_id_middleware())
        .layer(rate_limit_middleware(100, Duration::from_secs(60)));
}
```

## Quality Requirements

1. **Test Coverage**: Minimum 90% test coverage
2. **Documentation**: All public APIs documented with examples
3. **Performance**: No significant overhead compared to direct implementation
4. **Memory Safety**: No unsafe code without thorough justification
5. **Compatibility**: Support stable Rust (MSRV policy)

## Publication Preparation

1. Complete README with usage examples and installation
2. Choose appropriate license (MIT or Apache-2.0 recommended)
3. Set up CI/CD pipeline for testing and publishing
4. Create comprehensive changelog
5. Prepare crates.io metadata and keywords

## Success Criteria

The library is successful when:
1. A new web API project can be bootstrapped in <30 lines of code
2. All patterns from rust-web-api-patterns.md are supported
3. Common middleware needs are addressed out-of-the-box
4. Testing utilities enable easy test writing
5. Documentation enables self-service adoption

Use the `rust-web-api-patterns.md` document as the authoritative reference for all implementation details. Every pattern and example in that document should be supported and enabled by this library.