use sightline_cot::geo_vincenty::{vincenty_direct, vincenty_inverse};

#[test]
fn test_flinders_peak_to_buninyong() {
    let lat1 = -(37.0 + 57.0 / 60.0 + 3.72030 / 3600.0);
    let lon1 = 144.0 + 25.0 / 60.0 + 29.52440 / 3600.0;
    let lat2 = -(37.0 + 39.0 / 60.0 + 10.15610 / 3600.0);
    let lon2 = 143.0 + 55.0 / 60.0 + 35.38390 / 3600.0;
    let inv = vincenty_inverse(lat1, lon1, lat2, lon2).expect("inverse");
    assert!(
        (inv.distance_m - 54972.271).abs() < 0.001,
        "distance diff {} >= 1mm",
        (inv.distance_m - 54972.271).abs()
    );
}
