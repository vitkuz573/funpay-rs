//! Mock integration tests using wiremock
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};
use funpay_sdk::client::FunPayClient;
use funpay_sdk::models::Price;

#[tokio::test]
async fn test_mock_fetch_games() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/lots/"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"<div class="game-list">
                <div class="game-title">
                    <a href="/chips/91/" class="game-title" data-game-id="91">Test Game</a>
                </div>
            </div>"#
        ))
        .mount(&mock_server)
        .await;

    let client = FunPayClient::with_base_url(&mock_server.uri()).unwrap();
    let games = client.fetch_game_list().await.unwrap();
    let games: Vec<_> = games.into_iter().filter(|g| !g.title.is_empty()).collect();
    assert_eq!(games.len(), 1);
    assert_eq!(games[0].title, "Test Game");
}

#[tokio::test]
async fn test_mock_fetch_category_offers() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/lots/1/1/"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"<a class="tc-item">
                <div class="tc-price">25.50</div>
                <div class="tc-server">EU</div>
                <div class="tc-desc-text">Gold</div>
            </a>"#
        ))
        .mount(&mock_server)
        .await;

    let client = FunPayClient::with_base_url(&mock_server.uri()).unwrap();
    let offers = client.fetch_category_offers(1, 1).await.unwrap();
    assert_eq!(offers.len(), 1);
    assert_eq!(offers[0].price, Price(25.5));
    assert_eq!(offers[0].server.as_deref(), Some("EU"));
}

#[tokio::test]
async fn test_mock_fetch_chats() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/chats/"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"<div class="chat-item" data-chat-id="101" data-chat-type="user">
                <div class="chat-last-message">Hello</div>
                <div class="chat-unread">2</div>
            </div>"#
        ))
        .mount(&mock_server)
        .await;

    let client = FunPayClient::with_base_url(&mock_server.uri()).unwrap();
    let chats = client.fetch_chats().await.unwrap();
    assert_eq!(chats.len(), 1);
    assert_eq!(chats[0].last_message.as_deref(), Some("Hello"));
    assert_eq!(chats[0].unread_count, 2);
}

#[tokio::test]
async fn test_mock_fetch_chat_messages() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/chats/101/"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"<div class="msg" data-msg-id="501" data-sender-id="201" data-self="false">
                <div class="msg-text">Hi there!</div>
                <div class="msg-date">14:30</div>
            </div>"#
        ))
        .mount(&mock_server)
        .await;

    let client = FunPayClient::with_base_url(&mock_server.uri()).unwrap();
    let msgs = client.fetch_chat_messages(101).await.unwrap();
    assert_eq!(msgs.len(), 1);
    assert_eq!(msgs[0].text, "Hi there!");
}

#[tokio::test]
async fn test_mock_empty_response() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/lots/"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"<html><body></body></html>"#
        ))
        .mount(&mock_server)
        .await;

    let client = FunPayClient::with_base_url(&mock_server.uri()).unwrap();
    let games = client.fetch_game_list().await.unwrap();
    assert!(games.is_empty());
}

#[tokio::test]
async fn test_mock_http_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/lots/"))
        .respond_with(ResponseTemplate::new(500).set_body_string("Internal Server Error"))
        .mount(&mock_server)
        .await;

    let client = FunPayClient::with_base_url(&mock_server.uri()).unwrap();
    let result = client.fetch_game_list().await;
    // reqwest returns Ok even for 500 (status is in the response, not an error)
    // The parser returns an empty list from the error body
    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}

#[tokio::test]
async fn test_mock_404() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/users/999/"))
        .respond_with(ResponseTemplate::new(404).set_body_string("Not Found"))
        .mount(&mock_server)
        .await;

    let client = FunPayClient::with_base_url(&mock_server.uri()).unwrap();
    let result = client.fetch_seller_profile(999).await;
    // Parser returns Some with default fields when page has no seller data
    assert!(result.is_ok());
    let seller = result.unwrap();
    assert!(seller.name.is_empty());
}

#[tokio::test]
async fn test_mock_multiple_offers() {
    let mock_server = MockServer::start().await;

    let mut body = String::new();
    for i in 0..50 {
        body.push_str(&format!(
            r#"<a class="tc-item"><div class="tc-price">{}</div><div class="tc-server">S{}</div></a>"#,
            i as f64 * 10.0, i
        ));
    }

    Mock::given(method("GET"))
        .and(path("/lots/1/1/"))
        .respond_with(ResponseTemplate::new(200).set_body_string(body))
        .mount(&mock_server)
        .await;

    let client = FunPayClient::with_base_url(&mock_server.uri()).unwrap();
    let offers = client.fetch_category_offers(1, 1).await.unwrap();
    assert_eq!(offers.len(), 50);
}
