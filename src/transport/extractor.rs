//! extractor module docs

use crate::transport::error::AppError;
use axum::{
    body::Bytes,
    extract::{FromRequest, FromRequestParts, Query, Request},
    http::request::Parts,
};
use axum_extra::extract::Multipart;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::fmt::Debug;
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

/// Validated Form extractor for multipart/form-data that automatically validates the input data
///
/// This extractor provides detailed error messages including field names and actual values
/// when validation fails, making debugging easier for form submissions.
#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedForm<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedForm<T>
where
    T: DeserializeOwned + Validate + Debug,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // Extract multipart form data
        let mut multipart = Multipart::from_request(req, state)
            .await
            .map_err(|e| AppError::Parsing(format!("Failed to parse multipart form: {}", e)))?;

        // Collect form fields into a JSON value for deserialization
        let mut form_data = serde_json::Map::new();

        while let Some(field) = multipart
            .next_field()
            .await
            .map_err(|e| AppError::Parsing(format!("Failed to read form field: {}", e)))?
        {
            let field_name = field
                .name()
                .ok_or_else(|| AppError::Parsing("Form field missing name".to_string()))?
                .to_string();

            // Handle file fields differently if needed
            if field.file_name().is_some() {
                // For file fields, we might want to store metadata or handle specially
                // For now, skip file fields in validation
                continue;
            }

            let field_value = field.text().await.map_err(|e| {
                AppError::Parsing(format!("Failed to read field '{}': {}", field_name, e))
            })?;

            // Store field value in JSON map
            form_data.insert(field_name, serde_json::Value::String(field_value));
        }

        // Convert map to JSON value
        let json_value = serde_json::Value::Object(form_data);

        // Deserialize JSON value into target type
        let value: T = serde_json::from_value(json_value.clone())
            .map_err(|e| AppError::Parsing(format!("Failed to parse form data: {}", e)))?;

        // Validate with enhanced error messages
        value.validate().map_err(|errors| {
            // Build detailed error message with field names and actual values
            let mut error_messages = Vec::new();

            for (field_name, field_errors) in errors.field_errors() {
                for error in field_errors {
                    // Get the actual value from the form data
                    let actual_value = json_value
                        .get(field_name.as_ref())
                        .and_then(|v| v.as_str())
                        .unwrap_or("<unknown>");

                    let message = if let Some(ref custom_msg) = error.message {
                        format!(
                            "{}: {} (received: '{}')",
                            field_name, custom_msg, actual_value
                        )
                    } else {
                        format!(
                            "{}: validation failed for code '{}' (received: '{}')",
                            field_name, error.code, actual_value
                        )
                    };
                    error_messages.push(message);
                }
            }

            // Handle nested errors if any
            for (field_name, error_kind) in errors.errors() {
                use validator::ValidationErrorsKind;
                match error_kind {
                    ValidationErrorsKind::Struct(nested_errors) => {
                        for (nested_field, nested_field_errors) in nested_errors.field_errors() {
                            for error in nested_field_errors {
                                let full_field = format!("{}.{}", field_name, nested_field);
                                let message = if let Some(ref custom_msg) = error.message {
                                    format!("{}: {}", full_field, custom_msg)
                                } else {
                                    format!("{}: validation failed", full_field)
                                };
                                error_messages.push(message);
                            }
                        }
                    }
                    ValidationErrorsKind::List(list_errors) => {
                        for (index, nested_errors) in list_errors {
                            for (nested_field, nested_field_errors) in nested_errors.field_errors()
                            {
                                for error in nested_field_errors {
                                    let full_field =
                                        format!("{}[{}].{}", field_name, index, nested_field);
                                    let message = if let Some(ref custom_msg) = error.message {
                                        format!("{}: {}", full_field, custom_msg)
                                    } else {
                                        format!("{}: validation failed", full_field)
                                    };
                                    error_messages.push(message);
                                }
                            }
                        }
                    }
                    ValidationErrorsKind::Field(field_errors) => {
                        for error in field_errors {
                            let actual_value = json_value
                                .get(field_name.as_ref())
                                .and_then(|v| v.as_str())
                                .unwrap_or("<unknown>");

                            let message = if let Some(ref custom_msg) = error.message {
                                format!(
                                    "{}: {} (received: '{}')",
                                    field_name, custom_msg, actual_value
                                )
                            } else {
                                format!(
                                    "{}: validation failed (received: '{}')",
                                    field_name, actual_value
                                )
                            };
                            error_messages.push(message);
                        }
                    }
                }
            }

            let combined_message = if error_messages.is_empty() {
                "Validation failed".to_string()
            } else {
                error_messages.join(", ")
            };

            AppError::Validation(combined_message)
        })?;

        Ok(ValidatedForm(value))
    }
}

/// File data from multipart form
#[derive(Debug, Clone)]
pub struct FileData {
    /// The original filename from the upload
    pub filename: Option<String>,
    /// The MIME content type of the file
    pub content_type: Option<String>,
    /// The raw bytes of the file content
    pub bytes: Vec<u8>,
}

