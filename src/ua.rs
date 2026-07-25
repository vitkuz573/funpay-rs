//! User-Agent rotation for request fingerprint diversity.
//!
//! Provides a pool of realistic browser User-Agent strings
//! and a function to randomly select one.

use rand::seq::SliceRandom;

const USER_AGENTS: &[&str] = &[
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:128.0) Gecko/20100101 Firefox/128.0",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.5 Safari/605.1.15",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36 Edg/125.0.0.0",
];

/// Returns a random User-Agent string from the built-in pool.
pub fn random_ua() -> &'static str {
    USER_AGENTS.choose(&mut rand::thread_rng()).unwrap_or(&USER_AGENTS[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_ua_returns_non_empty() {
        let ua = random_ua();
        assert!(!ua.is_empty());
        assert!(ua.contains("Mozilla"));
    }

    #[test]
    fn random_ua_returns_known_ua() {
        for _ in 0..100 {
            let ua = random_ua();
            assert!(USER_AGENTS.contains(&ua));
        }
    }
}
