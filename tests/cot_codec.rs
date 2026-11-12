use sightline_cot::cot::{format_iso8601_utc, parse_iso8601_utc, CotEvent};

fn event(uid: &str, time_ms: i64) -> CotEvent {
    CotEvent {
        uid: uid.into(),
        cot_type: "a-f-G-U-C".into(),
        how: "m-g".into(),
        time_ms,
        stale_ms: time_ms + 30_000,
        lat_deg: 41.0123456,
        lon_deg: 28.9765432,
        hae_m: 100.5,
        ce_m: 10.0,
        le_m: 15.0,
        course_deg: 180.0,
        speed_mps: 25.0,
    }
}

#[test]
fn roundtrip_unusual_identifiers() {
    for uid in [
        "",
        "a b",
        "x&y",
        "<tag>",
        "q\"'",
        "caf\u{e9}-\u{4e2d}",
        "&amp;",
        "a;b&c;",
    ] {
        let ev = event(uid, 1_700_000_000_000);
        let back = CotEvent::from_xml(&ev.to_xml()).expect("parse");
        assert_eq!(back.uid, uid, "uid {uid:?}");
    }
}

#[test]
fn roundtrip_extreme_coordinates() {
    for (lat, lon) in [
        (90.0, 180.0),
        (-90.0, -180.0),
        (0.0, 0.0),
        (-0.0000001, 0.0000001),
    ] {
        let mut ev = event("X", 1000);
        ev.lat_deg = lat;
        ev.lon_deg = lon;
        let back = CotEvent::from_xml(&ev.to_xml()).expect("parse");
        assert!((back.lat_deg - lat).abs() < 1e-7);
        assert!((back.lon_deg - lon).abs() < 1e-7);
    }
}

#[test]
fn roundtrip_timestamp_range() {
    let samples = [0_i64, -1, 999, 951_782_400_000, 4_102_444_800_000];
    for ms in samples {
        let ts = format_iso8601_utc(ms);
        assert_eq!(parse_iso8601_utc(&ts).unwrap(), ms, "{ts}");
        let back = CotEvent::from_xml(&event("T", ms).to_xml()).expect("parse");
        assert_eq!((back.time_ms, back.stale_ms), (ms, ms + 30_000));
    }
}
