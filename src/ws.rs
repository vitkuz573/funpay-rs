use futures_util::StreamExt;
use tokio::sync::broadcast;
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::error::FunPayError;

/// Real-time offer monitor via FunPay WebSocket
pub struct WsMonitor {
    url: String,
}

impl WsMonitor {
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

    /// Connect and stream offer events
    pub async fn connect(&self) -> Result<broadcast::Receiver<WsEvent>, FunPayError> {
        let (ws_stream, _) = connect_async(&self.url)
            .await
            .map_err(|e| FunPayError::WebSocket(e.to_string()))?;

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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum WsEvent {
    OfferUpdate { offer_id: String, price: String },
    NewOffer { offer_id: String },
    OfferRemoved { offer_id: String },
}
