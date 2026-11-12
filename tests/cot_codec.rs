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
