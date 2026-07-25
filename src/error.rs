//! Error types for the FunPay SDK.
//!
//! All fallible operations return [`Result<T>`] with [`FunPayError`].
//!
//! # Error Variants
//!
//! | Variant | Cause | Suggestion |
//! |---------|-------|------------|
//! | `Http` | Network or HTTP error | Check connectivity, retry |
//! | `Parse` | HTML structure mismatch | Report as upstream change |
//! | `Auth` | Authentication failure | Re-authenticate with valid golden key |
//! | `RateLimited` | Too many requests | Wait for the specified duration |
//! | `Timeout` | Request exceeded deadline | Retry or increase timeout |

use thiserror::Error;

/// Errors from parsing HTML responses from FunPay.
///
/// These typically indicate the upstream page structure has changed.
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("missing data-app-data attribute — page structure may have changed")]
    NoDataAppData,
    #[error("unclosed data-app-data attribute — malformed HTML response")]
    UnclosedDataAppData,
    #[error("JSON parse error: {0}")]
    JsonParse(String),
    #[error("missing required field: {0} — page may need updated selectors")]
    MissingField(String),
}

/// Errors from authentication operations.
#[derive(Debug, Error)]
pub enum AuthError {
    #[error("invalid golden key — check your credentials")]
    InvalidGoldenKey,
    #[error("CSRF token mismatch — session may have expired, re-authenticate")]
    CsrfMismatch,
}

/// The unified error type for all FunPay SDK operations.
///
/// # Examples
///
/// ```rust
/// use funpay_sdk::error::{FunPayError, ParseError};
///
/// let err: FunPayError = ParseError::MissingField("seller_id".into()).into();
/// assert!(err.to_string().contains("seller_id"));
/// ```
#[derive(Debug, Error)]
pub enum FunPayError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("parse error: {0}")]
    Parse(#[from] ParseError),
    #[error("auth error: {0}")]
    Auth(#[from] AuthError),
    #[error("rate limited — retry after {0:?}")]
    RateLimited(std::time::Duration),
    #[error("request timeout after {0:?}")]
    Timeout(std::time::Duration),
}

/// Convenience type alias for `std::result::Result<T, FunPayError>`.
pub type Result<T> = std::result::Result<T, FunPayError>;
