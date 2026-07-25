use crate::client::FunPayClient;
use crate::models::Offer;

pub async fn search_offers(
    client: &FunPayClient,
    game_id: u64,
    category_id: u64,
    max_price: f64,
) -> Result<Vec<Offer>, crate::error::FunPayError> {
    let offers = client.fetch_category_offers(game_id, category_id).await?;
    Ok(offers.into_iter().filter(|o| *o.price.inner() <= max_price).collect())
}
