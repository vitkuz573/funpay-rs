use criterion::{black_box, criterion_group, criterion_main, Criterion};
use funpay_sdk::parser::Parser;

fn bench_parse_game_list_100(c: &mut Criterion) {
    let html = generate_game_list_html(100);
    let parser = Parser::new();
    c.bench_function("parse_game_list_100", |b| {
        b.iter(|| parser.parse_game_list(black_box(&html)))
    });
}

fn bench_parse_game_list_1000(c: &mut Criterion) {
    let html = generate_game_list_html(1000);
    let parser = Parser::new();
    c.bench_function("parse_game_list_1000", |b| {
        b.iter(|| parser.parse_game_list(black_box(&html)))
    });
}

fn bench_parse_offers_100(c: &mut Criterion) {
    let html = generate_offers_html(100);
    let parser = Parser::new();
    c.bench_function("parse_offers_100", |b| {
        b.iter(|| parser.parse_category_offers(black_box(&html)))
    });
}

fn bench_parse_offers_1000(c: &mut Criterion) {
    let html = generate_offers_html(1000);
    let parser = Parser::new();
    c.bench_function("parse_offers_1000", |b| {
        b.iter(|| parser.parse_category_offers(black_box(&html)))
    });
}

fn bench_parse_chat_messages(c: &mut Criterion) {
    let html = generate_chat_messages_html(200);
    let parser = Parser::new();
    c.bench_function("parse_chat_messages_200", |b| {
        b.iter(|| parser.parse_chat_messages(black_box(&html)))
    });
}

fn bench_parse_seller_profile(c: &mut Criterion) {
    let html = generate_seller_profile_html();
    let parser = Parser::new();
    c.bench_function("parse_seller_profile", |b| {
        b.iter(|| parser.parse_seller_profile(black_box(&html)))
    });
}

fn bench_parse_user_offers(c: &mut Criterion) {
    let html = generate_user_offers_html(100);
    let parser = Parser::new();
    c.bench_function("parse_user_offers_100", |b| {
        b.iter(|| parser.parse_user_offers(black_box(&html)))
    });
}

fn generate_game_list_html(count: usize) -> String {
    let mut html = String::from("<html><body><div class='game-list'>");
    for i in 0..count {
        html.push_str(&format!(
            r#"<div class="game-title"><a href="/game/{}/" class="game-title" data-game-id="{}">Game {}</a><img class="game-icon" src="/icon{}.png"></div>"#,
            i, i, i, i
        ));
    }
    html.push_str("</div></body></html>");
    html
}

fn generate_offers_html(count: usize) -> String {
    let mut html = String::from("<html><body>");
    for i in 0..count {
        html.push_str(&format!(
            r#"<a class="tc-item" data-order="{}" data-user-id="{}" data-mark="single">
                <div class="tc-server">Server {}</div>
                <div class="tc-price">{}</div>
                <div class="tc-desc-text">Item {} description with some details</div>
            </a>"#,
            i, i, i, i as f64 * 10.5, i
        ));
    }
    html.push_str("</body></html>");
    html
}

fn generate_chat_messages_html(count: usize) -> String {
    let mut html = String::from("<html><body>");
    for i in 0..count {
        html.push_str(&format!(
            r#"<div class="msg" data-msg-id="{}" data-sender-id="{}" data-self="{}">
                <div class="msg-text">Message {} with some content</div>
                <div class="msg-date">2024-01-{} {}:{:02}</div>
            </div>"#,
            i, i % 2, i % 2 == 0, i, (i % 28) + 1, i % 24, i % 60
        ));
    }
    html.push_str("</body></html>");
    html
}

fn generate_seller_profile_html() -> String {
    r#"<div class="seller-avatar"><img src="https://example.com/avatar.png"></div>
        <div class="seller-info">
            <span class="seller-name">TestSeller</span>
            <span data-user-id="42"></span>
        </div>
        <span class="seller-reviews">150</span>
        <span class="seller-online">online</span>
        <span class="seller-rating">4.8</span>
        <span class="seller-response-time">30 minutes</span>"#.to_string()
}

fn generate_user_offers_html(count: usize) -> String {
    let mut html = String::from("<html><body>");
    for i in 0..count {
        html.push_str(&format!(
            r#"<a class="tc-item" data-order="{}">
                <div class="tc-server">Server {}</div>
                <div class="tc-price">{}</div>
                <div class="tc-desc-text">Lot {} description</div>
            </a>"#,
            i, i, i as f64 * 5.0, i
        ));
    }
    html.push_str("</body></html>");
    html
}

criterion_group!(
    benches,
    bench_parse_game_list_100,
    bench_parse_game_list_1000,
    bench_parse_offers_100,
    bench_parse_offers_1000,
    bench_parse_chat_messages,
    bench_parse_seller_profile,
    bench_parse_user_offers,
);
criterion_main!(benches);
