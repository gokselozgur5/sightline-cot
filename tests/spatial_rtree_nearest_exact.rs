use sightline_cot::spatial_rtree::{haversine_distance_m, SpatialEnvelope, SpatialRTree};

#[test]
fn nearest_neighbor_matches_brute_force_at_high_latitude() {
    let pts = [(85.0, 95.0), (89.0, 91.0), (80.0, 91.0)];
    let mut tree = SpatialRTree::with_max_entries(2);
    for (i, p) in pts.iter().enumerate() {
        tree.insert(SpatialEnvelope::from_point(p.0, p.1), i);
    }
    let (qlat, qlon) = (80.0, 0.0);
    let dist = |i: usize| haversine_distance_m(qlat, qlon, pts[i].0, pts[i].1);
    assert!(dist(1) < dist(0) && dist(0) < dist(2));
    let (got, d) = tree.nearest_neighbor(qlat, qlon).unwrap();
    assert_eq!(*got, 1, "got item {got} at {d} m, nearest is {} m", dist(1));
}
