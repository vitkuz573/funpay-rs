//! Request middleware for customizing HTTP requests.
//!
//! Middlewares are applied to every request made by [`FunPayClient`](crate::client::FunPayClient).

use reqwest::Request;

/// Trait for implementing request middleware.
pub trait RequestMiddleware: Send + Sync {
    /// Called before each request is sent.
    fn on_request(&self, request: &mut Request);
}

/// Middleware that logs every request at debug level.
pub struct LoggingMiddleware;

impl RequestMiddleware for LoggingMiddleware {
    fn on_request(&self, request: &mut Request) {
        log::debug!("{} {}", request.method(), request.url());
    }
}

/// Middleware that rotates the User-Agent header on each request.
pub struct UserAgentRotationMiddleware;

impl RequestMiddleware for UserAgentRotationMiddleware {
    fn on_request(&self, request: &mut Request) {
        use crate::ua::random_ua;
        if let Ok(val) = random_ua().parse() {
            request.headers_mut().insert("User-Agent", val);
        }
    }
}
