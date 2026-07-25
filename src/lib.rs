//! # FunPay SDK
//!
//! An async Rust SDK for interacting with the FunPay marketplace.
//!
//! ## Quick Start
//!
//! ```no_run
//! use funpay_sdk::client::FunPayClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), funpay_sdk::error::FunPayError> {
//!     let client = FunPayClient::new()?;
//!     let games = client.fetch_game_list().await?;
//!     for game in &games {
//!         println!("{}: {}", game.title, game.url);
//!     }
//!     Ok(())
//! }
//! ```
//!
//! ## Features
//!
//! - **HTML Parsing**: Robust parsing of FunPay pages into typed models
//! - **Async Client**: Full async HTTP client with cookie persistence
//! - **WebSocket Monitoring**: Real-time offer price monitoring
//! - **Search & Filter**: Built-in search queries with price/seller filtering
//! - **Export**: JSON and CSV export of offer data
//! - **Rate Limiting**: Built-in request throttling and retry logic

pub mod models;
pub mod parser;
pub mod client;
pub mod error;
pub mod cookies;
pub mod middleware;
pub mod retry;
pub mod auth;
pub mod search;
pub mod ua;
pub mod monitor;
pub mod stream;
pub mod export;
pub mod ws;
