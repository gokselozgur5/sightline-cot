use sightline_cot::{ElevationMaskValidator, MaskingStatus, ObstaclePoint, ObstacleProfile};

#[test]
fn obstacle_beyond_the_target_does_not_mask_it() {
    let validator = ElevationMaskValidator::new(0.0, -90.0).unwrap();
    let ridge_behind = ObstaclePoint::new(5000.0, 1000.0, 0.0).unwrap();
    let profile = ObstacleProfile::new(vec![ridge_behind]).unwrap();
    let eval = validator
        .validate_target_profile(90.0, 1000.0, 50.0, &profile, false)
        .unwrap();
    assert_eq!(eval.status, MaskingStatus::Clear);
}
