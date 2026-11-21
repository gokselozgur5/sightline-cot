use sightline_cot::track_assoc::{compute_spatial_matrix_2d, solve_optimal_hungarian};

#[test]
fn hungarian_keeps_in_gate_pair_when_cheaper_total_uses_gated_pairs() {
    let tracks = [[0.0, 0.0], [-52.0, 0.0]];
    let dets = [[-1.0, 0.0], [51.0, 0.0]];
    let mat = compute_spatial_matrix_2d(&tracks, &dets).unwrap();
    let res = solve_optimal_hungarian(&mat, 50.0);
    assert_eq!(res.matches.len(), 1, "matches: {:?}", res.matches);
    assert_eq!((res.matches[0].track_idx, res.matches[0].det_idx), (0, 0));
    assert_eq!(res.unmatched_tracks, vec![1]);
    assert_eq!(res.unmatched_dets, vec![1]);
}
