//! Real-time offer monitoring via FunPay WebSocket.
//!
//! Connects to the FunPay WebSocket endpoint and streams
//! offer events (updates, new offers, removals).
//!
//! # Examples
//!
//! ```no_run
//! use funpay_sdk::ws::WsMonitor;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let monitor = WsMonitor::new("https://funpay.com");
//! let mut rx = monitor.connect().await?;
//! while let Ok(event) = rx.recv().await {
//!     println!("WS event: {:?}", event);
//! }
//! # Ok(())
//! # }
//! ```

use futures_util::StreamExt;
use tokio::sync::broadcast;
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::error::{FunPayError, ParseError};

/// Real-time offer monitor via FunPay WebSocket.
pub struct WsMonitor {
    url: String,
}

impl WsMonitor {
    /// Create a new WebSocket monitor for the given base URL.
    pub fn new(base_url: &str) -> Self {
        Self {
            url: format!(
                "wss://{}/ws",
                base_url
                    .replace("https://", "")
                    .replace("http://", "")
            ),
        }
    }

    /// Connect and stream offer events via a broadcast channel.
    ///
    /// # Errors
    ///
    /// Returns [`FunPayError::Parse`] if the WebSocket connection fails.
    pub async fn connect(&self) -> Result<broadcast::Receiver<WsEvent>, FunPayError> {
        let (ws_stream, _) = connect_async(&self.url)
            .await
            .map_err(|e| FunPayError::Parse(ParseError::JsonParse(e.to_string())))?;

        let (_, mut read) = ws_stream.split();
        let (tx, rx) = broadcast::channel(256);

        tokio::spawn(async move {
            while let Some(msg) = read.next().await {
                if let Ok(Message::Text(text)) = msg {
                    if let Ok(event) = serde_json::from_str::<WsEvent>(&text) {
                        let _ = tx.send(event);
                    }
                }
            }
        });

        Ok(rx)
    }
}

/// WebSocket event types from FunPay.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum WsEvent {
    /// An existing offer was updated (price change, etc.).
    OfferUpdate { offer_id: String, price: String },
    /// A new offer was created.
    NewOffer { offer_id: String },
    /// An offer was removed.
    OfferRemoved { offer_id: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ws_monitor_url_construction() {
        let monitor = WsMonitor::new("https://funpay.com");
        assert_eq!(monitor.url, "wss://funpay.com/ws");
    }

    #[test]
    fn test_ws_monitor_strips_http() {
        let monitor = WsMonitor::new("http://funpay.com");
        assert_eq!(monitor.url, "wss://funpay.com/ws");
    }

    #[test]
    fn test_ws_event_serialization() {
        let event = WsEvent::NewOffer { offer_id: "123".into() };
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("NewOffer"));
        assert!(json.contains("123"));
    }
}
