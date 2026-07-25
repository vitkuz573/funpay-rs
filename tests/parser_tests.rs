use funpay_sdk::parser::Parser;
use funpay_sdk::models::Price;

#[test]
fn test_parse_category_offers_empty() {
    let html = r#"<html><body></body></html>"#;
    let parser = Parser::new();
    let offers = parser.parse_category_offers(html);
    assert!(offers.is_empty());
}

#[test]
fn test_parse_category_offers_with_data() {
    let html = r#"<a class="tc-item" data-order="101" data-user-id="202" data-mark="single">
        <div class="tc-price">150</div>
        <div class="tc-server">EU</div>
        <div class="tc-desc-text">Gold</div>
    </a>"#;
    let parser = Parser::new();
    let offers = parser.parse_category_offers(html);
    assert_eq!(offers.len(), 1);
    assert_eq!(offers[0].price, Price(150.0));
    assert_eq!(offers[0].server.as_deref(), Some("EU"));
    assert_eq!(offers[0].description.as_deref(), Some("Gold"));
}

#[test]
fn test_parse_category_offers_multiple() {
    let html = r#"<a class="tc-item">
        <div class="tc-price">100</div>
        <div class="tc-server">RU</div>
    </a>
    <a class="tc-item">
        <div class="tc-price">200</div>
        <div class="tc-server">EU</div>
    </a>"#;
    let parser = Parser::new();
    let offers = parser.parse_category_offers(html);
    assert_eq!(offers.len(), 2);
    assert_eq!(offers[0].price, Price(100.0));
    assert_eq!(offers[1].price, Price(200.0));
}

#[test]
fn test_parse_seller_profile() {
    let html = r#"<div class="seller-avatar"><img src="avatar.png"></div>
        <div class="seller-info">
            <span class="seller-name">TestSeller</span>
            <span data-user-id="12345"></span>
        </div>
        <span class="seller-reviews">100</span>
        <span class="seller-online">online</span>
        <span class="seller-rating">4.5</span>"#;
    let parser = Parser::new();
    let seller = parser.parse_seller_profile(html);
    assert!(seller.is_some());
    let seller = seller.unwrap();
    assert_eq!(seller.name, "TestSeller");
    assert_eq!(seller.rating, 4.5);
    assert_eq!(seller.reviews_count, 100);
}

#[test]
fn test_parse_game_list() {
    let html = r#"<div class="game-list">
        <div class="game-title">
            <a href="/chips/91/" class="game-title" data-game-id="91">Lost Ark</a>
            <img class="game-icon" src="/icon1.png">
        </div>
    </div>"#;
    let parser = Parser::new();
    let games = parser.parse_game_list(html);
    let with_title: Vec<_> = games.iter().filter(|g| !g.title.is_empty()).collect();
    assert_eq!(with_title.len(), 1);
    assert_eq!(with_title[0].title, "Lost Ark");
    assert_eq!(with_title[0].url, "/chips/91/");
}

#[test]
fn test_parse_chat_messages() {
    let html = r#"<div class="msg" data-msg-id="501" data-sender-id="101" data-self="1">
        <div class="msg-text">Hello!</div>
        <div class="msg-date">12:00</div>
    </div>"#;
    let parser = Parser::new();
    let msgs = parser.parse_chat_messages(html);
    assert_eq!(msgs.len(), 1);
    assert_eq!(msgs[0].text, "Hello!");
}

#[test]
fn test_parse_user_offers() {
    let html = r#"<a class="tc-item" data-order="501">
        <div class="tc-server">EU</div>
        <div class="tc-price">99.5</div>
        <div class="tc-desc-text">Rare item</div>
    </a>"#;
    let parser = Parser::new();
    let lots = parser.parse_user_offers(html);
    assert_eq!(lots.len(), 1);
    assert_eq!(lots[0].price, Price(99.5));
    assert_eq!(lots[0].server.as_deref(), Some("EU"));
}

#[test]
fn test_parse_chats() {
    let html = r#"<div class="chat-item" data-chat-id="701" data-chat-type="user">
        <div class="chat-last-message">Hey there</div>
        <div class="chat-unread">3</div>
        <div class="chat-date">Today</div>
    </div>"#;
    let parser = Parser::new();
    let chats = parser.parse_chats(html);
    assert_eq!(chats.len(), 1);
    assert_eq!(chats[0].last_message.as_deref(), Some("Hey there"));
    assert_eq!(chats[0].unread_count, 3);
}

#[test]
fn test_parse_orders() {
    let html = r#"<div class="order-item" data-order-id="501">
        <div class="order-price">25.50<div class="currency">USD</div></div>
        <div class="order-game">CS2</div>
        <div class="order-status">completed</div>
        <div class="order-date">2024-01-15</div>
    </div>"#;
    let parser = Parser::new();
    let orders = parser.parse_orders(html);
    assert_eq!(orders.len(), 1);
    assert_eq!(orders[0].game.as_deref(), Some("CS2"));
}

#[test]
fn test_parse_reviews() {
    let html = r#"<div class="review-item" data-review-id="301">
        <div class="review-text">Great seller!</div>
        <div class="review-date">2024-01-10</div>
        <div class="review-rating">5.0</div>
    </div>"#;
    let parser = Parser::new();
    let reviews = parser.parse_reviews(html);
    assert_eq!(reviews.len(), 1);
    assert_eq!(reviews[0].text.as_deref(), Some("Great seller!"));
    assert_eq!(reviews[0].rating, 5.0);
}

#[test]
fn test_parse_user_profile() {
    let html = r#"<div class="profile-avatar"><img src="avatar.png"></div>
        <div class="profile-title">Gamer123</div>
        <div class="user-status">online</div>
        <div class="profile-user-id" data-user-id="999"></div>
        <div class="profile-regdate">15.01.2020</div>"#;
    let parser = Parser::new();
    let user = parser.parse_user_profile(html);
    assert!(user.is_some());
    let user = user.unwrap();
    assert_eq!(user.username, "Gamer123");
    assert_eq!(user.status.as_deref(), Some("online"));
    assert_eq!(user.registration_date.as_deref(), Some("15.01.2020"));
}

#[test]
fn test_parse_user_profile_minimal() {
    let html = r#"<div class="profile-title">MinimalUser</div>"#;
    let parser = Parser::new();
    let user = parser.parse_user_profile(html);
    assert!(user.is_some());
    let user = user.unwrap();
    assert_eq!(user.username, "MinimalUser");
    assert!(user.status.is_none());
}
