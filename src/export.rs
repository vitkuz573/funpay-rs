use crate::models::Offer;
use serde::Serialize;

#[derive(Serialize)]
struct FlatOffer {
    offer_id: String,
    seller_id: String,
    server: String,
    description: String,
    price: f64,
    currency: String,
}

impl From<&Offer> for FlatOffer {
    fn from(o: &Offer) -> Self {
        Self {
            offer_id: o.id.to_string(),
            seller_id: o.seller_id.to_string(),
            server: o.server.clone().unwrap_or_default(),
            description: o.description.clone().unwrap_or_default(),
            price: *o.price.inner(),
            currency: o.currency.clone(),
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
