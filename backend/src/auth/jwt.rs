use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, errors::Error as JwtError, Algorithm, DecodingKey, EncodingKey, Header,
    Validation,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use thiserror::Error;

/// Errors that can occur during JWT operations
#[derive(Debug, Error)]
pub enum JwtManagerError {
    #[error("Failed to read private key: {0}")]
    PrivateKeyRead(#[from] std::io::Error),

    #[error("Failed to encode token: {0}")]
    TokenEncode(#[from] JwtError),

    #[error("Invalid token")]
    InvalidToken,

    #[error("Token expired")]
    TokenExpired,
}

/// Claims contained in JWT tokens
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: String,
    /// Email address
    pub email: String,
    /// User's name
    pub name: Option<String>,
    /// Expiration time (Unix timestamp)
    pub exp: i64,
    /// Issued at (Unix timestamp)
    pub iat: i64,
    /// Token type: "access" or "refresh"
    pub token_type: String,
}

/// JWT Manager for creating and validating tokens
pub struct JwtManager {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    access_token_duration: Duration,
    refresh_token_duration: Duration,
}

impl JwtManager {
    /// Create a new JwtManager by reading RSA keys from disk
    ///
    /// # Arguments
    /// * `private_key_path` - Path to the RSA private key (PEM format)
    /// * `public_key_path` - Path to the RSA public key (PEM format)
    /// * `access_token_duration_minutes` - Duration in minutes for access tokens
    /// * `refresh_token_duration_days` - Duration in days for refresh tokens
    ///
    /// # Returns
    /// A Result containing the JwtManager or an error
    pub fn new<P: AsRef<Path>>(
        private_key_path: P,
        public_key_path: P,
        access_token_duration_minutes: i64,
        refresh_token_duration_days: i64,
    ) -> Result<Self, JwtManagerError> {
        // Read private key for signing
        let private_key_pem = fs::read(private_key_path)?;
        let encoding_key = EncodingKey::from_rsa_pem(&private_key_pem)?;

        // Read public key for verification
        let public_key_pem = fs::read(public_key_path)?;
        let decoding_key = DecodingKey::from_rsa_pem(&public_key_pem)?;

        Ok(Self {
            encoding_key,
            decoding_key,
            access_token_duration: Duration::minutes(access_token_duration_minutes),
            refresh_token_duration: Duration::days(refresh_token_duration_days),
        })
    }

    /// Generate an access token for a user
    ///
    /// Access tokens are short-lived (15 minutes) and used for API authentication
    ///
    /// # Arguments
    /// * `user_id` - The user's ID
    /// * `email` - The user's email address
    /// * `name` - The user's name (optional)
    ///
    /// # Returns
    /// A Result containing the JWT token string or an error
    pub fn generate_access_token(
        &self,
        user_id: i64,
        email: &str,
        name: Option<String>,
    ) -> Result<String, JwtManagerError> {
        let now = Utc::now();
        let exp = now + self.access_token_duration;

        let claims = Claims {
            sub: user_id.to_string(),
            email: email.to_string(),
            name,
            exp: exp.timestamp(),
            iat: now.timestamp(),
            token_type: "access".to_string(),
        };

        let header = Header::new(Algorithm::RS256);
        let token = encode(&header, &claims, &self.encoding_key)?;
        Ok(token)
    }

    /// Generate a refresh token for a user
    ///
    /// Refresh tokens are long-lived (7 days) and used to obtain new access tokens
    ///
    /// # Arguments
    /// * `user_id` - The user's ID
    /// * `email` - The user's email address
    /// * `name` - The user's name (optional)
    ///
    /// # Returns
    /// A Result containing the JWT token string or an error
    pub fn generate_refresh_token(
        &self,
        user_id: i64,
        email: &str,
        name: Option<String>,
    ) -> Result<String, JwtManagerError> {
        let now = Utc::now();
        let exp = now + self.refresh_token_duration;

        let claims = Claims {
            sub: user_id.to_string(),
            email: email.to_string(),
            name,
            exp: exp.timestamp(),
            iat: now.timestamp(),
            token_type: "refresh".to_string(),
        };

        let header = Header::new(Algorithm::RS256);
        let token = encode(&header, &claims, &self.encoding_key)?;
        Ok(token)
    }

