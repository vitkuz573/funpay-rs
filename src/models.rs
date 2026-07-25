use serde::{Deserialize, Serialize};

/// Creates a newtype ID wrapper with common trait implementations.
macro_rules! define_id_type {
    ($name:ident) => {
        /// A strongly-typed identifier for a FunPay entity.
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct $name(pub String);

        impl $name {
            /// Returns the ID as a string slice.
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

/// Supported currency types on FunPay.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Currency {
    RUB,
    USD,
    EUR,
}

impl Currency {
    /// Parses a currency from its symbol (e.g. `"$"`, `"€"`).
    pub fn from_symbol(symbol: &str) -> Self {
        match symbol {
            "$" => Currency::USD,
            "€" => Currency::EUR,
            _ => Currency::RUB,
        }
    }

    /// Returns the display symbol for this currency.
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

/// The type of a game listing on FunPay (chips or lots).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GameCategory {
    Chips,
    Lots,
}

impl GameCategory {
    /// Parses the game category from a URL path segment.
    pub fn from_url(url: &str) -> Option<Self> {
        if url.contains("/chips/") {
            Some(GameCategory::Chips)
        } else if url.contains("/lots/") {
            Some(GameCategory::Lots)
        } else {
            None
        }
    }

    /// Returns the URL path segment for this category.
    pub fn as_path(&self) -> &'static str {
        match self {
            GameCategory::Chips => "chips",
            GameCategory::Lots => "lots",
        }
    }
}

/// Online/offline status of a seller.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OnlineStatus {
    Online,
    Offline,
}

impl OnlineStatus {
    /// Parses an online status from a string.
    pub fn from_str(s: &str) -> Self {
        if s == "online" {
            OnlineStatus::Online
        } else {
            OnlineStatus::Offline
        }
    }

    /// Returns `true` if the seller is online.
    pub fn is_online(&self) -> bool {
        matches!(self, OnlineStatus::Online)
    }
}

/// A specific lot (pricing tier) within an offer.
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

/// Seller profile information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Seller {
    pub user_id: UserId,
    pub name: String,
    pub rating: f64,
    pub reviews: u32,
    pub online: bool,
}

/// A game available on FunPay with its category URLs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub id: GameId,
    pub name: String,
    pub chips_url: Option<url::Url>,
    pub lots_url: Option<url::Url>,
    pub category: Option<GameCategory>,
}

/// A FunPay user profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub user_id: UserId,
    pub username: String,
    pub rating: f64,
    pub reviews: u32,
    pub online: bool,
    pub registered: Option<chrono::NaiveDate>,
}

/// A sellable offer listing on FunPay.
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

/// An order placed on FunPay.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub order_id: String,
    pub offer_id: OfferId,
    pub seller: Seller,
    pub buyer: Seller,
    pub price: f64,
    pub currency: Currency,
    pub status: OrderStatus,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub completed_at: Option<chrono::NaiveDateTime>,
}

/// The status of an order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderStatus {
    Pending,
    Active,
    Completed,
    Cancelled,
    Disputed,
    Unknown(String),
}

/// A chat conversation between two users.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chat {
    pub chat_id: String,
    pub other_user: Seller,
    pub last_message: Option<ChatMessage>,
    pub unread_count: u32,
}

/// A single message in a chat conversation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub message_id: String,
    pub sender_id: UserId,
    pub text: String,
    pub timestamp: Option<chrono::NaiveDateTime>,
    pub is_read: bool,
}

/// A review left by a buyer after completing an order.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    pub review_id: String,
    pub reviewer: Seller,
    pub rating: f64,
    pub text: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub order_id: Option<String>,
}
