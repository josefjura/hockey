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
        }
    }
}