/// Validated Multipart Form extractor for multipart/form-data with file handling
///
/// This extractor handles both form fields and file uploads, providing:
/// - Validated form data (non-file fields)
/// - File data mapped by field name
/// - Detailed validation error messages including field values
#[derive(Debug)]
pub struct ValidatedMultipartForm<T> {
    /// The validated form data (non-file fields)
    pub form: T,
    /// File data mapped by field name
    pub files: HashMap<String, FileData>,
}

impl<T, S> FromRequest<S> for ValidatedMultipartForm<T>
where
    T: DeserializeOwned + Validate + Debug,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // Extract multipart form data
        let mut multipart = Multipart::from_request(req, state)
            .await
            .map_err(|e| AppError::Parsing(format!("Failed to parse multipart form: {}", e)))?;

        // Collect form fields and files separately
        let mut form_data = serde_json::Map::new();
        let mut files = HashMap::new();

        while let Some(field) = multipart
            .next_field()
            .await
            .map_err(|e| AppError::Parsing(format!("Failed to read form field: {}", e)))?
        {
            let field_name = field
                .name()
                .ok_or_else(|| AppError::Parsing("Form field missing name".to_string()))?
                .to_string();

            // Check if this is a file field
            if let Some(filename) = field.file_name() {
                // Handle file field
                let filename = filename.to_string();
                let content_type = field.content_type().map(|ct| ct.to_string());
                let bytes = field.bytes().await.map_err(|e| {
                    AppError::Parsing(format!("Failed to read file '{}': {}", field_name, e))
                })?;

                files.insert(
                    field_name.clone(),
                    FileData {
                        filename: Some(filename),
                        content_type,
                        bytes: bytes.to_vec(),
                    },
                );
            } else {
                // Handle regular form field
                let field_value = field.text().await.map_err(|e| {
                    AppError::Parsing(format!("Failed to read field '{}': {}", field_name, e))
                })?;

                // Store field value in JSON map
                form_data.insert(field_name, serde_json::Value::String(field_value));
            }
        }

        // Convert map to JSON value
        let json_value = serde_json::Value::Object(form_data);

        // Deserialize JSON value into target type
        let form: T = serde_json::from_value(json_value.clone())
            .map_err(|e| AppError::Parsing(format!("Failed to parse form data: {}", e)))?;

        // Validate with enhanced error messages
        form.validate().map_err(|errors| {
            // Build detailed error message with field names and actual values
            let mut error_messages = Vec::new();

            for (field_name, field_errors) in errors.field_errors() {
                for error in field_errors {
                    // Get the actual value from the form data
                    let actual_value = json_value
                        .get(field_name.as_ref())
                        .and_then(|v| v.as_str())
                        .unwrap_or("<unknown>");

                    let message = if let Some(ref custom_msg) = error.message {
                        format!(
                            "{}: {} (received: '{}')",
                            field_name, custom_msg, actual_value
                        )
                    } else {
                        format!(
                            "{}: validation failed for code '{}' (received: '{}')",
                            field_name, error.code, actual_value
                        )
                    };
                    error_messages.push(message);
                }
            }

            // Handle nested errors if any
            for (field_name, error_kind) in errors.errors() {
                use validator::ValidationErrorsKind;
                match error_kind {
                    ValidationErrorsKind::Struct(nested_errors) => {
                        for (nested_field, nested_field_errors) in nested_errors.field_errors() {
                            for error in nested_field_errors {
                                let full_field = format!("{}.{}", field_name, nested_field);
                                let message = if let Some(ref custom_msg) = error.message {
                                    format!("{}: {}", full_field, custom_msg)
                                } else {
                                    format!("{}: validation failed", full_field)
                                };
                                error_messages.push(message);
                            }
                        }
                    }
                    ValidationErrorsKind::List(list_errors) => {
                        for (index, nested_errors) in list_errors {
                            for (nested_field, nested_field_errors) in nested_errors.field_errors()
                            {
                                for error in nested_field_errors {
                                    let full_field =
                                        format!("{}[{}].{}", field_name, index, nested_field);
                                    let message = if let Some(ref custom_msg) = error.message {
                                        format!("{}: {}", full_field, custom_msg)
                                    } else {
                                        format!("{}: validation failed", full_field)
                                    };
                                    error_messages.push(message);
                                }
                            }
                        }
                    }
                    ValidationErrorsKind::Field(field_errors) => {
                        for error in field_errors {
                            let actual_value = json_value
                                .get(field_name.as_ref())
                                .and_then(|v| v.as_str())
                                .unwrap_or("<unknown>");

                            let message = if let Some(ref custom_msg) = error.message {
                                format!(
                                    "{}: {} (received: '{}')",
                                    field_name, custom_msg, actual_value
                                )
                            } else {
                                format!(
                                    "{}: validation failed (received: '{}')",
                                    field_name, actual_value
                                )
                            };
                            error_messages.push(message);
                        }
                    }
                }
            }

            let combined_message = if error_messages.is_empty() {
                "Validation failed".to_string()
            } else {
                error_messages.join(", ")
            };

            AppError::Validation(combined_message)
        })?;

        Ok(ValidatedMultipartForm { form, files })
    }
}
