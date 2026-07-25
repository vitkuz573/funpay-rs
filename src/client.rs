//! Client for interacting with the FunPay API.
//!
//! The [`FunPayClient`] handles HTTP requests, cookie persistence,
//! and HTML parsing for all FunPay endpoints.
//!
//! # Examples
//!
//! ```no_run
//! use funpay_sdk::client::FunPayClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), funpay_sdk::error::FunPayError> {
//!     let client = FunPayClient::new()?;
//!
//!     // Fetch game catalog
//!     let games = client.fetch_game_list().await?;
//!     println!("Found {} games", games.len());
//!
//!     // Fetch offers for a specific game/category
//!     if let Some(game) = games.first() {
//!         let offers = client.fetch_category_offers(game.id.inner(), 1).await?;
//!         for offer in &offers {
//!             println!("{}: {} {}", offer.price, offer.currency, offer.description.as_deref().unwrap_or(""));
//!         }
//!     }
//!     Ok(())
//! }
//! ```

use reqwest::Client;
use crate::error::FunPayError;
use crate::models::*;
use crate::parser::Parser;

/// Default FunPay base URL.
pub const DEFAULT_BASE_URL: &str = "https://funpay.com";

/// Async HTTP client for the FunPay marketplace.
///
/// Provides methods to fetch games, offers, profiles, chats, and orders.
/// All methods return typed [`models`] on success.
pub struct FunPayClient {
    /// The underlying HTTP client.
    pub client: Client,
    /// Base URL for all requests.
    pub base_url: String,
}

impl FunPayClient {
    /// Create a new client with the default FunPay base URL.
    ///
    /// # Errors
    ///
    /// Returns [`FunPayError::Http`] if the HTTP client cannot be built.
    pub fn new() -> Result<Self, FunPayError> {
        Self::with_base_url(DEFAULT_BASE_URL)
    }

    /// Create a new client with a custom base URL.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use funpay_sdk::client::FunPayClient;
    /// let client = FunPayClient::with_base_url("https://example.com").unwrap();
    /// assert_eq!(client.base_url, "https://example.com");
    /// ```
    pub fn with_base_url(base_url: &str) -> Result<Self, FunPayError> {
        let client = Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .cookie_store(true)
            .timeout(std::time::Duration::from_secs(30))
            .build()?;
        Ok(Self { client, base_url: base_url.to_string() })
    }

    /// Fetch raw HTML from a path (relative to base URL) or full URL.
    pub async fn get(&self, path: &str) -> Result<String, FunPayError> {
        let url = if path.starts_with("http") {
            path.to_string()
        } else {
            format!("{}{}", self.base_url, path)
        };
        let resp = self.client.get(&url).send().await?;
        Ok(resp.text().await?)
    }

    /// Fetch the user's order history.
    pub async fn fetch_orders(&self) -> Result<Vec<Order>, FunPayError> {
        let html = self.get("/orders/").await?;
        Ok(Parser::new().parse_orders(&html))
    }

    /// Fetch offer lots for a specific user.
    pub async fn fetch_user_offers(&self, user_id: u64) -> Result<Vec<OfferLot>, FunPayError> {
        let html = self.get(&format!("/users/{}/lots/", user_id)).await?;
        Ok(Parser::new().parse_user_offers(&html))
    }

    /// Fetch offers in a specific game category.
    pub async fn fetch_category_offers(&self, game_id: u64, category_id: u64) -> Result<Vec<Offer>, FunPayError> {
        let html = self.get(&format!("/lots/{}/{}/", game_id, category_id)).await?;
        Ok(Parser::new().parse_category_offers(&html))
    }

    /// Fetch a seller's public profile.
    pub async fn fetch_seller_profile(&self, seller_id: u64) -> Result<Seller, FunPayError> {
        let html = self.get(&format!("/users/{}/", seller_id)).await?;
        Parser::new().parse_seller_profile(&html).ok_or_else(|| {
            FunPayError::Parse(crate::error::ParseError::MissingField(
                "Seller profile not found on page".into(),
            ))
        })
    }

    /// Fetch a user's public profile.
    pub async fn fetch_user_profile(&self, user_id: u64) -> Result<User, FunPayError> {
        let html = self.get(&format!("/users/{}/", user_id)).await?;
        Parser::new().parse_user_profile(&html).ok_or_else(|| {
            FunPayError::Parse(crate::error::ParseError::MissingField(
                "User profile not found on page".into(),
            ))
        })
    }

    /// Fetch chat messages for a specific chat.
    pub async fn fetch_chat_messages(&self, chat_id: u64) -> Result<Vec<ChatMessage>, FunPayError> {
        let html = self.get(&format!("/chats/{}/", chat_id)).await?;
        Ok(Parser::new().parse_chat_messages(&html))
    }

    /// Fetch the full game catalog.
    pub async fn fetch_game_list(&self) -> Result<Vec<Game>, FunPayError> {
        let html = self.get("/lots/").await?;
        Ok(Parser::new().parse_game_list(&html))
    }

    /// Fetch the user's chat list.
    pub async fn fetch_chats(&self) -> Result<Vec<Chat>, FunPayError> {
        let html = self.get("/chats/").await?;
        Ok(Parser::new().parse_chats(&html))
    }
}
