use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum MonitorEvent {
    PriceChanged { offer_id: String, old_price: f64, new_price: f64 },
    NewOffer { offer_id: String },
    OfferRemoved { offer_id: String },
}

pub struct Monitor {
    seen_offers: HashMap<String, f64>,
}

impl Monitor {
    pub fn new() -> Self {
        Self { seen_offers: HashMap::new() }
    }
    
    pub fn check_for_changes(&mut self, offers: Vec<(String, f64)>) -> Vec<MonitorEvent> {
        let mut events = Vec::new();
        let current: HashMap<String, f64> = offers.into_iter().collect();
        
        for (id, price) in &current {
            if let Some(old_price) = self.seen_offers.get(id) {
                if (old_price - price).abs() > 0.01 {
                    events.push(MonitorEvent::PriceChanged {
                        offer_id: id.clone(),
                        old_price: *old_price,
                        new_price: *price,
                    });
                }
            } else {
                events.push(MonitorEvent::NewOffer { offer_id: id.clone() });
            }
        }
        
        for id in self.seen_offers.keys() {
            if !current.contains_key(id) {
                events.push(MonitorEvent::OfferRemoved { offer_id: id.clone() });
            }
        }
        
        self.seen_offers = current;
        events
    }
}
