use funpay_rs::parser::Parser;

#[test]
fn test_parse_lot_offer_id() {
    let html = r#"<a class="tc-item" data-offer-id="12345">"#;
    let parser = Parser::new();
    let ids = parser.extract_offer_ids(html);
    assert!(ids.contains(&"12345".to_string()));
}

#[test]
fn test_parse_price() {
    let html = r#"<div class="tc-price">150.50 ₽</div>"#;
    let parser = Parser::new();
    let price = parser.extract_price(html);
    assert_eq!(price, Some(150.50));
}

#[test]
fn test_parse_seller_name() {
    let html = r#"<div class="media-user-name">TestSeller</div>"#;
    let parser = Parser::new();
    let name = parser.extract_seller_name(html);
    assert_eq!(name, Some("TestSeller".to_string()));
}
