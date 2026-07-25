use std::sync::Mutex;
use std::time::{Duration, Instant};

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
    pub fn delay_for_attempt(&self, attempt: u32) -> Duration {
        let delay = self.base_delay_ms * 2u64.pow(attempt);
        Duration::from_millis(delay.min(self.max_delay_ms))
    }
}

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
    pub fn min_interval(&self) -> Duration {
        Duration::from_millis(self.min_interval_ms)
    }
}

pub struct RateLimiterState {
    last_request: Mutex<Option<Instant>>,
    limiter: RateLimiter,
}

impl RateLimiterState {
    pub fn new(limiter: RateLimiter) -> Self {
        Self {
            last_request: Mutex::new(None),
            limiter,
        }
    }

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

pub fn is_retryable_status(status: u16) -> bool {
    status == 429 || (status >= 500 && status < 600)
}
