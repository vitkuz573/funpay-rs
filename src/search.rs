use crate::models::Offer;
use crate::client::FunPayClient;
use crate::error::FunPayError;

pub struct SearchQuery {
    keyword: String,
    max_price: Option<f64>,
    online_only: bool,
}

impl SearchQuery {
    pub fn new(keyword: &str) -> Self {
        Self {
            keyword: keyword.to_string(),
            max_price: None,
            online_only: false,
        }
    }

    pub fn max_price(mut self, price: f64) -> Self {
        self.max_price = Some(price);
        self
    }

    pub fn online_only(mut self) -> Self {
        self.online_only = true;
        self
    }

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
