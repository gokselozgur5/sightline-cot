use sightline_cot::imm::SubFilter;

#[test]
fn turn_prediction_propagates_velocity_variance() {
    let mut ct = SubFilter::new([0.0; 3], 4.0, 10.0);
    let mut cv = ct.clone();
    ct.predict_ct(1.0, 0.5, 0.0);
    cv.predict_cv(1.0, 0.0);
    for i in 0..3 {
        let d = (ct.cov[i][i] - cv.cov[i][i]).abs();
        assert!(d < 0.5, "axis {i}: {} vs {}", ct.cov[i][i], cv.cov[i][i]);
    }
    assert!(ct.cov[0][3].abs() > 1.0, "{}", ct.cov[0][3]);
}

#[test]
fn turn_prediction_keeps_covariance_symmetric() {
    let mut f = SubFilter::new([0.0; 3], 4.0, 10.0);
    for _ in 0..20 {
        f.predict_ct(0.1, 0.5, 1.5);
    }
    for i in 0..6 {
        for j in 0..6 {
            assert!((f.cov[i][j] - f.cov[j][i]).abs() < 1e-9, "{i}{j}");
        }
    }
}
