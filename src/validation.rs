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

/// Validates event statistics values (goals and assists)
///
/// # Arguments
/// * `goals` - Number of goals
/// * `assists` - Number of assists
///
/// # Returns
/// * `Ok(())` - If validation passes
/// * `Err(&'static str)` - Error message if validation fails
///
/// # Validation Rules
/// * Goals and assists must be non-negative (>= 0)
/// * Goals and assists must not exceed 10,000 (reasonable maximum)
///
/// # Examples
/// ```
/// let result = validate_event_stats(5, 10);
/// assert!(result.is_ok());
///
/// let result = validate_event_stats(-1, 5);
/// assert!(result.is_err());
///
/// let result = validate_event_stats(15000, 5);
/// assert!(result.is_err());
/// ```
pub fn validate_event_stats(goals: i32, assists: i32) -> Result<(), &'static str> {
    if goals < 0 || assists < 0 {
        return Err("Goals and assists cannot be negative");
    }

    if goals > 10000 || assists > 10000 {
        return Err("Value exceeds maximum allowed (10,000)");
    }

    Ok(())
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

    #[test]
    fn test_validate_event_stats_success() {
        assert!(validate_event_stats(0, 0).is_ok());
        assert!(validate_event_stats(5, 10).is_ok());
        assert!(validate_event_stats(100, 200).is_ok());
        assert!(validate_event_stats(10000, 10000).is_ok());
    }

    #[test]
    fn test_validate_event_stats_negative() {
        assert_eq!(
            validate_event_stats(-1, 5).unwrap_err(),
            "Goals and assists cannot be negative"
        );
        assert_eq!(
            validate_event_stats(5, -1).unwrap_err(),
            "Goals and assists cannot be negative"
        );
        assert_eq!(
            validate_event_stats(-1, -1).unwrap_err(),
            "Goals and assists cannot be negative"
        );
    }

    #[test]
    fn test_validate_event_stats_exceeds_maximum() {
        assert_eq!(
            validate_event_stats(10001, 5).unwrap_err(),
            "Value exceeds maximum allowed (10,000)"
        );
        assert_eq!(
            validate_event_stats(5, 10001).unwrap_err(),
            "Value exceeds maximum allowed (10,000)"
        );
        assert_eq!(
            validate_event_stats(15000, 15000).unwrap_err(),
            "Value exceeds maximum allowed (10,000)"
        );
    }
}
