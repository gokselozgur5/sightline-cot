use sightline_cot::track_assoc::{
    compute_spatial_matrix_2d, solve_nearest_neighbor, solve_optimal_hungarian, AssocConfig,
    AssocError, DistanceMatrix,
};

#[test]
fn test_integration_track_assoc_full_pipeline() {
    let cfg = AssocConfig::default();
    assert!(cfg.validate().is_ok());

    let tracks = [[0.0, 0.0], [100.0, 100.0], [500.0, 500.0]];
    let detections = [[1.0, 1.0], [101.0, 99.0], [1000.0, 1000.0]];
    let mat = compute_spatial_matrix_2d(&tracks, &detections).unwrap();

    let res = solve_optimal_hungarian(&mat, cfg.gate_threshold);
    assert_eq!(res.matches.len(), 2);
    assert_eq!(res.matches[0].track_idx, 0);
    assert_eq!(res.matches[0].det_idx, 0);
    assert_eq!(res.matches[1].track_idx, 1);
    assert_eq!(res.matches[1].det_idx, 1);
    assert_eq!(res.unmatched_tracks, vec![2]);
    assert_eq!(res.unmatched_dets, vec![2]);
}

#[test]
fn test_integration_gating_and_edge_rejection() {
    let mat = DistanceMatrix::new(2, 2, 100.0);
    let res = solve_nearest_neighbor(&mat, 50.0);
    assert!(res.matches.is_empty());
    assert_eq!(res.unmatched_tracks, vec![0, 1]);
    assert_eq!(res.unmatched_dets, vec![0, 1]);

    let nan_tracks = [[f64::NAN, 0.0]];
    let valid_dets = [[0.0, 0.0]];
    let err = compute_spatial_matrix_2d(&nan_tracks, &valid_dets);
    assert_eq!(err, Err(AssocError::NonFiniteInput));
}
