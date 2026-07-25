use scraper::{Html, Selector};
use std::collections::HashMap;
use crate::error::ParseError;
use crate::models::{
    User, Offer, Game, GameCategory, Currency, OfferId, LotId, UserId, GameId, Server, Order, Chat,
    ChatMessage, Review,
};

/// HTML parser for FunPay pages.
///
/// Provides methods to extract structured data from FunPay HTML responses.
pub struct Parser;

impl Parser {
    /// Creates a new parser instance.
    pub fn new() -> Self {
        Self
    }

    fn extract_text(&self, document: &Html, selector: &str) -> Option<String> {
        let sel = Selector::parse(selector).ok()?;
        let element = document.select(&sel).next()?;
        element.text().next().map(|s| s.trim().to_string())
    }

    /// Extracts all offer IDs from an HTML page.
    pub fn extract_offer_ids(&self, html: &str) -> Vec<OfferId> {
        let document = Html::parse_document(html);
        let selector = Selector::parse("[data-offer-id]").unwrap();
        document
            .select(&selector)
            .filter_map(|el| {
                el.value()
                    .attr("data-offer-id")
                    .map(|s| OfferId(s.to_string()))
            })
            .collect()
    }

    /// Extracts the price from a single offer HTML snippet.
    pub fn extract_price(&self, html: &str) -> Option<f64> {
        let document = Html::parse_document(html);
        let selector = Selector::parse(".tc-price").ok()?;
        let element = document.select(&selector).next()?;
        let text = element.text().next()?;
        let cleaned: String = text
            .chars()
            .filter(|c| c.is_ascii_digit() || *c == '.')
            .collect();
        cleaned.parse().ok()
    }

    /// Extracts the seller name from an HTML snippet.
    pub fn extract_seller_name(&self, html: &str) -> Option<String> {
        let document = Html::parse_document(html);
        let selector = Selector::parse(".media-user-name").ok()?;
        let element = document.select(&selector).next()?;
        element.text().next().map(|s| s.to_string())
    }

