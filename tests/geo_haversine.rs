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
