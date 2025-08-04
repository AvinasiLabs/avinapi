//! Custom validation functions for common business scenarios.
//!
//! This module provides validation functions that are commonly needed in business
//! applications but are not available in the validator crate. These functions
//! follow the validator crate's function signature pattern.

use validator::ValidationError;

/// Validates username format for web applications.
///
/// Requirements:
/// - 3-20 characters long
/// - Only alphanumeric characters and underscores
/// - Cannot start or end with underscore
/// - Cannot have consecutive underscores
///
/// # Example
///
/// ```
/// use validator::Validate;
/// use avinapi::validation::validate_username;
///
/// #[derive(Validate)]
/// struct UserProfile {
///     #[validate(custom(function = "validate_username"))]
///     username: String,
/// }
/// ```
pub fn validate_username(username: &str) -> Result<(), ValidationError> {
    if username.len() < 3 {
        return Err(ValidationError::new("username_too_short"));
    }

    if username.len() > 20 {
        return Err(ValidationError::new("username_too_long"));
    }

    if !username.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(ValidationError::new("username_invalid_characters"));
    }

    if username.starts_with('_') || username.ends_with('_') {
        return Err(ValidationError::new(
            "username_invalid_underscore_placement",
        ));
    }

    if username.contains("__") {
        return Err(ValidationError::new("username_consecutive_underscores"));
    }

    Ok(())
}

/// Validates phone number in various international formats.
///
/// Accepts formats like:
/// - +1234567890
/// - +1 234 567 8900
/// - +1-234-567-8900
/// - +1 (234) 567-8900
///
/// # Example
///
/// ```
/// use validator::Validate;
/// use avinapi::validation::validate_phone_number;
///
/// #[derive(Validate)]
/// struct ContactInfo {
///     #[validate(custom(function = "validate_phone_number"))]
///     phone: String,
/// }
/// ```
pub fn validate_phone_number(phone: &str) -> Result<(), ValidationError> {
    // Remove all non-digit characters except +
    let cleaned: String = phone
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == '+')
        .collect();

    if !cleaned.starts_with('+') {
        return Err(ValidationError::new("phone_missing_country_code"));
    }

    let digits = &cleaned[1..]; // Remove the +

    if digits.len() < 10 || digits.len() > 15 {
        return Err(ValidationError::new("phone_invalid_length"));
    }

    if !digits.chars().all(|c| c.is_ascii_digit()) {
        return Err(ValidationError::new("phone_invalid_format"));
    }

    Ok(())
}

/// Validates URL slug format for SEO-friendly URLs.
///
/// Requirements:
/// - 1-100 characters long
/// - Only lowercase letters, numbers, and hyphens
/// - Cannot start or end with hyphen
/// - Cannot have consecutive hyphens
///
/// # Example
///
/// ```
/// use validator::Validate;
/// use avinapi::validation::validate_url_slug;
///
/// #[derive(Validate)]
/// struct BlogPost {
///     #[validate(custom(function = "validate_url_slug"))]
///     slug: String,
/// }
/// ```
pub fn validate_url_slug(slug: &str) -> Result<(), ValidationError> {
    if slug.is_empty() {
        return Err(ValidationError::new("slug_empty"));
    }

    if slug.len() > 100 {
        return Err(ValidationError::new("slug_too_long"));
    }

    if !slug
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    {
        return Err(ValidationError::new("slug_invalid_characters"));
    }

    if slug.starts_with('-') || slug.ends_with('-') {
        return Err(ValidationError::new("slug_invalid_hyphen_placement"));
    }

    if slug.contains("--") {
        return Err(ValidationError::new("slug_consecutive_hyphens"));
    }

    Ok(())
}

