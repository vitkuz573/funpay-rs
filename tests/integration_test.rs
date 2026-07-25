use funpay_rs::client::FunPayClient;
use funpay_rs::error::{FunPayError, ParseError};
use funpay_rs::parser::Parser;
use funpay_rs::monitor::Monitor;

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
fn test_monitor_creation() {
    let _monitor = Monitor::new();
}
