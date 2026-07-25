//! Retry and rate-limiting utilities.
//!
//! Provides exponential backoff retry policies and token-bucket rate limiting
//! to avoid overwhelming FunPay's servers.

use std::sync::Mutex;
use std::time::{Duration, Instant};

/// Exponential backoff retry policy.
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    pub max_retries: u32,
    pub base_delay_ms: u64,
    pub max_delay_ms: u64,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay_ms: 1000,
            max_delay_ms: 30000,
        }
    }
}

impl RetryPolicy {
    /// Calculate the delay for a given retry attempt (0-indexed).
    pub fn delay_for_attempt(&self, attempt: u32) -> Duration {
        let delay = self.base_delay_ms * 2u64.pow(attempt);
        Duration::from_millis(delay.min(self.max_delay_ms))
    }
}

/// Rate limiter configuration.
#[derive(Debug, Clone)]
pub struct RateLimiter {
    pub requests_per_second: f64,
    pub min_interval_ms: u64,
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self {
            requests_per_second: 2.0,
            min_interval_ms: 500,
        }
    }
}

impl RateLimiter {
    /// Get the minimum interval between requests.
    pub fn min_interval(&self) -> Duration {
        Duration::from_millis(self.min_interval_ms)
    }
}

/// Thread-safe rate limiter state that tracks last request time.
pub struct RateLimiterState {
    last_request: Mutex<Option<Instant>>,
    limiter: RateLimiter,
}

impl RateLimiterState {
    /// Create a new rate limiter state.
    pub fn new(limiter: RateLimiter) -> Self {
        Self {
            last_request: Mutex::new(None),
            limiter,
        }
    }

    /// Wait until the rate limit allows the next request.
    #[allow(clippy::await_holding_lock)]
    pub async fn wait(&self) {
        let mut last = self.last_request.lock().unwrap();
        let now = Instant::now();
        if let Some(prev) = *last {
            let elapsed = now.duration_since(prev);
            let min_interval = self.limiter.min_interval();
            if elapsed < min_interval {
                let wait_time = min_interval - elapsed;
                drop(last);
                tokio::time::sleep(wait_time).await;
                let mut last = self.last_request.lock().unwrap();
                *last = Some(Instant::now());
                return;
            }
        }
        *last = Some(now);
    }
}

/// Check if an HTTP status code is retryable (429 or 5xx).
pub fn is_retryable_status(status: u16) -> bool {
    status == 429 || (500..600).contains(&status)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retry_delay_exponential() {
        let policy = RetryPolicy::default();
        assert_eq!(policy.delay_for_attempt(0), Duration::from_millis(1000));
        assert_eq!(policy.delay_for_attempt(1), Duration::from_millis(2000));
        assert_eq!(policy.delay_for_attempt(2), Duration::from_millis(4000));
        assert_eq!(policy.delay_for_attempt(3), Duration::from_millis(8000));
    }

    #[test]
    fn test_retry_delay_capped() {
        let policy = RetryPolicy { max_delay_ms: 5000, ..Default::default() };
        assert_eq!(policy.delay_for_attempt(10), Duration::from_millis(5000));
    }

    #[test]
    fn test_is_retryable_status() {
        assert!(is_retryable_status(429));
        assert!(is_retryable_status(500));
        assert!(is_retryable_status(503));
        assert!(!is_retryable_status(200));
        assert!(!is_retryable_status(404));
        assert!(!is_retryable_status(400));
    }

    #[test]
    fn test_rate_limiter_default() {
        let limiter = RateLimiter::default();
        assert_eq!(limiter.min_interval(), Duration::from_millis(500));
    }
}
