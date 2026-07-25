//! HTML parser for FunPay page responses.
//!
//! Converts raw HTML into typed [`models`] structs. Uses CSS selectors
//! via the [`scraper`] crate for robust parsing.
//!
//! # Examples
//!
//! ```rust
//! use funpay_sdk::parser::Parser;
//!
//! let parser = Parser::new();
//! let offers = parser.parse_category_offers(r#"
//!     <a class="tc-item">
//!         <div class="tc-price">25.50</div>
//!         <div class="tc-server">EU</div>
//!         <div class="tc-desc-text">1000 Gold</div>
//!     </a>
//! "#);
//! assert_eq!(offers.len(), 1);
//! ```

use scraper::{Html, Selector};
use crate::models::*;

fn extract_text(el: &scraper::ElementRef, selector: &str) -> Option<String> {
    let sel = Selector::parse(selector).ok()?;
    el.select(&sel).next()?.text().next().map(|s| s.trim().to_string())
}

fn extract_attr(el: &scraper::ElementRef, selector: &str, attr: &str) -> Option<String> {
    let sel = Selector::parse(selector).ok()?;
    el.select(&sel).next()?.value().attr(attr).map(|s| s.to_string())
}

fn extract_self_attr(el: &scraper::ElementRef, attr: &str) -> Option<String> {
    el.value().attr(attr).map(|s| s.to_string())
}

