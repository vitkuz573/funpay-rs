use crate::models::Offer;
use rust_decimal::Decimal;
use serde::Serialize;

#[derive(Serialize)]
struct FlatOffer {
    offer_id: String,
    lot_id: String,
    server: String,
    description: String,
    price: Decimal,
    currency: String,
    stock: u32,
    seller_id: String,
    seller_name: String,
    seller_rating: f64,
    seller_reviews: u32,
    seller_online: bool,
}

impl From<&Offer> for FlatOffer {
    fn from(o: &Offer) -> Self {
        Self {
            offer_id: o.offer_id.0.clone(),
            lot_id: o.lot_id.0.clone(),
            server: o.server.0.clone(),
            description: o.description.clone(),
            price: o.price,
            currency: o.currency.to_string(),
            stock: o.stock,
            seller_id: o.seller.user_id.0.clone(),
            seller_name: o.seller.name.clone(),
            seller_rating: o.seller.rating,
            seller_reviews: o.seller.reviews,
            seller_online: o.seller.online,
        }
    }
}

pub fn to_json(offers: &[Offer]) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(offers)
}

pub fn to_json_file(offers: &[Offer], path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json = to_json(offers)?;
    std::fs::write(path, json)?;
    Ok(())
}

#[cfg(feature = "export")]
pub fn to_csv(offers: &[Offer]) -> String {
    let mut wtr = csv::Writer::from_writer(vec![]);
    for offer in offers {
        let flat = FlatOffer::from(offer);
        let _ = wtr.serialize(flat);
    }
    String::from_utf8(wtr.into_inner().unwrap_or_default()).unwrap_or_default()
}

#[cfg(feature = "export")]
pub fn to_csv_file(offers: &[Offer], path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let csv = to_csv(offers);
    std::fs::write(path, csv)?;
    Ok(())
}
