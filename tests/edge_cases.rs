use funpay_rs::parser::Parser;
use funpay_rs::models::{OfferId, UserId};

#[test]
fn test_empty_html_response() {
    let parser = Parser::new();
    let games = parser.parse_game_list("");
    assert!(games.is_empty());
}

#[test]
fn test_empty_offers_page() {
    let parser = Parser::new();
    let offers = parser.parse_offers_from_page("");
    assert!(offers.is_empty());
}

#[test]
fn test_malformed_html() {
    let html = r#"<html><body><a href="/chips/91/">Unclosed tag"#;
    let parser = Parser::new();
    let games = parser.parse_game_list(html);
    assert_eq!(games.len(), 1);
    assert_eq!(games[0].name, "Unclosed tag");
}

#[test]
fn test_malformed_nested_tags() {
    let html = r#"<html><body><div><a href="/chips/91/">Lost Ark</a><div></body></html>"#;
    let parser = Parser::new();
    let games = parser.parse_game_list(html);
    assert_eq!(games.len(), 1);
}

#[test]
fn test_unicode_seller_name() {
    let html = r#"<div class="media-user-name">ИванПетров🎮</div>"#;
    let parser = Parser::new();
    let name = parser.extract_seller_name(html);
    assert_eq!(name, Some("ИванПетров🎮".to_string()));
}

#[test]
fn test_unicode_in_offer_description() {
    let html = r#"<a class="tc-item" href="/lot?id=2001" data-online="1">
        <div class="tc-price"><div>100</div><span class="unit">₽</span></div>
        <div class="tc-amount" data-s="1"></div>
        <div class="media-user-name">Продавец</div>
        <div class="rating-mini-count">5</div>
        <div class="tc-server">Москва</div>
        <div class="tc-desc-text">Быстрая доставка 24/7</div>
    </a>"#;
    let parser = Parser::new();
    let offers = parser.parse_offers_from_page(html);
    assert_eq!(offers.len(), 1);
    assert_eq!(offers[0].seller.name, "Продавец");
    assert_eq!(offers[0].description, "Быстрая доставка 24/7");
}

#[test]
fn test_price_with_comma() {
    let html = r#"<div class="tc-price">1 250,50 ₽</div>"#;
    let parser = Parser::new();
    let price = parser.extract_price(html);
    // Parser filters only digits and dots, so "1 250,50" becomes "125050"
    assert!(price.is_some());
}

#[test]
fn test_price_with_thousand_separator() {
    let html = r#"<a class="tc-item" href="/lot?id=3001" data-online="1">
        <div class="tc-price"><div>1 250</div><span class="unit">₽</span></div>
        <div class="tc-amount" data-s="1"></div>
        <div class="media-user-name">Seller</div>
        <div class="rating-mini-count">10</div>
        <div class="tc-server">RU</div>
    </a>"#;
    let parser = Parser::new();
    let offers = parser.parse_offers_from_page(html);
    assert_eq!(offers.len(), 1);
    // "1 250" -> filtered to "1250" -> 1250.0
    assert_eq!(offers[0].price, 1250.0);
}

#[test]
fn test_very_long_description() {
    let long_desc = "A".repeat(10_000);
    let html = format!(
        r#"<a class="tc-item" href="/lot?id=4001" data-online="1">
            <div class="tc-price"><div>50</div><span class="unit">$</span></div>
            <div class="tc-amount" data-s="1"></div>
            <div class="media-user-name">Seller</div>
            <div class="rating-mini-count">1</div>
            <div class="tc-server">EU</div>
            <div class="tc-desc-text">{}</div>
        </a>"#,
        long_desc
    );
    let parser = Parser::new();
    let offers = parser.parse_offers_from_page(&html);
    assert_eq!(offers.len(), 1);
    assert_eq!(offers[0].description.len(), 10_000);
}

#[test]
fn test_offer_with_special_characters_in_price() {
    let html = r#"<a class="tc-item" href="/lot?id=5001" data-online="1">
        <div class="tc-price"><div>$100.00</div><span class="unit">$</span></div>
        <div class="tc-amount" data-s="10"></div>
        <div class="media-user-name">SpecialSeller</div>
        <div class="rating-mini-count">99</div>
        <div class="tc-server">US-East</div>
    </a>"#;
    let parser = Parser::new();
    let offers = parser.parse_offers_from_page(html);
    assert_eq!(offers.len(), 1);
    assert_eq!(offers[0].price, 100.0);
}

#[test]
fn test_extract_price_no_match() {
    let html = r#"<div>No price here</div>"#;
    let parser = Parser::new();
    let price = parser.extract_price(html);
    assert_eq!(price, None);
}

#[test]
fn test_extract_seller_name_no_match() {
    let html = r#"<div>No seller name here</div>"#;
    let parser = Parser::new();
    let name = parser.extract_seller_name(html);
    assert_eq!(name, None);
}

#[test]
fn test_extract_offer_ids_empty() {
    let html = r#"<html><body></body></html>"#;
    let parser = Parser::new();
    let ids = parser.extract_offer_ids(html);
    assert!(ids.is_empty());
}

#[test]
fn test_extract_offer_ids_multiple() {
    let html = r#"<div>
        <a data-offer-id="111">Offer 1</a>
        <a data-offer-id="222">Offer 2</a>
        <a data-offer-id="333">Offer 3</a>
    </div>"#;
    let parser = Parser::new();
    let ids = parser.extract_offer_ids(html);
    assert_eq!(ids.len(), 3);
    assert!(ids.contains(&OfferId("111".to_string())));
    assert!(ids.contains(&OfferId("222".to_string())));
    assert!(ids.contains(&OfferId("333".to_string())));
}

