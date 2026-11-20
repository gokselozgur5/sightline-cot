use sightline_cot::{interpolate_by_step, RhumbLineError, RhumbPath, RhumbWaypoint};

#[test]
fn interpolate_by_step_rejects_a_step_that_overflows_the_sample_count() {
    let a = RhumbWaypoint::new(0.0, 0.0);
    let b = RhumbWaypoint::new(1.0, 1.0);
    assert_eq!(
        interpolate_by_step(&a, &b, 1e-300),
        Err(RhumbLineError::InvalidParameter)
    );
}

#[test]
fn resample_by_step_rejects_a_step_that_overflows_the_sample_count() {
    let path = RhumbPath::from_waypoints(vec![
        RhumbWaypoint::new(0.0, 0.0),
        RhumbWaypoint::new(1.0, 1.0),
    ])
    .unwrap();
    assert_eq!(
        path.resample_by_step(1e-300),
        Err(RhumbLineError::InvalidParameter)
    );
}