/// Validates hex color code format.
///
/// Accepts formats like:
/// - #FF0000 (6 digits)
/// - #F00 (3 digits)
/// - Both with and without # prefix
///
/// # Example
///
/// ```
/// use validator::Validate;
/// use avinapi::validation::validate_hex_color;
///
/// #[derive(Validate)]
/// struct ThemeSettings {
///     #[validate(custom(function = "validate_hex_color"))]
///     primary_color: String,
/// }
/// ```
pub fn validate_hex_color(color: &str) -> Result<(), ValidationError> {
    let color = if color.starts_with('#') {
        &color[1..]
    } else {
        color
    };

    if color.len() != 3 && color.len() != 6 {
        return Err(ValidationError::new("color_invalid_length"));
    }

    if !color.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(ValidationError::new("color_invalid_format"));
    }

    Ok(())
}

/// Validates age within reasonable human ranges.
///
/// Accepts ages between 0 and 150 years old.
///
/// # Example
///
/// ```
/// use validator::Validate;
/// use avinapi::validation::validate_age;
///
/// #[derive(Validate)]
/// struct UserProfile {
///     #[validate(custom(function = "validate_age"))]
///     age: u32,
/// }
/// ```
pub fn validate_age(age: u32) -> Result<(), ValidationError> {
    if age > 150 {
        return Err(ValidationError::new("age_too_high"));
    }

    Ok(())
}

/// Validates file extension against allowed list.
///
/// This is a factory function that returns a validator for specific file extensions.
///
/// # Example
///
/// ```
/// use validator::{Validate, ValidationError};
/// use avinapi::validation::validate_file_extension;
///
/// fn validate_image_extension(filename: &str) -> Result<(), ValidationError> {
///     validate_file_extension(filename, &["jpg", "jpeg", "png", "gif", "webp"])
/// }
///
/// #[derive(Validate)]
/// struct UploadRequest {
///     #[validate(custom(function = "validate_image_extension"))]
///     filename: String,
/// }
/// ```
pub fn validate_file_extension(
    filename: &str,
    allowed_extensions: &[&str],
) -> Result<(), ValidationError> {
    if let Some(extension) = filename.split('.').last() {
        let extension = extension.to_lowercase();
        if allowed_extensions.contains(&extension.as_str()) {
            Ok(())
        } else {
            Err(ValidationError::new("file_extension_not_allowed"))
        }
    } else {
        Err(ValidationError::new("file_no_extension"))
    }
}

