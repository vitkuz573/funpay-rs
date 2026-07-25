use thiserror::Error;

#[derive(Debug, Error)]
pub enum FunPayError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("Auth error: {0}")]
    Auth(String),
}

pub type Result<T> = std::result::Result<T, FunPayError>;
