//! Integration tests against real FunPay.com
//! All tests are marked #[ignore] — run with: cargo test --features full -- --ignored

use funpay_rs::client::FunPayClient;
use funpay_rs::search::SearchQuery;
use funpay_rs::models::Currency;

#[tokio::test]
#[ignore]
async fn test_fetch_games_real() {
    let client = FunPayClient::new().unwrap();
    let games = client.fetch_all_games().await.unwrap();
    assert!(!games.is_empty(), "Should find games on FunPay");
    println!("Found {} games", games.len());

    // Verify some known games exist
    let names: Vec<&str> = games.iter().map(|g| g.name.as_str()).collect();
    assert!(
        names.iter().any(|n| n.contains("Dota") || n.contains("CS") || n.contains("Fortnite")),
        "Expected to find Dota, CS, or Fortnite among {} games: {:?}",
        names.len(),
        &names[..5.min(names.len())]
    );
}

#[tokio::test]
#[ignore]
async fn test_fetch_offers_real() {
    let client = FunPayClient::new().unwrap();
    let games = client.fetch_all_games().await.unwrap();

    // Find a game with chips
    let game = games
        .iter()
        .find(|g| g.chips_url.is_some())
        .expect("Should have at least one game with chips");
    let url = game.chips_url.as_ref().unwrap();

    let offers = client.fetch_category_offers(url.as_str()).await.unwrap();
    assert!(!offers.is_empty(), "Should find offers on {}", game.name);
    println!("{}: {} offers", game.name, offers.len());

    // Verify offer structure
    for offer in offers.iter().take(5) {
        assert!(!offer.offer_id.as_str().is_empty());
        assert!(offer.price > rust_decimal::Decimal::ZERO);
    }
}

#[tokio::test]
#[ignore]
async fn test_search_real() {
    let client = FunPayClient::new().unwrap();
    let results = SearchQuery::new("gold")
        .max_price(1000.0)
        .execute(&client)
        .await
        .unwrap();

    println!("Found {} gold offers under 1000", results.len());
}

#[tokio::test]
#[ignore]
async fn test_fetch_user_profile_real() {
    let client = FunPayClient::new().unwrap();
    let games = client.fetch_all_games().await.unwrap();
    let game = games
        .iter()
        .find(|g| g.chips_url.is_some())
        .unwrap();
    let offers = client
        .fetch_category_offers(game.chips_url.as_ref().unwrap().as_str())
        .await
        .unwrap();
    let offer = offers.first().expect("Should have offers");

    // Verify seller info is present in the offer
    assert!(!offer.seller.user_id.as_str().is_empty());
    assert!(!offer.seller.name.is_empty());
    assert!(offer.seller.rating >= 0.0);
}

#[tokio::test]
#[ignore]
async fn test_pagination_real() {
    let client = FunPayClient::new().unwrap();
    let games = client.fetch_all_games().await.unwrap();
    let game = games
        .iter()
        .find(|g| g.chips_url.is_some())
        .unwrap();
    let url = game.chips_url.as_ref().unwrap();

    // Single page
    let single_page = client.fetch_category_offers(url.as_str()).await.unwrap();
    // All pages
    let all_pages = client.fetch_all_category_offers(url.as_str()).await.unwrap();

    println!(
        "Single page: {}, All pages: {}",
        single_page.len(),
        all_pages.len()
    );
    assert!(all_pages.len() >= single_page.len());
}

#[tokio::test]
#[ignore]
async fn test_fetch_offers_with_lots_real() {
    let client = FunPayClient::new().unwrap();
    let games = client.fetch_all_games().await.unwrap();

    // Find a game with lots
    let game = games
        .iter()
        .find(|g| g.lots_url.is_some())
        .expect("Should have at least one game with lots");
    let url = game.lots_url.as_ref().unwrap();

    let offers = client.fetch_category_offers(url.as_str()).await.unwrap();
    assert!(!offers.is_empty(), "Should find lot offers on {}", game.name);
    println!("{} lots: {} offers", game.name, offers.len());
}

#[tokio::test]
#[ignore]
async fn test_search_with_currency_real() {
    let client = FunPayClient::new().unwrap();
    let results = SearchQuery::new("skin")
        .max_price(50.0)
        .currency(Currency::USD)
        .execute(&client)
        .await
        .unwrap();

    println!("Found {} skin offers in USD under 50", results.len());
    for offer in results.iter().take(3) {
        assert_eq!(offer.currency, Currency::USD);
    }
}

#[tokio::test]
#[ignore]
async fn test_search_online_only_real() {
    let client = FunPayClient::new().unwrap();
    let results = SearchQuery::new("gold")
        .online_only()
        .execute(&client)
        .await
        .unwrap();

    println!("Found {} online-only gold offers", results.len());
    for offer in results.iter().take(5) {
        assert!(offer.seller.online, "Seller should be online");
    }
}
