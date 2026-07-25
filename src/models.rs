use serde::{Deserialize, Serialize};

macro_rules! define_id_type {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct $name(pub String);

        impl $name {
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<String> for $name {
            fn from(s: String) -> Self {
                Self(s)
            }
        }

        impl From<&str> for $name {
            fn from(s: &str) -> Self {
                Self(s.to_string())
            }
        }
    };
}

define_id_type!(OfferId);
define_id_type!(LotId);
define_id_type!(UserId);
define_id_type!(GameId);
define_id_type!(Server);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Currency {
    RUB,
    USD,
    EUR,
}

impl Currency {
    pub fn from_symbol(symbol: &str) -> Self {
        match symbol {
            "$" => Currency::USD,
            "€" => Currency::EUR,
            _ => Currency::RUB,
        }
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            Currency::RUB => "₽",
            Currency::USD => "$",
            Currency::EUR => "€",
        }
    }
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GameCategory {
    Chips,
    Lots,
}

impl GameCategory {
    pub fn from_url(url: &str) -> Option<Self> {
        if url.contains("/chips/") {
            Some(GameCategory::Chips)
        } else if url.contains("/lots/") {
            Some(GameCategory::Lots)
        } else {
            None
        }
    }

    pub fn as_path(&self) -> &'static str {
        match self {
            GameCategory::Chips => "chips",
            GameCategory::Lots => "lots",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OnlineStatus {
    Online,
    Offline,
}

impl OnlineStatus {
    pub fn from_str(s: &str) -> Self {
        if s == "online" {
            OnlineStatus::Online
        } else {
            OnlineStatus::Offline
        }
    }

    pub fn is_online(&self) -> bool {
        matches!(self, OnlineStatus::Online)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lot {
    pub offer_id: OfferId,
    pub server: Server,
    pub description: String,
    pub price: f64,
    pub currency: Currency,
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
    pub id: GameId,
    pub name: String,
    pub chips_url: Option<String>,
    pub lots_url: Option<String>,
    pub category: Option<GameCategory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub user_id: UserId,
    pub username: String,
    pub rating: f64,
    pub reviews: u32,
    pub online: bool,
    pub registered: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Offer {
    pub offer_id: OfferId,
    pub lot_id: LotId,
    pub server: Server,
    pub description: String,
    pub price: f64,
    pub currency: Currency,
    pub stock: u32,
    pub seller: Seller,
    pub fields: std::collections::HashMap<String, String>,
}
