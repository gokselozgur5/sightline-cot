use sightline_cot::format_mgrs;
use sightline_cot::utm_mgrs::{utm_to_mgrs, utm_to_mgrs_with_band};
use sightline_cot::{wgs84_to_utm, GridError, UtmCoord};

#[test]
fn test_utm_to_mgrs_matches_format_mgrs() {
    let test_coords = [
        (37.7749, -122.4194),
        (40.7128, -74.0060),
        (51.5074, -0.1278),
        (35.6762, 139.6503),
        (-33.8688, 151.2093),
        (0.0, 3.0),
    ];
    for (lat, lon) in test_coords {
        let utm = wgs84_to_utm(lat, lon).unwrap();
        for prec in 1..=5 {
            let direct = utm_to_mgrs(&utm, prec).unwrap();
            let from_geo = format_mgrs(lat, lon, prec).unwrap();
            assert_eq!(direct.as_str(), from_geo.as_str());
        }
    }
}

#[test]
fn test_utm_to_mgrs_with_band_and_errors() {
    let u = UtmCoord {
        zone: 31,
        hemisphere: 'N',
        easting: 500000.0,
        northing: 0.0,
    };
    assert_eq!(
        utm_to_mgrs_with_band(&u, 'N', 5).unwrap().as_str(),
        "31NEA0000000000"
    );
    assert_eq!(
        utm_to_mgrs_with_band(&u, 'N', 0),
        Err(GridError::InvalidNorthing)
    );
    assert_eq!(
        utm_to_mgrs_with_band(&u, 'N', 6),
        Err(GridError::InvalidNorthing)
    );
}
