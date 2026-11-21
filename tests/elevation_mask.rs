//! Integration tests for terrain elevation masking validator.

use sightline_cot::elevation_mask::*;

#[test]
fn test_elevation_mask_error_display() {
    let err = ElevationMaskError::InvalidAzimuth;
    assert!(!format!("{}", err).is_empty());
    assert_eq!(
        format!("{}", err),
        "azimuth must be in range [0.0, 360.0) degrees"
    );
}

#[test]
fn test_elevation_mask_validator_full_integration() {
    let mut val = ElevationMaskValidator::new(50.0, 3.0).unwrap();
    val.add_sector(MaskSector::new(0.0, 90.0, 10.0).unwrap());
    val.add_sector(MaskSector::new(270.0, 360.0, 6.0).unwrap());

    assert_eq!(val.get_mask_angle_at_azimuth(45.0).unwrap(), 10.0);
    assert_eq!(val.get_mask_angle_at_azimuth(180.0).unwrap(), 3.0);
    assert_eq!(val.get_mask_angle_at_azimuth(300.0).unwrap(), 6.0);

    let res_clear = val.validate_target_angles(45.0, 12.0).unwrap();
    assert_eq!(res_clear.status, MaskingStatus::Clear);
    assert!(res_clear.is_visible());
    assert!((res_clear.clearance_margin_deg - 2.0).abs() < 1e-6);

    let obs_pt1 = ObstaclePoint::new(1000.0, 80.0, 0.0).unwrap();
    let obs_pt2 = ObstaclePoint::new(2500.0, 200.0, 0.0).unwrap();
    let profile = ObstacleProfile::new(vec![obs_pt1, obs_pt2]).unwrap();

    let prof_eval = val
        .validate_target_profile(45.0, 4000.0, 1000.0, &profile, false)
        .unwrap();
    assert!(prof_eval.is_visible());
    assert_eq!(prof_eval.status, MaskingStatus::Clear);

    let masked_eval = val
        .validate_target_profile(45.0, 4000.0, 400.0, &profile, false)
        .unwrap();
    assert!(!masked_eval.is_visible());
    assert_eq!(masked_eval.status, MaskingStatus::Masked);
}