/// Validates postal/ZIP code format (basic validation).
///
/// Accepts various formats:
/// - 5 digits (US ZIP)
/// - 5 digits + 4 digits with hyphen (US ZIP+4)
/// - 6 characters alternating letter-digit (Canadian postal code)
/// - Other international formats with 3-10 alphanumeric characters
///
/// # Example
///
/// ```
/// use validator::Validate;
/// use avinapi::validation::validate_postal_code;
///
/// #[derive(Validate)]
/// struct Address {
///     #[validate(custom(function = "validate_postal_code"))]
///     postal_code: String,
/// }
/// ```
pub fn validate_postal_code(postal_code: &str) -> Result<(), ValidationError> {
    let cleaned: String = postal_code
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect();

    if cleaned.len() < 3 || cleaned.len() > 10 {
        return Err(ValidationError::new("postal_code_invalid_length"));
    }

    // US ZIP code format
    if cleaned.len() == 5 && cleaned.chars().all(|c| c.is_ascii_digit()) {
        return Ok(());
    }

    // US ZIP+4 format (remove hyphen for this check)
    let zip_plus_4: String = postal_code.chars().filter(|c| c.is_ascii_digit()).collect();
    if zip_plus_4.len() == 9 && postal_code.contains('-') {
        return Ok(());
    }

    // Canadian postal code format (alternating letter-digit)
    if cleaned.len() == 6 {
        let chars: Vec<char> = cleaned.chars().collect();
        if chars[0].is_ascii_alphabetic()
            && chars[1].is_ascii_digit()
            && chars[2].is_ascii_alphabetic()
            && chars[3].is_ascii_digit()
            && chars[4].is_ascii_alphabetic()
            && chars[5].is_ascii_digit()
        {
            return Ok(());
        }
    }

    // Generic international format (3-10 alphanumeric characters)
    if cleaned.chars().all(|c| c.is_alphanumeric()) {
        Ok(())
    } else {
        Err(ValidationError::new("postal_code_invalid_format"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_username() {
        // Valid usernames
        assert!(validate_username("john_doe").is_ok());
        assert!(validate_username("user123").is_ok());
        assert!(validate_username("test_user_123").is_ok());

        // Invalid usernames
        assert!(validate_username("ab").is_err()); // Too short
        assert!(validate_username("a".repeat(21).as_str()).is_err()); // Too long
        assert!(validate_username("_john").is_err()); // Starts with underscore
        assert!(validate_username("john_").is_err()); // Ends with underscore
        assert!(validate_username("john__doe").is_err()); // Consecutive underscores
        assert!(validate_username("john-doe").is_err()); // Invalid character
    }

    #[test]
    fn test_phone_number() {
        // Valid phone numbers
        assert!(validate_phone_number("+1234567890").is_ok());
        assert!(validate_phone_number("+1 234 567 8900").is_ok());
        assert!(validate_phone_number("+1-234-567-8900").is_ok());
        assert!(validate_phone_number("+1 (234) 567-8900").is_ok());

        // Invalid phone numbers
        assert!(validate_phone_number("1234567890").is_err()); // No country code
        assert!(validate_phone_number("+123456789").is_err()); // Too short
        assert!(validate_phone_number("+1234567890123456").is_err()); // Too long
    }

    #[test]
    fn test_url_slug() {
        // Valid slugs
        assert!(validate_url_slug("hello-world").is_ok());
        assert!(validate_url_slug("test123").is_ok());
        assert!(validate_url_slug("my-awesome-blog-post").is_ok());

        // Invalid slugs
        assert!(validate_url_slug("").is_err()); // Empty
        assert!(validate_url_slug("-hello").is_err()); // Starts with hyphen
        assert!(validate_url_slug("hello-").is_err()); // Ends with hyphen
        assert!(validate_url_slug("hello--world").is_err()); // Consecutive hyphens
        assert!(validate_url_slug("Hello-World").is_err()); // Uppercase
        assert!(validate_url_slug("hello_world").is_err()); // Underscore
    }

    #[test]
    fn test_hex_color() {
        // Valid colors
        assert!(validate_hex_color("#FF0000").is_ok());
        assert!(validate_hex_color("#F00").is_ok());
        assert!(validate_hex_color("FF0000").is_ok());
        assert!(validate_hex_color("F00").is_ok());

        // Invalid colors
        assert!(validate_hex_color("#FF00").is_err()); // Wrong length
        assert!(validate_hex_color("#GG0000").is_err()); // Invalid hex
        assert!(validate_hex_color("#FF00000").is_err()); // Too long
    }

    #[test]
    fn test_age() {
        // Valid ages
        assert!(validate_age(0).is_ok());
        assert!(validate_age(25).is_ok());
        assert!(validate_age(150).is_ok());

        // Invalid ages
        assert!(validate_age(151).is_err()); // Too high
    }

    #[test]
    fn test_file_extension() {
        let image_extensions = &["jpg", "jpeg", "png", "gif"];

        // Valid extensions
        assert!(validate_file_extension("photo.jpg", image_extensions).is_ok());
        assert!(validate_file_extension("image.PNG", image_extensions).is_ok()); // Case insensitive

        // Invalid extensions
        assert!(validate_file_extension("document.pdf", image_extensions).is_err());
        assert!(validate_file_extension("file", image_extensions).is_err()); // No extension
    }

    #[test]
    fn test_postal_code() {
        // Valid postal codes
        assert!(validate_postal_code("12345").is_ok()); // US ZIP
        assert!(validate_postal_code("12345-6789").is_ok()); // US ZIP+4
        assert!(validate_postal_code("K1A0A6").is_ok()); // Canadian
        assert!(validate_postal_code("SW1A1AA").is_ok()); // UK format

        // Invalid postal codes
        assert!(validate_postal_code("12").is_err()); // Too short
        assert!(validate_postal_code("12345678901").is_err()); // Too long
    }
}
