use sightline_cot::imm::{ImmConfig, ImmFilter, SubFilter};

fn assert_psd(f: &SubFilter) {
    for i in 0..6 {
        assert!(f.cov[i][i] >= 0.0, "negative variance at {i}");
        for j in 0..6 {
            let d = (f.cov[i][j] - f.cov[j][i]).abs();
            assert!(d <= 1e-9 * (1.0 + f.cov[i][j].abs()), "asym {i}{j}");
        }
    }
    for i in 0..3 {
        let det = f.cov[i][i] * f.cov[i + 3][i + 3] - f.cov[i][i + 3].powi(2);
        assert!(det >= -1e-9, "negative minor {i}: {det}");
    }
}

#[test]
fn predictions_keep_covariance_psd() {
    let mut f = SubFilter::new([0.0; 3], 4.0, 10.0);
    for _ in 0..50 {
        f.predict_cv(0.1, 0.5);
        assert_psd(&f);
        f.predict_ca(0.1, 2.5);
        assert_psd(&f);
        f.predict_ct(0.1, 0.5, 1.5);
        assert_psd(&f);
    }
}

#[test]
fn long_run_updates_keep_covariance_psd() {
    let mut imm = ImmFilter::new([0.0; 3], ImmConfig::default());
    for k in 1..=300 {
        let t = k as f64 * 0.1;
        imm.step(0.1, [10.0 * t, (0.3 * t).sin() * 20.0, 5.0]);
        imm.filters.iter().for_each(assert_psd);
    }
}

#[test]
fn nonfinite_measurement_keeps_probs_normalized() {
    let mut imm = ImmFilter::new([0.0; 3], ImmConfig::default());
    imm.step(0.1, [1.0, 0.0, 0.0]);
    let before = imm.clone();
    imm.update([f64::NAN, 0.0, 0.0]);
    imm.update([f64::INFINITY, 0.0, 0.0]);
    assert_eq!(imm, before);
}
