//! Geodesic perimeter and segment bearings.

pub use crate::geo_greatcircle::EARTH_RADIUS_M;

/// Great-circle distance between two `[lat_deg, lon_deg]` points in meters.
pub fn segment_distance_m(p0: [f64; 2], p1: [f64; 2]) -> f64 {
    crate::geo_greatcircle::haversine_distance(p0[0], p0[1], p1[0], p1[1])
}

/// Initial forward bearing in degrees in [0, 360) from `p0` to `p1`.
pub fn segment_bearing_deg(p0: [f64; 2], p1: [f64; 2]) -> f64 {
    crate::geo_greatcircle::initial_bearing(p0[0], p0[1], p1[0], p1[1])
}

/// Total geodesic perimeter along an open polyline in meters.
pub fn polyline_perimeter(pts: &[[f64; 2]]) -> Option<f64> {
    if pts.len() < 2 || pts.iter().flatten().any(|c| !c.is_finite()) {
        return None;
    }
    Some(pts.windows(2).map(|w| segment_distance_m(w[0], w[1])).sum())
}

/// Total geodesic perimeter around a closed polygon in meters.
pub fn polygon_perimeter(poly: &[[f64; 2]]) -> Option<f64> {
    let n = poly.len();
    if n < 3 || poly.iter().flatten().any(|c| !c.is_finite()) {
        return None;
    }
    Some(
        (0..n)
            .map(|i| segment_distance_m(poly[i], poly[(i + 1) % n]))
            .sum(),
    )
}

/// Forward bearing in degrees [0, 360) for each edge of a polygon.
pub fn polygon_segment_bearings(poly: &[[f64; 2]]) -> Option<Vec<f64>> {
    let n = poly.len();
    if n < 3 || poly.iter().flatten().any(|c| !c.is_finite()) {
        return None;
    }
    Some(
        (0..n)
            .map(|i| segment_bearing_deg(poly[i], poly[(i + 1) % n]))
            .collect(),
    )
}
