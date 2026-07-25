//! Streaming search with price filtering.
//!
//! Convenience wrapper around [`FunPayClient::fetch_category_offers`]
//! that applies a maximum price filter.
//!
//! # Examples
//!
//! ```no_run
//! use funpay_sdk::client::FunPayClient;
//! use funpay_sdk::stream::search_offers;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), funpay_sdk::error::FunPayError> {
//! let client = FunPayClient::new()?;
//! let cheap_offers = search_offers(&client, 1, 1, 50.0).await?;
//! println!("Found {} offers under $50", cheap_offers.len());
//! # Ok(())
//! # }
//! ```

use crate::client::FunPayClient;
use crate::models::Offer;

/// Fetch offers filtered by maximum price.
pub async fn search_offers(
    client: &FunPayClient,
    game_id: u64,
    category_id: u64,
    max_price: f64,
) -> Result<Vec<Offer>, crate::error::FunPayError> {
    let offers = client.fetch_category_offers(game_id, category_id).await?;
    Ok(offers.into_iter().filter(|o| *o.price.inner() <= max_price).collect())
}
