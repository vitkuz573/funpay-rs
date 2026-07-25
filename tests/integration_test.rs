use funpay_sdk::error::{FunPayError, ParseError};
use funpay_sdk::client::FunPayClient;
use funpay_sdk::parser::Parser;

#[test]
fn test_error_display_parse() {
    let error = FunPayError::from(ParseError::MissingField("test field".to_string()));
    assert!(error.to_string().contains("test field"));
}

#[test]
fn test_error_display_auth() {
    let error = FunPayError::from(funpay_sdk::error::AuthError::InvalidGoldenKey);
    assert!(error.to_string().contains("invalid golden key"));
}

#[test]
fn test_error_display_rate_limited() {
    let error = FunPayError::RateLimited(std::time::Duration::from_secs(30));
    assert!(error.to_string().contains("30s"));
}

#[test]
fn test_error_display_timeout() {
    let error = FunPayError::Timeout(std::time::Duration::from_secs(60));
    assert!(error.to_string().contains("60s"));
}

#[test]
fn test_parser_creation() {
    let _parser = Parser::new();
}

#[test]
fn test_client_creation() {
    let _client = FunPayClient::new().unwrap();
}

#[test]
fn test_client_with_custom_base_url() {
    let client = FunPayClient::with_base_url("https://example.com").unwrap();
    assert_eq!(client.base_url, "https://example.com");
}

#[test]
fn test_parse_error_variants() {
    let e1 = ParseError::NoDataAppData;
    assert!(e1.to_string().contains("data-app-data"));

    let e2 = ParseError::UnclosedDataAppData;
    assert!(e2.to_string().contains("unclosed"));

    let e3 = ParseError::JsonParse("bad json".into());
    assert!(e3.to_string().contains("bad json"));

    let e4 = ParseError::MissingField("field".into());
    assert!(e4.to_string().contains("field"));
}

#[test]
fn test_funpay_error_from_parse() {
    let err: FunPayError = ParseError::MissingField("x".into()).into();
    assert!(matches!(err, FunPayError::Parse(_)));
}

#[test]
fn test_funpay_error_from_http() {
    let err: FunPayError = FunPayError::Timeout(std::time::Duration::from_secs(5));
    assert!(matches!(err, FunPayError::Timeout(_)));
}
