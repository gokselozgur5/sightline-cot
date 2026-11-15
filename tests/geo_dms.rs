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
