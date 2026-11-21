use sightline_cot::cot::CotEvent;

#[test]
fn cot_event_carries_start_time() {
    let xml = r#"<event version="2.0" uid="u" type="a-f-G" how="m-g" time="2026-02-16T12:00:00.000Z" start="2026-02-16T12:05:00.000Z" stale="2026-02-16T12:10:00.000Z"><point lat="1" lon="1" hae="0" ce="1" le="1"/></event>"#;
    let ev = CotEvent::from_xml(xml).unwrap();
    assert_eq!(ev.start_ms, 1771243500000);
}
