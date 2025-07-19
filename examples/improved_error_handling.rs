//! Example demonstrating the improved error handling and type aliases in avinapi.
//!
//! This example shows:
//! 1. Simplified error handling with AppError::IntoResponse and ? operator
//! 2. Type aliases (JsonResult<T>) for cleaner handler signatures
//! 3. Custom validation functions for business logic
//! 4. Automatic error conversion from AppError to ResponseCode
//! 5. Enhanced error logging with tracing integration

use avinapi::prelude::*;
use axum::{Router, extract::Path, routing::post};
use serde::{Deserialize, Serialize};

use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
struct CreateUserRequest {
    /// User's email address
    #[validate(email)]
    #[schema(example = "user@example.com")]
    email: String,

    /// Username (3-20 chars, alphanumeric + underscore, no leading/trailing underscores)
    #[validate(custom(function = "validate_username"))]
    #[schema(example = "john_doe")]
    username: String,

    /// Strong password with complexity requirements
    #[validate(custom(function = "validate_password_strength"))]
    #[schema(example = "SecurePass123!")]
    password: String,

    /// User's age
    #[validate(custom(function = "validate_age"))]
    #[schema(example = 25)]
    age: u32,

    /// Phone number in international format
    #[validate(custom(function = "validate_phone_number"))]
    #[schema(example = "+1-234-567-8900")]
    phone: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
struct UserResponse {
    id: u32,
    email: String,
    username: String,
    age: u32,
    phone: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
struct UserListResponse {
    users: Vec<UserResponse>,
    total: usize,
}

/// Create a new user with validation and error handling.
///
/// This handler demonstrates:
/// - JsonResult<T> type alias for clean return types
/// - Automatic validation with custom functions
/// - Simplified error handling with ? operator
/// - AppError automatically converts to proper HTTP responses

async fn create_user(
    ValidatedJson(request): ValidatedJson<CreateUserRequest>,
) -> JsonResult<UserResponse> {
    // Simulate business logic checks
    // In a real app, you'd check against the database

    // Simulate database operation that might fail
    let user_id = simulate_database_operation()?;

    let user = UserResponse {
        id: user_id,
        email: request.email,
        username: request.username,
        age: request.age,
        phone: request.phone,
    };

    // In a real app, you'd save to database here

    // Return success using the data! macro
    data!(user)
}

/// Get user by ID with error handling.
///
/// Demonstrates:
/// - Path parameter extraction
/// - NotFound error handling
/// - Automatic AppError to HTTP response conversion

async fn get_user(Path(id): Path<u32>) -> JsonResult<UserResponse> {
    // Simulate user lookup - in real app, query database
    if id == 0 {
        return Err(AppError::not_found(format!(
            "User with ID {} not found",
            id
        )));
    }

    let user = UserResponse {
        id,
        email: "example@test.com".to_string(),
        username: "example_user".to_string(),
        age: 25,
        phone: "+1-234-567-8900".to_string(),
    };

    data!(user)
}

/// List all users with potential database error.
///
/// Shows how database errors are automatically handled.
async fn list_users() -> JsonResult<UserListResponse> {
    // Simulate a database error that might occur
    simulate_potential_database_error()?;

    // Simulate user list - in real app, query database
    let users = vec![
        UserResponse {
            id: 1,
            email: "user1@example.com".to_string(),
            username: "user_one".to_string(),
            age: 25,
            phone: "+1-234-567-8900".to_string(),
        },
        UserResponse {
            id: 2,
            email: "user2@example.com".to_string(),
            username: "user_two".to_string(),
            age: 30,
            phone: "+1-234-567-8901".to_string(),
        },
    ];

    let response = UserListResponse {
        total: users.len(),
        users,
    };

    data!(response)
}

/// Simulate a database operation that might fail.
///
/// This demonstrates how external errors (like sqlx::Error) are automatically
/// converted to AppError through the From trait implementation.
fn simulate_database_operation() -> AppResult<u32> {
    // In a real app, this might be:
    // let result = sqlx::query!("INSERT INTO users ...")
    //     .execute(&pool)
    //     .await?; // sqlx::Error automatically converts to AppError
    //
    // For demo, we'll simulate success
    Ok(42)
}

/// Simulate potential database connectivity issues.
fn simulate_potential_database_error() -> AppResult<()> {
    // Uncomment to simulate a database error:
    // return Err(AppError::database("Connection pool exhausted"));

    Ok(())
}

/// Advanced error handling example showing error chaining.
async fn complex_operation() -> JsonResult<String> {
    // Chain of operations that might fail
    let step1_result = step_that_might_fail()?;
    let step2_result = another_step_that_might_fail(&step1_result)?;
    let final_result = final_processing_step(&step2_result)?;

    data!(final_result)
}

fn step_that_might_fail() -> AppResult<String> {
    // Simulate some processing
    Ok("step1_complete".to_string())
}

fn another_step_that_might_fail(input: &str) -> AppResult<String> {
    if input.is_empty() {
        return Err(AppError::validation("Input cannot be empty"));
    }
    Ok(format!("processed_{}", input))
}

fn final_processing_step(input: &str) -> AppResult<String> {
    // Simulate final processing
    Ok(format!("final_{}", input))
}

// Note: All validation functions (validate_username, validate_password_strength,
// validate_age, validate_phone_number) are now imported from avinapi::validation
// This demonstrates the built-in validation functions provided by the library.

#[tokio::main]
async fn main() {
    // Initialize tracing for error logging
    tracing_subscriber::fmt::init();

    // Build the router with improved error handling
    let _app = Router::<()>::new()
        .route("/users", post(create_user).get(list_users))
        .route("/users/:id", axum::routing::get(get_user))
        .route("/complex", axum::routing::get(complex_operation));

    println!("🚀 Server starting on http://localhost:3000");
    println!("📖 Key improvements demonstrated:");
    println!("   • JsonResult<T> type alias for clean handler signatures");
    println!("   • AppError with ? operator - no more error! macros");
    println!("   • Automatic error conversion via From traits");
    println!("   • Enhanced error logging with tracing integration");
    println!("   • Custom validation functions for business logic");
    println!("   • Structured error responses with detailed messages");

    println!("\n🧪 Try these requests:");
    println!("   POST /users - Create user with validation");
    println!("   GET /users/1 - Get user (demonstrates NotFound)");
    println!("   GET /users - List users");
    println!("   GET /complex - Complex operation with error chaining");

    // Note: In a real application, you'd run the server like this:
    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // axum::serve(listener, app).await.unwrap();

    // For this example, we just show the setup
    println!("\n✅ Example compiled successfully!");
    println!("💡 This demonstrates the new simplified error handling patterns.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_conversion() {
        let app_error = AppError::validation("Test validation error");
        let response_code = ResponseCode::from(app_error);
        assert_eq!(response_code, ResponseCode::ValidationError);
    }

    #[test]
    fn test_custom_validations() {
        // Test username validation
        assert!(validate_username("valid_user123").is_ok());
        assert!(validate_username("ab").is_err()); // too short
        assert!(validate_username("_invalid").is_err()); // starts with underscore

        // Test password validation
        assert!(validate_password_strength("ValidPass123").is_ok());
        assert!(validate_password_strength("weak").is_err()); // too weak

        // Test age validation
        assert!(validate_age(&25).is_ok());
        assert!(validate_age(&200).is_err()); // too high
    }

    #[tokio::test]
    async fn test_error_chain_logging() {
        // This test demonstrates that errors are properly logged
        // In debug mode, the full error chain is captured
        let error = AppError::database("Connection failed");
        let response_code = ResponseCode::from(&error);

        assert_eq!(response_code, ResponseCode::DatabaseError);
        assert!(error.to_string().contains("Database error"));
    }
}
