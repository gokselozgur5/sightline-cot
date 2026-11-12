use sightline_cot::imm::{ImmConfig, ImmFilter, SubFilter, NUM_MODELS};

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

fn filter() -> ImmFilter {
    ImmFilter::new([0.0; 3], ImmConfig::default())
}

#[test]
fn default_trans_mat_is_valid() {
    assert!(filter().trans_mat_is_valid(1e-9));
}

#[test]
fn bad_row_sum_is_rejected() {
    let mut imm = filter();
    imm.trans_mat[1] = [0.5, 0.5, 0.5];
    assert!(!imm.trans_mat_is_valid(1e-9));
}

#[test]
fn negative_entry_is_rejected() {
    let mut imm = filter();
    imm.trans_mat[0] = [1.1, -0.1, 0.0];
    assert!(!imm.trans_mat_is_valid(1e-9));
}

#[test]
fn nan_entry_is_rejected() {
    let mut imm = filter();
    imm.trans_mat[2][0] = f64::NAN;
    assert!(!imm.trans_mat_is_valid(1e-9));
}

#[test]
fn mixing_weights_stay_normalized() {
    let imm = filter();
    let total: f64 = imm.compute_c_bar().iter().sum();
    assert!((total - 1.0).abs() < 1e-12);
}

fn check(imm: &ImmFilter) {
    let sum: f64 = imm.probs.iter().sum();
    assert!((sum - 1.0).abs() < 1e-9, "sum {sum}");
    assert!(imm.probs.iter().all(|p| (0.0..=1.0).contains(p)));
    assert!(imm.fused_state().iter().all(|v| v.is_finite()));
}

#[test]
fn straight_turn_straight_keeps_invariants() {
    let mut imm = ImmFilter::new([0.0; 3], ImmConfig::default());
    let (mut x, mut y, mut h) = (0.0_f64, 0.0_f64, 0.0_f64);
    let mut peak: f64 = 0.0;
    for k in 0..150 {
        let w = if (50..100).contains(&k) { 0.5 } else { 0.0 };
        h += w * 0.1;
        x += 30.0 * h.cos() * 0.1;
        y += 30.0 * h.sin() * 0.1;
        imm.step(0.1, [x, y, 0.0]);
        check(&imm);
        if (50..100).contains(&k) {
            peak = peak.max(imm.probs[1] + imm.probs[2]);
        }
    }
    assert!(peak > 0.5, "maneuver never detected: {peak}");
}

#[test]
fn accelerate_then_cruise_keeps_invariants() {
    let mut imm = ImmFilter::new([0.0; 3], ImmConfig::default());
    let (mut x, mut v) = (0.0, 0.0);
    for k in 0..120 {
        let a = if k < 40 { 15.0 } else { 0.0 };
        v += a * 0.1;
        x += v * 0.1;
        imm.step(0.1, [x, 0.0, 0.0]);
        check(&imm);
    }
    assert!((imm.fused_state()[3] - v).abs() < 8.0);
}

#[test]
fn far_outlier_does_not_break_normalization() {
    let mut imm = ImmFilter::new([0.0; 3], ImmConfig::default());
    for _ in 0..5 {
        imm.step(0.1, [0.0; 3]);
    }
    imm.step(0.1, [1.0e9, -1.0e9, 1.0e9]);
    check(&imm);
    assert_eq!(imm.probs.len(), NUM_MODELS);
}
