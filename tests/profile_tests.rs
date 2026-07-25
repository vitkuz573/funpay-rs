use funpay_sdk::parser::Parser;

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
    assert!(user.registration_date.is_none());
}

#[test]
fn test_parse_user_profile_online_status() {
    let html = r#"<div class="profile-title">User</div>
        <div class="profile-online">online</div>"#;
    let parser = Parser::new();
    let user = parser.parse_user_profile(html).unwrap();
    assert_eq!(user.online_status, funpay_sdk::models::OnlineStatus::Online);
}

#[test]
fn test_parse_user_profile_offline_status() {
    let html = r#"<div class="profile-title">User</div>
        <div class="profile-online">offline</div>"#;
    let parser = Parser::new();
    let user = parser.parse_user_profile(html).unwrap();
    assert_eq!(user.online_status, funpay_sdk::models::OnlineStatus::Offline);
}

#[test]
fn test_parse_seller_profile_full() {
    let html = r#"<div class="seller-avatar"><img src="https://example.com/avatar.png"></div>
        <div class="seller-info">
            <span class="seller-name">TopSeller</span>
            <span data-user-id="42"></span>
        </div>
        <span class="seller-reviews">500</span>
        <span class="seller-online">online</span>
        <span class="seller-rating">4.9</span>
        <span class="seller-response-time">1 hour</span>"#;
    let parser = Parser::new();
    let seller = parser.parse_seller_profile(html).unwrap();
    assert_eq!(seller.name, "TopSeller");
    assert_eq!(seller.rating, 4.9);
    assert_eq!(seller.reviews_count, 500);
    assert_eq!(seller.response_time.as_deref(), Some("1 hour"));
    assert_eq!(seller.avatar_url, "https://example.com/avatar.png");
}

#[test]
fn test_parse_seller_profile_empty() {
    let html = r#"<html><body></body></html>"#;
    let parser = Parser::new();
    let seller = parser.parse_seller_profile(html);
    assert!(seller.is_some());
    let seller = seller.unwrap();
    assert!(seller.name.is_empty());
    assert_eq!(seller.rating, 0.0);
}

#[test]
fn test_parse_user_profile_avatar() {
    let html = r#"<div class="profile-title">User</div>
        <img class="profile-avatar" src="https://cdn.example.com/pic.jpg">"#;
    let parser = Parser::new();
    let user = parser.parse_user_profile(html).unwrap();
    assert_eq!(user.avatar_url.as_deref(), Some("https://cdn.example.com/pic.jpg"));
}
