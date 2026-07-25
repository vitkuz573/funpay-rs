#[cfg(feature = "streaming")]
use async_stream::stream;
#[cfg(feature = "streaming")]
use futures::Stream;
#[cfg(feature = "streaming")]
use rust_decimal::Decimal;
#[cfg(feature = "streaming")]
use crate::client::FunPayClient;
#[cfg(feature = "streaming")]
use crate::models::Offer;

#[cfg(feature = "streaming")]
pub fn search_stream<'a>(
    client: &'a FunPayClient,
    keyword: &'a str,
    max_price: Decimal,
) -> impl Stream<Item = Offer> + 'a {
    stream! {
        let games = match client.fetch_all_games().await {
            Ok(games) => games,
            Err(_) => return,
        };

        let keyword_lower = keyword.to_lowercase();

        for game in games.iter().filter(|g| g.name.to_lowercase().contains(&keyword_lower)) {
            let urls: Vec<&url::Url> = [
                game.chips_url.as_ref(),
                game.lots_url.as_ref(),
            ].into_iter().flatten().collect();

            for url in urls {
                if let Ok(offers) = client.fetch_category_offers(url.as_str()).await {
                    for offer in offers {
                        if offer.price <= max_price {
                            yield offer;
                        }
                    }
                }
            }
        }
    }
}
