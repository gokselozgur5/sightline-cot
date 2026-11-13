use sightline_cot::{simplify_polyline, LocalFrame};

#[test]
fn test_endpoints_kept_and_collinear_removed() {
    let pts = vec![
        [45.0, 10.0],
        [45.002, 10.0],
        [45.004, 10.0],
        [45.006, 10.0],
        [45.010, 10.0],
    ];
    let res = simplify_polyline(&pts, 1.0);
    assert_eq!(res.len(), 2);
    assert_eq!(res[0], [45.0, 10.0]);
    assert_eq!(res[1], [45.010, 10.0]);
}

#[test]
fn test_spike_survives_tolerance() {
    let frame = LocalFrame::new(45.0, 10.0);
    let start = frame.unproject([0.0, 0.0]);
    let spike = frame.unproject([500.0, 100.0]);
    let end = frame.unproject([1000.0, 0.0]);
    let pts = vec![start, spike, end];

    let kept = simplify_polyline(&pts, 10.0);
    assert_eq!(kept.len(), 3);
    assert_eq!(kept[1], spike);

    let smoothed = simplify_polyline(&pts, 150.0);
    assert_eq!(smoothed.len(), 2);
    assert_eq!(smoothed[0], start);
    assert_eq!(smoothed[1], end);
}

#[test]
fn test_edge_cases() {
    assert!(simplify_polyline(&[], 10.0).is_empty());
    let single = vec![[45.0, 10.0]];
    assert_eq!(simplify_polyline(&single, 10.0), single);
    let pair = vec![[45.0, 10.0], [45.1, 10.1]];
    assert_eq!(simplify_polyline(&pair, 10.0), pair);
    assert_eq!(simplify_polyline(&pair, -5.0), pair);
}

#[test]
fn test_local_frame_roundtrip() {
    let frame = LocalFrame::new(37.5, -122.2);
    let pt = [37.55, -122.15];
    let xy = frame.project(pt);
    let back = frame.unproject(xy);
    assert!((back[0] - pt[0]).abs() < 1e-9);
    assert!((back[1] - pt[1]).abs() < 1e-9);
}
