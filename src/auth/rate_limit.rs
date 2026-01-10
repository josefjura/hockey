use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{Html, IntoResponse, Response},
};
use governor::{
    clock::{Clock, DefaultClock},
    state::{InMemoryState, NotKeyed},
    Quota, RateLimiter,
};
use std::num::NonZeroU32;
use std::sync::Arc;

/// Rate limiter for login endpoint
/// Prevents brute force attacks by limiting login attempts
#[derive(Clone)]
pub struct LoginRateLimiter {
    limiter: Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>,
}

impl LoginRateLimiter {
    /// Create a new rate limiter
    /// Allows 5 attempts per minute with burst of 5
    pub fn new() -> Self {
        let quota =
            Quota::per_minute(NonZeroU32::new(5).unwrap()).allow_burst(NonZeroU32::new(5).unwrap());

        Self {
            limiter: Arc::new(RateLimiter::direct(quota)),
        }
    }

    /// Check if the request should be rate limited
    /// Returns None if allowed, Some(retry_after_secs) if rate limited
    pub fn check(&self) -> Option<u64> {
        match self.limiter.check() {
            Ok(_) => None,
            Err(not_until) => {
                let wait_time = not_until.wait_time_from(DefaultClock::default().now());
                Some(wait_time.as_secs() + 1) // Add 1 to round up
            }
        }
    }
}

impl Default for LoginRateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

/// Middleware function for rate limiting login attempts
/// Returns 429 Too Many Requests if rate limit exceeded
pub async fn rate_limit_login(
    State(limiter): State<LoginRateLimiter>,
    request: Request,
    next: Next,
) -> Response {
    if let Some(retry_after) = limiter.check() {
        let error_html = format!(
            r#"<div class="p-4 bg-red-100 border border-red-400 text-red-700 rounded">
                <p><strong>Too many login attempts.</strong></p>
                <p>Please try again in {} seconds.</p>
            </div>"#,
            retry_after
        );

        return (StatusCode::TOO_MANY_REQUESTS, Html(error_html)).into_response();
    }

    next.run(request).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limiter_allows_initial_requests() {
        let limiter = LoginRateLimiter::new();

        // First 5 requests should be allowed
        for _ in 0..5 {
            assert!(limiter.check().is_none(), "Request should be allowed");
        }
    }

    #[test]
    fn test_rate_limiter_blocks_after_limit() {
        let limiter = LoginRateLimiter::new();

        // Exhaust the quota
        for _ in 0..5 {
            limiter.check();
        }

        // Next request should be blocked
        let result = limiter.check();
        assert!(result.is_some(), "Request should be rate limited");
        assert!(result.unwrap() > 0, "Should have a retry-after time");
    }
}
