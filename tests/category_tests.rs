use funpay_rs::parser::Parser;

#[test]
fn test_parse_game_list() {
    let html = r#"
    <a href="/chips/91/" class="promo-game">Lost Ark</a>
    <a href="/lots/332/" class="promo-game">CS2</a>
    "#;
    let parser = Parser::new();
    let games = parser.parse_game_list(html);
    assert_eq!(games.len(), 2);
    assert_eq!(games[0].name, "Lost Ark");
}
