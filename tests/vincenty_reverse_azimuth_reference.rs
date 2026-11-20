use sightline_cot::{vincenty_inverse_ellipsoid, Ellipsoid};

#[test]
fn reverse_azimuth_matches_published_flinders_buninyong_value() {
    let lat1 = -(37.0 + 57.0 / 60.0 + 3.72030 / 3600.0);
    let lon1 = 144.0 + 25.0 / 60.0 + 29.52440 / 3600.0;
    let lat2 = -(37.0 + 39.0 / 60.0 + 10.15610 / 3600.0);
    let lon2 = 143.0 + 55.0 / 60.0 + 35.38390 / 3600.0;
    let inv = vincenty_inverse_ellipsoid(lat1, lon1, lat2, lon2, &Ellipsoid::GRS80).unwrap();
    let forward = 306.0 + 52.0 / 60.0 + 5.37 / 3600.0;
    let reverse = 127.0 + 10.0 / 60.0 + 25.07 / 3600.0;
    assert!((inv.distance_m - 54972.271).abs() < 1e-3);
    assert!((inv.forward_azimuth_deg - forward).abs() < 1e-5);
    assert!(
        (inv.reverse_azimuth_deg - reverse).abs() < 1e-5,
        "reverse azimuth {} expected {}",
        inv.reverse_azimuth_deg,
        reverse
    );
}
