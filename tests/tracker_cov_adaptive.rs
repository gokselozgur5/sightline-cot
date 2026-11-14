use sightline_cot::tracker_cov_adaptive::*;

#[test]
fn test_adaptive_covariance_config_defaults() {
    let cfg = AdaptiveCovarianceConfig::default();
    assert_eq!(cfg.gate_threshold, 5.991);
    assert_eq!(cfg.min_scale, 1.0);
    assert_eq!(cfg.max_scale, 10.0);
    assert!(cfg.validate().is_ok());
}

#[test]
fn test_adaptive_covariance_config_invalid() {
    let mut cfg = AdaptiveCovarianceConfig::default();
    cfg.gate_threshold = -1.0;
    assert_eq!(cfg.validate(), Err(AdaptiveCovarianceError::InvalidConfig));

    let mut cfg2 = AdaptiveCovarianceConfig::default();
    cfg2.min_scale = 0.5;
    assert_eq!(cfg2.validate(), Err(AdaptiveCovarianceError::InvalidConfig));

    let mut cfg3 = AdaptiveCovarianceConfig::default();
    cfg3.max_scale = 0.9;
    assert_eq!(cfg3.validate(), Err(AdaptiveCovarianceError::InvalidConfig));
}

#[test]
fn test_adaptive_scale_bounds() {
    let cfg = AdaptiveCovarianceConfig::default();
    assert_eq!(compute_adaptive_scale(0.0, &cfg).unwrap(), 1.0);
    assert_eq!(compute_adaptive_scale(5.991, &cfg).unwrap(), 1.0);
    assert_eq!(compute_adaptive_scale(59.91, &cfg).unwrap(), 10.0);
    assert_eq!(compute_adaptive_scale(100.0, &cfg).unwrap(), 10.0);
}

#[test]
fn test_scale_measurement_variance() {
    let cfg = AdaptiveCovarianceConfig::default();
    let r_base = 4.0;
    let scaled = scale_measurement_variance(r_base, 11.982, &cfg).unwrap();
    assert!((scaled - 8.0).abs() < 1e-4);
    assert!(scale_measurement_variance(-1.0, 5.0, &cfg).is_err());
    assert!(scale_measurement_variance(r_base, -1.0, &cfg).is_err());
}



