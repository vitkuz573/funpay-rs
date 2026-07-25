//! Data models for the FunPay SDK.
//!
//! All parsed HTML responses are returned as instances of these structs.
//! Types use newtype wrappers for IDs and prices to prevent confusion.
//!
//! # Examples
//!
//! ```rust
//! use funpay_sdk::models::{Offer, Price, OfferId, UserId};
//!
//! let offer = Offer {
//!     image_url: None,
//!     currency: "RUB".into(),
//!     sale_type: Default::default(),
//!     seller_id: UserId::new(123),
//!     id: OfferId::new(456),
//!     description: Some("1000 Gold".into()),
//!     item_count: None,
//!     price: Price::new(25.50),
//!     server: Some("EU".into()),
//! };
//! assert_eq!(offer.price.to_string(), "25.5");
//! ```

use serde::{Deserialize, Serialize};
use std::hash::Hash;

pub trait IdType: std::fmt::Display + std::str::FromStr + Clone + Copy + PartialEq + Eq + Hash {
    fn new(id: u64) -> Self;
    fn inner(self) -> u64;
}

macro_rules! define_id_type {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
        pub struct $name(u64);

        impl $name {
            pub fn new(id: u64) -> Self { Self(id) }
            pub fn inner(self) -> u64 { self.0 }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl std::str::FromStr for $name {
            type Err = std::num::ParseIntError;
            fn from_str(s: &str) -> Result<Self, Self::Err> { Ok(Self(s.parse()?)) }
        }
    };
}

define_id_type!(OfferId);
define_id_type!(UserId);
define_id_type!(GameId);
define_id_type!(LotId);
define_id_type!(CategoryId);
define_id_type!(ChatId);
define_id_type!(MessageId);
define_id_type!(ReviewId);
define_id_type!(TransactionId);
define_id_type!(NotificationId);
define_id_type!(ServerId);

/// A monetary price with f64 precision.
///
/// # Examples
///
/// ```rust
/// use funpay_sdk::models::Price;
///
/// let p = Price::new(19.99);
/// assert_eq!(p.to_string(), "19.99");
///
/// let p2: Price = "42.5".parse().unwrap();
/// assert_eq!(p2, Price(42.5));
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Price(pub f64);

impl Price {
    pub fn new(v: impl Into<f64>) -> Self { Self(v.into()) }
    pub fn inner(&self) -> &f64 { &self.0 }
}

impl std::fmt::Display for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for Price {
    fn default() -> Self { Self(0.0) }
}

impl std::str::FromStr for Price {
    type Err = std::num::ParseFloatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> { Ok(Self(s.parse()?)) }
}

/// Account balance (newtype over f64).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Balance(pub f64);

impl Default for Balance {
    fn default() -> Self { Self(0.0) }
}

impl std::str::FromStr for Balance {
    type Err = std::num::ParseFloatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> { Ok(Self(s.parse()?)) }
}

/// Whether a lot is sold individually or in bulk.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum LotSaleType {
    #[default]
    #[serde(rename = "single")]
    Single,
    #[serde(rename = "bulk")]
    Bulk,
}

impl std::fmt::Display for LotSaleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LotSaleType::Single => write!(f, "single"),
            LotSaleType::Bulk => write!(f, "bulk"),
        }
    }
}

/// Status of an order.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum OrderStatus {
    #[default]
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "disputed")]
    Disputed,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "active")]
    Active,
}

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderStatus::Completed => write!(f, "completed"),
            OrderStatus::Disputed => write!(f, "disputed"),
            OrderStatus::Cancelled => write!(f, "cancelled"),
            OrderStatus::Pending => write!(f, "pending"),
            OrderStatus::Active => write!(f, "active"),
        }
    }
}

/// Whether an offer order is auto or manual.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum OfferOrderType {
    #[default]
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "manual")]
    Manual,
}

impl std::fmt::Display for OfferOrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OfferOrderType::Auto => write!(f, "auto"),
            OfferOrderType::Manual => write!(f, "manual"),
        }
    }
}

/// Online/offline status of a user.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum OnlineStatus {
    #[default]
    #[serde(rename = "offline")]
    Offline,
    #[serde(rename = "online")]
    Online,
}

impl std::fmt::Display for OnlineStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OnlineStatus::Offline => write!(f, "offline"),
            OnlineStatus::Online => write!(f, "online"),
        }
    }
}

/// Type of server tag (server vs platform).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum ServerTagType {
    #[default]
    #[serde(rename = "server")]
    Server,
    #[serde(rename = "platform")]
    Platform,
}

impl std::fmt::Display for ServerTagType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerTagType::Server => write!(f, "server"),
            ServerTagType::Platform => write!(f, "platform"),
        }
    }
}

/// Type of transaction (deposit, withdrawal, etc.).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum TransactionType {
    #[default]
    #[serde(rename = "deposit")]
    Deposit,
    #[serde(rename = "refund")]
    Refund,
    #[serde(rename = "bonus")]
    Bonus,
    #[serde(rename = "withdrawal")]
    Withdrawal,
    #[serde(rename = "sale")]
    Sale,
    #[serde(rename = "purchase")]
    Purchase,
}

impl std::fmt::Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionType::Deposit => write!(f, "deposit"),
            TransactionType::Refund => write!(f, "refund"),
            TransactionType::Bonus => write!(f, "bonus"),
            TransactionType::Withdrawal => write!(f, "withdrawal"),
            TransactionType::Sale => write!(f, "sale"),
            TransactionType::Purchase => write!(f, "purchase"),
        }
    }
}

