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
