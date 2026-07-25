//! Integration tests against real FunPay.com
//! All tests are marked #[ignore] — run with: cargo test -- --ignored

use funpay_sdk::client::FunPayClient;

#[tokio::test]
#[ignore]
async fn test_fetch_games_real() {
    let client = FunPayClient::new().unwrap();
    let games = client.fetch_game_list().await.unwrap();
    assert!(!games.is_empty(), "Should find games on FunPay");
    println!("Found {} games", games.len());

    let titles: Vec<&str> = games.iter().map(|g| g.title.as_str()).collect();
    assert!(
        titles.iter().any(|n| n.contains("Dota") || n.contains("CS") || n.contains("Fortnite")),
        "Expected to find Dota, CS, or Fortnite among {} games: {:?}",
        titles.len(),
        &titles[..5.min(titles.len())]
    );
}

#[tokio::test]
#[ignore]
async fn test_fetch_offers_real() {
    let client = FunPayClient::new().unwrap();
    let games = client.fetch_game_list().await.unwrap();
    let game = games.first().expect("Should have at least one game");

    let game_id = game.id.inner();
    let offers = client.fetch_category_offers(game_id, 1).await.unwrap();
    assert!(!offers.is_empty(), "Should find offers on {}", game.title);
    println!("{}: {} offers", game.title, offers.len());

    for offer in offers.iter().take(5) {
        assert!(offer.price.0 > 0.0);
    }
}

#[tokio::test]
#[ignore]
async fn test_fetch_user_profile_real() {
    let client = FunPayClient::new().unwrap();
    let games = client.fetch_game_list().await.unwrap();
    let game = games.first().expect("Should have games");
    let game_id = game.id.inner();
    let offers = client.fetch_category_offers(game_id, 1).await.unwrap();
    let offer = offers.first().expect("Should have offers");

    assert!(!offer.seller_id.to_string().is_empty());
}
