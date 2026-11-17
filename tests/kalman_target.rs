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
