use sightline_cot::Egm96Geoid;

#[test]
fn coarse_grid_is_within_25_m_of_egm96_test_points() {
    let grid = Egm96Geoid::coarse_reference_grid();
    let cases = [
        (38.6281550, 269.7791550, -31.628),
        (46.8743190, 102.4487290, -43.575),
        (38.6254730, 359.9995000, 50.066),
        (-0.4667440, 0.0023000, 17.329),
    ];
    for (lat, lon, expected) in cases {
        let n = grid.undulation(lat, lon).unwrap();
        assert!(
            (n - expected).abs() < 25.0,
            "({lat}, {lon}): {n} vs EGM96 {expected}"
        );
    }
}
