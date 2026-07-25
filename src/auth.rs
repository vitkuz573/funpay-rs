//! CSRF token extraction for FunPay authentication.
//!
//! Parses the `data-app-data` attribute from HTML to extract
//! the CSRF token required for authenticated requests.
//!
//! # Examples
//!
//! ```rust
//! use funpay_sdk::auth::CsrfTokens;
//!
//! let html = r#"<div data-app-data='{"csrf-token":"abc123"}'>"#;
//! let tokens = CsrfTokens::from_html(html).unwrap();
//! assert_eq!(tokens.form_token, "abc123");
//! ```

use serde_json::Value;
use crate::error::ParseError;

/// CSRF tokens extracted from a FunPay HTML page.
pub struct CsrfTokens {
    /// The form submission token.
    pub form_token: String,
    /// The header authentication token.
    pub header_token: String,
}

impl CsrfTokens {
    /// Extract CSRF tokens from an HTML page's `data-app-data` attribute.
    ///
    /// # Errors
    ///
    /// Returns [`ParseError::NoDataAppData`] if the attribute is missing,
    /// [`ParseError::UnclosedDataAppData`] if malformed, or
    /// [`ParseError::JsonParse`] / [`ParseError::MissingField`] for content issues.
    pub fn from_html(html: &str) -> Result<Self, ParseError> {
        let start = html.find("data-app-data='").ok_or(ParseError::NoDataAppData)?;
        let json_start = start + 15;
        let json_end = html[json_start..].find("'").ok_or(ParseError::UnclosedDataAppData)?;
        let json_str = &html[json_start..json_start + json_end];

        let data: Value = serde_json::from_str(json_str)
            .map_err(|e| ParseError::JsonParse(e.to_string()))?;

        let form_token = data["csrf-token"].as_str()
            .ok_or_else(|| ParseError::MissingField("csrf-token".into()))?
            .to_string();

        Ok(Self { form_token: form_token.clone(), header_token: form_token })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_csrf_extraction_unclosed() {
        let html = r#"<div data-app-data='{"csrf-token":"abc"}>"#;
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
}
