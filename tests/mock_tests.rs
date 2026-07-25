use funpay_rs::client::FunPayClient;
use funpay_rs::models::Currency;
use funpay_rs::search::SearchQuery;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn test_game_list_html() -> &'static str {
    r#"<html><body>
        <a href="/chips/91/">Lost Ark</a>
        <a href="/lots/332/">CS2</a>
    </body></html>"#
}

fn test_offers_html() -> &'static str {
    r#"<html><body>
        <a class="tc-item" href="/lot?id=1001" data-online="1">
            <div class="tc-price"><div>250.00</div><span class="unit">₽</span></div>
            <div class="tc-amount" data-s="5"></div>
            <div class="media-user-name">SellerOne</div>
            <div class="rating-mini-count">128</div>
            <div class="tc-server">RU</div>
            <div class="tc-desc-text">Fast delivery</div>
        </a>
        <a class="tc-item" href="/lot?id=1002" data-online="0">
            <div class="tc-price"><div>150.50</div><span class="unit">$</span></div>
            <div class="tc-amount" data-s="1"></div>
            <div class="media-user-name">SellerTwo</div>
            <div class="rating-mini-count">42</div>
            <div class="tc-server">EU</div>
            <div class="tc-desc-text">Reliable</div>
        </a>
    </body></html>"#
}

#[tokio::test]
async fn test_fetch_games_mock() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(ResponseTemplate::new(200).set_body_string(test_game_list_html()))
        .mount(&mock_server)
        .await;

    let client = FunPayClient::builder()
        .base_url(&mock_server.uri())
        .build()
        .unwrap();

    let games = client.fetch_all_games().await.unwrap();
    assert_eq!(games.len(), 2);
    assert_eq!(games[0].name, "Lost Ark");
    assert!(games[0].chips_url.is_some());
    assert_eq!(games[1].name, "CS2");
    assert!(games[1].lots_url.is_some());
}

#[tokio::test]
async fn test_fetch_category_offers_mock() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/chips/91/"))
        .respond_with(ResponseTemplate::new(200).set_body_string(test_offers_html()))
        .mount(&mock_server)
        .await;

    let client = FunPayClient::builder()
        .base_url(&mock_server.uri())
        .build()
        .unwrap();

    let offers = client.fetch_category_offers("/chips/91/").await.unwrap();
    assert_eq!(offers.len(), 2);

    assert_eq!(offers[0].offer_id.0, "1001");
    assert_eq!(offers[0].price, 250.0);
    assert_eq!(offers[0].currency, Currency::RUB);
    assert_eq!(offers[0].stock, 5);
    assert_eq!(offers[0].seller.name, "SellerOne");
    assert!(offers[0].seller.online);

    assert_eq!(offers[1].offer_id.0, "1002");
    assert_eq!(offers[1].price, 150.5);
    assert_eq!(offers[1].currency, Currency::USD);
    assert_eq!(offers[1].stock, 1);
    assert_eq!(offers[1].seller.name, "SellerTwo");
    assert!(!offers[1].seller.online);
}

#[tokio::test]
async fn test_fetch_games_empty_response() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(ResponseTemplate::new(200).set_body_string(""))
        .mount(&mock_server)
        .await;

    let client = FunPayClient::builder()
        .base_url(&mock_server.uri())
        .build()
        .unwrap();

    let games = client.fetch_all_games().await.unwrap();
    assert!(games.is_empty());
}

#[tokio::test]
async fn test_fetch_games_http_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(ResponseTemplate::new(500).set_body_string("Internal Server Error"))
        .mount(&mock_server)
        .await;

    let client = FunPayClient::builder()
        .base_url(&mock_server.uri())
        .retry_policy(funpay_rs::retry::RetryPolicy {
            max_retries: 0,
            base_delay_ms: 100,
            max_delay_ms: 1000,
        })
        .build()
        .unwrap();

    let result = client.fetch_all_games().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_search_with_mock_server() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(ResponseTemplate::new(200).set_body_string(test_game_list_html()))
        .mount(&mock_server)
        .await;

    let mock_uri = mock_server.uri();

    Mock::given(method("GET"))
        .and(path("/chips/91/"))
        .respond_with(ResponseTemplate::new(200).set_body_string(test_offers_html()))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/lots/332/"))
        .respond_with(ResponseTemplate::new(200).set_body_string(test_offers_html()))
        .mount(&mock_server)
        .await;

    let client = FunPayClient::builder()
        .base_url(&mock_uri)
        .build()
        .unwrap();

    let offers = client.search_all_categories("Lost", 500.0).await.unwrap();
    assert!(!offers.is_empty());
    for offer in &offers {
        assert!(offer.price <= 500.0);
    }
}

