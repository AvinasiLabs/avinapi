# Sponge - Development Guide

## Project Structure

### Package Configuration
- **server**: Main API server binary
- **docs**: OpenAPI documentation generator binary
- **lib**: Core business logic library

### Core Design Philosophy

This project follows a **unified response pattern** with **separation of concerns** and **error handling consistency**. The key principle is that **all business logic errors are handled uniformly**, while **HTTP is just a transport layer**.

## Core Design Principles

### 1. Unified HTTP Response Pattern

**Principle**: All API endpoints return HTTP 200 OK. Business status is indicated in the `code` field of the response body.

**Why**: This provides consistent error handling across all clients and simplifies debugging.

```rust
pub struct ApiResponse<T> {
    pub code: ResponseCode,           // Business status (SUCCESS, VALIDATION_ERROR, etc.)
    pub data: Option<T>,             // Present only on success
    pub message: Option<String>,     // Present only on error with details
}
```

**Application**: Every handler returns `AppResult<Json<ApiResponse<T>>>`, where business errors are converted to appropriate response codes.

### 2. Transport Layer Response Macros

**Principle**: Simplify response creation with macros that eliminate boilerplate.

```rust
// Instead of: Ok(Json(ApiResponse { code: ResponseCode::Success, data: Some(user), message: None }))
data!(user)

// Instead of: Ok(Json(ApiResponse { code: ResponseCode::Success, data: None, message: None }))
empty!()
```

**Application**: Use `data!()` for successful responses with data, `empty!()` for successful responses without data.

### 3. Automatic Validation with ValidatedJson

**Principle**: Validation happens automatically at the transport layer using `ValidatedJson<T>` extractor.

```rust
pub async fn handler(
    ValidatedJson(request): ValidatedJson<CreateUserRequest>,
) -> AppResult<Json<ApiResponse<UserResponse>>> {
    // Request is automatically validated here
    // Business logic can focus on business rules
}
```

**Application**: Define validation rules on structs using `#[validate(...)]` attributes. ValidatedJson automatically validates and returns appropriate error responses.

### 4. Fat Model Pattern

**Principle**: Business logic belongs in the model layer, not in handlers.

**Why**: This ensures business rules are reusable and testable independently of the web layer.

```rust
impl User {
    pub async fn create_with_verification(
        db: &PgPool,
        email: &str,
        name: &str,
        password: &str,
    ) -> AppResult<Self> {
        // All business logic for user creation
        // Including validation, password hashing, etc.
    }
}

// Handler just orchestrates
pub async fn register(
    State(state): State<AppState>,
    ValidatedJson(request): ValidatedJson<RegisterRequest>,
) -> AppResult<Json<ApiResponse<AuthResponse>>> {
    let user = User::create_with_verification(
        &state.db,
        &request.email,
        &request.name,
        &request.password,
    ).await?;
    
    data!(user.to_auth_response())
}
```

### 5. Unified Error Handling

**Principle**: All errors are handled through a single `AppError` enum with generic categories. Specific error details are carried in the error message.

**Why**: This provides consistent error responses while allowing detailed error information.

```rust
pub enum AppError {
    Validation(String),      // Maps to VALIDATION_ERROR
    Authentication(String),  // Maps to AUTHENTICATION_ERROR  
    Authorization(String),   // Maps to AUTHORIZATION_ERROR
    NotFound(String),       // Maps to NOT_FOUND_ERROR
    // ... other generic categories
}
```

**Application**: 
- Use generic error types (e.g., `AppError::Validation`)
- Put specific details in the message (e.g., "Energy values cannot be empty")
- Never create specific error codes like `ENERGY_EMPTY` - use `VALIDATION_ERROR` with descriptive message

### 6. Schema Documentation Standards

**Principle**: Use `#[schema(...)]` attributes to provide clear API documentation.

```rust
#[derive(Validate, ToSchema)]
pub struct CreateUserRequest {
    #[schema(example = "user@example.com")]
    #[validate(email)]
    pub email: String,
    
    #[schema(example = "John Doe")]
    #[validate(length(min = 1, max = 100))]
    pub name: String,
}
```

**Application**: Always provide realistic examples and descriptions that help API consumers understand expected input.

## Implementation Patterns

### Handler Function Signature
```rust
pub async fn handler_name(
    State(state): State<AppState>,
    auth_user: AuthUser,  // Optional: for protected endpoints
    ValidatedJson(request): ValidatedJson<RequestType>,
) -> AppResult<Json<ApiResponse<ResponseType>>> {
    // Business logic delegation to model layer
    // Error handling through ? operator
    // Response creation using macros
}
```

