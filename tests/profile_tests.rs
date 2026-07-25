use funpay_rs::parser::Parser;
use funpay_rs::models::UserId;

#[test]
fn test_parse_user_profile() {
    let html = r#"
    <div class="profile">
        <div class="profile-name">Gamer123</div>
        <div class="rating">4.8</div>
        <div class="reviews">156</div>
        <div class="online-status">online</div>
        <div class="reg-date">Jan 2020</div>
    </div>
    "#;
    let parser = Parser::new();
    let user = parser.parse_user(html, UserId("999".to_string()));
    assert!(user.is_some());
    let user = user.unwrap();
    assert_eq!(user.username, "Gamer123");
    assert_eq!(user.rating, 4.8);
    assert_eq!(user.reviews, 156);
    assert!(user.online);
}

#[test]
fn test_parse_user_offline() {
    let html = r#"
    <div class="profile">
        <div class="profile-name">OfflineUser</div>
        <div class="rating">3.5</div>
        <div class="reviews">10</div>
        <div class="online-status">offline</div>
    </div>
    "#;
    let parser = Parser::new();
    let user = parser.parse_user(html, UserId("888".to_string()));
    assert!(user.is_some());
    assert!(!user.unwrap().online);
}
