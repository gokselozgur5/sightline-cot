use sightline_cot::imm::{ImmConfig, ImmFilter, SubFilter};

#[test]
fn predict_cv_propagates_cross_axis_covariance() {
    let mut f = SubFilter::new([0.0; 3], 1.0, 1.0);
    f.cov[0][4] = 0.5;
    f.cov[4][0] = 0.5;
    f.predict_cv(2.0, 0.0);
    assert!(
        (f.cov[0][1] - 1.0).abs() < 1e-12,
        "cov[0][1] = {}",
        f.cov[0][1]
    );
}

#[test]
fn imm_velocity_covariance_stays_positive_semidefinite() {
    let mut imm = ImmFilter::new([0.0; 3], ImmConfig::default());
    for k in 1..=300 {
        let t = k as f64 * 0.1;
        imm.step(0.1, [10.0 * t, 5.0 * t, 0.0]);
        for (j, f) in imm.filters.iter().enumerate() {
            let c = &f.cov;
            let minor = c[3][3] * c[4][4] - c[3][4] * c[4][3];
            assert!(minor >= -1e-9, "step {k} model {j}: vx/vy minor {minor}");
        }
    }
}
