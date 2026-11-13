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
