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

/// Validates score event time fields
///
/// # Arguments
/// * `period` - Hockey period (1-5: regular periods 1-3, overtime 4-5)
/// * `time_minutes` - Optional minutes value (0-60)
/// * `time_seconds` - Optional seconds value (0-59)
///
/// # Returns
/// * `Ok(())` - If validation passes
/// * `Err(&'static str)` - Error message if validation fails
///
/// # Validation Rules
/// * Period must be between 1 and 5 (inclusive)
/// * Minutes must be between 0 and 60 (inclusive) if provided
/// * Seconds must be between 0 and 59 (inclusive) if provided
///
/// # Examples
/// ```
/// let result = validate_score_event_time(1, Some(15), Some(30));
/// assert!(result.is_ok());
///
/// let result = validate_score_event_time(6, Some(15), Some(30));
/// assert!(result.is_err());
///
/// let result = validate_score_event_time(1, Some(61), Some(30));
/// assert!(result.is_err());
/// ```
pub fn validate_score_event_time(
    period: i32,
    time_minutes: Option<i32>,
    time_seconds: Option<i32>,
) -> Result<(), &'static str> {
    if !(1..=5).contains(&period) {
        return Err("Period must be between 1 and 5");
    }

    if let Some(minutes) = time_minutes {
        if !(0..=60).contains(&minutes) {
            return Err("Minutes must be between 0 and 60");
        }
    }

    if let Some(seconds) = time_seconds {
        if !(0..=59).contains(&seconds) {
            return Err("Seconds must be between 0 and 59");
        }
    }

    Ok(())
}

/// Validates player height in centimeters
///
/// Ensures height is within reasonable human biological limits.
/// Based on professional hockey player data.
///
/// # Arguments
/// * `height` - Optional height in centimeters
///
/// # Returns
/// * `Ok(Some(i64))` - If height is valid
/// * `Ok(None)` - If height is None
/// * `Err(&'static str)` - Error message if validation fails
///
/// # Valid Range
/// * Minimum: 100 cm (3'3") - very short but possible
/// * Maximum: 250 cm (8'2") - very tall (Zdeno Chára is 206cm)
///
/// # Examples
/// ```
/// let result = validate_height_cm(Some(185));
/// assert!(result.is_ok());
///
/// let result = validate_height_cm(Some(-50));
/// assert!(result.is_err());
///
/// let result = validate_height_cm(None);
/// assert!(result.is_ok());
/// ```
pub fn validate_height_cm(height: Option<i64>) -> Result<Option<i64>, &'static str> {
    match height {
        None => Ok(None),
        Some(h) if h < 100 => Err("Height must be at least 100 cm (3'3\")"),
        Some(h) if h > 250 => Err("Height must be less than 250 cm (8'2\")"),
        Some(h) => Ok(Some(h)),
    }
}

