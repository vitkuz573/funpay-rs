use funpay_sdk::parser::Parser;

#[test]
fn test_parse_category_offers() {
    let html = r#"<a class="tc-item">
        <div class="tc-server">EU Server</div>
        <div class="tc-desc-text">1000 Gold</div>
        <div class="tc-price">25.00</div>
    </a>"#;
    let parser = Parser::new();
    let offers = parser.parse_category_offers(html);
    assert_eq!(offers.len(), 1);
    assert_eq!(offers[0].server.as_deref(), Some("EU Server"));
    assert_eq!(offers[0].description.as_deref(), Some("1000 Gold"));
    assert_eq!(offers[0].price, 25.0);
}

#[test]
fn test_parse_category_offers_empty() {
    let html = r#"<html><body></body></html>"#;
    let parser = Parser::new();
    let offers = parser.parse_category_offers(html);
    assert!(offers.is_empty());
}

#[test]
fn test_parse_category_offers_multiple() {
    let html = r#"<a class="tc-item">
        <div class="tc-price">100</div>
        <div class="tc-server">RU</div>
    </a>
    <a class="tc-item">
        <div class="tc-price">200</div>
        <div class="tc-server">EU</div>
    </a>"#;
    let parser = Parser::new();
    let offers = parser.parse_category_offers(html);
    assert_eq!(offers.len(), 2);
    assert_eq!(offers[0].price, 100.0);
    assert_eq!(offers[1].price, 200.0);
    assert_eq!(offers[0].server.as_deref(), Some("RU"));
    assert_eq!(offers[1].server.as_deref(), Some("EU"));
}
