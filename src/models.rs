use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lot {
    pub offer_id: String,
    pub server: String,
    pub description: String,
    pub price: f64,
    pub currency: String,
    pub stock: u32,
    pub seller: Seller,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Seller {
    pub name: String,
    pub rating: f64,
    pub reviews: u32,
    pub online: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub id: String,
    pub name: String,
    pub chips_url: Option<String>,
    pub lots_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub user_id: String,
    pub username: String,
    pub rating: f64,
    pub reviews: u32,
    pub online: bool,
    pub registered: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Offer {
    pub offer_id: String,
    pub lot_id: String,
    pub server: String,
    pub description: String,
    pub price: f64,
    pub currency: String,
    pub stock: u32,
    pub seller: Seller,
    pub fields: std::collections::HashMap<String, String>,
}