/// Validates player weight in kilograms
///
/// Ensures weight is within reasonable human biological limits.
/// Based on professional hockey player data.
///
/// # Arguments
/// * `weight` - Optional weight in kilograms
///
/// # Returns
/// * `Ok(Some(i64))` - If weight is valid
/// * `Ok(None)` - If weight is None
/// * `Err(&'static str)` - Error message if validation fails
///
/// # Valid Range
/// * Minimum: 40 kg (88 lbs) - light but possible for youth
/// * Maximum: 200 kg (440 lbs) - very heavy
///
/// # Examples
/// ```
/// let result = validate_weight_kg(Some(85));
/// assert!(result.is_ok());
///
/// let result = validate_weight_kg(Some(-10));
/// assert!(result.is_err());
///
/// let result = validate_weight_kg(None);
/// assert!(result.is_ok());
/// ```
pub fn validate_weight_kg(weight: Option<i64>) -> Result<Option<i64>, &'static str> {
    match weight {
        None => Ok(None),
        Some(w) if w < 40 => Err("Weight must be at least 40 kg (88 lbs)"),
        Some(w) if w > 200 => Err("Weight must be less than 200 kg (440 lbs)"),
        Some(w) => Ok(Some(w)),
    }
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

    #[test]
    fn test_validate_score_event_time_success() {
        assert!(validate_score_event_time(1, Some(0), Some(0)).is_ok());
        assert!(validate_score_event_time(3, Some(15), Some(30)).is_ok());
        assert!(validate_score_event_time(5, Some(60), Some(59)).is_ok());
        assert!(validate_score_event_time(1, None, None).is_ok());
        assert!(validate_score_event_time(2, Some(20), None).is_ok());
        assert!(validate_score_event_time(4, None, Some(45)).is_ok());
    }

    #[test]
    fn test_validate_score_event_time_invalid_period() {
        assert_eq!(
            validate_score_event_time(0, Some(15), Some(30)).unwrap_err(),
            "Period must be between 1 and 5"
        );
        assert_eq!(
            validate_score_event_time(6, Some(15), Some(30)).unwrap_err(),
            "Period must be between 1 and 5"
        );
        assert_eq!(
            validate_score_event_time(-1, Some(15), Some(30)).unwrap_err(),
            "Period must be between 1 and 5"
        );
    }

    #[test]
    fn test_validate_score_event_time_invalid_minutes() {
        assert_eq!(
            validate_score_event_time(1, Some(-1), Some(30)).unwrap_err(),
            "Minutes must be between 0 and 60"
        );
        assert_eq!(
            validate_score_event_time(1, Some(61), Some(30)).unwrap_err(),
            "Minutes must be between 0 and 60"
        );
    }

    #[test]
    fn test_validate_score_event_time_invalid_seconds() {
        assert_eq!(
            validate_score_event_time(1, Some(15), Some(-1)).unwrap_err(),
            "Seconds must be between 0 and 59"
        );
        assert_eq!(
            validate_score_event_time(1, Some(15), Some(60)).unwrap_err(),
            "Seconds must be between 0 and 59"
        );
    }

    #[test]
    fn test_validate_height_cm_success() {
        assert_eq!(validate_height_cm(Some(185)).unwrap(), Some(185));
        assert_eq!(validate_height_cm(Some(100)).unwrap(), Some(100));
        assert_eq!(validate_height_cm(Some(250)).unwrap(), Some(250));
        assert_eq!(validate_height_cm(None).unwrap(), None);
    }

    #[test]
    fn test_validate_height_cm_too_small() {
        assert!(validate_height_cm(Some(99)).is_err());
        assert!(validate_height_cm(Some(0)).is_err());
        assert!(validate_height_cm(Some(-100)).is_err());
        assert_eq!(
            validate_height_cm(Some(99)).unwrap_err(),
            "Height must be at least 100 cm (3'3\")"
        );
    }

    #[test]
    fn test_validate_height_cm_too_large() {
        assert!(validate_height_cm(Some(251)).is_err());
        assert!(validate_height_cm(Some(500)).is_err());
        assert_eq!(
            validate_height_cm(Some(251)).unwrap_err(),
            "Height must be less than 250 cm (8'2\")"
        );
    }

    #[test]
    fn test_validate_weight_kg_success() {
        assert_eq!(validate_weight_kg(Some(85)).unwrap(), Some(85));
        assert_eq!(validate_weight_kg(Some(40)).unwrap(), Some(40));
        assert_eq!(validate_weight_kg(Some(200)).unwrap(), Some(200));
        assert_eq!(validate_weight_kg(None).unwrap(), None);
    }

    #[test]
    fn test_validate_weight_kg_too_small() {
        assert!(validate_weight_kg(Some(39)).is_err());
        assert!(validate_weight_kg(Some(0)).is_err());
        assert!(validate_weight_kg(Some(-50)).is_err());
        assert_eq!(
            validate_weight_kg(Some(39)).unwrap_err(),
            "Weight must be at least 40 kg (88 lbs)"
        );
    }

    #[test]
    fn test_validate_weight_kg_too_large() {
        assert!(validate_weight_kg(Some(201)).is_err());
        assert!(validate_weight_kg(Some(500)).is_err());
        assert_eq!(
            validate_weight_kg(Some(201)).unwrap_err(),
            "Weight must be less than 200 kg (440 lbs)"
        );
    }
}
