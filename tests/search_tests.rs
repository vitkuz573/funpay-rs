use funpay_sdk::parser::Parser;
use funpay_sdk::models::Price;

#[test]
fn test_parse_offers_from_page() {
    let html = r#"<a class="tc-item">
        <div class="tc-server">EU</div>
        <div class="tc-price">150</div>
        <div class="tc-desc-text">Some description</div>
    </a>"#;
    let parser = Parser::new();
    let offers = parser.parse_category_offers(html);
    assert_eq!(offers.len(), 1);
    assert_eq!(offers[0].price, Price(150.0));
    assert_eq!(offers[0].server.as_deref(), Some("EU"));
}

#[test]
fn test_parse_multiple_offers() {
    let html = r#"<a class="tc-item">
        <div class="tc-server">EU</div>
        <div class="tc-price">100</div>
        <div class="tc-desc-text">Gold coins</div>
    </a>
    <a class="tc-item">
        <div class="tc-server">US</div>
        <div class="tc-price">200</div>
        <div class="tc-desc-text">Silver coins</div>
    </a>"#;
    let parser = Parser::new();
    let offers = parser.parse_category_offers(html);
    assert_eq!(offers.len(), 2);
    assert_eq!(offers[0].server.as_deref(), Some("EU"));
    assert_eq!(offers[1].server.as_deref(), Some("US"));
}

#[test]
fn test_parse_offers_empty_page() {
    let html = r#"<html><body></body></html>"#;
    let parser = Parser::new();
    let offers = parser.parse_category_offers(html);
    assert_eq!(offers.len(), 0);
}


