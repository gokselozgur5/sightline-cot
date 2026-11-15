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

#[test]
fn test_dms_format_variants() {
    let cases = [
        "37° 46' 29.8\" N, 122° 25' 10.0\" W",
        "37° 46' 29.8\" N 122° 25' 10.0\" W",
        "N37° 46' 29.8', W122° 25' 10.0'",
        "+37 46 29.8, -122 25 10.0",
    ];
    let (exp_lat, exp_lon) = (
        37.0 + 46.0 / 60.0 + 29.8 / 3600.0,
        -(122.0 + 25.0 / 60.0 + 10.0 / 3600.0),
    );
    for s in cases {
        let (lat, lon) = parse_dms(s).unwrap_or_else(|e| panic!("failed {}: {:?}", s, e));
        assert!((lat - exp_lat).abs() < 1e-6);
        assert!((lon - exp_lon).abs() < 1e-6);
    }
}

#[test]
fn test_dms_south_east() {
    let s_hemi = "S 33 51 30.5 E 151 12 45.2";
    let s_signed = "-33 51 30.5 151 12 45.2";
    let (lat1, lon1) = parse_dms(s_hemi).expect("se hemi parse");
    let (lat2, lon2) = parse_dms(s_signed).expect("se signed parse");
    assert!((lat1 - lat2).abs() < 1e-6);
    assert!((lon1 - lon2).abs() < 1e-6);
    let exp_lat = -(33.0 + 51.0 / 60.0 + 30.5 / 3600.0);
    let exp_lon = 151.0 + 12.0 / 60.0 + 45.2 / 3600.0;
    assert!((lat1 - exp_lat).abs() < 1e-6);
    assert!((lon1 - exp_lon).abs() < 1e-6);
}

use sightline_cot::geo_dms::DmsError;

#[test]
fn test_dms_error_cases() {
    assert_eq!(parse_dms(""), Err(DmsError::EmptyInput));
    assert_eq!(parse_dms("   "), Err(DmsError::EmptyInput));
    assert_eq!(parse_dms("37 46"), Err(DmsError::InvalidFormat));
    assert_eq!(parse_dms("N 37 65 10 W 122 10 10"), Err(DmsError::MinutesOutOfRange));
    assert_eq!(parse_dms("N 37 10 75 W 122 10 10"), Err(DmsError::SecondsOutOfRange));
    assert_eq!(parse_dms("N 95 00 00 W 122 10 10"), Err(DmsError::LatitudeOutOfRange));
    assert_eq!(parse_dms("N 37 00 00 W 185 10 10"), Err(DmsError::LongitudeOutOfRange));
    assert_eq!(parse_dms("-37 10 10 N W 122 10 10"), Err(DmsError::ConflictingHemisphere));
    assert_eq!(parse_dms("N 37 10 10 N 45 10 10"), Err(DmsError::ConflictingHemisphere));
}

use sightline_cot::geo_dms::{parse_dms_lat, parse_dms_lon, Dms};

#[test]
fn test_dms_struct_and_component_parsing() {
    let dms = Dms::new(37, 46, 29.8, 'N').expect("dms new");
    assert_eq!(dms.degrees, 37);
    assert_eq!(dms.minutes, 46);
    assert!((dms.seconds - 29.8).abs() < 1e-6);
    assert_eq!(dms.hemisphere, 'N');
    let deg = dms.to_degrees();
    assert!((deg - (37.0 + 46.0 / 60.0 + 29.8 / 3600.0)).abs() < 1e-6);
    let lat = parse_dms_lat("37° 46' 29.8\" N").expect("parse lat");
    let lon = parse_dms_lon("122° 25' 10.0\" W").expect("parse lon");
    assert!((lat - deg).abs() < 1e-6);
    assert!(lon < 0.0);
    let dms_from_str: Dms = "37° 46' 29.8\" N".parse().expect("from_str");
    assert_eq!(dms_from_str.degrees, 37);
}