    /// Validate and decode a JWT token
    ///
    /// # Arguments
    /// * `token` - The JWT token string to validate
    ///
    /// # Returns
    /// A Result containing the Claims or an error
    pub fn validate_token(&self, token: &str) -> Result<Claims, JwtManagerError> {
        let mut validation = Validation::new(Algorithm::RS256);
        validation.validate_exp = true;

        let token_data = decode::<Claims>(token, &self.decoding_key, &validation).map_err(|e| {
            match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => JwtManagerError::TokenExpired,
                _ => JwtManagerError::InvalidToken,
            }
        })?;

        Ok(token_data.claims)
    }

    /// Validate an access token
    ///
    /// This validates that the token is valid AND is an access token (not refresh)
    ///
    /// # Arguments
    /// * `token` - The JWT token string to validate
    ///
    /// # Returns
    /// A Result containing the Claims or an error
    pub fn validate_access_token(&self, token: &str) -> Result<Claims, JwtManagerError> {
        let claims = self.validate_token(token)?;

        if claims.token_type != "access" {
            return Err(JwtManagerError::InvalidToken);
        }

        Ok(claims)
    }

    /// Validate a refresh token
    ///
    /// This validates that the token is valid AND is a refresh token (not access)
    ///
    /// # Arguments
    /// * `token` - The JWT token string to validate
    ///
    /// # Returns
    /// A Result containing the Claims or an error
    pub fn validate_refresh_token(&self, token: &str) -> Result<Claims, JwtManagerError> {
        let claims = self.validate_token(token)?;

        if claims.token_type != "refresh" {
            return Err(JwtManagerError::InvalidToken);
        }

        Ok(claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration as StdDuration;

    fn create_test_jwt_manager() -> JwtManager {
        // Use the actual RSA keys from the project with default durations
        JwtManager::new("jwt_private.pem", "jwt_public.pem", 15, 7)
            .expect("Failed to create JwtManager")
    }

    #[test]
    fn test_generate_access_token() {
        let manager = create_test_jwt_manager();
        let token = manager
            .generate_access_token(1, "test@example.com", Some("Test User".to_string()))
            .expect("Failed to generate access token");

        // Token should be a non-empty string
        assert!(!token.is_empty());

        // Token should have 3 parts separated by dots (header.payload.signature)
        assert_eq!(token.matches('.').count(), 2);
    }

    #[test]
    fn test_generate_refresh_token() {
        let manager = create_test_jwt_manager();
        let token = manager
            .generate_refresh_token(1, "test@example.com", Some("Test User".to_string()))
            .expect("Failed to generate refresh token");

        // Token should be a non-empty string
        assert!(!token.is_empty());

        // Token should have 3 parts separated by dots
        assert_eq!(token.matches('.').count(), 2);
    }

    #[test]
    fn test_validate_access_token() {
        let manager = create_test_jwt_manager();
        let token = manager
            .generate_access_token(42, "user@example.com", Some("John Doe".to_string()))
            .expect("Failed to generate token");

        let claims = manager
            .validate_access_token(&token)
            .expect("Failed to validate token");

        assert_eq!(claims.sub, "42");
        assert_eq!(claims.email, "user@example.com");
        assert_eq!(claims.name, Some("John Doe".to_string()));
        assert_eq!(claims.token_type, "access");
    }

    #[test]
    fn test_validate_refresh_token() {
        let manager = create_test_jwt_manager();
        let token = manager
            .generate_refresh_token(42, "user@example.com", Some("John Doe".to_string()))
            .expect("Failed to generate token");

        let claims = manager
            .validate_refresh_token(&token)
            .expect("Failed to validate token");

        assert_eq!(claims.sub, "42");
        assert_eq!(claims.email, "user@example.com");
        assert_eq!(claims.name, Some("John Doe".to_string()));
        assert_eq!(claims.token_type, "refresh");
    }

    #[test]
    fn test_access_token_rejected_as_refresh() {
        let manager = create_test_jwt_manager();
        let token = manager
            .generate_access_token(1, "test@example.com", None)
            .expect("Failed to generate access token");

        let result = manager.validate_refresh_token(&token);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), JwtManagerError::InvalidToken));
    }

    #[test]
    fn test_refresh_token_rejected_as_access() {
        let manager = create_test_jwt_manager();
        let token = manager
            .generate_refresh_token(1, "test@example.com", None)
            .expect("Failed to generate refresh token");

        let result = manager.validate_access_token(&token);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), JwtManagerError::InvalidToken));
    }

    #[test]
    fn test_invalid_token() {
        let manager = create_test_jwt_manager();
        let result = manager.validate_token("invalid.token.here");

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), JwtManagerError::InvalidToken));
    }

    #[test]
    fn test_token_expiration() {
        // Create a manager with very short expiration for testing
        let manager = create_test_jwt_manager();

        // We can't easily test expiration without waiting, so we'll just verify
        // that the exp claim is in the future
        let token = manager
            .generate_access_token(1, "test@example.com", None)
            .expect("Failed to generate token");

        let claims = manager
            .validate_access_token(&token)
            .expect("Token should be valid");

        let now = Utc::now().timestamp();
        assert!(claims.exp > now, "Token expiration should be in the future");
        assert!(
            claims.iat <= now,
            "Token issued time should be in the past or now"
        );
    }

    #[test]
    fn test_tokens_are_different() {
        let manager = create_test_jwt_manager();

        let token1 = manager
            .generate_access_token(1, "test@example.com", None)
            .expect("Failed to generate token 1");

        // Sleep for 1 second to ensure different iat timestamps
        thread::sleep(StdDuration::from_secs(1));

        let token2 = manager
            .generate_access_token(1, "test@example.com", None)
            .expect("Failed to generate token 2");

        // Tokens should be different due to different iat times
        assert_ne!(token1, token2);
    }

    #[test]
    fn test_different_users_different_tokens() {
        let manager = create_test_jwt_manager();

        let token1 = manager
            .generate_access_token(1, "user1@example.com", Some("User One".to_string()))
            .expect("Failed to generate token 1");

        let token2 = manager
            .generate_access_token(2, "user2@example.com", Some("User Two".to_string()))
            .expect("Failed to generate token 2");

        assert_ne!(token1, token2);

        let claims1 = manager.validate_access_token(&token1).unwrap();
        let claims2 = manager.validate_access_token(&token2).unwrap();

        assert_eq!(claims1.sub, "1");
        assert_eq!(claims2.sub, "2");
    }

    #[test]
    fn test_custom_token_durations() {
        // Create a manager with custom durations
        let custom_access_minutes = 30_i64; // 30 minutes instead of default 15
        let custom_refresh_days = 14_i64; // 14 days instead of default 7

        let manager = JwtManager::new(
            "jwt_private.pem",
            "jwt_public.pem",
            custom_access_minutes,
            custom_refresh_days,
        )
        .expect("Failed to create JwtManager");

        // Generate access token
        let access_token = manager
            .generate_access_token(1, "test@example.com", None)
            .expect("Failed to generate access token");

        let access_claims = manager
            .validate_access_token(&access_token)
            .expect("Failed to validate access token");

        // Generate refresh token
        let refresh_token = manager
            .generate_refresh_token(1, "test@example.com", None)
            .expect("Failed to generate refresh token");

        let refresh_claims = manager
            .validate_refresh_token(&refresh_token)
            .expect("Failed to validate refresh token");

        let now = Utc::now().timestamp();

        // Verify access token expiration is approximately 30 minutes from now
        let access_duration = access_claims.exp - access_claims.iat;
        assert!(
            access_duration >= custom_access_minutes * 60 - 5
                && access_duration <= custom_access_minutes * 60 + 5,
            "Access token duration should be approximately {} seconds (30 minutes), got {}",
            custom_access_minutes * 60,
            access_duration
        );

        // Verify refresh token expiration is approximately 14 days from now
        let refresh_duration = refresh_claims.exp - refresh_claims.iat;
        let expected_refresh_seconds = custom_refresh_days * 24 * 60 * 60;
        assert!(
            refresh_duration >= expected_refresh_seconds - 5
                && refresh_duration <= expected_refresh_seconds + 5,
            "Refresh token duration should be approximately {} seconds (14 days), got {}",
            expected_refresh_seconds,
            refresh_duration
        );

        // Verify tokens are valid now but will expire in the future
        assert!(access_claims.exp > now);
        assert!(refresh_claims.exp > now);

        // Verify access token expires before refresh token
        assert!(access_claims.exp < refresh_claims.exp);
    }
}
