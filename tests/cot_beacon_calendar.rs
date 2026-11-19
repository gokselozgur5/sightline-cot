use sightline_cot::cot_beacon::EmergencyBeacon;

#[test]
fn beacon_time_and_stale_use_the_real_calendar() {
    let beacon = EmergencyBeacon::new_911("UID-911", "MEDEVAC-1", 34.5, -118.2)
        .unwrap()
        .with_time(1_771_243_200_000);
    let xml = beacon.to_cot_xml();
    assert!(xml.contains("time=\"2026-02-16T12:00:00"), "{xml}");
    assert!(xml.contains("stale=\"2026-02-16T12:02:00"), "{xml}");
}
