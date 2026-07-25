# funpay-rs

Unofficial Rust SDK for [FunPay.com](https://funpay.com) — P2P gaming marketplace.

## Features

- **Builder pattern** — Configure clients with fluent API (base URL, auth, retry, rate limits)
- **SearchQuery** — Filtered search with price, currency, server, stock, and online-only filters
- **Streaming** — Real-time offer discovery via async streams
- **Retry with backoff** — Exponential backoff for transient failures (5xx, 429)
- **Rate limiting** — Token-bucket rate limiter respects `Retry-After` headers
- **Models** — Typed `Offer`, `Order`, `Chat`, `ChatMessage`, `Review`, `Game`, `Seller`, `User`
- **Typed errors** — Structured `FunPayError` enum with `Parse`, `Auth`, `RateLimited`, `Timeout`, `Blocked`
- **Mock testing** — Mock HTTP layer for offline unit tests

## Usage

```rust
use funpay_rs::client::FunPayClient;

#[tokio::main]
async fn main() -> Result<(), funpay_rs::error::FunPayError> {
    let client = FunPayClient::new()?;

    let games = client.fetch_all_games().await?;
    println!("Found {} games", games.len());

    let offers = client.fetch_category_offers("/chips/6/").await?;
    println!("Found {} offers", offers.len());

    Ok(())
}
```

## Builder Pattern

```rust
use funpay_rs::client::FunPayClient;
use funpay_rs::retry::{RetryPolicy, RateLimiter};

let client = FunPayClient::builder()
    .base_url("https://funpay.com")
    .golden_key("your-golden-key")
    .timeout(60)
    .retry_policy(RetryPolicy {
        max_retries: 5,
        base_delay_ms: 500,
        max_delay_ms: 30_000,
    })
    .rate_limiter(RateLimiter {
        requests_per_second: 1.0,
        min_interval_ms: 1000,
    })
    .build()?;
```

## SearchQuery

```rust
use funpay_rs::client::FunPayClient;
use funpay_rs::search::SearchQuery;
use funpay_rs::models::Currency;

let client = FunPayClient::new()?;
let offers = SearchQuery::new("CS2 skins")
    .max_price(100.0)
    .currency(Currency::USD)
    .online_only()
    .min_stock(5)
    .execute(&client)
    .await?;
```

## Streaming

```rust
use funpay_rs::client::FunPayClient;
use funpay_rs::stream::search_stream;
use futures::StreamExt;

let client = FunPayClient::new()?;
let mut stream = search_stream(&client, "Rust skins", 500.0);

while let Some(offer) = stream.next().await {
    println!("{}: {} {}", offer.seller.name, offer.price, offer.currency);
}
```

## Error Handling

```rust
use funpay_rs::error::FunPayError;

match client.fetch_all_games().await {
    Ok(games) => { /* ... */ }
    Err(FunPayError::RateLimited { retry_after }) => {
        eprintln!("Rate limited, retry after {:?}", retry_after);
    }
    Err(FunPayError::Timeout(dur)) => {
        eprintln!("Request timed out after {:?}", dur);
    }
    Err(FunPayError::Blocked(reason)) => {
        eprintln!("Blocked: {}", reason);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

## API Reference

| Module | Types |
|--------|-------|
| `client` | `FunPayClient`, `FunPayClientBuilder` |
| `search` | `SearchQuery` |
| `stream` | `search_stream()` |
| `models` | `Offer`, `Order`, `OrderStatus`, `Chat`, `ChatMessage`, `Review`, `Game`, `Seller`, `User`, `Currency`, `Lot`, `OnlineStatus` |
| `error` | `FunPayError`, `ParseError`, `AuthError` |
| `retry` | `RetryPolicy`, `RateLimiter` |
| `parser` | `Parser` |
| `monitor` | `Monitor`, `MonitorEvent` |
| `auth` | CSRF token extraction |

## Code Generation

This SDK is generated from [funpay-spec](../funpay-spec/spec/funpay.yaml) using [funpay-codegen](../funpay-codegen/).

### Regenerate from spec

```sh
# Using Makefile (recommended)
make generate    # Generate only
make build       # Generate + build
make test        # Generate + test

# Using build script
bash build.sh

# Using codegen directly
cd funpay-codegen
cargo run -- --spec ../funpay-spec/spec/funpay.yaml --output ../funpay-rs/generated
```

### Project structure

```
funpay-spec/       → OpenAPI-like spec (funpay.yaml)
funpay-codegen/    → Rust code generator
funpay-rs/         → Generated + hand-written SDK
  src/             → Hand-written modules (auth, middleware, retry, etc.)
  generated/       → Auto-generated code (models, parser, client, error)
  build.sh         → Generate + merge script
```

### Hand-written modules

These modules are maintained manually in `src/` and copied into `generated/` by `build.sh`:

- `auth.rs` — CSRF token extraction
- `cookies.rs` — Cookie persistence
- `middleware.rs` — Request middleware trait
- `retry.rs` — Exponential backoff & rate limiter
- `search.rs` — SearchQuery builder
- `stream.rs` — Async streaming (feature-gated)
- `export.rs` — JSON/CSV export (feature-gated)
- `ws.rs` — WebSocket support (feature-gated)
- `ua.rs` — User-Agent rotation
- `monitor.rs` — Real-time price monitoring

## Testing

Run the full test suite (71 tests):

```sh
cargo test
```

Tests include mock HTTP layer tests, parser edge cases, search filtering, auth, monitor events, and integration tests.

## Consumers

- [funpay-deal-finder](https://github.com/vitkuz573/funpay-deal-finder) — CLI tool for finding the best deals on FunPay with configurable filters
- [funpay-cli](https://github.com/vitkuz573/funpay-cli) — CLI tool for interacting with FunPay.com (search, games, sellers, export)

## License

MIT
