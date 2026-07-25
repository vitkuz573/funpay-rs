use funpay_rs::monitor::{Monitor, MonitorEvent};

#[test]
fn test_monitor_creation() {
    let _monitor = Monitor::new();
    assert!(true);
}

#[test]
fn test_monitor_detects_new_offers() {
    let mut monitor = Monitor::new();
    let offers = vec![("1".to_string(), 100.0), ("2".to_string(), 200.0)];
    let events = monitor.check_for_changes(offers);
    assert_eq!(events.len(), 2);
    assert!(matches!(&events[0], MonitorEvent::NewOffer { .. }));
}

#[test]
fn test_monitor_detects_price_change() {
    let mut monitor = Monitor::new();
    monitor.check_for_changes(vec![("1".to_string(), 100.0)]);
    let events = monitor.check_for_changes(vec![("1".to_string(), 90.0)]);
    assert_eq!(events.len(), 1);
    assert!(matches!(&events[0], MonitorEvent::PriceChanged { .. }));
}

#[test]
fn test_monitor_detects_removed_offer() {
    let mut monitor = Monitor::new();
    monitor.check_for_changes(vec![("1".to_string(), 100.0)]);
    let events = monitor.check_for_changes(vec![]);
    assert_eq!(events.len(), 1);
    assert!(matches!(&events[0], MonitorEvent::OfferRemoved { .. }));
}
