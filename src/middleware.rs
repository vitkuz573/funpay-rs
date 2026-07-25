use reqwest::Request;

pub trait RequestMiddleware: Send + Sync {
    fn on_request(&self, request: &mut Request);
}

pub struct LoggingMiddleware;

impl RequestMiddleware for LoggingMiddleware {
    fn on_request(&self, request: &mut Request) {
        log::debug!("{} {}", request.method(), request.url());
    }
}

pub struct UserAgentRotationMiddleware;

impl RequestMiddleware for UserAgentRotationMiddleware {
    fn on_request(&self, request: &mut Request) {
        use crate::ua::random_ua;
        if let Ok(val) = random_ua().parse() {
            request.headers_mut().insert("User-Agent", val);
        }
    }
}
