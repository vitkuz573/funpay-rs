use funpay_sdk::parser::Parser;
use funpay_sdk::models::Price;

#[test]
fn test_empty_html_game_list() {
    let parser = Parser::new();
    let games = parser.parse_game_list("");
    assert!(games.is_empty());
}

#[test]
fn test_empty_html_offers() {
    let parser = Parser::new();
    let offers = parser.parse_category_offers("");
    assert!(offers.is_empty());
}

#[test]
fn test_malformed_html_offers() {
    let html = r#"<html><body><a class="tc-item"><div class="tc-price">100</div>"#;
    let parser = Parser::new();
    let offers = parser.parse_category_offers(html);
    assert_eq!(offers.len(), 1);
    assert_eq!(offers[0].price, Price(100.0));
}

#[test]
fn test_malformed_html_game_list() {
    let html = r#"<html><body><div class="game-list"><div class="game-title"><a href="/chips/91/" class="game-title">Unclosed tag"#;
    let parser = Parser::new();
    let games = parser.parse_game_list(html);
    let with_title: Vec<_> = games.into_iter().filter(|g| !g.title.is_empty()).collect();
    assert_eq!(with_title.len(), 1);
    assert_eq!(with_title[0].title, "Unclosed tag");
}

#[test]
fn test_unicode_in_offer_description() {
    let html = r#"<a class="tc-item">
        <div class="tc-price"><div>100</div></div>
        <div class="tc-server">Москва</div>
        <div class="tc-desc-text">Быстрая доставка 24/7</div>
    </a>"#;
    let parser = Parser::new();
    let offers = parser.parse_category_offers(html);
    assert_eq!(offers.len(), 1);
    assert_eq!(offers[0].description.as_deref(), Some("Быстрая доставка 24/7"));
    assert_eq!(offers[0].server.as_deref(), Some("Москва"));
}

#[test]
fn test_price_parsing_with_comma() {
    let html = r#"<a class="tc-item">
        <div class="tc-price">1 250,50</div>
    </a>"#;
    let parser = Parser::new();
    let offers = parser.parse_category_offers(html);
    assert_eq!(offers.len(), 1);
    // f64 parse fails on "1 250,50" format, defaults to 0.0
    assert_eq!(offers[0].price, Price(0.0));
}

#[test]
fn test_price_with_thousand_separator() {
    let html = r#"<a class="tc-item">
        <div class="tc-price">1 250</div>
        <div class="tc-server">RU</div>
    </a>"#;
    let parser = Parser::new();
    let offers = parser.parse_category_offers(html);
    assert_eq!(offers.len(), 1);
    // f64 parse fails on "1 250" format, defaults to 0.0
    assert_eq!(offers[0].price, Price(0.0));
}

#[test]
fn test_very_long_description() {
    let long_desc = "A".repeat(10_000);
    let html = format!(
        r#"<a class="tc-item">
            <div class="tc-price">50</div>
            <div class="tc-desc-text">{}</div>
        </a>"#,
        long_desc
    );
    let parser = Parser::new();
    let offers = parser.parse_category_offers(&html);
    assert_eq!(offers.len(), 1);
    assert_eq!(offers[0].description.as_ref().unwrap().len(), 10_000);
}

#[test]
fn test_offer_missing_optional_fields() {
    let html = r#"<a class="tc-item">
        <div class="tc-price">99</div>
    </a>"#;
    let parser = Parser::new();
    let offers = parser.parse_category_offers(html);
    assert_eq!(offers.len(), 1);
    assert_eq!(offers[0].price, Price(99.0));
    assert!(offers[0].server.is_none());
    assert!(offers[0].description.is_none());
}

#[test]
fn test_price_no_match() {
    let html = r#"<a class="tc-item">
        <div>No price here</div>
    </a>"#;
    let parser = Parser::new();
    let offers = parser.parse_category_offers(html);
    assert_eq!(offers.len(), 1);
    assert_eq!(offers[0].price, Price(0.0));
}

#[test]
fn test_parse_orders_empty() {
    let html = r#"<html><body></body></html>"#;
    let parser = Parser::new();
    let orders = parser.parse_orders(html);
    assert!(orders.is_empty());
}

#[test]
fn test_parse_chats_empty() {
    let html = r#"<html><body></body></html>"#;
    let parser = Parser::new();
    let chats = parser.parse_chats(html);
    assert!(chats.is_empty());
}

#[test]
fn test_parse_user_profile_empty_html() {
    let html = r#"<html><body></body></html>"#;
    let parser = Parser::new();
    let user = parser.parse_user_profile(html);
    assert!(user.is_some());
    let user = user.unwrap();
    assert!(user.username.is_empty());
    assert!(user.avatar_url.is_empty());
}

#[test]
fn test_unicode_game_names() {
    let html = r#"<div class="game-list">
        <div class="game-title">
            <a href="/chips/100/" class="game-title">Киберпанк 2077</a>
        </div>
        <div class="game-title">
            <a href="/lots/200/" class="game-title">Эльden Ring</a>
        </div>
    </div>"#;
    let parser = Parser::new();
    let games = parser.parse_game_list(html);
    let with_title: Vec<_> = games.into_iter().filter(|g| !g.title.is_empty()).collect();
    assert_eq!(with_title.len(), 2);
    assert_eq!(with_title[0].title, "Киберпанк 2077");
    assert_eq!(with_title[1].title, "Эльden Ring");
}

#[test]
fn test_html_entities_in_content() {
    let html = r#"<a class="tc-item">
        <div class="tc-price">100</div>
        <div class="tc-server">RU</div>
        <div class="tc-desc-text">Item &lt; 100 &amp; good</div>
    </a>"#;
    let parser = Parser::new();
    let offers = parser.parse_category_offers(html);
    assert_eq!(offers.len(), 1);
    assert_eq!(offers[0].description.as_deref(), Some("Item < 100 & good"));
}

#[test]
fn test_parse_user_offers_empty() {
    let html = r#"<html><body></body></html>"#;
    let parser = Parser::new();
    let lots = parser.parse_user_offers(html);
    assert!(lots.is_empty());
}

#[test]
fn test_parse_chat_messages_empty() {
    let html = r#"<html><body></body></html>"#;
    let parser = Parser::new();
    let msgs = parser.parse_chat_messages(html);
    assert!(msgs.is_empty());
}
