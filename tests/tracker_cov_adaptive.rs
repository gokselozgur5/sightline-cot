use sightline_cot::tracker_cov_adaptive::*;

#[test]
fn test_adaptive_covariance_config_defaults() {
    let cfg = AdaptiveCovarianceConfig::default();
    assert_eq!(cfg.gate_threshold, 5.991);
    assert_eq!(cfg.min_scale, 1.0);
    assert_eq!(cfg.max_scale, 10.0);
    assert!(cfg.validate().is_ok());
}
