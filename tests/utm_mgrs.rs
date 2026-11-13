use sightline_cot::format_mgrs;
use sightline_cot::utm_mgrs::utm_to_mgrs;
use sightline_cot::wgs84_to_utm;

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
