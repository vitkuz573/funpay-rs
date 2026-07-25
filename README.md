# funpay-rs

Unofficial Rust SDK for [FunPay.com](https://funpay.com) — P2P gaming marketplace.

## Features

- **Client** — HTTP client with cookie support
- **Parser** — Extract offers, games, users from HTML
- **Auth** — CSRF token extraction
- **Monitor** — Track price changes in real-time
- **Search** — Find offers across all categories by keyword

## Usage

```rust
use funpay_rs::client::FunPayClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = FunPayClient::new()?;
    
    // Fetch all games
    let games = client.fetch_all_games().await?;
    println!("Found {} games", games.len());
    
    // Fetch offers from a category
    let offers = client.fetch_category_offers("https://funpay.com/chips/6/").await?;
    println!("Found {} offers", offers.len());
    
    // Search across all categories
    let results = client.search_all_categories("kimi", 3000.0).await?;
    println!("Found {} matching offers", results.len());
    
    Ok(())
}
```

## Examples

```rust
use funpay_rs::client::FunPayClient;
use funpay_rs::monitor::{Monitor, MonitorEvent};

let client = FunPayClient::new()?;
let mut monitor = Monitor::new();

// Track price changes
let offers = vec![("1".to_string(), 100.0), ("2".to_string(), 200.0)];
let events = monitor.check_for_changes(offers);

for event in &events {
    match event {
        MonitorEvent::PriceChanged { offer_id, old_price, new_price } => {
            println!("{}: {} -> {}", offer_id, old_price, new_price);
        }
        MonitorEvent::NewOffer { offer_id } => {
            println!("New offer: {}", offer_id);
        }
        MonitorEvent::OfferRemoved { offer_id } => {
            println!("Removed: {}", offer_id);
        }
    }
}
```

## License

MIT
