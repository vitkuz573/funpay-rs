#![no_main]

use libfuzzer_sys::fuzz_target;
use funpay_rs::parser::Parser;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let parser = Parser::new();
        let _ = parser.parse_offers_from_page(s);
        let _ = parser.parse_game_list(s);
    }
});
