use sightline_cot::cot_beacon::{EmergencyBeacon, EmergencyType};

#[test]
fn test_repro_911_beacon_creation() {
    let beacon = EmergencyBeacon::new_911("UID-911", "MEDEVAC-1", 34.5, -118.2)
        .expect("create 911 beacon");
    assert_eq!(beacon.emergency_type, EmergencyType::Alert911);
    assert_eq!(beacon.callsign, "MEDEVAC-1");
    assert!(!beacon.cancel);
}
