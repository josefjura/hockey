/// Common validation functions used across the application
/// Validates a name field (non-empty, max 255 characters)
///
/// # Arguments
/// * `name` - The name string to validate
///
/// # Returns
/// * `Ok(String)` - The trimmed, validated name
/// * `Err(&'static str)` - Error message if validation fails
///
/// # Examples
/// ```
/// let result = validate_name("  John Doe  ");
/// assert_eq!(result.unwrap(), "John Doe");
///
/// let result = validate_name("");
/// assert!(result.is_err());
/// ```
pub fn validate_name(name: &str) -> Result<String, &'static str> {
    let trimmed = name.trim();

    if trimmed.is_empty() {
        return Err("Name cannot be empty");
    }

    if trimmed.len() > 255 {
        return Err("Name cannot exceed 255 characters");
    }

    Ok(trimmed.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_name_success() {
        assert_eq!(validate_name("John Doe").unwrap(), "John Doe");
        assert_eq!(validate_name("  Spaces  ").unwrap(), "Spaces");
    }

    #[test]
    fn test_validate_name_empty() {
        assert!(validate_name("").is_err());
        assert!(validate_name("   ").is_err());
    }

    #[test]
    fn test_validate_name_too_long() {
        let long_name = "a".repeat(256);
        assert!(validate_name(&long_name).is_err());
    }
}
