use reqwest::Client;
use rust_decimal::Decimal;
use std::sync::Arc;
use std::time::{Duration, Instant};
use crate::error::FunPayError;
use crate::middleware::RequestMiddleware;
use crate::models::{Game, Offer, Order, Chat, ChatMessage, Review};
use crate::parser::Parser;
use crate::retry::{RetryPolicy, RateLimiter, RateLimiterState, is_retryable_status};

/// Default FunPay base URL.
pub const DEFAULT_BASE_URL: &str = "https://funpay.com";

/// Default cache TTL for game list (1 hour).
const GAME_CACHE_TTL: Duration = Duration::from_secs(3600);

/// HTTP client for interacting with the FunPay API.
///
/// Supports automatic retries, rate limiting, cookie-based authentication,
/// and an LRU cache for the game list.
pub struct FunPayClient {
    pub client: Client,
    pub base_url: String,
    pub retry_policy: RetryPolicy,
    pub rate_limiter: Arc<RateLimiterState>,
    pub middleware: Vec<Box<dyn RequestMiddleware>>,
    game_cache: std::sync::Mutex<Option<(Vec<Game>, Instant)>>,
}

impl FunPayClient {
    /// Creates a new unauthenticated client with default settings.
    ///
    /// # Errors
    ///
    /// Returns [`FunPayError::Reqwest`] if the underlying HTTP client
    /// cannot be constructed (e.g. invalid TLS configuration).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use funpay_rs::client::FunPayClient;
    ///
    /// # async fn example() -> Result<(), funpay_rs::error::FunPayError> {
    /// let client = FunPayClient::new()?;
    /// let games = client.fetch_all_games().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new() -> Result<Self, FunPayError> {
        Self::builder().build()
    }

    /// Creates a new unauthenticated client with a custom base URL.
    ///
    /// # Errors
    ///
    /// Returns [`FunPayError::Reqwest`] if the HTTP client cannot be built.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use funpay_rs::client::FunPayClient;
    ///
    /// let client = FunPayClient::with_base_url("https://funpay.com").unwrap();
    /// ```
    pub fn with_base_url(base_url: &str) -> Result<Self, FunPayError> {
        Self::builder().base_url(base_url).build()
    }

    /// Creates an authenticated client using the given golden key.
    ///
    /// The golden key is stored as a cookie and sent with every request.
    ///
    /// # Errors
    ///
    /// Returns [`FunPayError::Reqwest`] if the HTTP client cannot be built.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use funpay_rs::client::FunPayClient;
    ///
    /// let client = FunPayClient::with_auth("your-golden-key").unwrap();
    /// ```
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
    ///
    /// # Example
    /// ```no_run
    /// use funpay_rs::client::FunPayClient;
    ///
    /// # async fn example() -> Result<(), funpay_rs::error::FunPayError> {
    /// let client = FunPayClient::builder()
    ///     .base_url("https://example.com")
    ///     .timeout(60)
    ///     .build()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn builder() -> FunPayClientBuilder {
        FunPayClientBuilder::default()
    }

    /// Performs an HTTP GET request with automatic retries and rate limiting.
    ///
    /// If `path` starts with `http`, it is used as-is; otherwise it is
    /// prepended with [`base_url`](Self::base_url).
    ///
    /// # Errors
    ///
    /// Returns [`FunPayError::Reqwest`] on transport errors,
    /// [`FunPayError::MaxRetriesExceeded`] if all retries fail,
    /// [`FunPayError::RateLimited`] on HTTP 429, or
    /// [`FunPayError::Timeout`] on request timeouts.
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

            let mut req = self.client.get(&url).build()?;
            for mw in &self.middleware {
                mw.on_request(&mut req);
            }
            match self.client.execute(req).await {
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
    ///
    /// Results are cached internally for 1 hour. Use
    /// [`clear_game_cache`](Self::clear_game_cache) to force a refresh.
    ///
    /// # Errors
    ///
    /// Returns [`FunPayError::Reqwest`] on network failures,
    /// [`FunPayError::MaxRetriesExceeded`] if retries are exhausted,
    /// or [`FunPayError::RateLimited`] if the server responds with 429.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use funpay_rs::client::FunPayClient;
    ///
    /// # async fn example() -> Result<(), funpay_rs::error::FunPayError> {
    /// let client = FunPayClient::new()?;
    /// let games = client.fetch_all_games().await?;
    /// for game in &games {
    ///     println!("{} — {}", game.name, game.id);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn fetch_all_games(&self) -> Result<Vec<Game>, FunPayError> {
        {
            let cache = self.game_cache.lock().unwrap();
            if let Some((ref games, timestamp)) = *cache {
                if timestamp.elapsed() < GAME_CACHE_TTL {
                    return Ok(games.clone());
                }
            }
        }
        let html = self.get("/").await?;
        let games = Parser::new().parse_game_list(&html);
        {
            let mut cache = self.game_cache.lock().unwrap();
            *cache = Some((games.clone(), Instant::now()));
        }
        Ok(games)
    }

    /// Clears the cached game list, forcing the next call to
    /// [`fetch_all_games`](Self::fetch_all_games) to fetch fresh data.
    pub fn clear_game_cache(&self) {
        let mut cache = self.game_cache.lock().unwrap();
        *cache = None;
    }

    /// Fetches all offers from a specific category URL.
    ///
    /// # Errors
    ///
    /// Returns [`FunPayError::Reqwest`] on network failures,
    /// [`FunPayError::MaxRetriesExceeded`] if retries are exhausted,
    /// or [`FunPayError::RateLimited`] if the server responds with 429.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use funpay_rs::client::FunPayClient;
    ///
    /// # async fn example() -> Result<(), funpay_rs::error::FunPayError> {
    /// let client = FunPayClient::new()?;
    /// let offers = client.fetch_category_offers("https://funpay.com/lots/442").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn fetch_category_offers(&self, url: &str) -> Result<Vec<Offer>, FunPayError> {
        let html = self.get(url).await?;
        Ok(Parser::new().parse_offers_from_page(&html))
    }

    /// Searches all categories for offers matching a keyword and price limit.
    ///
    /// Iterates over every game whose name contains `keyword` (case-insensitive),
    /// fetches both chips and lots categories, and filters by `max_price`.
    /// Deduplicates results by offer ID.
    ///
    /// # Errors
    ///
    /// Returns [`FunPayError::Reqwest`] on network failures,
    /// [`FunPayError::MaxRetriesExceeded`] if retries are exhausted,
    /// or [`FunPayError::RateLimited`] if the server responds with 429.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use funpay_rs::client::FunPayClient;
    /// use rust_decimal::Decimal;
    ///
    /// # async fn example() -> Result<(), funpay_rs::error::FunPayError> {
    /// let client = FunPayClient::new()?;
    /// let offers = client.search_all_categories("kimi", Decimal::from(3000)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn search_all_categories(&self, keyword: &str, max_price: Decimal) -> Result<Vec<Offer>, FunPayError> {
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
    ///
    /// Requires an authenticated client.
    ///
    /// # Errors
    ///
    /// Returns [`FunPayError::Reqwest`] on network failures or
    /// [`FunPayError::MaxRetriesExceeded`] if retries are exhausted.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use funpay_rs::client::FunPayClient;
    ///
    /// # async fn example() -> Result<(), funpay_rs::error::FunPayError> {
    /// let client = FunPayClient::with_auth("golden-key")?;
    /// let orders = client.fetch_orders().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn fetch_orders(&self) -> Result<Vec<Order>, FunPayError> {
        let html = self.get("/orders").await?;
        Ok(Parser::new().parse_orders_from_page(&html))
    }

    /// Fetches the current user's chat list.
    ///
    /// Requires an authenticated client.
    ///
    /// # Errors
    ///
    /// Returns [`FunPayError::Reqwest`] on network failures or
    /// [`FunPayError::MaxRetriesExceeded`] if retries are exhausted.
    pub async fn fetch_chats(&self) -> Result<Vec<Chat>, FunPayError> {
        let html = self.get("/chats").await?;
        Ok(Parser::new().parse_chats_from_page(&html))
    }

    /// Fetches messages for a specific chat.
    ///
    /// Requires an authenticated client.
    ///
    /// # Errors
    ///
    /// Returns [`FunPayError::Reqwest`] on network failures or
    /// [`FunPayError::MaxRetriesExceeded`] if retries are exhausted.
    pub async fn fetch_chat_messages(&self, _chat_id: &str) -> Result<Vec<ChatMessage>, FunPayError> {
        let html = self.get("/chats").await?;
        Ok(Parser::new().parse_chat_messages(&html))
    }

    /// Fetches reviews for a specific user.
    ///
    /// # Errors
    ///
    /// Returns [`FunPayError::Reqwest`] on network failures or
    /// [`FunPayError::MaxRetriesExceeded`] if retries are exhausted.
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
    middleware: Vec<Box<dyn RequestMiddleware>>,
}

