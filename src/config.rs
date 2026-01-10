use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub session_secret: String,
    pub environment: Environment,
    pub port: u16,
    pub db_max_connections: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Environment {
    Development,
    Production,
}

impl Config {
    pub fn from_env() -> Result<Self, anyhow::Error> {
        // Load .env file if it exists
        let _ = dotenvy::dotenv();
        Self::from_env_vars()
    }

    fn from_env_vars() -> Result<Self, anyhow::Error> {
        let database_url =
            env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:./hockey.db".to_string());

        // Determine environment first to apply appropriate validation
        let environment = match env::var("ENVIRONMENT")
            .unwrap_or_else(|_| "development".to_string())
            .to_lowercase()
            .as_str()
        {
            "production" => Environment::Production,
            _ => Environment::Development,
        };

        // Handle SESSION_SECRET based on environment
        let session_secret = match environment {
            Environment::Production => {
                // Required in production
                env::var("SESSION_SECRET").map_err(|_| {
                    anyhow::anyhow!(
                        "SESSION_SECRET environment variable is required in production\n\
                         \n\
                         Generate a secure secret with:\n\
                         \u{0020} openssl rand -hex 32\n\
                         \n\
                         Then set it in your environment:\n\
                         \u{0020} export SESSION_SECRET=<generated_secret>"
                    )
                })?
            }
            Environment::Development => {
                // Optional in development with warning
                env::var("SESSION_SECRET").unwrap_or_else(|_| {
                    tracing::warn!(
                        "SESSION_SECRET not set, using insecure default. \
                         Generate one with: openssl rand -hex 32"
                    );
                    "development-secret-key-DO-NOT-USE-IN-PRODUCTION".to_string()
                })
            }
        };

        // Validate minimum length for security
        if session_secret.len() < 32 {
            anyhow::bail!(
                "SESSION_SECRET must be at least 32 characters (current: {} characters)\n\
                 Generate a secure secret with: openssl rand -hex 32",
                session_secret.len()
            );
        }

        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()?;

        let db_max_connections = env::var("DB_MAX_CONNECTIONS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(5);

        Ok(Config {
            database_url,
            session_secret,
            environment,
            port,
            db_max_connections,
        })
    }

    #[allow(dead_code)]
    pub fn is_production(&self) -> bool {
        self.environment == Environment::Production
    }

    #[allow(dead_code)]
    pub fn is_development(&self) -> bool {
        self.environment == Environment::Development
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    // Helper to clear relevant environment variables
    fn clear_env() {
        env::remove_var("ENVIRONMENT");
        env::remove_var("SESSION_SECRET");
        env::remove_var("DATABASE_URL");
        env::remove_var("PORT");
        env::remove_var("DB_MAX_CONNECTIONS");
    }

    #[test]
    #[serial]
    fn test_production_requires_session_secret() {
        clear_env();
        env::set_var("ENVIRONMENT", "production");

        let result = Config::from_env_vars();
        assert!(result.is_err());
        let error = result.unwrap_err().to_string();
        assert!(error.contains("SESSION_SECRET"));
        assert!(error.contains("required in production"));
    }

    #[test]
    #[serial]
    fn test_session_secret_minimum_length_validation() {
        clear_env();
        env::set_var("ENVIRONMENT", "production");
        env::set_var("SESSION_SECRET", "tooshort");

        let result = Config::from_env_vars();
        assert!(result.is_err());
        let error = result.unwrap_err().to_string();
        assert!(error.contains("at least 32 characters"));
    }

    #[test]
    #[serial]
    fn test_development_allows_default_secret() {
        clear_env();
        env::set_var("ENVIRONMENT", "development");

        let result = Config::from_env_vars();
        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(config.environment, Environment::Development);
        assert_eq!(
            config.session_secret,
            "development-secret-key-DO-NOT-USE-IN-PRODUCTION"
        );
    }

    #[test]
    #[serial]
    fn test_production_with_valid_secret_succeeds() {
        clear_env();
        env::set_var("ENVIRONMENT", "production");
        env::set_var(
            "SESSION_SECRET",
            "a".repeat(32), // Exactly 32 characters
        );

        let result = Config::from_env_vars();
        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(config.environment, Environment::Production);
        assert_eq!(config.session_secret.len(), 32);
    }

    #[test]
    #[serial]
    fn test_development_with_explicit_secret() {
        clear_env();
        env::set_var("ENVIRONMENT", "development");
        env::set_var("SESSION_SECRET", "a".repeat(32));

        let result = Config::from_env_vars();
        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(config.environment, Environment::Development);
        assert_eq!(config.session_secret.len(), 32);
    }

    #[test]
    #[serial]
    fn test_environment_defaults_to_development() {
        clear_env();
        env::set_var("SESSION_SECRET", "a".repeat(32));

        let result = Config::from_env_vars();
        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(config.environment, Environment::Development);
    }

    #[test]
    #[serial]
    fn test_production_secret_64_chars_accepted() {
        clear_env();
        env::set_var("ENVIRONMENT", "production");
        env::set_var("SESSION_SECRET", "a".repeat(64));

        let result = Config::from_env_vars();
        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(config.session_secret.len(), 64);
    }

    #[test]
    #[serial]
    fn test_default_db_max_connections() {
        clear_env();
        env::set_var("ENVIRONMENT", "development");

        let result = Config::from_env_vars();
        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(config.db_max_connections, 5);
    }

    #[test]
    #[serial]
    fn test_custom_db_max_connections() {
        clear_env();
        env::set_var("ENVIRONMENT", "development");
        env::set_var("DB_MAX_CONNECTIONS", "10");

        let result = Config::from_env_vars();
        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(config.db_max_connections, 10);
    }

    #[test]
    #[serial]
    fn test_invalid_db_max_connections_uses_default() {
        clear_env();
        env::set_var("ENVIRONMENT", "development");
        env::set_var("DB_MAX_CONNECTIONS", "invalid");

        let result = Config::from_env_vars();
        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(config.db_max_connections, 5);
    }
}
