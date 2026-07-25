use funpay_sdk::monitor::{Monitor, MonitorEvent};
use funpay_sdk::models::OfferId;

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
fn test_monitor_multiple_changes() {
    let mut monitor = Monitor::new();
    monitor.check_for_changes(vec![
        (OfferId::new(1), 100.0),
        (OfferId::new(2), 200.0),
    ]);

    let events = monitor.check_for_changes(vec![
        (OfferId::new(1), 150.0),
        (OfferId::new(3), 300.0),
    ]);
    // PriceChanged for 1, NewOffer for 3, OfferRemoved for 2
    assert_eq!(events.len(), 3);
    assert!(events.iter().any(|e| matches!(e, MonitorEvent::PriceChanged { .. })));
    assert!(events.iter().any(|e| matches!(e, MonitorEvent::NewOffer { .. })));
    assert!(events.iter().any(|e| matches!(e, MonitorEvent::OfferRemoved { .. })));
}

#[test]
fn test_monitor_price_within_tolerance() {
    let mut monitor = Monitor::new();
    monitor.check_for_changes(vec![(OfferId::new(1), 100.0)]);

    let events = monitor.check_for_changes(vec![(OfferId::new(1), 100.005)]);
    assert!(events.is_empty());
}

#[test]
fn test_monitor_empty_to_populated() {
    let mut monitor = Monitor::new();
    let events = monitor.check_for_changes(vec![(OfferId::new(1), 50.0)]);
    assert_eq!(events.len(), 1);
    assert!(matches!(&events[0], MonitorEvent::NewOffer { .. }));
}

#[test]
fn test_monitor_populated_to_empty() {
    let mut monitor = Monitor::new();
    monitor.check_for_changes(vec![(OfferId::new(1), 50.0)]);
    let events = monitor.check_for_changes(vec![]);
    assert_eq!(events.len(), 1);
    assert!(matches!(&events[0], MonitorEvent::OfferRemoved { .. }));
}
