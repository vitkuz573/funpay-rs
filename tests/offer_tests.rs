use funpay_sdk::parser::Parser;
use funpay_sdk::models::*;

#[test]
fn test_parse_category_offers_sale_type_bulk() {
    let html = r#"<a class="tc-item" data-mark="bulk">
        <div class="tc-price">25.00</div>
        <div class="tc-desc-text">1000 Gold</div>
    </a>"#;
    let parser = Parser::new();
    let offers = parser.parse_category_offers(html);
    assert_eq!(offers.len(), 1);
    assert_eq!(offers[0].sale_type, LotSaleType::Bulk);
}

#[test]
fn test_parse_category_offers_sale_type_default() {
    let html = r#"<a class="tc-item">
        <div class="tc-price">25.00</div>
    </a>"#;
    let parser = Parser::new();
    let offers = parser.parse_category_offers(html);
    assert_eq!(offers.len(), 1);
    assert_eq!(offers[0].sale_type, LotSaleType::Single);
}

#[test]
fn test_parse_category_offers_with_item_count() {
    let html = r#"<a class="tc-item" data-lot-size="10">
        <div class="tc-price">5.00</div>
    </a>"#;
    let parser = Parser::new();
    let offers = parser.parse_category_offers(html);
    assert_eq!(offers.len(), 1);
    assert_eq!(offers[0].item_count, Some(10));
}

#[test]
fn test_parse_category_offers_with_image() {
    let html = r#"<a class="tc-item">
        <img src="https://example.com/item.png">
        <div class="tc-price">50</div>
    </a>"#;
    let parser = Parser::new();
    let offers = parser.parse_category_offers(html);
    assert_eq!(offers.len(), 1);
    assert_eq!(offers[0].image_url.as_deref(), Some("https://example.com/item.png"));
}

#[test]
fn test_parse_category_offers_with_currency() {
    let html = r#"<a class="tc-item">
        <div class="tc-price"><span class="currency">$</span>25.00</div>
    </a>"#;
    let parser = Parser::new();
    let offers = parser.parse_category_offers(html);
    assert_eq!(offers.len(), 1);
    assert_eq!(offers[0].currency, "$");
}

#[test]
fn test_parse_category_offers_negative_price() {
    let html = r#"<a class="tc-item">
        <div class="tc-price">-10</div>
    </a>"#;
    let parser = Parser::new();
    let offers = parser.parse_category_offers(html);
    assert_eq!(offers.len(), 1);
    assert_eq!(offers[0].price, Price(-10.0));
}

#[test]
fn test_parse_category_offers_zero_price() {
    let html = r#"<a class="tc-item">
        <div class="tc-price">0</div>
    </a>"#;
    let parser = Parser::new();
    let offers = parser.parse_category_offers(html);
    assert_eq!(offers.len(), 1);
    assert_eq!(offers[0].price, Price(0.0));
}

#[test]
fn test_parse_category_offers_large_price() {
    let html = r#"<a class="tc-item">
        <div class="tc-price">9999999.99</div>
    </a>"#;
    let parser = Parser::new();
    let offers = parser.parse_category_offers(html);
    assert_eq!(offers.len(), 1);
    assert_eq!(offers[0].price, Price(9999999.99));
}

#[test]
fn test_parse_category_offers_non_numeric_price() {
    let html = r#"<a class="tc-item">
        <div class="tc-price">abc</div>
    </a>"#;
    let parser = Parser::new();
    let offers = parser.parse_category_offers(html);
    assert_eq!(offers.len(), 1);
    assert_eq!(offers[0].price, Price(0.0));
}

#[test]
fn test_parse_category_offers_100_items() {
    let mut html = String::new();
    for i in 0..100 {
        html.push_str(&format!(
            r#"<a class="tc-item"><div class="tc-price">{}</div></a>"#,
            i as f64 * 1.5
        ));
    }
    let parser = Parser::new();
    let offers = parser.parse_category_offers(&html);
    assert_eq!(offers.len(), 100);
}

#[test]
fn test_parse_category_offers_preserves_order() {
    let html = r#"<a class="tc-item"><div class="tc-price">1</div></a>
        <a class="tc-item"><div class="tc-price">2</div></a>
        <a class="tc-item"><div class="tc-price">3</div></a>"#;
    let parser = Parser::new();
    let offers = parser.parse_category_offers(html);
    assert_eq!(offers[0].price, Price(1.0));
    assert_eq!(offers[1].price, Price(2.0));
    assert_eq!(offers[2].price, Price(3.0));
}
