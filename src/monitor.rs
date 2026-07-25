use std::collections::HashMap;
use rust_decimal::Decimal;
use tokio::sync::broadcast;
use crate::models::OfferId;

#[derive(Debug, Clone)]
pub enum MonitorEvent {
    PriceChanged { offer_id: OfferId, old_price: Decimal, new_price: Decimal },
    NewOffer { offer_id: OfferId },
    OfferRemoved { offer_id: OfferId },
}

pub struct Monitor {
    seen_offers: HashMap<OfferId, Decimal>,
}

impl Monitor {
    pub fn new() -> Self {
        Self { seen_offers: HashMap::new() }
    }

    pub fn check_for_changes(&mut self, offers: Vec<(OfferId, Decimal)>) -> Vec<MonitorEvent> {
        let mut events = Vec::new();
        let current: HashMap<OfferId, Decimal> = offers.into_iter().collect();

        for (id, price) in &current {
            if let Some(old_price) = self.seen_offers.get(id) {
                if (*old_price - price).abs() > Decimal::from(1).checked_div(Decimal::from(100)).unwrap_or_default() {
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

    pub fn spawn_monitoring(
        &self,
        mut rx: broadcast::Receiver<Vec<(OfferId, Decimal)>>,
        event_tx: broadcast::Sender<MonitorEvent>,
    ) {
        let mut monitor = Monitor::new();
        tokio::spawn(async move {
            while let Ok(offers) = rx.recv().await {
                let events = monitor.check_for_changes(offers);
                for event in events {
                    let _ = event_tx.send(event);
                }
            }
        });
    }
}
