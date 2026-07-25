use funpay_rs::client::FunPayClient;
use funpay_rs::error::FunPayError;
use funpay_rs::parser::Parser;
use funpay_rs::monitor::Monitor;

#[test]
fn test_error_display() {
    let error = FunPayError::Parse("test error".to_string());
    assert_eq!(error.to_string(), "Parse error: test error");
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
