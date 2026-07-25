use funpay_sdk::client::FunPayClient;
use funpay_sdk::error::{FunPayError, ParseError};
use funpay_sdk::parser::Parser;

#[test]
fn test_error_display() {
    let error = FunPayError::from(ParseError::MissingField("test field".to_string()));
    assert_eq!(error.to_string(), "parse error: missing field: test field");
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
