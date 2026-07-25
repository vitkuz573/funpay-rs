use funpay_rs::parser::Parser;

#[test]
fn test_parse_offers_from_page() {
    let html = r#"
    <a href="https://funpay.com/lots/offer?id=123" class="tc-item" data-online="1">
        <div class="tc-server">EU</div>
        <div class="tc-price" data-s="150"><div>150 ₽</div></div>
        <div class="tc-amount" data-s="1000">1000</div>
        <div class="media-user-name">Seller1</div>
        <div class="tc-desc-text">Some description</div>
    </a>
    "#;
    let parser = Parser::new();
    let offers = parser.parse_offers_from_page(html);
    assert_eq!(offers.len(), 1);
    assert_eq!(offers[0].price, 150.0);
    assert_eq!(offers[0].offer_id, "123");
}

#[test]
fn test_parse_multiple_offers() {
    let html = r#"
    <a href="https://funpay.com/lots/offer?id=1" class="tc-item" data-online="1">
        <div class="tc-server">EU</div>
        <div class="tc-price" data-s="100"><div>100 ₽</div></div>
        <div class="tc-amount" data-s="50">50</div>
        <div class="media-user-name">Seller1</div>
        <div class="tc-desc-text">Gold coins</div>
    </a>
    <a href="https://funpay.com/lots/offer?id=2" class="tc-item" data-online="0">
        <div class="tc-server">US</div>
        <div class="tc-price" data-s="200"><div>200 ₽</div></div>
        <div class="tc-amount" data-s="100">100</div>
        <div class="media-user-name">Seller2</div>
        <div class="tc-desc-text">Silver coins</div>
    </a>
    "#;
    let parser = Parser::new();
    let offers = parser.parse_offers_from_page(html);
    assert_eq!(offers.len(), 2);
    assert_eq!(offers[0].server, "EU");
    assert_eq!(offers[1].server, "US");
    assert!(offers[0].seller.online);
    assert!(!offers[1].seller.online);
}

#[test]
fn test_parse_offers_empty_page() {
    let html = r#"<html><body></body></html>"#;
    let parser = Parser::new();
    let offers = parser.parse_offers_from_page(html);
    assert_eq!(offers.len(), 0);
}