/// Type of chat conversation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum ChatType {
    #[default]
    #[serde(rename = "user")]
    User,
    #[serde(rename = "support")]
    Support,
    #[serde(rename = "order")]
    Order,
}

impl std::fmt::Display for ChatType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChatType::User => write!(f, "user"),
            ChatType::Support => write!(f, "support"),
            ChatType::Order => write!(f, "order"),
        }
    }
}

/// Type of notification.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum NotificationType {
    #[default]
    #[serde(rename = "review")]
    Review,
    #[serde(rename = "order")]
    Order,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "message")]
    Message,
    #[serde(rename = "payment")]
    Payment,
}

impl std::fmt::Display for NotificationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NotificationType::Review => write!(f, "review"),
            NotificationType::Order => write!(f, "order"),
            NotificationType::System => write!(f, "system"),
            NotificationType::Message => write!(f, "message"),
            NotificationType::Payment => write!(f, "payment"),
        }
    }
}

/// Sort order for search results.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum SearchSort {
    #[default]
    #[serde(rename = "relevance")]
    Relevance,
    #[serde(rename = "price_desc")]
    PriceDesc,
    #[serde(rename = "price_asc")]
    PriceAsc,
    #[serde(rename = "rating")]
    Rating,
    #[serde(rename = "date")]
    Date,
}

/// A seller on the FunPay marketplace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Seller {
    pub avatar_url: String,
    pub name: String,
    pub rating: f64,
    pub online: OnlineStatus,
    pub reviews_count: u32,
    pub response_time: Option<String>,
    pub user_id: UserId,
}

/// A chat conversation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chat {
    pub last_message_date: Option<String>,
    pub chat_id: ChatId,
    pub user: Seller,
    pub chat_type: ChatType,
    pub unread_count: u32,
    pub last_message: Option<String>,
}

/// A single chat message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub message_id: MessageId,
    pub sender: UserId,
    pub text: String,
    pub date: Option<String>,
    pub is_self: bool,
}

/// A completed order.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub price: Price,
    pub currency: String,
    pub buyer: Seller,
    pub game: Option<String>,
    pub order_id: String,
    pub status: OrderStatus,
    pub date: Option<String>,
    pub seller: Seller,
}

/// A user profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub online_status: OnlineStatus,
    pub id: UserId,
    pub username: String,
    pub avatar_url: Option<String>,
    pub status: Option<String>,
    pub registration_date: Option<String>,
}

/// A game category with subcategories.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub url: String,
    pub game_id: Option<GameId>,
    pub title: String,
    pub id: CategoryId,
    pub offers_count: Option<u32>,
}

/// A marketplace offer (lot listing).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Offer {
    pub image_url: Option<String>,
    pub currency: String,
    pub sale_type: LotSaleType,
    pub seller_id: UserId,
    pub id: OfferId,
    pub description: Option<String>,
    pub item_count: Option<u32>,
    pub price: Price,
    pub server: Option<String>,
}

/// A seller review.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    pub author: Seller,
    pub order_link: Option<String>,
    pub review_id: ReviewId,
    pub text: Option<String>,
    pub date: Option<String>,
    pub rating: f64,
}

/// A subcategory within a game category.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameCategory {
    pub id: CategoryId,
    pub name: String,
    pub url: String,
}

/// A game server with offer counts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameServer {
    pub name: String,
    pub id: ServerId,
    pub game_id: GameId,
    pub offers_count: u32,
    pub platform: Option<String>,
}

/// A balance transaction (deposit, withdrawal, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub related_order_id: Option<String>,
    #[serde(rename = "type")]
    pub r#type: TransactionType,
    pub id: TransactionId,
    pub date: String,
    pub currency: String,
    pub amount: Price,
    pub balance_after: Option<Balance>,
    pub description: Option<String>,
}

/// User settings (notifications, language, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub timezone: String,
    pub notification_email: bool,
    pub notification_push: bool,
    pub language: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

/// A game in the FunPay catalog.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub id: GameId,
    pub title: String,
    pub url: String,
    pub icon_url: Option<String>,
}

/// A lot (purchasable item listing).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lot {
    pub description: Option<String>,
    pub id: LotId,
    pub server: Option<String>,
    pub price: Price,
    pub game: Option<String>,
    pub seller: Seller,
    pub currency: String,
}

/// A user's offer lot (their listed items for sale).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfferLot {
    pub server: Option<String>,
    pub offer_id: OfferId,
    pub price: Price,
    pub description: Option<String>,
}

/// An individual lot item (e.g., in-game item).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LotItem {
    pub name: String,
    pub image_url: Option<String>,
    pub id: LotId,
}

/// Search results from FunPay.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Search {
    pub offers: Vec<Offer>,
    pub results_count: u32,
    pub query: String,
    pub sort: SearchSort,
}

/// A subcategory within a game category.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubCategory {
    pub parent_id: CategoryId,
    pub id: CategoryId,
    pub name: String,
    pub url: String,
}

/// A notification (review, order update, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    #[serde(rename = "type")]
    pub r#type: NotificationType,
    pub id: NotificationId,
    pub title: String,
    pub text: Option<String>,
    pub date: Option<String>,
    pub is_read: bool,
    pub link: Option<String>,
}
