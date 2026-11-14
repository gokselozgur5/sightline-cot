use sightline_cot::format_mgrs;
use sightline_cot::mgrs_decode::{decode_mgrs, MgrsDecodeError};

#[test]
fn test_decode_mgrs_10_digit_roundtrip() {
    let test_points = [
        (37.7749, -122.4194),
        (40.7128, -74.0060),
        (51.5074, -0.1278),
        (35.6762, 139.6503),
        (-33.8688, 151.2093),
        (0.0, 3.0),
    ];
    for (lat, lon) in test_points {
        let mgrs = format_mgrs(lat, lon, 5).unwrap();
        let (lat_dec, lon_dec) = decode_mgrs(mgrs.as_str()).unwrap();
        assert!((lat - lat_dec).abs() < 1e-4);
        assert!((lon - lon_dec).abs() < 1e-4);
    }
}

#[test]
fn test_decode_mgrs_errors() {
    assert_eq!(decode_mgrs(""), Err(MgrsDecodeError::InvalidLength));
    assert_eq!(decode_mgrs("00NAA1234"), Err(MgrsDecodeError::InvalidZone));
    assert_eq!(decode_mgrs("61NAA1234"), Err(MgrsDecodeError::InvalidZone));
    assert_eq!(decode_mgrs("31IAA1234"), Err(MgrsDecodeError::InvalidBand));
    assert_eq!(
        decode_mgrs("31N111234"),
        Err(MgrsDecodeError::InvalidSquare)
    );
    assert_eq!(decode_mgrs("31NEA123"), Err(MgrsDecodeError::InvalidDigits));
    assert!(decode_mgrs("31NA°12345").is_err());
}
