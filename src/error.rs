use thiserror::Error;

/// Errors that occur when parsing HTML from FunPay pages.
#[derive(Debug, Error)]
pub enum ParseError {
    /// The expected `data-app-data` attribute was not found.
    #[error("missing data-app-data attribute")]
    NoDataAppData,
    /// The `data-app-data` attribute was not properly closed.
    #[error("unclosed data-app-data attribute")]
    UnclosedDataAppData,
    /// Failed to parse JSON content.
    #[error("JSON parse error: {0}")]
    JsonParse(String),
    /// A required field was missing from the parsed data.
    #[error("missing field: {0}")]
    MissingField(String),
}

/// Errors related to authentication.
#[derive(Debug, Error)]
pub enum AuthError {
    /// The provided golden key is invalid.
    #[error("invalid golden key")]
    InvalidGoldenKey,
    /// The CSRF token did not match the expected value.
    #[error("CSRF token mismatch")]
    CsrfMismatch,
}

/// The main error type for all FunPay operations.
#[derive(Debug, Error)]
pub enum FunPayError {
    /// An HTTP request failed.
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    /// Parsing an HTML page or JSON failed.
    #[error("parse error: {0}")]
    Parse(#[from] ParseError),
    /// An authentication operation failed.
    #[error("auth error: {0}")]
    Auth(#[from] AuthError),
    /// The server returned a 429 rate limit response.
    #[error("rate limited, retry after {retry_after:?}")]
    RateLimited { retry_after: std::time::Duration },
    /// The request timed out.
    #[error("request timeout after {0:?}")]
    Timeout(std::time::Duration),
    /// The client has been blocked by FunPay.
    #[error("blocked: {0}")]
    Blocked(String),
    /// All retry attempts have been exhausted.
    #[error("max retries ({0}) exceeded")]
    MaxRetriesExceeded(u32),
}

/// A convenience type alias for `Result<T, FunPayError>`.
pub type Result<T> = std::result::Result<T, FunPayError>;
