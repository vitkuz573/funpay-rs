# funpay-rs

## Overview
High-performance Rust SDK for funpay.com — P2P gaming marketplace.

## Goals
- Type-safe API for all FunPay operations
- Cookie-based authentication with CSRF handling
- HTML parsing with proper error recovery
- Real-time monitoring via polling
- Zero-cost abstractions where possible

## Tech Stack
- Rust (edition 2021)
- reqwest (HTTP client with cookies)
- scraper (HTML parsing)
- tokio (async runtime)
- thiserror (error handling)
- serde (serialization)

## Architecture
- Client: holds reqwest::Client with cookie jar
- Auth: golden_key token management
- Models: strongly-typed data structures
- Parser: HTML → Models conversion
- Monitor: polling loop with change detection
