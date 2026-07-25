use reqwest::Client;
use crate::error::FunPayError;
use crate::models::{Game, Offer};
use crate::parser::Parser;

pub const DEFAULT_BASE_URL: &str = "https://funpay.com";

pub struct FunPayClient {
    pub client: Client,
    pub base_url: String,
}

impl FunPayClient {
    pub fn new() -> Result<Self, FunPayError> {
        Self::with_base_url(DEFAULT_BASE_URL)
    }

    pub fn with_base_url(base_url: &str) -> Result<Self, FunPayError> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".parse().unwrap());
        
        let client = Client::builder()
            .default_headers(headers)
            .cookie_store(true)
            .build()?;
        
        Ok(Self { client, base_url: base_url.to_string() })
    }

    pub fn with_auth(golden_key: &str) -> Result<Self, FunPayError> {
        Self::with_auth_and_base_url(golden_key, DEFAULT_BASE_URL)
    }

    pub fn with_auth_and_base_url(golden_key: &str, base_url: &str) -> Result<Self, FunPayError> {
        let mut this = Self::with_base_url(base_url)?;
        this.client = Client::builder()
            .default_headers({
                let mut h = reqwest::header::HeaderMap::new();
                h.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".parse().unwrap());
                h.insert("Cookie", reqwest::header::HeaderValue::from_str(&format!("golden_key={}", golden_key)).unwrap());
                h
            })
            .cookie_store(true)
            .build()?;
        Ok(this)
    }

    pub async fn get(&self, path: &str) -> Result<String, FunPayError> {
        let url = if path.starts_with("http") {
            path.to_string()
        } else {
            format!("{}{}", self.base_url, path)
        };
        let resp = self.client.get(&url).send().await?;
        Ok(resp.text().await?)
    }

    pub async fn fetch_all_games(&self) -> Result<Vec<Game>, FunPayError> {
        let html = self.get("/").await?;
        Ok(Parser::new().parse_game_list(&html))
    }

    pub async fn fetch_category_offers(&self, url: &str) -> Result<Vec<Offer>, FunPayError> {
        let html = self.get(url).await?;
        Ok(Parser::new().parse_offers_from_page(&html))
    }

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
}