### Model CRUD Pattern
```rust
impl ModelName {
    pub async fn create(db: &PgPool, data: CreateData) -> AppResult<Self> { /* ... */ }
    pub async fn find_by_id(db: &PgPool, id: Uuid) -> AppResult<Option<Self>> { /* ... */ }
    pub async fn find_by_user(db: &PgPool, user_id: Uuid) -> AppResult<Vec<Self>> { /* ... */ }
    pub async fn update(db: &PgPool, id: Uuid, data: UpdateData) -> AppResult<Self> { /* ... */ }
    pub fn to_response(self) -> ResponseType { /* ... */ }
}
```

### Request/Response Structure Pattern
- **Request structs**: Include validation attributes
- **Response structs**: Include schema documentation
- **Model structs**: Include database mappings
- **Query structs**: Resource-specific query parameters in handler modules
- **Conversion**: Use `From` traits for model-to-response conversion

## Error Handling Pattern

### Unified Error Design
1. **Generic Categories**: Use broad error types (Validation, Authentication, etc.)
2. **Specific Messages**: Include detailed error information in the message field
3. **Consistent Mapping**: 1:1 mapping from `AppError` to `ResponseCode`
4. **Automatic Conversion**: `ValidatedJson` automatically handles validation errors

### Best Practices
- Never create specific error codes for business rules
- Use descriptive error messages that help users understand what went wrong
- Rely on the type system and validation attributes for input validation
- Handle business logic errors in the model layer, not in handlers

## Query Parameter Design Pattern

### Composition Over Duplication
- **Generic parameters**: Common pagination (`PaginationQuery`) in `transport/query.rs`
- **Resource-specific parameters**: Compose with generic parameters using `#[serde(flatten)]`

```rust
// In transport/query.rs - Generic pagination
#[derive(Debug, Deserialize, IntoParams, ToSchema, Validate)]
pub struct PaginationQuery {
    /// Page number (default: 1)
    #[schema(example = 1)]
    pub page: Option<u32>,
    
    /// Items per page (default: 20, max: 100)
    #[schema(example = 20)]
    pub per_page: Option<i64>,
}

// In handlers/alert.rs - Direct composition using multiple Query extractors
#[utoipa::path(
    get,
    path = "/alert",
    params(DateTimeRangeQuery, PaginationQuery),
    // ...
)]
pub async fn get_user_alerts(
    State(state): State<AppState>,
    Query(datetime_query): Query<DateTimeRangeQuery>,
    Query(pagination_query): Query<PaginationQuery>,
    auth_user: AuthUser,
) -> AppResult<Json<ApiResponse<PaginatedData<AlertResponse>>>> {
    // Use datetime_query.start, datetime_query.end
    // Use pagination_query.get_page(), pagination_query.get_per_page()
}
```

**Application**: Direct composition through multiple Query extractors provides maximum flexibility. Resources can combine any query parameter types without creating intermediate structs, eliminating duplication entirely.

## Tracing and Instrumentation Pattern</text>


### Instrument Macro Usage
```rust
#[instrument(skip(state), fields(email = request.email))]
pub async fn login(
    State(state): State<AppState>,
    ValidatedJson(request): ValidatedJson<LoginRequest>,
) -> AppResult<Json<ApiResponse<AuthResponse>>> {
    // Automatic tracing with structured fields
}
```

**Application**: Use `#[instrument]` on all public functions. Skip sensitive data, include relevant identifiers.

### Security Considerations
- Never log passwords, tokens, or sensitive data
- Use `skip` parameter for sensitive fields
- Include user IDs and request IDs for correlation

## OpenAPI Documentation Configuration

### Modern utoipa 5.0.0 Pattern
```rust
#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::user::create_user,
        crate::handlers::user::get_user,
    ),
    components(
        schemas(CreateUserRequest, UserResponse, ApiResponse<UserResponse>)
    )
)]
pub struct ApiDoc;
```

### Handler Documentation
```rust
#[utoipa::path(
    post,
    path = "/users",
    tags = ["User"],
    summary = "Create a new user",
    description = "Creates a new user account with the provided information",
    request_body = CreateUserRequest,
    responses(
        (status = 200, description = "User created successfully", body = ApiResponse<UserResponse>)
    )
)]
pub async fn create_user(/* ... */) -> AppResult<Json<ApiResponse<UserResponse>>> {
    // Implementation
}
```

## Key Takeaways

1. **Consistency Over Convenience**: Follow established patterns even if they seem verbose
2. **Separation of Concerns**: Keep business logic in models, validation in structs, transport in handlers
3. **Error Handling**: Use generic error types with specific messages
4. **Documentation**: Provide clear examples and descriptions for all public APIs
5. **Tracing**: Instrument all functions for observability
6. **Validation**: Let the framework handle input validation automatically

This design ensures maintainability, testability, and consistency across the entire application.