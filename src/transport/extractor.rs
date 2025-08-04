//! extractor module docs

use crate::transport::error::AppError;
use axum::{
    body::Bytes,
    extract::{FromRequest, FromRequestParts, Query, Request},
    http::request::Parts,
};
use serde::Serialize;
use serde::de::DeserializeOwned;
use validator::Validate;

/// Validated JSON extractor that automatically validates the input data
#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate + Serialize,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // Extract the request body as bytes
        let bytes = Bytes::from_request(req, state)
            .await
            .map_err(|_| AppError::Parsing("Failed to read request body".to_string()))?;

        // Parse JSON with detailed error handling
        let value: T = serde_json::from_slice(&bytes).map_err(|err| {
            let error_message = format!(
                "JSON parsing error at line {}, column {}: {}",
                err.line(),
                err.column(),
                err
            );
            AppError::Parsing(error_message)
        })?;

        value.validate().map_err(AppError::from)?;

        Ok(ValidatedJson(value))
    }
}

/// Validated Query extractor that automatically validates the input data
#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedQuery<T>(pub T);

impl<T, S> FromRequestParts<S> for ValidatedQuery<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Query(value) = Query::<T>::from_request_parts(parts, state)
            .await
            .map_err(|e| AppError::Parsing(format!("Invalid query parameters: {}", e)))?;

        value.validate().map_err(AppError::from)?;

        Ok(ValidatedQuery(value))
    }
}
