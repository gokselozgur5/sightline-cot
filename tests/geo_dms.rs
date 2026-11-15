use sightline_cot::geo_dms::parse_dms;

#[test]
fn test_dms_parsing_basic() {
    let s = "37° 46' 29.8\" N, 122° 25' 10.0\" W";
    let (lat, lon) = parse_dms(s).expect("dms parse");
    let exp_lat = 37.0 + 46.0 / 60.0 + 29.8 / 3600.0;
    let exp_lon = -(122.0 + 25.0 / 60.0 + 10.0 / 3600.0);
    assert!((lat - exp_lat).abs() < 1e-6);
    assert!((lon - exp_lon).abs() < 1e-6);
}

use sightline_cot::geo_dms::{format_dms, format_dms_lat, format_dms_lon};

#[test]
fn test_dms_formatting_and_roundtrip() {
    let lat = 37.774929;
    let lon = -122.419416;
    let s_lat = format_dms_lat(lat).expect("fmt lat");
    let s_lon = format_dms_lon(lon).expect("fmt lon");
    assert!(s_lat.ends_with('N'));
    assert!(s_lon.ends_with('W'));
    let formatted = format_dms(lat, lon).expect("fmt pair");
    let (parsed_lat, parsed_lon) = parse_dms(&formatted).expect("roundtrip parse");
    assert!((lat - parsed_lat).abs() < 1e-4);
    assert!((lon - parsed_lon).abs() < 1e-4);
}
