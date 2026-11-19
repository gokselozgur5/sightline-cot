use sightline_cot::cot::{parse_iso8601_utc, CotEvent};
use sightline_cot::stanag4609::StanagTimestamp;

#[test]
fn year_beyond_i64_millis_is_rejected() {
    assert!(parse_iso8601_utc("300000000000000-01-01T00:00:00Z").is_err());
}

#[test]
fn event_with_huge_year_is_rejected() {
    let xml = "<event version=\"2.0\" uid=\"U\" type=\"a-f-G\" how=\"m-g\" \
        time=\"2026-02-16T12:00:00Z\" start=\"2026-02-16T12:00:00Z\" \
        stale=\"300000000000000-01-01T00:00:00Z\">\
        <point lat=\"1\" lon=\"1\" hae=\"0\" ce=\"1\" le=\"1\"/></event>";
    assert!(CotEvent::from_xml(xml).is_err());
}

#[test]
fn stanag_from_millis_out_of_range_is_rejected() {
    assert!(StanagTimestamp::from_millis(i64::MAX).is_err());
}
