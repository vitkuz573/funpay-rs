use crate::models::{Currency, Server, GameId, Offer};
use crate::client::FunPayClient;
use crate::error::FunPayError;

/// A builder for constructing filtered search queries against FunPay offers.
///
/// # Example
/// ```no_run
/// use funpay_rs::search::SearchQuery;
/// use funpay_rs::models::Currency;
///
/// # async fn example() -> Result<(), funpay_rs::error::FunPayError> {
/// let client = funpay_rs::client::FunPayClient::new()?;
/// let offers = SearchQuery::new("CS2 skins")
///     .max_price(100.0)
///     .currency(Currency::USD)
///     .online_only()
///     .min_stock(5)
///     .execute(&client)
///     .await?;
/// # Ok(())
/// # }
/// ```
pub struct SearchQuery {
    keyword: String,
    max_price: Option<f64>,
    currency: Option<Currency>,
    servers: Vec<Server>,
    game_id: Option<GameId>,
    online_only: bool,
    min_stock: Option<u32>,
}

impl SearchQuery {
    /// Creates a new search query with the given keyword.
    pub fn new(keyword: &str) -> Self {
        Self {
            keyword: keyword.to_string(),
            max_price: None,
            currency: None,
            servers: Vec::new(),
            game_id: None,
            online_only: false,
            min_stock: None,
        }
    }

    /// Sets the maximum price filter.
    pub fn max_price(mut self, price: f64) -> Self {
        self.max_price = Some(price);
        self
    }

    /// Sets the currency filter.
    pub fn currency(mut self, currency: Currency) -> Self {
        self.currency = Some(currency);
        self
    }

    /// Adds a server filter. Can be called multiple times to filter by several servers.
    pub fn server(mut self, server: Server) -> Self {
        self.servers.push(server);
        self
    }

    /// Sets the game ID filter.
    pub fn game_id(mut self, game_id: GameId) -> Self {
        self.game_id = Some(game_id);
        self
    }

    /// Filters results to only include offers from online sellers.
    pub fn online_only(mut self) -> Self {
        self.online_only = true;
        self
    }

    /// Sets the minimum stock quantity filter.
    pub fn min_stock(mut self, stock: u32) -> Self {
        self.min_stock = Some(stock);
        self
    }

    /// Executes the search query against FunPay and returns matching offers.
    pub async fn execute(&self, client: &FunPayClient) -> Result<Vec<Offer>, FunPayError> {
        let max_price = self.max_price.unwrap_or(f64::MAX);
        let mut offers = client
            .search_all_categories(&self.keyword, max_price)
            .await?;

        offers.retain(|o| {
            if let Some(ref currency) = self.currency {
                if o.currency != *currency {
                    return false;
                }
            }
            if !self.servers.is_empty() && !self.servers.contains(&o.server) {
                return false;
            }
            if self.online_only && !o.seller.online {
                return false;
            }
            if let Some(min_stock) = self.min_stock {
                if o.stock < min_stock {
                    return false;
                }
            }
            true
        });

        Ok(offers)
    }
}
