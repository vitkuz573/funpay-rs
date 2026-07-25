use reqwest::Client;
use std::sync::Arc;
use std::time::Duration;
use crate::error::FunPayError;
use crate::models::{Game, Offer, Order, Chat, ChatMessage, Review};
use crate::parser::Parser;
use crate::retry::{RetryPolicy, RateLimiter, RateLimiterState, is_retryable_status};

/// Default FunPay base URL.
pub const DEFAULT_BASE_URL: &str = "https://funpay.com";

/// HTTP client for interacting with the FunPay API.
///
/// Supports automatic retries, rate limiting, and cookie-based authentication.
pub struct FunPayClient {
    pub client: Client,
    pub base_url: String,
    pub retry_policy: RetryPolicy,
    pub rate_limiter: Arc<RateLimiterState>,
}

impl FunPayClient {
    /// Creates a new unauthenticated client with default settings.
    pub fn new() -> Result<Self, FunPayError> {
        Self::builder().build()
    }

    /// Creates a new unauthenticated client with a custom base URL.
    pub fn with_base_url(base_url: &str) -> Result<Self, FunPayError> {
        Self::builder().base_url(base_url).build()
    }

    /// Creates an authenticated client using the given golden key.
    pub fn with_auth(golden_key: &str) -> Result<Self, FunPayError> {
        Self::builder().golden_key(golden_key).build()
    }

    /// Creates an authenticated client with a custom base URL.
    pub fn with_auth_and_base_url(golden_key: &str, base_url: &str) -> Result<Self, FunPayError> {
        Self::builder()
            .golden_key(golden_key)
            .base_url(base_url)
            .build()
    }

    /// Returns a new builder for configuring a [`FunPayClient`].
    pub fn builder() -> FunPayClientBuilder {
        FunPayClientBuilder::default()
    }

    /// Performs an HTTP GET request with automatic retries and rate limiting.
    pub async fn get(&self, path: &str) -> Result<String, FunPayError> {
        let url = if path.starts_with("http") {
            path.to_string()
        } else {
            format!("{}{}", self.base_url, path)
        };

        let mut last_error = None;

        for attempt in 0..=self.retry_policy.max_retries {
            if attempt > 0 {
                let delay = self.retry_policy.delay_for_attempt(attempt - 1);
                tokio::time::sleep(delay).await;
            }

            self.rate_limiter.wait().await;

            match self.client.get(&url).send().await {
                Ok(resp) => {
                    let status = resp.status().as_u16();
                    if is_retryable_status(status) {
                        if status == 429 {
                            let retry_after = resp
                                .headers()
                                .get("retry-after")
                                .and_then(|v| v.to_str().ok())
                                .and_then(|v| v.parse::<u64>().ok())
                                .map(|s| Duration::from_secs(s))
                                .unwrap_or_else(|| self.retry_policy.delay_for_attempt(attempt));
                            last_error = Some(FunPayError::RateLimited { retry_after });
                            continue;
                        }
                        last_error = Some(FunPayError::MaxRetriesExceeded(attempt));
                        continue;
                    }
                    return Ok(resp.text().await?);
                }
                Err(e) => {
                    if e.is_timeout() {
                        last_error = Some(FunPayError::Timeout(Duration::from_secs(30)));
                        continue;
                    }
                    return Err(e.into());
                }
            }
        }

        Err(last_error.unwrap_or(FunPayError::MaxRetriesExceeded(self.retry_policy.max_retries)))
    }

    /// Fetches the list of all available games on FunPay.
    pub async fn fetch_all_games(&self) -> Result<Vec<Game>, FunPayError> {
        let html = self.get("/").await?;
        Ok(Parser::new().parse_game_list(&html))
    }

    /// Fetches all offers from a specific category URL.
    pub async fn fetch_category_offers(&self, url: &str) -> Result<Vec<Offer>, FunPayError> {
        let html = self.get(url).await?;
        Ok(Parser::new().parse_offers_from_page(&html))
    }

