//! Export utilities for offer data.
//!
//! Convert offer lists to JSON or CSV format for external analysis.

use crate::models::Offer;
use serde::Serialize;

#[allow(dead_code)]
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

/// Serialize offers to a JSON string.
pub fn to_json(offers: &[Offer]) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(offers)
}

/// Write offers to a JSON file.
pub fn to_json_file(offers: &[Offer], path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json = to_json(offers)?;
    std::fs::write(path, json)?;
    Ok(())
}

/// Serialize offers to a CSV string.
#[cfg(feature = "export")]
pub fn to_csv(offers: &[Offer]) -> String {
    let mut wtr = csv::Writer::from_writer(vec![]);
    for offer in offers {
        let flat = FlatOffer::from(offer);
        let _ = wtr.serialize(flat);
    }
    String::from_utf8(wtr.into_inner().unwrap_or_default()).unwrap_or_default()
}

/// Write offers to a CSV file.
#[cfg(feature = "export")]
pub fn to_csv_file(offers: &[Offer], path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let csv = to_csv(offers);
    std::fs::write(path, csv)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::*;

    fn sample_offers() -> Vec<Offer> {
        vec![
            Offer {
                image_url: None,
                currency: "RUB".into(),
                sale_type: LotSaleType::Single,
                seller_id: UserId::new(1),
                id: OfferId::new(100),
                description: Some("1000 Gold".into()),
                item_count: None,
                price: Price::new(25.5),
                server: Some("EU".into()),
            },
            Offer {
                image_url: None,
                currency: "USD".into(),
                sale_type: LotSaleType::Bulk,
                seller_id: UserId::new(2),
                id: OfferId::new(200),
                description: Some("Silver pack".into()),
                item_count: Some(10),
                price: Price::new(9.99),
                server: None,
            },
        ]
    }

    #[test]
    fn test_to_json() {
        let json = to_json(&sample_offers()).unwrap();
        assert!(json.contains("1000 Gold"));
        assert!(json.contains("25.5"));
    }

    #[test]
    fn test_flat_offer_conversion() {
        let offers = sample_offers();
        let flat = FlatOffer::from(&offers[0]);
        assert_eq!(flat.offer_id, "100");
        assert_eq!(flat.seller_id, "1");
        assert_eq!(flat.server, "EU");
        assert_eq!(flat.price, 25.5);
    }

    #[test]
    fn test_flat_offer_defaults() {
        let offer = Offer {
            image_url: None,
            currency: "RUB".into(),
            sale_type: Default::default(),
            seller_id: UserId::new(1),
            id: OfferId::new(1),
            description: None,
            item_count: None,
            price: Price::new(0.0),
            server: None,
        };
        let flat = FlatOffer::from(&offer);
        assert!(flat.server.is_empty());
        assert!(flat.description.is_empty());
    }
}
