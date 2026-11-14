use sightline_cot::geo_ddm::parse_ddm;

#[test]
fn test_acceptance_n_w_and_signed() {
    let s_hemi = "N 40 26.767 W 79 58.933";
    let s_signed = "40 26.767 -79 58.933";
    let (lat1, lon1) = parse_ddm(s_hemi).expect("hemi parse");
    let (lat2, lon2) = parse_ddm(s_signed).expect("signed parse");
    assert!((lat1 - lat2).abs() < 1e-6);
    assert!((lon1 - lon2).abs() < 1e-6);
    let expected_lat = 40.0 + 26.767 / 60.0;
    let expected_lon = -(79.0 + 58.933 / 60.0);
    assert!((lat1 - expected_lat).abs() < 1e-6);
    assert!((lon1 - expected_lon).abs() < 1e-6);
}

#[test]
fn test_ddm_format_variants() {
    let cases = [
        "40 26.767 N, 79 58.933 W",
        "40° 26.767' N 79° 58.933' W",
        "N40° 26.767', W079° 58.933'",
        "+40 26.767, -79 58.933",
    ];
    let (exp_lat, exp_lon) = (40.0 + 26.767 / 60.0, -(79.0 + 58.933 / 60.0));
    for s in cases {
        let (lat, lon) = parse_ddm(s).unwrap_or_else(|e| panic!("failed {}: {:?}", s, e));
        assert!((lat - exp_lat).abs() < 1e-6);
        assert!((lon - exp_lon).abs() < 1e-6);
    }
}

#[test]
fn test_ddm_south_east() {
    let (lat1, lon1) = parse_ddm("S 33 51.500 E 151 12.500").expect("se parse");
    let (lat2, lon2) = parse_ddm("-33 51.500 151 12.500").expect("se signed parse");
    assert!((lat1 - lat2).abs() < 1e-6);
    assert!((lon1 - lon2).abs() < 1e-6);
    let exp_lat = -(33.0 + 51.5 / 60.0);
    let exp_lon = 151.0 + 12.5 / 60.0;
    assert!((lat1 - exp_lat).abs() < 1e-6);
    assert!((lon1 - exp_lon).abs() < 1e-6);
}

use sightline_cot::geo_ddm::DdmError;

#[test]
fn test_ddm_error_cases() {
    assert_eq!(parse_ddm(""), Err(DdmError::EmptyInput));
    assert_eq!(parse_ddm("   "), Err(DdmError::EmptyInput));
    assert_eq!(parse_ddm("40 26.767"), Err(DdmError::InvalidFormat));
    assert_eq!(
        parse_ddm("N 40 65.0 W 79 10.0"),
        Err(DdmError::MinutesOutOfRange)
    );
    assert_eq!(
        parse_ddm("N 95 00.0 W 79 10.0"),
        Err(DdmError::LatitudeOutOfRange)
    );
    assert_eq!(
        parse_ddm("N 40 00.0 W 185 10.0"),
        Err(DdmError::LongitudeOutOfRange)
    );
    assert_eq!(
        parse_ddm("-40 26.767 N W 79 58.933"),
        Err(DdmError::ConflictingHemisphere)
    );
    assert_eq!(
        parse_ddm("N 40 10.0 N 45 10.0"),
        Err(DdmError::ConflictingHemisphere)
    );
}

#[test]
fn test_ddm_unsigned_first_prefix_second() {
    let (lat, lon) = parse_ddm("40 26.767, W 79 58.933").unwrap();
    let expected_lat = 40.0 + 26.767 / 60.0;
    let expected_lon = -(79.0 + 58.933 / 60.0);
    assert!((lat - expected_lat).abs() < 1e-6);
    assert!((lon - expected_lon).abs() < 1e-6);
}
