use sightline_cot::geo_polyline_nearest::*;

#[test]
fn test_point_50m_off_segment() {
    let a = [0.0, 0.0];
    let b = [0.0, 1.0];
    let d_lat = (50.0 / DEFAULT_EARTH_RADIUS_M).to_degrees();
    let query = [d_lat, 0.5];
    let res = nearest_point_on_segment(a, b, query);
    assert!((res.distance_m - 50.0).abs() < 1e-4);
    assert!(res.point[0].abs() < 1e-6);
    assert!((res.point[1] - 0.5).abs() < 1e-6);
    assert!((res.fraction - 0.5).abs() < 1e-4);
}

#[test]
fn test_endpoint_query() {
    let a = [0.0, 0.0];
    let b = [0.0, 1.0];
    let res_a = nearest_point_on_segment(a, b, a);
    assert!(res_a.distance_m < 1e-6);
    assert_eq!(res_a.point, a);
    assert_eq!(res_a.fraction, 0.0);
    let res_b = nearest_point_on_segment(a, b, b);
    assert!(res_b.distance_m < 1e-6);
    assert_eq!(res_b.point, b);
    assert_eq!(res_b.fraction, 1.0);
}

#[test]
fn test_polyline_multi_segment() {
    let poly = vec![[0.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
    let res = nearest_point_on_polyline(&poly, [0.5, 1.0001]).unwrap();
    assert_eq!(res.segment_index, 1);
    assert!((res.point[0] - 0.5).abs() < 1e-4);
    assert!((res.point[1] - 1.0).abs() < 1e-4);
    assert_eq!(nearest_point_on_polyline(&[], [0.0, 0.0]), None);
    let single = nearest_point_on_polyline(&[[10.0, 20.0]], [10.0, 20.0]).unwrap();
    assert_eq!(single.point, [10.0, 20.0]);
    assert!(single.distance_m < 1e-6);
}

#[test]
fn test_antimeridian_segment() {
    let a = [0.0, 179.0];
    let b = [0.0, -179.0];
    let res = nearest_point_on_segment(a, b, [0.0, 180.0]);
    assert!(res.distance_m < 1e-4);
    assert!((res.fraction - 0.5).abs() < 1e-4);
}
