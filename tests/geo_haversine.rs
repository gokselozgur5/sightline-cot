use sightline_cot::{haversine_bearing_deg, haversine_distance_m};

#[test]
fn test_known_city_pairs_distance_within_half_percent() {
    let london = [51.5074, -0.1278];
    let paris = [48.8566, 2.3522];
    let dist_lp = haversine_distance_m(london, paris);
    let ref_lp = 343_556.0;
    let rel_err = (dist_lp - ref_lp).abs() / ref_lp;
    assert!(rel_err < 0.005, "got rel_err {rel_err}");

    let ny = [40.7128, -74.0060];
    let dist_nyl = haversine_distance_m(ny, london);
    let ref_nyl = 5_570_000.0;
    let rel_nyl = (dist_nyl - ref_nyl).abs() / ref_nyl;
    assert!(rel_nyl < 0.005, "got rel_err {rel_nyl}");
}

#[test]
fn test_initial_bearing_cardinal_and_reference_pairs() {
    let origin = [0.0, 0.0];
    assert!((haversine_bearing_deg(origin, [10.0, 0.0]) - 0.0).abs() < 1e-6);
    assert!((haversine_bearing_deg(origin, [0.0, 10.0]) - 90.0).abs() < 1e-6);
    assert!((haversine_bearing_deg([10.0, 0.0], origin) - 180.0).abs() < 1e-6);
    assert!((haversine_bearing_deg([0.0, 10.0], origin) - 270.0).abs() < 1e-6);

    let london = [51.5074, -0.1278];
    let paris = [48.8566, 2.3522];
    let b = haversine_bearing_deg(london, paris);
    assert!((b - 148.12).abs() < 0.1, "got bearing {b}");
}

#[test]
fn test_coincident_and_antipodal_points() {
    let p = [35.6895, 139.6917];
    assert!(haversine_distance_m(p, p) < 1e-6);

    let north_pole = [90.0, 0.0];
    let south_pole = [-90.0, 0.0];
    let d = haversine_distance_m(north_pole, south_pole);
    let half_circumference = std::f64::consts::PI * 6_371_008.8;
    assert!((d - half_circumference).abs() < 1e-4);
}
