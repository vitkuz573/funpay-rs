use funpay_rs::parser::Parser;
use funpay_rs::models::OfferId;

#[test]
fn test_parse_offer_page() {
    let html = r#"
    <div class="tc-server">EU Server</div>
    <div class="tc-desc">1000 Gold</div>
    <div class="tc-price">25.00</div>
    <div class="tc-qty">5</div>
    <div class="media-user-name">Seller1</div>
    "#;
    let parser = Parser::new();
    let offer = parser.parse_offer(html, OfferId("12345".to_string()));
    assert!(offer.is_some());
}
