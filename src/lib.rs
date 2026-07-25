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
//! ## Architecture
//!
//! - `generated/` — Auto-generated from webspec (DO NOT EDIT)
//! - Root modules — Hand-written extensions

// Generated modules (from webspec)
pub mod generated;

// Re-export generated types at crate root for convenience
pub use generated::models;
pub use generated::parser;
pub use generated::error;
pub use generated::client;

// Hand-written modules
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