#[tokio::test]
async fn test_search_query_with_mock() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(ResponseTemplate::new(200).set_body_string(test_game_list_html()))
        .mount(&mock_server)
        .await;

    let mock_uri = mock_server.uri();

    Mock::given(method("GET"))
        .and(path("/chips/91/"))
        .respond_with(ResponseTemplate::new(200).set_body_string(test_offers_html()))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/lots/332/"))
        .respond_with(ResponseTemplate::new(200).set_body_string(test_offers_html()))
        .mount(&mock_server)
        .await;

    let client = FunPayClient::builder()
        .base_url(&mock_uri)
        .build()
        .unwrap();

    let offers = SearchQuery::new("Lost")
        .max_price(300.0)
        .currency(Currency::RUB)
        .execute(&client)
        .await
        .unwrap();

    assert!(!offers.is_empty());
    for offer in &offers {
        assert!(offer.price <= 300.0);
        assert_eq!(offer.currency, Currency::RUB);
    }
}

#[tokio::test]
async fn test_search_online_only_with_mock() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(ResponseTemplate::new(200).set_body_string(test_game_list_html()))
        .mount(&mock_server)
        .await;

    let mock_uri = mock_server.uri();

    Mock::given(method("GET"))
        .and(path("/chips/91/"))
        .respond_with(ResponseTemplate::new(200).set_body_string(test_offers_html()))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/lots/332/"))
        .respond_with(ResponseTemplate::new(200).set_body_string(test_offers_html()))
        .mount(&mock_server)
        .await;

    let client = FunPayClient::builder()
        .base_url(&mock_uri)
        .build()
        .unwrap();

    let offers = SearchQuery::new("Lost")
        .online_only()
        .execute(&client)
        .await
        .unwrap();

    assert!(!offers.is_empty());
    for offer in &offers {
        assert!(offer.seller.online);
    }
}

#[tokio::test]
async fn test_search_min_stock_with_mock() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(ResponseTemplate::new(200).set_body_string(test_game_list_html()))
        .mount(&mock_server)
        .await;

    let mock_uri = mock_server.uri();

    Mock::given(method("GET"))
        .and(path("/chips/91/"))
        .respond_with(ResponseTemplate::new(200).set_body_string(test_offers_html()))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/lots/332/"))
        .respond_with(ResponseTemplate::new(200).set_body_string(test_offers_html()))
        .mount(&mock_server)
        .await;

    let client = FunPayClient::builder()
        .base_url(&mock_uri)
        .build()
        .unwrap();

    let offers = SearchQuery::new("Lost")
        .min_stock(3)
        .execute(&client)
        .await
        .unwrap();

    assert!(!offers.is_empty());
    for offer in &offers {
        assert!(offer.stock >= 3);
    }
}

#[tokio::test]
async fn test_fetch_games_single_game() {
    let html = r#"<html><body>
        <a href="/chips/42/">Dota 2</a>
    </body></html>"#;

    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(ResponseTemplate::new(200).set_body_string(html))
        .mount(&mock_server)
        .await;

    let client = FunPayClient::builder()
        .base_url(&mock_server.uri())
        .build()
        .unwrap();

    let games = client.fetch_all_games().await.unwrap();
    assert_eq!(games.len(), 1);
    assert_eq!(games[0].name, "Dota 2");
    assert_eq!(games[0].id.0, "42");
}

#[tokio::test]
async fn test_fetch_games_no_matching_links() {
    let html = r#"<html><body>
        <a href="/other/123/">Not a game</a>
        <div>Some random content</div>
    </body></html>"#;

    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(ResponseTemplate::new(200).set_body_string(html))
        .mount(&mock_server)
        .await;

    let client = FunPayClient::builder()
        .base_url(&mock_server.uri())
        .build()
        .unwrap();

    let games = client.fetch_all_games().await.unwrap();
    assert!(games.is_empty());
}

#[tokio::test]
async fn test_fetch_offers_empty_page() {
    let html = r#"<html><body></body></html>"#;

    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/chips/91/"))
        .respond_with(ResponseTemplate::new(200).set_body_string(html))
        .mount(&mock_server)
        .await;

    let client = FunPayClient::builder()
        .base_url(&mock_server.uri())
        .build()
        .unwrap();

    let offers = client.fetch_category_offers("/chips/91/").await.unwrap();
    assert!(offers.is_empty());
}

#[tokio::test]
async fn test_search_no_games_found() {
    let html = r#"<html><body>
        <a href="/other/1/">Something else</a>
    </body></html>"#;

    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(ResponseTemplate::new(200).set_body_string(html))
        .mount(&mock_server)
        .await;

    let client = FunPayClient::builder()
        .base_url(&mock_server.uri())
        .build()
        .unwrap();

    let offers = client.search_all_categories("Nonexistent", 100.0).await.unwrap();
    assert!(offers.is_empty());
}
