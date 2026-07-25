use funpay_sdk::parser::Parser;

#[test]
fn test_parse_category_offers_empty() {
    let html = r#"<html><body></body></html>"#;
    let parser = Parser::new();
    let offers = parser.parse_category_offers(html);
    assert!(offers.is_empty());
}

#[test]
fn test_parse_category_offers_with_data() {
    let html = r#"<a class="tc-item">
        <div class="tc-price">150</div>
        <div class="tc-server">EU</div>
        <div class="tc-desc-text">Gold</div>
    </a>"#;
    let parser = Parser::new();
    let offers = parser.parse_category_offers(html);
    assert_eq!(offers.len(), 1);
    assert_eq!(offers[0].price, 150.0);
    assert_eq!(offers[0].server.as_deref(), Some("EU"));
    assert_eq!(offers[0].description.as_deref(), Some("Gold"));
}

#[test]
fn test_parse_seller_profile() {
    let html = r#"<div class="seller-avatar"><img src="avatar.png"></div>
        <div class="seller-info">
            <span class="seller-name">TestSeller</span>
            <span data-user-id="12345"></span>
        </div>
        <span class="seller-reviews">100</span>
        <span class="seller-online">online</span>
        <span class="seller-rating">4.5</span>"#;
    let parser = Parser::new();
    let seller = parser.parse_seller_profile(html);
    assert!(seller.is_some());
    let seller = seller.unwrap();
    assert_eq!(seller.name, "TestSeller");
}
