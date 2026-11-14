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

