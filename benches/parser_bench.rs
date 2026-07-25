use criterion::{black_box, criterion_group, criterion_main, Criterion};
use funpay_rs::parser::Parser;

fn bench_parse_game_list(c: &mut Criterion) {
    let html = generate_game_list_html(100);
    let parser = Parser::new();
    c.bench_function("parse_game_list_100", |b| {
        b.iter(|| parser.parse_game_list(black_box(&html)))
    });
}

fn bench_parse_offers(c: &mut Criterion) {
    let html = generate_offers_html(200);
    let parser = Parser::new();
    c.bench_function("parse_offers_200", |b| {
        b.iter(|| parser.parse_offers_from_page(black_box(&html)))
    });
}

fn generate_game_list_html(count: usize) -> String {
    let mut html = String::from("<html><body>");
    for i in 0..count {
        html.push_str(&format!("<a href=\"/chips/{}/\">Game {}</a>", i, i));
    }
    html.push_str("</body></html>");
    html
}

fn generate_offers_html(count: usize) -> String {
    let mut html = String::from("<html><body>");
    for i in 0..count {
        html.push_str(&format!(
            r#"<a href="https://funpay.com/lots/offer?id={}" class="tc-item" data-online="1">
                <div class="tc-server">Server {}</div>
                <div class="tc-price" data-s="{}"><div>{} ₽</div></div>
                <div class="tc-amount" data-s="100">100</div>
                <div class="media-user-name">Seller{}</div>
                <div class="rating-mini-count">50</div>
            </a>"#,
            i, i, i as f64 * 10.0, i, i
        ));
    }
    html.push_str("</body></html>");
    html
}

criterion_group!(benches, bench_parse_game_list, bench_parse_offers);
criterion_main!(benches);
