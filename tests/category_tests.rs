use funpay_sdk::parser::Parser;

#[test]
fn test_parse_game_list_empty() {
    let html = r#"<html><body></body></html>"#;
    let parser = Parser::new();
    let games = parser.parse_game_list(html);
    assert!(games.is_empty());
}

#[test]
fn test_parse_game_list_with_games() {
    let html = r#"<div class="game-list">
        <div class="game-title">
            <a href="/chips/91/" class="game-title" data-game-id="91">Lost Ark</a>
            <img class="game-icon" src="/icon1.png">
        </div>
        <div class="game-title">
            <a href="/lots/332/" class="game-title" data-game-id="332">CS2</a>
            <img class="game-icon" src="/icon2.png">
        </div>
    </div>"#;
    let parser = Parser::new();
    let games = parser.parse_game_list(html);
    let games_with_title: Vec<_> = games.iter().filter(|g| !g.title.is_empty()).collect();
    assert_eq!(games_with_title.len(), 2);
    assert_eq!(games_with_title[0].title, "Lost Ark");
    assert_eq!(games_with_title[1].title, "CS2");
    assert_eq!(games_with_title[0].url, "/chips/91/");
    assert_eq!(games_with_title[1].url, "/lots/332/");
}

#[test]
fn test_parse_game_list_with_icon() {
    let html = r#"<div class="game-list">
        <div class="game-title">
            <a href="/game/1/" class="game-title" data-game-id="1">Game</a>
            <img class="game-icon" src="https://cdn.example.com/icon.png">
        </div>
    </div>"#;
    let parser = Parser::new();
    let games = parser.parse_game_list(html);
    let with_icon: Vec<_> = games.iter().filter(|g| g.icon_url.is_some()).collect();
    assert_eq!(with_icon.len(), 1);
}

#[test]
fn test_parse_game_list_many_games() {
    let mut html = String::from("<div class='game-list'>");
    for i in 0..1000 {
        html.push_str(&format!(
            r#"<div class="game-title"><a href="/game/{}/" class="game-title" data-game-id="{}">Game {}</a></div>"#,
            i, i, i
        ));
    }
    html.push_str("</div>");
    let parser = Parser::new();
    let games = parser.parse_game_list(&html);
    let with_title: Vec<_> = games.iter().filter(|g| !g.title.is_empty()).collect();
    assert_eq!(with_title.len(), 1000);
}
