use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("missing data-app-data attribute")]
    NoDataAppData,
    #[error("unclosed data-app-data attribute")]
    UnclosedDataAppData,
    #[error("JSON parse error: {0}")]
    JsonParse(String),
    #[error("missing field: {0}")]
    MissingField(String),
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("invalid golden key")]
    InvalidGoldenKey,
    #[error("CSRF token mismatch")]
    CsrfMismatch,
}

#[derive(Debug, Error)]
pub enum FunPayError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("parse error: {0}")]
    Parse(#[from] ParseError),
    #[error("auth error: {0}")]
    Auth(#[from] AuthError),
}

pub type Result<T> = std::result::Result<T, FunPayError>;
