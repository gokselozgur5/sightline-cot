use sightline_cot::imm::{ImmConfig, ImmFilter, SubFilter, MODEL_CA, MODEL_CT, MODEL_CV};

#[test]
fn step_rejects_non_positive_and_non_finite_dt() {
    let mut imm = ImmFilter::new([0.0; 3], ImmConfig::default());
    imm.step(0.1, [1.0, 2.0, 3.0]);
    let before = imm.clone();
    for bad_dt in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, 0.0, -0.1] {
        let out = imm.step(bad_dt, [9.0, 9.0, 9.0]);
        assert_eq!(imm, before, "filter mutated on dt={bad_dt}");
        assert_eq!(out, before.fused_state());
    }
    assert!(imm.step(0.1, [2.0, 2.0, 3.0]).iter().all(|v| v.is_finite()));
}

#[test]
fn predict_ca_advances_velocity_and_quadratic_position() {
    let mut ca = SubFilter::new([0.0; 3], 1.0, 10.0);
    let mut cv = ca.clone();
    for k in 1..=25 {
        let z = [10.0 * (k as f64 * 0.1).powi(2), 0.0, 0.0];
        ca.predict_ca(0.1, 2.5);
        ca.update_pos(z, 0.01);
        cv.predict_cv(0.1, 0.5);
        cv.update_pos(z, 0.01);
    }
    let (x0, v0, c_vv0) = (ca.state[0], ca.state[3], ca.cov[3][3]);
    ca.predict_ca(0.5, 0.0);
    cv.predict_cv(0.5, 0.0);
    assert!(((ca.state[3] - v0) - 10.0).abs() < 1.5);
    assert!(((ca.state[0] - x0 - 0.5 * v0) - 2.5).abs() < 0.5);
    assert!((ca.cov[3][3] - c_vv0).abs() > 1e-6);
    assert!((ca.state[0] - 90.0).abs() * 4.0 < (cv.state[0] - 90.0).abs());
}

#[test]
fn imm_ca_tracks_quadratic_and_handles_mirrored_turn() {
    let mut imm = ImmFilter::new([0.0; 3], ImmConfig::default());
    imm.config.pos_noise_m = 0.5;
    for k in 1..=30 {
        imm.step(0.1, [10.0 * (k as f64 * 0.1).powi(2), 0.0, 0.0]);
    }
    assert!(imm.probs[MODEL_CA] > imm.probs[MODEL_CV] && (imm.fused_state()[3] - 60.0).abs() < 3.0);
    let mut turn = ImmFilter::new([0.0; 3], ImmConfig::default());
    for k in 1..=30 {
        let wt = std::f64::consts::FRAC_PI_6 * k as f64 * 0.1;
        turn.step(0.1, [wt.sin() * 50.0, -(1.0 - wt.cos()) * 50.0, 0.0]);
    }
    assert!(turn.is_maneuvering() && turn.probs[MODEL_CT] > turn.probs[MODEL_CV]);
}
