use funpay_rs::client::FunPayClient;
use funpay_rs::auth::CsrfTokens;

#[test]
fn test_client_creation() {
    let client = FunPayClient::new();
    assert!(client.is_ok());
}

#[test]
fn test_csrf_extraction() {
    let html = r#"<body data-app-data='{"csrf-token":"abc123","userId":12345}'></body>"#;
    let tokens = CsrfTokens::from_html(html);
    assert!(tokens.is_ok());
    assert_eq!(tokens.unwrap().form_token, "abc123");
}

#[test]
fn test_csrf_extraction_invalid() {
    let html = "<body>no data app here</body>";
    let tokens = CsrfTokens::from_html(html);
    assert!(tokens.is_err());
}
