//! ValidatedJson extractor for automatic JSON validation in web handlers.
//!
//! This module provides the `ValidatedJson<T>` extractor that combines JSON parsing
//! with automatic validation using the validator crate. It follows the unified
//! response pattern by converting validation errors to appropriate API responses.

use crate::error::AppError;
use axum::{Json, body::Body, extract::FromRequest, http::Request};
use serde::de::DeserializeOwned;
use std::future::Future;

use validator::Validate;

/// JSON extractor with automatic validation.
///
/// This extractor combines JSON parsing with validation, automatically handling
/// both JSON parsing errors and validation errors according to the unified
/// response pattern. It provides a seamless way to ensure request data is both
/// well-formed and valid before reaching handler logic.
///
/// # Type Requirements
///
/// The wrapped type `T` must implement:
/// - `DeserializeOwned` - for JSON parsing
/// - `Validate` - for automatic validation
///
/// # Error Handling
///
/// This extractor handles two types of errors:
///
/// 1. **JSON Parsing Errors**: Malformed JSON, incorrect types, etc.
///    - Converted to `AppError::Validation` with descriptive messages
///    - Results in `VALIDATION_ERROR` response code
///
/// 2. **Validation Errors**: Failed validation rules from validator crate
///    - Field-level errors collected and formatted
///    - Converted to `AppError::Validation` with field details
///    - Results in `VALIDATION_ERROR` response code
///
/// # Examples
///
/// Basic usage in a handler:
///
/// ```rust,ignore
/// use avinapi::{ValidatedJson, AppResult, ApiResponse, data};
/// use axum::Json;
/// use serde::{Deserialize, Serialize};
/// use validator::Validate;
/// use utoipa::ToSchema;
///
/// #[derive(Deserialize, Validate, ToSchema)]
/// struct CreateUserRequest {
///     #[validate(email)]
///     email: String,
///
///     #[validate(length(min = 1, max = 100))]
///     name: String,
///
///     #[validate(length(min = 8))]
///     password: String,
/// }
///
/// #[derive(Serialize, ToSchema)]
/// struct UserResponse {
///     id: u32,
///     email: String,
///     name: String,
/// }
///
/// async fn create_user(
///     ValidatedJson(request): ValidatedJson<CreateUserRequest>,
/// ) -> AppResult<Json<ApiResponse<UserResponse>>> {
///     // At this point, request is guaranteed to be:
///     // 1. Valid JSON that deserializes to CreateUserRequest
///     // 2. Passed all validation rules (valid email, name length, etc.)
///
///     let user = UserResponse {
///         id: 123,
///         email: request.email,
///         name: request.name,
///     };
///
///     data!(user)
/// }
/// ```
#[derive(Debug)]
pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate + Send,
    S: Send + Sync,
{
    type Rejection = AppError;

    fn from_request(
        req: Request<Body>,
        state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        Box::pin(async move {
            // First, extract JSON using axum's Json extractor
            let Json(data) = Json::<T>::from_request(req, state)
                .await
                .map_err(|rejection| {
                    // Convert axum's JSON rejection to our AppError
                    AppError::validation(format!("JSON parsing failed: {}", rejection))
                })?;

            // Then validate the parsed data
            data.validate().map_err(AppError::from)?;

            Ok(ValidatedJson(data))
        })
    }
}

// Implement Deref for convenient access to the inner data
impl<T> std::ops::Deref for ValidatedJson<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Implement DerefMut for mutable access to the inner data
impl<T> std::ops::DerefMut for ValidatedJson<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Method, Request},
    };
    use serde::{Deserialize, Serialize};
    use validator::Validate;

    #[derive(Debug, Deserialize, Serialize, Validate, PartialEq, Clone)]
    struct TestData {
        #[validate(email)]
        email: String,
        #[validate(length(min = 1, max = 50))]
        name: String,
        #[validate(range(min = 18, max = 120))]
        age: u32,
    }

    #[tokio::test]
    async fn test_valid_json_and_validation() {
        let json_body = r#"{"email":"test@example.com","name":"John","age":25}"#;
        let request = Request::builder()
            .method(Method::POST)
            .header("content-type", "application/json")
            .body(Body::from(json_body))
            .unwrap();

        let result = ValidatedJson::<TestData>::from_request(request, &()).await;
        assert!(result.is_ok());

        let ValidatedJson(data) = result.unwrap();
        assert_eq!(data.email, "test@example.com");
        assert_eq!(data.name, "John");
        assert_eq!(data.age, 25);
    }

    #[tokio::test]
    async fn test_invalid_json_syntax() {
        let json_body = r#"{"email":"test@example.com","name":"John""#; // Missing closing brace
        let request = Request::builder()
            .method(Method::POST)
            .header("content-type", "application/json")
            .body(Body::from(json_body))
            .unwrap();

        let result = ValidatedJson::<TestData>::from_request(request, &()).await;
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert!(matches!(error, AppError::Validation(_)));
        assert!(error.to_string().contains("JSON parsing failed"));
    }

    #[tokio::test]
    async fn test_validation_failure() {
        let json_body = r#"{"email":"invalid-email","name":"","age":15}"#; // Invalid email, empty name, age too low
        let request = Request::builder()
            .method(Method::POST)
            .header("content-type", "application/json")
            .body(Body::from(json_body))
            .unwrap();

        let result = ValidatedJson::<TestData>::from_request(request, &()).await;
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert!(matches!(error, AppError::Validation(_)));

        let error_message = error.to_string();
        // Should contain validation errors for multiple fields
        assert!(error_message.contains("Validation error"));
    }

    #[tokio::test]
    async fn test_missing_content_type() {
        let json_body = r#"{"email":"test@example.com","name":"John","age":25}"#;
        let request = Request::builder()
            .method(Method::POST)
            // Missing content-type header
            .body(Body::from(json_body))
            .unwrap();

        let result = ValidatedJson::<TestData>::from_request(request, &()).await;
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert!(matches!(error, AppError::Validation(_)));
    }

    #[test]
    fn test_deref() {
        let test_data = TestData {
            email: "test@example.com".to_string(),
            name: "John".to_string(),
            age: 25,
        };
        let validated = ValidatedJson(test_data);

        // Test Deref
        assert_eq!(validated.email, "test@example.com");
        assert_eq!(validated.name, "John");
        assert_eq!(validated.age, 25);
    }

    #[test]
    fn test_direct_access() {
        let test_data = TestData {
            email: "test@example.com".to_string(),
            name: "John".to_string(),
            age: 25,
        };
        let validated = ValidatedJson(test_data);

        // Test direct access to inner value
        let extracted = validated.0;
        assert_eq!(extracted.email, "test@example.com");
        assert_eq!(extracted.name, "John");
        assert_eq!(extracted.age, 25);
    }

    #[derive(Debug, Deserialize, Validate)]
    struct MinimalTest {
        #[validate(length(min = 1))]
        value: String,
    }

    #[tokio::test]
    async fn test_minimal_validation_success() {
        let json_body = r#"{"value":"test"}"#;
        let request = Request::builder()
            .method(Method::POST)
            .header("content-type", "application/json")
            .body(Body::from(json_body))
            .unwrap();

        let result = ValidatedJson::<MinimalTest>::from_request(request, &()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_minimal_validation_failure() {
        let json_body = r#"{"value":""}"#; // Empty string should fail length validation
        let request = Request::builder()
            .method(Method::POST)
            .header("content-type", "application/json")
            .body(Body::from(json_body))
            .unwrap();

        let result = ValidatedJson::<MinimalTest>::from_request(request, &()).await;
        assert!(result.is_err());
    }
}
