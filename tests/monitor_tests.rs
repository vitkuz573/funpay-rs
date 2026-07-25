use funpay_rs::monitor::{Monitor, MonitorEvent};
use funpay_rs::models::OfferId;
use rust_decimal::Decimal;

#[test]
fn test_monitor_creation() {
    let _monitor = Monitor::new();
    assert!(true);
}

#[test]
fn test_monitor_detects_new_offers() {
    let mut monitor = Monitor::new();
    let offers = vec![(OfferId("1".to_string()), Decimal::from(100)), (OfferId("2".to_string()), Decimal::from(200))];
    let events = monitor.check_for_changes(offers);
    assert_eq!(events.len(), 2);
    assert!(matches!(&events[0], MonitorEvent::NewOffer { .. }));
}

#[test]
fn test_monitor_detects_price_change() {
    let mut monitor = Monitor::new();
    monitor.check_for_changes(vec![(OfferId("1".to_string()), Decimal::from(100))]);
    let events = monitor.check_for_changes(vec![(OfferId("1".to_string()), Decimal::from(90))]);
    assert_eq!(events.len(), 1);
    assert!(matches!(&events[0], MonitorEvent::PriceChanged { .. }));
}

#[test]
fn test_monitor_detects_removed_offer() {
    let mut monitor = Monitor::new();
    monitor.check_for_changes(vec![(OfferId("1".to_string()), Decimal::from(100))]);
    let events = monitor.check_for_changes(vec![]);
    assert_eq!(events.len(), 1);
    assert!(matches!(&events[0], MonitorEvent::OfferRemoved { .. }));
}