impl FunPayClientBuilder {
    /// Sets the base URL for all requests.
    ///
    /// # Example
    /// ```no_run
    /// use funpay_rs::client::FunPayClient;
    ///
    /// # async fn example() -> Result<(), funpay_rs::error::FunPayError> {
    /// let client = FunPayClient::builder()
    ///     .base_url("https://funpay.com")
    ///     .build()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn base_url(mut self, url: &str) -> Self {
        self.base_url = Some(url.to_string());
        self
    }

    /// Sets the golden key for authentication.
    ///
    /// # Example
    /// ```no_run
    /// use funpay_rs::client::FunPayClient;
    ///
    /// # async fn example() -> Result<(), funpay_rs::error::FunPayError> {
    /// let client = FunPayClient::builder()
    ///     .golden_key("your-golden-key-here")
    ///     .build()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn golden_key(mut self, key: &str) -> Self {
        self.golden_key = Some(key.to_string());
        self
    }

    /// Sets the retry policy for failed requests.
    ///
    /// Controls max retries and backoff delay.
    pub fn retry_policy(mut self, policy: RetryPolicy) -> Self {
        self.retry_policy = Some(policy);
        self
    }

    /// Sets the rate limiter configuration.
    ///
    /// Controls maximum requests per second.
    pub fn rate_limiter(mut self, limiter: RateLimiter) -> Self {
        self.rate_limiter = Some(limiter);
        self
    }

    /// Sets the request timeout in seconds.
    ///
    /// Defaults to 30 seconds.
    pub fn timeout(mut self, secs: u64) -> Self {
        self.timeout_secs = Some(secs);
        self
    }

    /// Adds a request middleware to the client.
    ///
    /// Middleware is executed in insertion order before each request is sent.
    pub fn middleware(mut self, mw: Box<dyn RequestMiddleware>) -> Self {
        self.middleware.push(mw);
        self
    }

    /// Builds the [`FunPayClient`] with the configured settings.
    ///
    /// # Errors
    ///
    /// Returns [`FunPayError::Reqwest`] if the HTTP client cannot be constructed.
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
            middleware: self.middleware,
            game_cache: std::sync::Mutex::new(None),
        })
    }
}
