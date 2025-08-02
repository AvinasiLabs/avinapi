//! Validated query parameter extractor for automatic validation.
//!
//! This module provides a query extractor that automatically validates
//! the extracted data using the `validator` crate.

use axum::{
    extract::{FromRequestParts, Query},
    http::request::Parts,
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::error::AppError;

/// Validated query parameter extractor that automatically validates the input data.
///
/// This extractor works similarly to axum's `Query` extractor but additionally
/// performs validation using the `validator` crate after deserializing the data.
///
/// # Example
///
/// ```
/// use avinapi::extractor::ValidatedQuery;
/// use avinapi::response::ApiResponse;
/// use serde::{Deserialize, Serialize};
/// use validator::Validate;
///
/// #[derive(Debug, Deserialize, Validate)]
/// struct SearchParams {
///     #[validate(length(min = 1, max = 100))]
///     query: String,
///
///     #[validate(range(min = 1, max = 100))]
///     limit: Option<u32>,
/// }
///
/// async fn search(
///     ValidatedQuery(params): ValidatedQuery<SearchParams>
/// ) -> ApiResponse<Vec<String>> {
///     // params is guaranteed to be valid here
///     let results = vec![format!("Results for: {}", params.query)];
///     ApiResponse::success(results)
/// }
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedQuery<T>(pub T);

impl<T, S> FromRequestParts<S> for ValidatedQuery<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // First extract the query parameters using axum's Query extractor
        let Query(value) = Query::<T>::from_request_parts(parts, state)
            .await
            .map_err(|e| AppError::bad_request(format!("Invalid query parameters: {}", e)))?;

        // Then validate the extracted data
        value
            .validate()
            .map_err(|e| AppError::validation(format!("Query validation failed: {}", e)))?;

        Ok(ValidatedQuery(value))
    }
}

impl<T> std::ops::Deref for ValidatedQuery<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for ValidatedQuery<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        Router,
        body::Body,
        http::{Request, StatusCode},
        routing::get,
    };
    use serde::Deserialize;
    use tower::ServiceExt;

    #[derive(Debug, Deserialize, Validate)]
    struct TestQuery {
        #[validate(range(min = 1, max = 100))]
        page: u32,

        #[validate(length(min = 1, max = 50))]
        search: Option<String>,
    }

    async fn test_handler(ValidatedQuery(query): ValidatedQuery<TestQuery>) -> String {
        format!("Page: {}, Search: {:?}", query.page, query.search)
    }

    #[tokio::test]
    async fn test_validated_query_success() {
        let app = Router::new().route("/test", get(test_handler));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/test?page=10&search=hello")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(&body[..], b"Page: 10, Search: Some(\"hello\")");
    }

    #[tokio::test]
    async fn test_validated_query_validation_error() {
        let app = Router::new().route("/test", get(test_handler));

        // Test with invalid page (0 is less than minimum of 1)
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/test?page=0&search=hello")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK); // Our errors return 200
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["code"], "VALIDATION_ERROR");
        assert!(json["message"].as_str().unwrap().contains("validation"));
    }

    #[tokio::test]
    async fn test_validated_query_parse_error() {
        let app = Router::new().route("/test", get(test_handler));

        // Test with invalid query parameter type
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/test?page=not_a_number")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK); // Our errors return 200
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["code"], "VALIDATION_ERROR");
        assert!(
            json["message"]
                .as_str()
                .unwrap()
                .contains("Invalid query parameters")
        );
    }
}
