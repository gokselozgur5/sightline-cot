use sightline_cot::kalman_target::{TacticalTargetFilter, TargetFilterConfig};

#[test]
fn test_integration_kalman_target_pipeline() {
    let cfg = TargetFilterConfig::default();
    assert!(cfg.validate().is_ok());

    let mut filter = TacticalTargetFilter::new(
        "INT-TARGET-01",
        [100.0, 200.0, 50.0],
        [15.0, 25.0, 0.0],
        10.0,
        4.0,
        1000,
        cfg,
    )
    .unwrap();

    let d2 = filter
        .update_position([115.0, 225.0, 50.0], None, 2000)
        .unwrap();
    assert!(d2 >= 0.0);

    let traj = filter.predict_horizon(3.0, 1.0).unwrap();
    assert_eq!(traj.len(), 3);
}

fn sample_filter(vel: [f64; 3]) -> TacticalTargetFilter {
    let cfg = TargetFilterConfig::default();
    TacticalTargetFilter::new("T", [0.0; 3], vel, 25.0, 4.0, 0, cfg).unwrap()
}

#[test]
fn full_state_update_is_atomic_when_the_velocity_is_gated() {
    let mut f = sample_filter([0.0; 3]);
    let res = f.update_full_state([1.0; 3], [500.0, 0.0, 0.0], None, None, 1000);
    assert!(res.is_err());
    assert_eq!(f.update_count, 0, "position applied although Err returned");
}

#[test]
fn measurement_after_the_coast_limit_is_not_rejected_forever() {
    let mut f = sample_filter([0.0; 3]);
    assert!(f.update_position([0.0; 3], None, 61_000).is_ok());
}

#[test]
fn horizon_timestamps_do_not_drift() {
    let traj = sample_filter([1.0, 0.0, 0.0])
        .predict_horizon(1.0, 1.0 / 30.0)
        .unwrap();
    assert_eq!(traj.last().unwrap().timestamp_ms, 1000);
}

#[test]
fn predict_horizon_rejects_excessive_or_zero_step_counts() {
    let filter = sample_filter([1.0, 0.0, 0.0]);
    assert!(filter.predict_horizon(0.4, 1.0).is_err());
    assert!(filter.predict_horizon(200_000.0, 1.0).is_err());
}
