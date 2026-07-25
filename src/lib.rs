//! # funpay-rs
//!
//! Unofficial Rust SDK for [FunPay.com](https://funpay.com) — P2P gaming marketplace.
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use funpay_rs::client::FunPayClient;
//! use funpay_rs::search::SearchQuery;
//! use funpay_rs::models::Currency;
//!
//! let client = FunPayClient::new().unwrap();
//! let offers = SearchQuery::new("kimi")
//!     .max_price(rust_decimal::Decimal::from(3000))
//!     .currency(Currency::RUB)
//!     .execute(&client)
//!     .await
//!     .unwrap();
//! ```
//!
//! ## Features
//!
//! - [`client::FunPayClient`] — HTTP client with retry, rate limiting, and caching
//! - [`search::SearchQuery`] — Builder pattern for complex searches
//! - [`parser::Parser`] — HTML parsing for all FunPay entities
//! - [`models`] — Type-safe data models
//! - [`stream`] — Async streaming support (requires `streaming` feature)
//! - [`export`] — JSON/CSV export (requires `export` feature)
//! - [`monitor::Monitor`] — Real-time price change detection

pub mod client;
pub mod auth;
pub mod error;
pub mod models;
pub mod parser;
pub mod monitor;
pub mod search;
pub mod retry;

#[cfg(feature = "streaming")]
pub mod stream;

#[cfg(feature = "export")]
pub mod export;
