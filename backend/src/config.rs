/// The configuration parameters for the application.
///
/// These can either be passed on the command line, or pulled from environment variables.
/// The latter is preferred as environment variables are one of the recommended ways to
/// get configuration from Kubernetes Secrets in deployment.
///
/// This is a pretty simple configuration struct as far as backend APIs go. You could imagine
/// a bunch of other parameters going here, like API keys for external services
/// or flags enabling or disabling certain features or test modes of the API.
///
/// For development convenience, these can also be read from a `.env` file in the working
/// directory where the application is started.
///
/// See `.env.sample` in the repository root for details.
#[derive(clap::Parser, Clone)]
pub struct Config {
    /// The connection URL for the Postgres database this application should use.
    #[clap(long, env)]
    pub database_url: String,

    /// The HMAC signing and verification key used for login tokens (JWTs).
    ///
    /// There is no required structure or format to this key as it's just fed into a hash function.
    /// In practice, it should be a long, random string that would be infeasible to brute-force.
    #[clap(long, env)]
    pub hmac_key: String,

    /// The host address to bind the server to.
    #[clap(long, env, default_value = "0.0.0.0")]
    pub host: String,

    /// The port to bind the server to.
    #[clap(long, env, default_value = "8080")]
    pub port: u16,

    /// CORS allowed origins (comma-separated list or "*" for all)
    /// Example: "http://localhost:3000,http://localhost:4000,https://yourdomain.com"
    #[clap(long, env, default_value = "*")]
    pub cors_origins: String,

    /// CORS allowed methods (comma-separated list)
    #[clap(long, env, default_value = "GET,POST,PUT,DELETE,OPTIONS")]
    pub cors_methods: String,

    /// CORS allowed headers (comma-separated list or "*" for all)
    #[clap(long, env, default_value = "*")]
    pub cors_headers: String,

    /// Path to the RSA private key for JWT signing (PEM format)
    #[clap(long, env, default_value = "jwt_private.pem")]
    pub jwt_private_key_path: String,

    /// Path to the RSA public key for JWT verification (PEM format)
    #[clap(long, env, default_value = "jwt_public.pem")]
    pub jwt_public_key_path: String,

    /// Access token duration in minutes
    #[clap(long, env, default_value = "15")]
    pub jwt_access_token_duration_minutes: i64,

    /// Refresh token duration in days
    #[clap(long, env, default_value = "7")]
    pub jwt_refresh_token_duration_days: i64,

    /// Environment mode (development, production)
    #[clap(long, env, default_value = "development")]
    pub environment: String,
}

impl Config {
    /// Create a test configuration with sensible defaults
    /// This is useful for integration tests
    pub fn test_config() -> Self {
        Self {
            database_url: "sqlite::memory:".to_string(),
            hmac_key: "test-hmac-key".to_string(),
            host: "0.0.0.0".to_string(),
            port: 8080,
            cors_origins: "*".to_string(),
            cors_methods: "GET,POST,PUT,DELETE,OPTIONS".to_string(),
            cors_headers: "*".to_string(),
            jwt_private_key_path: "jwt_private.pem".to_string(),
            jwt_public_key_path: "jwt_public.pem".to_string(),
            jwt_access_token_duration_minutes: 15,
            jwt_refresh_token_duration_days: 7,
            environment: "development".to_string(),
        }
    }

    /// Check if running in production mode
    pub fn is_production(&self) -> bool {
        self.environment.to_lowercase() == "production"
    }