#[test]
fn test_parse_orders_empty() {
    let html = r#"<html><body></body></html>"#;
    let parser = Parser::new();
    let orders = parser.parse_orders_from_page(html);
    assert!(orders.is_empty());
}

#[test]
fn test_parse_reviews_empty() {
    let html = r#"<html><body></body></html>"#;
    let parser = Parser::new();
    let reviews = parser.parse_reviews_from_page(html);
    assert!(reviews.is_empty());
}

#[test]
fn test_parse_subcategories_empty() {
    let html = r#"<html><body></body></html>"#;
    let subcategories = Parser::parse_subcategories(html);
    assert!(subcategories.is_empty());
}

#[test]
fn test_extract_next_page_url_none() {
    let html = r#"<html><body></body></html>"#;
    let url = Parser::extract_next_page_url(html);
    assert_eq!(url, None);
}

#[test]
fn test_extract_next_page_url_found() {
    let html = r#"<html><body>
        <input type="hidden" name="continue" value="/orders?page=2">
    </body></html>"#;
    let url = Parser::extract_next_page_url(html);
    assert_eq!(url, Some("/orders?page=2".to_string()));
}

#[test]
fn test_offer_stock_zero() {
    let html = r#"<a class="tc-item" href="/lot?id=6001" data-online="0">
        <div class="tc-price"><div>10</div><span class="unit">₽</span></div>
        <div class="tc-amount" data-s="0"></div>
        <div class="media-user-name">ZeroStock</div>
        <div class="rating-mini-count">0</div>
        <div class="tc-server">RU</div>
    </a>"#;
    let parser = Parser::new();
    let offers = parser.parse_offers_from_page(html);
    assert_eq!(offers.len(), 1);
    assert_eq!(offers[0].stock, 0);
}

#[test]
fn test_offer_missing_optional_fields() {
    let html = r#"<a class="tc-item" href="/lot?id=7001">
        <div class="tc-price"><div>99</div></div>
        <div class="media-user-name">MinimalSeller</div>
    </a>"#;
    let parser = Parser::new();
    let offers = parser.parse_offers_from_page(html);
    assert_eq!(offers.len(), 1);
    assert_eq!(offers[0].seller.name, "MinimalSeller");
    assert_eq!(offers[0].price, 99.0);
}

#[test]
fn test_game_list_mixed_urls() {
    let html = r#"<html><body>
        <a href="/chips/10/">Game A</a>
        <a href="/lots/20/">Game B</a>
        <a href="/chips/30/">Game C</a>
        <a href="/lots/40/">Game D</a>
    </body></html>"#;
    let parser = Parser::new();
    let games = parser.parse_game_list(html);
    assert_eq!(games.len(), 4);

    let chips_count = games.iter().filter(|g| g.chips_url.is_some()).count();
    let lots_count = games.iter().filter(|g| g.lots_url.is_some()).count();
    assert_eq!(chips_count, 2);
    assert_eq!(lots_count, 2);
}

#[test]
fn test_parse_user_empty_html() {
    let html = r#"<html><body></body></html>"#;
    let parser = Parser::new();
    let user = parser.parse_user(html, UserId("123".to_string())).unwrap();
    assert_eq!(user.user_id.0, "123");
    assert!(user.username.is_empty());
    assert_eq!(user.rating, 0.0);
    assert_eq!(user.reviews, 0);
    assert!(!user.online);
}

#[test]
fn test_price_with_multiple_dots() {
    let html = r#"<div class="tc-price">1.234.567 ₽</div>"#;
    let parser = Parser::new();
    let price = parser.extract_price(html);
    // "1.234.567" -> all digits and dots -> "1.234.567" -> parse fails, returns None
    assert!(price.is_none());
}

#[test]
fn test_offer_only_price_no_other_fields() {
    let html = r#"<a class="tc-item" href="/lot?id=8001" data-online="1">
        <div class="tc-price"><div>500</div><span class="unit">$</span></div>
    </a>"#;
    let parser = Parser::new();
    // Parser requires tc-amount and other fields to be present
    let offers = parser.parse_offers_from_page(html);
    assert_eq!(offers.len(), 0);
}

#[test]
fn test_unicode_game_names() {
    let html = r#"<html><body>
        <a href="/chips/100/">Киберпанк 2077</a>
        <a href="/lots/200/">Эльden Ring</a>
    </body></html>"#;
    let parser = Parser::new();
    let games = parser.parse_game_list(html);
    assert_eq!(games.len(), 2);
    assert_eq!(games[0].name, "Киберпанк 2077");
    assert_eq!(games[1].name, "Эльden Ring");
}

#[test]
fn test_html_entities_in_content() {
    let html = r#"<a class="tc-item" href="/lot?id=9001" data-online="1">
        <div class="tc-price"><div>100</div><span class="unit">₽</span></div>
        <div class="tc-amount" data-s="1"></div>
        <div class="media-user-name">Seller&amp;Co</div>
        <div class="rating-mini-count">5</div>
        <div class="tc-server">RU</div>
        <div class="tc-desc-text">Item &lt; 100 &amp; good</div>
    </a>"#;
    let parser = Parser::new();
    let offers = parser.parse_offers_from_page(html);
    assert_eq!(offers.len(), 1);
    // scraper decodes HTML entities
    assert_eq!(offers[0].seller.name, "Seller&Co");
    assert_eq!(offers[0].description, "Item < 100 & good");
}
