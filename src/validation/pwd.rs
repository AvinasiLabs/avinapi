//! password validation module docs

use validator::ValidationError;

/// Validates password strength with configurable requirements.
///
/// Checks for:
/// - Minimum length (8 characters by default)
/// - At least one uppercase letter
/// - At least one lowercase letter
/// - At least one digit
/// - At least one special character (!@#$%^&*(),.?":{}|<>)
///
/// # Example
///
/// ```
/// use validator::Validate;
/// use avinapi::validation::validate_password_strength;
///
/// #[derive(Validate)]
/// struct RegisterRequest {
///     #[validate(custom(function = "validate_password_strength"))]
///     password: String,
/// }
/// ```
pub fn validate_password_strength(password: &str) -> Result<(), ValidationError> {
    if password.len() < 8 {
        return Err(ValidationError::new("password_too_short"));
    }

    if !password.chars().any(|c| c.is_ascii_uppercase()) {
        return Err(ValidationError::new("password_missing_uppercase"));
    }

    if !password.chars().any(|c| c.is_ascii_lowercase()) {
        return Err(ValidationError::new("password_missing_lowercase"));
    }

    if !password.chars().any(|c| c.is_ascii_digit()) {
        return Err(ValidationError::new("password_missing_digit"));
    }

    let special_chars = "!@#$%^&*(),.?\":{}|<>";
    if !password.chars().any(|c| special_chars.contains(c)) {
        return Err(ValidationError::new("password_missing_special"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_strength() {
        // Valid password
        assert!(validate_password_strength("Password123!").is_ok());

        // Too short
        assert!(validate_password_strength("Pass1!").is_err());

        // Missing uppercase
        assert!(validate_password_strength("password123!").is_err());

        // Missing lowercase
        assert!(validate_password_strength("PASSWORD123!").is_err());

        // Missing digit
        assert!(validate_password_strength("Password!").is_err());

        // Missing special character
        assert!(validate_password_strength("Password123").is_err());
    }
}