    /// Validate CORS configuration
    /// Returns an error if wildcard CORS is used in production mode
    pub fn validate_cors(&self) -> Result<(), String> {
        if self.is_production() {
            if self.cors_origins.contains('*') {
                return Err(
                    "Wildcard CORS origins (*) are not allowed in production mode. \
                     Please specify explicit origins (e.g., 'https://yourdomain.com,https://www.yourdomain.com')"
                        .to_string(),
                );
            }

            if self.cors_headers == "*" {
                return Err(
                    "Wildcard CORS headers (*) are not allowed in production mode. \
                     Please specify explicit headers (e.g., 'content-type,authorization,x-requested-with')"
                        .to_string(),
                );
            }

            if self.cors_methods == "*" {
                return Err(
                    "Wildcard CORS methods (*) are not allowed in production mode. \
                     Please specify explicit methods (e.g., 'GET,POST,PUT,DELETE,OPTIONS')"
                        .to_string(),
                );
            }

            // Validate that origins are properly formatted URLs
            for origin in self.cors_origins.split(',') {
                let trimmed = origin.trim();
                if !trimmed.starts_with("http://") && !trimmed.starts_with("https://") {
                    return Err(format!(
                        "Invalid CORS origin '{}'. Origins must start with 'http://' or 'https://'",
                        trimmed
                    ));
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_production() {
        let mut config = Config::test_config();

        config.environment = "development".to_string();
        assert!(!config.is_production());

        config.environment = "production".to_string();
        assert!(config.is_production());

        config.environment = "PRODUCTION".to_string();
        assert!(config.is_production());
    }

    #[test]
    fn test_cors_validation_development() {
        let mut config = Config::test_config();
        config.environment = "development".to_string();

        // Wildcards are allowed in development
        config.cors_origins = "*".to_string();
        config.cors_headers = "*".to_string();
        config.cors_methods = "*".to_string();
        assert!(config.validate_cors().is_ok());
    }

    #[test]
    fn test_cors_validation_production_wildcard_origins() {
        let mut config = Config::test_config();
        config.environment = "production".to_string();
        config.cors_origins = "*".to_string();

        let result = config.validate_cors();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Wildcard CORS origins"));
    }

    #[test]
    fn test_cors_validation_production_wildcard_headers() {
        let mut config = Config::test_config();
        config.environment = "production".to_string();
        config.cors_origins = "https://example.com".to_string();
        config.cors_headers = "*".to_string();

        let result = config.validate_cors();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Wildcard CORS headers"));
    }

    #[test]
    fn test_cors_validation_production_wildcard_methods() {
        let mut config = Config::test_config();
        config.environment = "production".to_string();
        config.cors_origins = "https://example.com".to_string();
        config.cors_headers = "content-type,authorization".to_string();
        config.cors_methods = "*".to_string();

        let result = config.validate_cors();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Wildcard CORS methods"));
    }

    #[test]
    fn test_cors_validation_production_invalid_origin_format() {
        let mut config = Config::test_config();
        config.environment = "production".to_string();
        config.cors_origins = "example.com".to_string();
        config.cors_headers = "content-type,authorization".to_string();
        config.cors_methods = "GET,POST,PUT,DELETE,OPTIONS".to_string();

        let result = config.validate_cors();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid CORS origin"));
    }

    #[test]
    fn test_cors_validation_production_valid_config() {
        let mut config = Config::test_config();
        config.environment = "production".to_string();
        config.cors_origins = "https://example.com,https://www.example.com".to_string();
        config.cors_headers = "content-type,authorization,x-requested-with".to_string();
        config.cors_methods = "GET,POST,PUT,DELETE,OPTIONS".to_string();

        let result = config.validate_cors();
        assert!(result.is_ok());
    }

    #[test]
    fn test_cors_validation_production_mixed_origins() {
        let mut config = Config::test_config();
        config.environment = "production".to_string();
        config.cors_origins = "https://example.com,*".to_string();
        config.cors_headers = "content-type,authorization".to_string();
        config.cors_methods = "GET,POST".to_string();

        let result = config.validate_cors();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Wildcard CORS origins"));
    }

    #[test]
    fn test_cors_validation_production_http_origin() {
        let mut config = Config::test_config();
        config.environment = "production".to_string();
        config.cors_origins = "http://localhost:3000".to_string();
        config.cors_headers = "content-type".to_string();
        config.cors_methods = "GET,POST".to_string();

        // http:// is allowed (for testing environments with production mode)
        let result = config.validate_cors();
        assert!(result.is_ok());
    }
}
