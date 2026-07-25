use scraper::{Html, Selector};
use std::collections::HashMap;
use std::str::FromStr;
use rust_decimal::Decimal;
use crate::error::ParseError;
use crate::models::{
    User, Offer, Game, GameCategory, Currency, OfferId, LotId, UserId, GameId, Server, Order,
    OrderStatus, Chat, ChatMessage, Review,
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
    pub fn extract_price(&self, html: &str) -> Option<Decimal> {
        let document = Html::parse_document(html);
        let selector = Selector::parse(".tc-price").ok()?;
        let element = document.select(&selector).next()?;
        let text = element.text().next()?;
        let cleaned: String = text
            .chars()
            .filter(|c| c.is_ascii_digit() || *c == '.')
            .collect();
        Decimal::from_str(&cleaned).ok()
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
            price: self.extract_price(html).unwrap_or_default(),
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

    /// Parses an orders listing page into a list of [`Order`] models.
    ///
    /// Works with both purchases (`/orders/`) and sales (`/orders/trade`) pages.
    pub fn parse_orders_from_page(&self, html: &str) -> Vec<Order> {
        let document = Html::parse_document(html);
        let selector = Selector::parse("a.tc-item").unwrap();

        document
            .select(&selector)
            .filter_map(|el| {
                let href = el.value().attr("href")?;
                let order_id = href
                    .split('/')
                    .filter(|s| !s.is_empty())
                    .last()?
                    .to_string();

                let status_class = el
                    .select(&Selector::parse("div.tc-status").ok()?)
                    .next()?
                    .value()
                    .attr("class")?
                    .to_string();

                let status = parse_order_status(&status_class);

                let price_text = el
                    .select(&Selector::parse("div.tc-price").ok()?)
                    .next()?
                    .text()
                    .collect::<String>();
                let price = parse_price_from_text(&price_text);
                let currency = detect_currency_from_text(&price_text);

                let date_text = el
                    .select(&Selector::parse("div.tc-date-time").ok()?)
                    .next()
                    .map(|e| e.text().collect::<String>())
                    .unwrap_or_default();

                let _title = el
                    .select(&Selector::parse("div.order-desc > div").ok()?)
                    .next()
                    .map(|e| e.text().collect::<String>())
                    .unwrap_or_default();

                let _category_text = el
                    .select(&Selector::parse("div.text-muted").ok()?)
                    .next()
                    .map(|e| e.text().collect::<String>())
                    .unwrap_or_default();

                let counterparty_name = el
                    .select(&Selector::parse("div.media-user-name").ok()?)
                    .next()
                    .map(|e| e.text().collect::<String>())
                    .unwrap_or_default();

                let online = el
                    .select(&Selector::parse("div.media-user").ok()?)
                    .next()
                    .map(|e| {
                        e.value()
                            .attr("class")
                            .map(|c| c.contains("online"))
                            .unwrap_or(false)
                    })
                    .unwrap_or(false);

                let created_at = parse_datetime_from_text(&date_text);

                Some(Order {
                    order_id,
                    offer_id: OfferId(String::new()),
                    seller: crate::models::Seller {
                        user_id: UserId(String::new()),
                        name: counterparty_name.clone(),
                        rating: 0.0,
                        reviews: 0,
                        online,
                    },
                    buyer: crate::models::Seller {
                        user_id: UserId(String::new()),
                        name: String::new(),
                        rating: 0.0,
                        reviews: 0,
                        online: false,
                    },
                    price,
                    currency,
                    status,
                    created_at,
                    completed_at: None,
                })
            })
            .collect()
    }

    /// Parses a chats listing page. Currently returns an empty list (stub).
    pub fn parse_chats_from_page(&self, _html: &str) -> Vec<Chat> {
        Vec::new()
    }

    /// Parses chat messages from a chat page. Currently returns an empty list (stub).
    pub fn parse_chat_messages(&self, _html: &str) -> Vec<ChatMessage> {
        Vec::new()
    }

    /// Parses reviews from a user profile page into a list of [`Review`] models.
    pub fn parse_reviews_from_page(&self, html: &str) -> Vec<Review> {
        let document = Html::parse_document(html);
        let selector = Selector::parse("div.review-container").unwrap();

        document
            .select(&selector)
            .filter_map(|review_div| {
                let date_sel = Selector::parse("div.review-item-date").ok()?;
                let date_text = review_div.select(&date_sel).next()?.text().collect::<String>();

                let text_sel = Selector::parse("div.review-item-text").ok()?;
                let text = review_div
                    .select(&text_sel)
                    .next()
                    .map(|e| e.text().collect::<String>())
                    .unwrap_or_default();

                let mut rating: f64 = 0.0;
                for i in 1..=5 {
                    let sel_str = format!("div.rating{}", i);
                    if let Ok(sel) = Selector::parse(&sel_str) {
                        if review_div.select(&sel).next().is_some() {
                            rating = i as f64;
                            break;
                        }
                    };
                }

                let order_sel = Selector::parse("div.review-item-order").ok()?;
                let order_id_text = review_div
                    .select(&order_sel)
                    .next()
                    .map(|e| e.text().collect::<String>())
                    .unwrap_or_default();
                let order_id = order_id_text
                    .split('#')
                    .last()
                    .map(|s| s.trim().to_string());

                let name_sel = Selector::parse("div.review-item-user div.media-user-name").ok()?;
                let reviewer_name = review_div.select(&name_sel).next()?.text().collect::<String>();

                let link_sel = Selector::parse("div.review-item-user a").ok()?;
                let link_el = review_div.select(&link_sel).next()?;
                let href = link_el.value().attr("href")?;
                let reviewer_id = href
                    .split('/')
                    .filter(|s| !s.is_empty())
                    .nth(1)?
                    .to_string();

                let created_at = parse_datetime_from_text(&date_text);

                Some(Review {
                    review_id: order_id.clone().unwrap_or_default(),
                    reviewer: crate::models::Seller {
                        user_id: UserId(reviewer_id),
                        name: reviewer_name,
                        rating: 0.0,
                        reviews: 0,
                        online: false,
                    },
                    rating,
                    text: if text.is_empty() { None } else { Some(text) },
                    created_at,
                    order_id,
                })
            })
            .collect()
    }

    /// Extracts the next page URL from a listing page.
    ///
    /// Checks for both `input[name="continue"]` (used on orders/reviews pages)
    /// and standard pagination links (used on lot/chips category pages).
    pub fn extract_next_page_url(html: &str) -> Option<String> {
        let document = Html::parse_document(html);

        // Check for hidden continue input (orders, reviews pages)
        let continue_input =
            Selector::parse(r#"input[type="hidden"][name="continue"]"#).ok()?;
        if let Some(input) = document.select(&continue_input).next() {
            if let Some(value) = input.value().attr("value") {
                if !value.is_empty() {
                    return Some(value.to_string());
                }
            }
        }

        // Check for pagination links (lot/chips category pages)
        // FunPay uses <a> elements with class "btn" inside a pagination wrapper,
        // or links containing ?page=N. Look for the "next" link.
        let next_link_selectors = &[
            "a.paging-next",
            "a.next",
            r#"a[rel="next"]"#,
            "li.next a",
        ];
        for sel_str in next_link_selectors {
            if let Ok(sel) = Selector::parse(sel_str) {
                if let Some(el) = document.select(&sel).next() {
                    if let Some(href) = el.value().attr("href") {
                        if !href.is_empty() {
                            let url = if href.starts_with("http") {
                                href.to_string()
                            } else {
                                format!("https://funpay.com{}", href)
                            };
                            return Some(url);
                        }
                    }
                }
            }
        }

        // Fallback: find any link with ?page= parameter that comes after current page indicators
        let page_link_sel = Selector::parse(r#"a[href*="page="]"#).ok()?;
        let paging_sel = Selector::parse(".paging, .pagination, .page-nav").ok();
        if let Some(paging_el) = paging_sel.and_then(|sel| document.select(&sel).next()) {
            for el in paging_el.select(&page_link_sel) {
                if let Some(href) = el.value().attr("href") {
                    let url = if href.starts_with("http") {
                        href.to_string()
                    } else {
                        format!("https://funpay.com{}", href)
                    };
                    // Check if this link looks like a "next" or higher page number
                    if href.contains("page=") {
                        return Some(url);
                    }
                }
            }
        }

        None
    }

    /// Parses game subcategories from a game page.
    ///
    /// Returns a list of `(name, url)` tuples for each subcategory.
    pub fn parse_subcategories(html: &str) -> Vec<(String, String)> {
        let document = Html::parse_document(html);

        // Counter items on game pages (chips/lots/other subcategories)
        let counter_selector = Selector::parse("a.counter-item").unwrap();
        let mut results: Vec<(String, String)> = document
            .select(&counter_selector)
            .filter_map(|el| {
                let href = el.value().attr("href")?;
                let param_sel = Selector::parse("div.counter-param").ok()?;
                let name = el.select(&param_sel).next()?.text().collect::<String>();
                let base = "https://funpay.com";
                let full_url = if href.starts_with("http") {
                    href.to_string()
                } else {
                    format!("{}{}", base, href)
                };
                Some((name, full_url))
            })
            .collect();

        // Also check for inline subcategory links (e.g. on lots pages with RU/EU/Free tabs)
        let inline_selector =
            Selector::parse("ul.list-inline.text-bold li a, ul.list-inline li a").unwrap();
        for el in document.select(&inline_selector) {
            let href = match el.value().attr("href") {
                Some(h) => h,
                None => continue,
            };
            if !href.contains("/chips/") && !href.contains("/lots/") {
                continue;
            }
            let name = el.text().collect::<String>();
            if name.is_empty() {
                continue;
            }
            let base = "https://funpay.com";
            let full_url = if href.starts_with("http") {
                href.to_string()
            } else {
                format!("{}{}", base, href)
            };
            // Avoid duplicates
            if !results.iter().any(|(_, url)| url == &full_url) {
                results.push((name, full_url));
            }
        }

        results
    }
}

/// Parses an order status from a CSS class string.
fn parse_order_status(class: &str) -> OrderStatus {
    if class.contains("status-success") || class.contains("tc-status-1") {
        OrderStatus::Completed
    } else if class.contains("status-cancel") || class.contains("tc-status-2") {
        OrderStatus::Cancelled
    } else if class.contains("status-dispute") || class.contains("tc-status-3") {
        OrderStatus::Disputed
    } else if class.contains("status-active") || class.contains("tc-status-0") {
        OrderStatus::Active
    } else if class.contains("status-pending") {
        OrderStatus::Pending
    } else {
        OrderStatus::Unknown(class.to_string())
    }
}

/// Parses a price from text containing digits and dots.
fn parse_price_from_text(text: &str) -> Decimal {
    let cleaned: String = text
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == '.')
        .collect();
    Decimal::from_str(&cleaned).unwrap_or_default()
}

/// Detects currency from text containing currency symbols.
fn detect_currency_from_text(text: &str) -> Currency {
    if text.contains('$') {
        Currency::USD
    } else if text.contains('€') {
        Currency::EUR
    } else {
        Currency::RUB
    }
}

/// Parses a datetime from FunPay text format.
fn parse_datetime_from_text(text: &str) -> Option<chrono::NaiveDateTime> {
    let text = text.trim();
    // Try common formats: "22.07.26 10:22" or "22 July, 10:22:21"
    for fmt in &[
        "%d.%m.%y %H:%M",
        "%d.%m.%Y %H:%M",
        "%d %B, %H:%M:%S",
        "%d %B %Y, %H:%M:%S",
    ] {
        if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(text, fmt) {
            return Some(dt);
        }
    }
    None
}
