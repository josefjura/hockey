use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub session_secret: String,
    pub environment: Environment,
    pub port: u16,
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

        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "sqlite:./hockey.db".to_string());

        let session_secret = env::var("SESSION_SECRET")
            .unwrap_or_else(|_| {
                tracing::warn!("SESSION_SECRET not set, using default (not secure for production)");
                "development-secret-key-change-in-production".to_string()
            });

        let environment = match env::var("ENVIRONMENT")
            .unwrap_or_else(|_| "development".to_string())
            .to_lowercase()
            .as_str()
        {
            "production" => Environment::Production,
            _ => Environment::Development,
        };

        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()?;

        Ok(Config {
            database_url,
            session_secret,
            environment,
            port,
        })
    }

    pub fn is_production(&self) -> bool {
        self.environment == Environment::Production
    }

    pub fn is_development(&self) -> bool {
        self.environment == Environment::Development
    }
}
