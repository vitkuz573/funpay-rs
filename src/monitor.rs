//! Offer price monitoring with event detection.
//!
//! Tracks offer prices over time and emits events when prices change,
//! new offers appear, or existing offers are removed.
//!
//! # Examples
//!
//! ```rust
//! use funpay_sdk::monitor::{Monitor, MonitorEvent};
//! use funpay_sdk::models::OfferId;
//!
//! let mut monitor = Monitor::new();
//!
//! // First check: new offers detected
//! let events = monitor.check_for_changes(vec![
//!     (OfferId::new(1), 100.0),
//!     (OfferId::new(2), 200.0),
//! ]);
//! assert_eq!(events.len(), 2);
//! assert!(matches!(&events[0], MonitorEvent::NewOffer { .. }));
//!
//! // Second check: price change detected
//! let events = monitor.check_for_changes(vec![
//!     (OfferId::new(1), 150.0),
//!     (OfferId::new(2), 200.0),
//! ]);
//! assert_eq!(events.len(), 1);
//! assert!(matches!(&events[0], MonitorEvent::PriceChanged { .. }));
//! ```

use std::collections::HashMap;
use tokio::sync::broadcast;
use crate::models::OfferId;

/// Events emitted by the price monitor.
#[derive(Debug, Clone)]
pub enum MonitorEvent {
    /// An offer's price changed.
    PriceChanged { offer_id: OfferId, old_price: f64, new_price: f64 },
    /// A new offer appeared.
    NewOffer { offer_id: OfferId },
    /// An offer was removed.
    OfferRemoved { offer_id: OfferId },
}

/// Stateful offer price tracker.
///
/// Call [`check_for_changes`](Monitor::check_for_changes) with the current
/// offer list to detect price changes, new offers, and removals.
pub struct Monitor {
    seen_offers: HashMap<OfferId, f64>,
}

impl Default for Monitor {
    fn default() -> Self {
        Self::new()
    }
}

impl Monitor {
    /// Create a new empty monitor.
    pub fn new() -> Self {
        Self { seen_offers: HashMap::new() }
    }

    /// Compare current offers against previous state and return events.
    pub fn check_for_changes(&mut self, offers: Vec<(OfferId, f64)>) -> Vec<MonitorEvent> {
        let mut events = Vec::new();
        let current: HashMap<OfferId, f64> = offers.into_iter().collect();

        for (id, price) in &current {
            if let Some(old_price) = self.seen_offers.get(id) {
                if (old_price - price).abs() > 0.01 {
                    events.push(MonitorEvent::PriceChanged {
                        offer_id: *id,
                        old_price: *old_price,
                        new_price: *price,
                    });
                }
            } else {
                events.push(MonitorEvent::NewOffer { offer_id: *id });
            }
        }

        for id in self.seen_offers.keys() {
            if !current.contains_key(id) {
                events.push(MonitorEvent::OfferRemoved { offer_id: *id });
            }
        }

        self.seen_offers = current;
        events
    }

    /// Spawn a background task that monitors offers via a broadcast channel.
    pub fn spawn_monitoring(
        &self,
        mut rx: broadcast::Receiver<Vec<(OfferId, f64)>>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitor_new_offers() {
        let mut monitor = Monitor::new();
        let events = monitor.check_for_changes(vec![
            (OfferId::new(1), 100.0),
            (OfferId::new(2), 200.0),
        ]);
        assert_eq!(events.len(), 2);
        assert!(events.iter().all(|e| matches!(e, MonitorEvent::NewOffer { .. })));
    }

    #[test]
    fn test_monitor_price_change() {
        let mut monitor = Monitor::new();
        monitor.check_for_changes(vec![(OfferId::new(1), 100.0)]);

        let events = monitor.check_for_changes(vec![(OfferId::new(1), 150.0)]);
        assert_eq!(events.len(), 1);
        match &events[0] {
            MonitorEvent::PriceChanged { offer_id, old_price, new_price } => {
                assert_eq!(*offer_id, OfferId::new(1));
                assert_eq!(*old_price, 100.0);
                assert_eq!(*new_price, 150.0);
            }
            _ => panic!("expected PriceChanged"),
        }
    }

    #[test]
    fn test_monitor_offer_removed() {
        let mut monitor = Monitor::new();
        monitor.check_for_changes(vec![
            (OfferId::new(1), 100.0),
            (OfferId::new(2), 200.0),
        ]);

        let events = monitor.check_for_changes(vec![(OfferId::new(1), 100.0)]);
        assert_eq!(events.len(), 1);
        assert!(matches!(&events[0], MonitorEvent::OfferRemoved { .. }));
    }

    #[test]
    fn test_monitor_no_change() {
        let mut monitor = Monitor::new();
        monitor.check_for_changes(vec![(OfferId::new(1), 100.0)]);

        let events = monitor.check_for_changes(vec![(OfferId::new(1), 100.0)]);
        assert!(events.is_empty());
    }

    #[test]
    fn test_monitor_price_within_tolerance() {
        let mut monitor = Monitor::new();
        monitor.check_for_changes(vec![(OfferId::new(1), 100.0)]);

        let events = monitor.check_for_changes(vec![(OfferId::new(1), 100.005)]);
        assert!(events.is_empty());
    }
}