/// HTML parser for FunPay pages.
///
/// Each method accepts a raw HTML string and returns a `Vec` of parsed models
/// or an `Option<T>` for single-resource pages.
pub struct Parser;

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser {
    pub fn new() -> Self { Self }

    /// Parse order history from `/orders/` page.
    #[allow(clippy::unnecessary_filter_map)]
    pub fn parse_orders(&self, html: &str) -> Vec<Order> {
        let document = Html::parse_document(html);
        let selector = Selector::parse(".order-item").unwrap();
        document.select(&selector)
            .filter_map(|el| {
                let price = extract_text(&el, ".order-price");
                let currency = extract_text(&el, ".order-price .currency").unwrap_or_default();
                let buyer = Seller { rating: 0.0, name: String::new(), avatar_url: Default::default(), reviews_count: 0u32, online: Default::default(), response_time: None, user_id: UserId::new(0) };
                let game = extract_text(&el, ".order-game");
                let order_id = extract_self_attr(&el, "data-order-id").unwrap_or_default();
                let status = extract_text(&el, ".order-status");
                let date = extract_text(&el, ".order-date");
                let seller = Seller { rating: 0.0, name: String::new(), avatar_url: Default::default(), reviews_count: 0u32, online: Default::default(), response_time: None, user_id: UserId::new(0) };
                Some(Order {
                    price: price.and_then(|s| s.parse::<Price>().ok()).unwrap_or_default(),
                    currency,
                    buyer,
                    game,
                    order_id,
                    status: status.and_then(|s| match s.as_str() { "completed" => Some(OrderStatus::Completed), "disputed" => Some(OrderStatus::Disputed), "cancelled" => Some(OrderStatus::Cancelled), "pending" => Some(OrderStatus::Pending), "active" => Some(OrderStatus::Active), _ => None }).unwrap_or_default(),
                    date,
                    seller,
                })
            })
            .collect()
    }

    /// Parse subcategories from a category page.
    #[allow(clippy::unnecessary_filter_map)]
    pub fn parse_subcategories(&self, html: &str) -> Vec<SubCategory> {
        let document = Html::parse_document(html);
        let selector = Selector::parse(".subcategory-item").unwrap();
        document.select(&selector)
            .filter_map(|el| {
                let parent_id = extract_self_attr(&el, "data-parent-id");
                let id = extract_self_attr(&el, "data-category-id");
                let name = extract_text(&el, ".subcategory-item").unwrap_or_default();
                let url = extract_attr(&el, ".subcategory-item a", "href").unwrap_or_default();
                Some(SubCategory {
                    parent_id: parent_id.and_then(|s| s.parse::<CategoryId>().ok()).unwrap_or_default(),
                    id: id.and_then(|s| s.parse::<CategoryId>().ok()).unwrap_or_default(),
                    name,
                    url,
                })
            })
            .collect()
    }

    /// Parse transaction history from balance page.
    #[allow(clippy::unnecessary_filter_map)]
    pub fn parse_transactions(&self, html: &str) -> Vec<Transaction> {
        let document = Html::parse_document(html);
        let selector = Selector::parse(".transaction-item").unwrap();
        document.select(&selector)
            .filter_map(|el| {
                let related_order_id = extract_attr(&el, ".transaction-order-link", "data-order-id");
                let r#type = extract_self_attr(&el, "data-type");
                let id = extract_self_attr(&el, "data-transaction-id");
                let date = extract_text(&el, ".transaction-date").unwrap_or_default();
                let currency = extract_text(&el, ".transaction-amount .currency").unwrap_or_default();
                let amount = extract_text(&el, ".transaction-amount");
                let balance_after = extract_text(&el, ".transaction-balance");
                let description = extract_text(&el, ".transaction-desc");
                Some(Transaction {
                    related_order_id,
                    r#type: r#type.and_then(|s| match s.as_str() { "deposit" => Some(TransactionType::Deposit), "refund" => Some(TransactionType::Refund), "bonus" => Some(TransactionType::Bonus), "withdrawal" => Some(TransactionType::Withdrawal), "sale" => Some(TransactionType::Sale), "purchase" => Some(TransactionType::Purchase), _ => None }).unwrap_or_default(),
                    id: id.and_then(|s| s.parse::<TransactionId>().ok()).unwrap_or_default(),
                    date,
                    currency,
                    amount: amount.and_then(|s| s.parse::<Price>().ok()).unwrap_or_default(),
                    balance_after: balance_after.and_then(|s| s.parse::<Balance>().ok()),
                    description,
                })
            })
            .collect()
    }

    /// Parse chat list from `/chats/` page.
    #[allow(clippy::unnecessary_filter_map)]
    pub fn parse_chats(&self, html: &str) -> Vec<Chat> {
        let document = Html::parse_document(html);
        let selector = Selector::parse(".chat-item").unwrap();
        document.select(&selector)
            .filter_map(|el| {
                let last_message_date = extract_text(&el, ".chat-date");
                let chat_id = extract_self_attr(&el, "data-chat-id");
                let user = Seller { avatar_url: Default::default(), name: String::new(), rating: 0.0, online: Default::default(), reviews_count: 0u32, response_time: None, user_id: UserId::new(0) };
                let chat_type = extract_self_attr(&el, "data-chat-type");
                let unread_count = extract_text(&el, ".chat-unread");
                let last_message = extract_text(&el, ".chat-last-message");
                Some(Chat {
                    last_message_date,
                    chat_id: chat_id.and_then(|s| s.parse::<ChatId>().ok()).unwrap_or_default(),
                    user,
                    chat_type: chat_type.and_then(|s| match s.as_str() { "support" => Some(ChatType::Support), "user" => Some(ChatType::User), "order" => Some(ChatType::Order), _ => None }).unwrap_or_default(),
                    unread_count: unread_count.and_then(|s| s.parse::<u32>().ok()).unwrap_or_default(),
                    last_message,
                })
            })
            .collect()
    }

    /// Parse game catalog from `/lots/` page.
    #[allow(clippy::unnecessary_filter_map)]
    pub fn parse_game_list(&self, html: &str) -> Vec<Game> {
        let document = Html::parse_document(html);
        let selector = Selector::parse(".game-title").unwrap();
        document.select(&selector)
            .filter_map(|el| {
                let id = extract_attr(&el, ".game-title", "data-game-id");
                let title = extract_text(&el, ".game-title").unwrap_or_default();
                let url = extract_attr(&el, ".game-title", "href").unwrap_or_default();
                let icon_url = extract_attr(&el, ".game-icon", "src");
                Some(Game {
                    id: id.and_then(|s| s.parse::<GameId>().ok()).unwrap_or_default(),
                    title,
                    url,
                    icon_url,
                })
            })
            .collect()
    }

    /// Parse seller reviews from user profile page.
    #[allow(clippy::unnecessary_filter_map)]
    pub fn parse_reviews(&self, html: &str) -> Vec<Review> {
        let document = Html::parse_document(html);
        let selector = Selector::parse(".review-item").unwrap();
        document.select(&selector)
            .filter_map(|el| {
                let author = Seller { avatar_url: Default::default(), name: String::new(), rating: 0.0, online: Default::default(), reviews_count: 0u32, response_time: None, user_id: UserId::new(0) };
                let order_link = extract_attr(&el, ".review-order-link", "href");
                let review_id = extract_self_attr(&el, "data-review-id");
                let text = extract_text(&el, ".review-text");
                let date = extract_text(&el, ".review-date");
                let rating = extract_text(&el, ".review-rating");
                Some(Review {
                    author,
                    order_link,
                    review_id: review_id.and_then(|s| s.parse::<ReviewId>().ok()).unwrap_or_default(),
                    text,
                    date,
                    rating: rating.and_then(|s| s.parse::<f64>().ok()).unwrap_or_default(),
                })
            })
            .collect()
    }

    /// Parse user offer lots from profile page.
    #[allow(clippy::unnecessary_filter_map)]
    pub fn parse_user_offers(&self, html: &str) -> Vec<OfferLot> {
        let document = Html::parse_document(html);
        let selector = Selector::parse(".tc-item").unwrap();
        document.select(&selector)
            .filter_map(|el| {
                let server = extract_text(&el, ".tc-server");
                let offer_id = extract_self_attr(&el, "data-order");
                let price = extract_text(&el, ".tc-price");
                let description = extract_text(&el, ".tc-desc-text");
                Some(OfferLot {
                    server,
                    offer_id: offer_id.and_then(|s| s.parse::<OfferId>().ok()).unwrap_or_default(),
                    price: price.and_then(|s| s.parse::<Price>().ok()).unwrap_or_default(),
                    description,
                })
            })
            .collect()
    }

    /// Parse game servers from server list page.
    #[allow(clippy::unnecessary_filter_map)]
    pub fn parse_game_servers(&self, html: &str) -> Vec<GameServer> {
        let document = Html::parse_document(html);
        let selector = Selector::parse(".server-item").unwrap();
        document.select(&selector)
            .filter_map(|el| {
                let name = extract_text(&el, ".server-item .server-name").unwrap_or_default();
                let id = extract_self_attr(&el, "data-server-id");
                let game_id = extract_self_attr(&el, "data-game-id");
                let offers_count = extract_text(&el, ".server-item .server-count");
                let platform = extract_text(&el, ".server-item .server-platform");
                Some(GameServer {
                    name,
                    id: id.and_then(|s| s.parse::<ServerId>().ok()).unwrap_or_default(),
                    game_id: game_id.and_then(|s| s.parse::<GameId>().ok()).unwrap_or_default(),
                    offers_count: offers_count.and_then(|s| s.parse::<u32>().ok()).unwrap_or_default(),
                    platform,
                })
            })
            .collect()
    }

    /// Parse search results page.
    #[allow(clippy::unnecessary_filter_map)]
    pub fn parse_search(&self, html: &str) -> Vec<Search> {
        let document = Html::parse_document(html);
        let selector = Selector::parse(".tc-item").unwrap();
        document.select(&selector)
            .filter_map(|el| {
                let offers = Vec::new();
                let results_count = extract_text(&el, ".search-results-count");
                let query = extract_text(&el, ".search-input").unwrap_or_default();
                let sort = extract_text(&el, ".search-sort");
                Some(Search {
                    offers,
                    results_count: results_count.and_then(|s| s.parse::<u32>().ok()).unwrap_or_default(),
                    query,
                    sort: sort.and_then(|s| match s.as_str() { "relevance" => Some(SearchSort::Relevance), "price_desc" => Some(SearchSort::PriceDesc), "price_asc" => Some(SearchSort::PriceAsc), "rating" => Some(SearchSort::Rating), "date" => Some(SearchSort::Date), _ => None }).unwrap_or_default(),
                })
            })
            .collect()
    }

    /// Parse category offers from `/lots/{game}/{category}/` page.
    #[allow(clippy::unnecessary_filter_map)]
    pub fn parse_category_offers(&self, html: &str) -> Vec<Offer> {
        let document = Html::parse_document(html);
        let selector = Selector::parse(".tc-item").unwrap();
        document.select(&selector)
            .filter_map(|el| {
                let image_url = extract_attr(&el, ".tc-item img", "src");
                let currency = extract_text(&el, ".tc-price .currency").unwrap_or_default();
                let sale_type = extract_self_attr(&el, "data-mark");
                let seller_id = extract_self_attr(&el, "data-user-id");
                let id = extract_self_attr(&el, "data-order");
                let description = extract_text(&el, ".tc-desc-text");
                let item_count = extract_self_attr(&el, "data-lot-size");
                let price = extract_text(&el, ".tc-price");
                let server = extract_text(&el, ".tc-server");
                Some(Offer {
                    image_url,
                    currency,
                    sale_type: sale_type.and_then(|s| match s.as_str() { "bulk" => Some(LotSaleType::Bulk), "single" => Some(LotSaleType::Single), _ => None }).unwrap_or_default(),
                    seller_id: seller_id.and_then(|s| s.parse::<UserId>().ok()).unwrap_or_default(),
                    id: id.and_then(|s| s.parse::<OfferId>().ok()).unwrap_or_default(),
                    description,
                    item_count: item_count.and_then(|s| s.parse::<u32>().ok()),
                    price: price.and_then(|s| s.parse::<Price>().ok()).unwrap_or_default(),
                    server,
                })
            })
            .collect()
    }

    /// Parse chat messages from `/chats/{id}/` page.
    #[allow(clippy::unnecessary_filter_map)]
    pub fn parse_chat_messages(&self, html: &str) -> Vec<ChatMessage> {
        let document = Html::parse_document(html);
        let selector = Selector::parse(".msg").unwrap();
        document.select(&selector)
            .filter_map(|el| {
                let message_id = extract_self_attr(&el, "data-msg-id");
                let sender = extract_self_attr(&el, "data-sender-id");
                let text = extract_text(&el, ".msg-text").unwrap_or_default();
                let date = extract_text(&el, ".msg-date");
                let is_self = extract_self_attr(&el, "data-self");
                Some(ChatMessage {
                    message_id: message_id.and_then(|s| s.parse::<MessageId>().ok()).unwrap_or_default(),
                    sender: sender.and_then(|s| s.parse::<UserId>().ok()).unwrap_or_default(),
                    text,
                    date,
                    is_self: is_self.and_then(|s| s.parse::<bool>().ok()).unwrap_or_default(),
                })
            })
            .collect()
    }

    /// Parse notifications from notification page.
    #[allow(clippy::unnecessary_filter_map)]
    pub fn parse_notifications(&self, html: &str) -> Vec<Notification> {
        let document = Html::parse_document(html);
        let selector = Selector::parse(".notification-item").unwrap();
        document.select(&selector)
            .filter_map(|el| {
                let r#type = extract_self_attr(&el, "data-type");
                let id = extract_self_attr(&el, "data-notification-id");
                let title = extract_text(&el, ".notification-title").unwrap_or_default();
                let text = extract_text(&el, ".notification-text");
                let date = extract_text(&el, ".notification-date");
                let is_read = extract_self_attr(&el, "data-read");
                let link = extract_attr(&el, ".notification-link", "href");
                Some(Notification {
                    r#type: r#type.and_then(|s| match s.as_str() { "review" => Some(NotificationType::Review), "order" => Some(NotificationType::Order), "system" => Some(NotificationType::System), "message" => Some(NotificationType::Message), "payment" => Some(NotificationType::Payment), _ => None }).unwrap_or_default(),
                    id: id.and_then(|s| s.parse::<NotificationId>().ok()).unwrap_or_default(),
                    title,
                    text,
                    date,
                    is_read: is_read.and_then(|s| s.parse::<bool>().ok()).unwrap_or_default(),
                    link,
                })
            })
            .collect()
    }

    /// Parse seller profile from `/users/{id}/` page.
    pub fn parse_seller_profile(&self, html: &str) -> Option<Seller> {
        let document = Html::parse_document(html);
        let el = document.root_element();
        let avatar_url = extract_attr(&el, ".seller-avatar img", "src").unwrap_or_default();
        let name = extract_text(&el, ".seller-name").unwrap_or_default();
        let rating = extract_text(&el, ".seller-rating");
        let online = extract_text(&el, ".seller-online");
        let reviews_count = extract_text(&el, ".seller-reviews");
        let response_time = extract_text(&el, ".seller-response-time");
        let user_id = extract_attr(&el, ".seller-info", "data-user-id");
        Some(Seller {
            avatar_url,
            name,
            rating: rating.and_then(|s| s.parse::<f64>().ok()).unwrap_or_default(),
            online: online.and_then(|s| match s.as_str() { "online" => Some(OnlineStatus::Online), "offline" => Some(OnlineStatus::Offline), _ => None }).unwrap_or_default(),
            reviews_count: reviews_count.and_then(|s| s.parse::<u32>().ok()).unwrap_or_default(),
            response_time,
            user_id: user_id.and_then(|s| s.parse::<UserId>().ok()).unwrap_or_default(),
        })
    }

    /// Parse user profile from `/users/{id}/` page.
    pub fn parse_user_profile(&self, html: &str) -> Option<User> {
        let document = Html::parse_document(html);
        let el = document.root_element();
        let online_status = extract_text(&el, ".profile-online");
        let id = extract_attr(&el, ".profile-user-id", "data-user-id");
        let username = extract_text(&el, ".profile-title").unwrap_or_default();
        let avatar_url = extract_attr(&el, ".profile-avatar img", "src");
        let status = extract_text(&el, ".user-status");
        let registration_date = extract_text(&el, ".profile-regdate");
        Some(User {
            online_status: online_status.and_then(|s| match s.as_str() { "online" => Some(OnlineStatus::Online), "offline" => Some(OnlineStatus::Offline), _ => None }).unwrap_or_default(),
            id: id.and_then(|s| s.parse::<UserId>().ok()).unwrap_or_default(),
            username,
            avatar_url,
            status,
            registration_date,
        })
    }

    /// Parse settings from settings page.
    pub fn parse_settings(&self, html: &str) -> Option<Settings> {
        let document = Html::parse_document(html);
        let el = document.root_element();
        let timezone = extract_text(&el, ".settings-timezone").unwrap_or_default();
        let notification_email = extract_text(&el, ".settings-notify-email");
        let notification_push = extract_text(&el, ".settings-notify-push");
        let language = extract_text(&el, ".settings-language").unwrap_or_default();
        let email = extract_text(&el, ".settings-email");
        let phone = extract_text(&el, ".settings-phone");
        Some(Settings {
            timezone,
            notification_email: notification_email.and_then(|s| s.parse::<bool>().ok()).unwrap_or_default(),
            notification_push: notification_push.and_then(|s| s.parse::<bool>().ok()).unwrap_or_default(),
            language,
            email,
            phone,
        })
    }

}
