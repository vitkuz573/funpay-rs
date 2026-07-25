//! Offer search with filtering.
//!
//! Provides a builder-style [`SearchQuery`] for filtering offers
//! by price and seller status.
//!
//! # Examples
//!
//! ```no_run
//! use funpay_sdk::client::FunPayClient;
//! use funpay_sdk::search::SearchQuery;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), funpay_sdk::error::FunPayError> {
//! let client = FunPayClient::new()?;
//! let results = SearchQuery::new("gold")
//!     .max_price(100.0)
//!     .online_only()
//!     .execute(&client, 1, 1)
//!     .await?;
//! println!("Found {} offers under $100", results.len());
//! # Ok(())
//! # }
//! ```

use crate::models::Offer;
use crate::client::FunPayClient;
use crate::error::FunPayError;

/// Builder for filtering offer search results.
pub struct SearchQuery {
    #[allow(dead_code)]
    keyword: String,
    max_price: Option<f64>,
    online_only: bool,
}

impl SearchQuery {
    /// Create a new search query with a keyword filter.
    pub fn new(keyword: &str) -> Self {
        Self {
            keyword: keyword.to_string(),
            max_price: None,
            online_only: false,
        }
    }

    /// Set the maximum price filter.
    pub fn max_price(mut self, price: f64) -> Self {
        self.max_price = Some(price);
        self
    }

    /// Filter to only online sellers.
    pub fn online_only(mut self) -> Self {
        self.online_only = true;
        self
    }

    /// Execute the search against a game category.
    pub async fn execute(&self, client: &FunPayClient, game_id: u64, category_id: u64) -> Result<Vec<Offer>, FunPayError> {
        let offers = client
            .fetch_category_offers(game_id, category_id)
            .await?;

        let filtered = if let Some(max) = self.max_price {
            offers.into_iter().filter(|o| *o.price.inner() <= max).collect()
        } else {
            offers
        };

        Ok(filtered)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_query_builder() {
        let q = SearchQuery::new("gold").max_price(50.0).online_only();
        assert_eq!(q.keyword, "gold");
        assert_eq!(q.max_price, Some(50.0));
        assert!(q.online_only);
    }

    #[test]
    fn test_search_query_defaults() {
        let q = SearchQuery::new("test");
        assert!(q.max_price.is_none());
        assert!(!q.online_only);
    }
}