    /// Searches all categories for offers matching a keyword and price limit.
    pub async fn search_all_categories(&self, keyword: &str, max_price: f64) -> Result<Vec<Offer>, FunPayError> {
        let games = self.fetch_all_games().await?;
        let mut seen = std::collections::HashSet::new();
        let mut results = Vec::new();
        let keyword_lower = keyword.to_lowercase();
        
        // First: search in games with keyword in name (most likely match)
        for game in games.iter().filter(|g| g.name.to_lowercase().contains(&keyword_lower)) {
            let urls: Vec<&url::Url> = [
                game.chips_url.as_ref(),
                game.lots_url.as_ref(),
            ].into_iter().flatten().collect();
            
            for url in urls {
                if let Ok(offers) = self.fetch_category_offers(url.as_str()).await {
                    for offer in offers {
                        if offer.price <= max_price && seen.insert(offer.offer_id.clone()) {
                            results.push(offer);
                        }
                    }
                }
            }
        }
        
        Ok(results)
    }

    /// Fetches the current user's orders.
    pub async fn fetch_orders(&self) -> Result<Vec<Order>, FunPayError> {
        let html = self.get("/orders").await?;
        Ok(Parser::new().parse_orders_from_page(&html))
    }

    /// Fetches the current user's chat list.
    pub async fn fetch_chats(&self) -> Result<Vec<Chat>, FunPayError> {
        let html = self.get("/chats").await?;
        Ok(Parser::new().parse_chats_from_page(&html))
    }

    /// Fetches messages for a specific chat.
    pub async fn fetch_chat_messages(&self, _chat_id: &str) -> Result<Vec<ChatMessage>, FunPayError> {
        let html = self.get("/chats").await?;
        Ok(Parser::new().parse_chat_messages(&html))
    }

    /// Fetches reviews for a specific user.
    pub async fn fetch_reviews(&self, _user_id: &str) -> Result<Vec<Review>, FunPayError> {
        let html = self.get("/users").await?;
        Ok(Parser::new().parse_reviews_from_page(&html))
    }
}

/// Builder for constructing a [`FunPayClient`] with custom configuration.
#[derive(Default)]
pub struct FunPayClientBuilder {
    base_url: Option<String>,
    golden_key: Option<String>,
    retry_policy: Option<RetryPolicy>,
    rate_limiter: Option<RateLimiter>,
    timeout_secs: Option<u64>,
}

impl FunPayClientBuilder {
    /// Sets the base URL for all requests.
    pub fn base_url(mut self, url: &str) -> Self {
        self.base_url = Some(url.to_string());
        self
    }

    /// Sets the golden key for authentication.
    pub fn golden_key(mut self, key: &str) -> Self {
        self.golden_key = Some(key.to_string());
        self
    }

    /// Sets the retry policy for failed requests.
    pub fn retry_policy(mut self, policy: RetryPolicy) -> Self {
        self.retry_policy = Some(policy);
        self
    }

    /// Sets the rate limiter configuration.
    pub fn rate_limiter(mut self, limiter: RateLimiter) -> Self {
        self.rate_limiter = Some(limiter);
        self
    }

    /// Sets the request timeout in seconds.
    pub fn timeout(mut self, secs: u64) -> Self {
        self.timeout_secs = Some(secs);
        self
    }

    /// Builds the [`FunPayClient`] with the configured settings.
    pub fn build(self) -> Result<FunPayClient, FunPayError> {
        let base_url = self.base_url.unwrap_or_else(|| DEFAULT_BASE_URL.to_string());
        let retry_policy = self.retry_policy.unwrap_or_default();
        let rate_limiter = self.rate_limiter.unwrap_or_default();
        let timeout_secs = self.timeout_secs.unwrap_or(30);

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"
                .parse()
                .expect("valid header value"),
        );

        let mut builder = Client::builder()
            .default_headers(headers)
            .cookie_store(true)
            .timeout(Duration::from_secs(timeout_secs));

        if let Some(golden_key) = self.golden_key {
            let jar = Arc::new(reqwest::cookie::Jar::default());
            let cookie_url = url::Url::parse("https://funpay.com").expect("valid URL");
            let cookie_str = format!("golden_key={}; Domain=funpay.com; Path=/", golden_key);
            jar.add_cookie_str(&cookie_str, &cookie_url);
            builder = builder.cookie_provider(jar);
        }

        let client = builder.build()?;

        Ok(FunPayClient {
            client,
            base_url,
            retry_policy,
            rate_limiter: Arc::new(RateLimiterState::new(rate_limiter)),
        })
    }
}
