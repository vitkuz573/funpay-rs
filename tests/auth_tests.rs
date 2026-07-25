//! Auth tests
use funpay_sdk::auth::CsrfTokens;

#[test]
fn test_csrf_extraction_valid() {
    let html = r#"<div data-app-data='{"csrf-token":"tok_abc123"}'>"#;
    let tokens = CsrfTokens::from_html(html).unwrap();
    assert_eq!(tokens.form_token, "tok_abc123");
    assert_eq!(tokens.header_token, "tok_abc123");
}

#[test]
fn test_csrf_extraction_no_attribute() {
    let html = r#"<div>"#;
    let result = CsrfTokens::from_html(html);
    assert!(result.is_err());
}

#[test]
fn test_csrf_extraction_missing_token() {
    let html = r#"<div data-app-data='{"other":"value"}'>"#;
    let result = CsrfTokens::from_html(html);
    assert!(result.is_err());
}

#[test]
fn test_csrf_extraction_invalid_json() {
    let html = r#"<div data-app-data='not-json'>"#;
    let result = CsrfTokens::from_html(html);
    assert!(result.is_err());
}

#[test]
fn test_csrf_extraction_empty_token() {
    let html = r#"<div data-app-data='{"csrf-token":""}'>"#;
    let tokens = CsrfTokens::from_html(html).unwrap();
    assert_eq!(tokens.form_token, "");
}