    /// Parses a user profile page into a [`User`] model.
    ///
    /// Returns `Err` if the HTML cannot be parsed as a valid user profile.
    pub fn parse_user(&self, html: &str, user_id: UserId) -> Result<User, ParseError> {
        let document = Html::parse_document(html);
        Ok(User {
            user_id,
            username: self
                .extract_text(&document, ".profile-name")
                .unwrap_or_default(),
            rating: self
                .extract_text(&document, ".rating")
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.0),
            reviews: self
                .extract_text(&document, ".reviews")
                .and_then(|s| s.parse().ok())
                .unwrap_or(0),
            online: self
                .extract_text(&document, ".online-status")
                .map(|s| s == "online")
                .unwrap_or(false),
            registered: self
                .extract_text(&document, ".reg-date")
                .and_then(|s| chrono::NaiveDate::parse_from_str(&s, "%d.%m.%Y").ok()),
        })
    }

    /// Parses a single offer page into an [`Offer`] model.
    ///
    /// Returns `Err` if the HTML cannot be parsed as a valid offer.
    pub fn parse_offer(&self, html: &str, offer_id: OfferId) -> Result<Offer, ParseError> {
        let document = Html::parse_document(html);
        Ok(Offer {
            offer_id,
            lot_id: LotId(String::new()),
            server: Server(
                self.extract_text(&document, ".tc-server")
                    .unwrap_or_default(),
            ),
            description: self
                .extract_text(&document, ".tc-desc")
                .unwrap_or_default(),
            price: self.extract_price(html).unwrap_or(0.0),
            currency: Currency::RUB,
            stock: self.extract_stock(html).unwrap_or(0),
            seller: crate::models::Seller {
                user_id: UserId(String::new()),
                name: self.extract_seller_name(html).unwrap_or_default(),
                rating: 0.0,
                reviews: 0,
                online: false,
            },
            fields: HashMap::new(),
        })
    }

    fn extract_stock(&self, html: &str) -> Option<u32> {
        let document = Html::parse_document(html);
        self.extract_text(&document, ".tc-qty")?.parse().ok()
    }

    /// Parses a game listing page into a list of [`Game`] models.
    pub fn parse_game_list(&self, html: &str) -> Vec<Game> {
        let document = Html::parse_document(html);
        let selector = Selector::parse("a[href*='/chips/'], a[href*='/lots/']").unwrap();
        document
            .select(&selector)
            .filter_map(|el| {
                let href = el.value().attr("href")?;
                let name = el.text().next()?.to_string();
                let base = "https://funpay.com";
                let full_url = if href.starts_with("http") {
                    href.to_string()
                } else {
                    format!("{}{}", base, href)
                };
                let parsed_url = url::Url::parse(&full_url).ok();
                Some(Game {
                    id: GameId(href.split('/').nth(2)?.to_string()),
                    name,
                    chips_url: if href.contains("/chips/") {
                        parsed_url.clone()
                    } else {
                        None
                    },
                    lots_url: if href.contains("/lots/") {
                        parsed_url
                    } else {
                        None
                    },
                    category: GameCategory::from_url(href),
                })
            })
            .collect()
    }

    /// Parses an offers listing page into a list of [`Offer`] models.
    pub fn parse_offers_from_page(&self, html: &str) -> Vec<Offer> {
        let document = Html::parse_document(html);
        let selector = Selector::parse("a.tc-item").unwrap();
        document
            .select(&selector)
            .filter_map(|el| {
                let href = el.value().attr("href")?;
                let offer_id = OfferId(href.split("id=").last()?.to_string());
                let online = el.value().attr("data-online") == Some("1");

                let price = el
                    .select(&Selector::parse(".tc-price div").ok()?)
                    .next()?
                    .text()
                    .next()?
                    .chars()
                    .filter(|c| c.is_ascii_digit() || *c == '.')
                    .collect::<String>()
                    .parse()
                    .ok()?;

                let currency = el
                    .select(&Selector::parse(".tc-price .unit").ok()?)
                    .next()
                    .and_then(|e| e.text().next())
                    .map(|t| t.trim())
                    .map(Currency::from_symbol)
                    .unwrap_or(Currency::RUB);

                let stock: u32 = el
                    .select(&Selector::parse(".tc-amount").ok()?)
                    .next()
                    .and_then(|e| e.value().attr("data-s"))
                    .map(|s| s.replace(' ', ""))
                    .unwrap_or_default()
                    .parse()
                    .unwrap_or(1);

                let seller_name = el
                    .select(&Selector::parse(".media-user-name").ok()?)
                    .next()?
                    .text()
                    .next()?
                    .trim()
                    .to_string();

                let reviews: u32 = el
                    .select(&Selector::parse(".rating-mini-count").ok()?)
                    .next()
                    .and_then(|e| e.text().next())
                    .map(|t| t.trim().replace(' ', ""))
                    .unwrap_or_default()
                    .parse()
                    .unwrap_or(0);

                let server_name = Server(
                    el.select(&Selector::parse(".tc-server").ok()?)
                        .next()
                        .map(|e| e.text().collect::<String>())
                        .unwrap_or_default(),
                );

                let side = el
                    .select(&Selector::parse(".tc-side").ok()?)
                    .next()
                    .map(|e| e.text().collect::<String>())
                    .unwrap_or_default();

                let desc_text = el
                    .select(&Selector::parse(".tc-desc-text").ok()?)
                    .next()
                    .map(|e| e.text().collect::<String>())
                    .unwrap_or_default();

                let description = if !desc_text.is_empty() {
                    desc_text
                } else if side.is_empty() {
                    server_name.to_string()
                } else {
                    format!("{} / {}", server_name, side)
                };

                // NOTE: lot_id is not available from listing page HTML.
                // FunPay listings only expose offer_id via ?id= param.
                Some(Offer {
                    offer_id,
                    lot_id: LotId(String::new()),
                    server: server_name,
                    description,
                    price,
                    currency,
                    stock,
                    seller: crate::models::Seller {
                        user_id: UserId(String::new()),
                        name: seller_name,
                        rating: 0.0,
                        reviews,
                        online,
                    },
                    fields: std::collections::HashMap::new(),
                })
            })
            .collect()
    }

    /// Parses an orders listing page. Currently returns an empty list (stub).
    pub fn parse_orders_from_page(&self, _html: &str) -> Vec<Order> {
        Vec::new()
    }

    /// Parses a chats listing page. Currently returns an empty list (stub).
    pub fn parse_chats_from_page(&self, _html: &str) -> Vec<Chat> {
        Vec::new()
    }

    /// Parses chat messages from a chat page. Currently returns an empty list (stub).
    pub fn parse_chat_messages(&self, _html: &str) -> Vec<ChatMessage> {
        Vec::new()
    }

    /// Parses reviews from a user profile page. Currently returns an empty list (stub).
    pub fn parse_reviews_from_page(&self, _html: &str) -> Vec<Review> {
        Vec::new()
    }
}
