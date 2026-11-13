//! Geodesic nearest point on a polyline or segment on the sphere.

/// Default Earth mean radius in meters.
pub const DEFAULT_EARTH_RADIUS_M: f64 = 6_371_000.0;

/// Result of finding the nearest point on a polyline or segment.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PolylineNearest {
    /// Nearest point on the polyline as [latitude_deg, longitude_deg].
    pub point: [f64; 2],
    /// Geodesic distance in meters from the query point to the nearest point.
    pub distance_m: f64,
    /// 0-based index of the polyline segment containing the nearest point.
    pub segment_index: usize,
    /// Fractional position along the segment in [0.0, 1.0].
    pub fraction: f64,
}

fn to_unit_vec(lat_deg: f64, lon_deg: f64) -> [f64; 3] {
    let (phi, lam) = (lat_deg.to_radians(), lon_deg.to_radians());
    let c = phi.cos();
    [c * lam.cos(), c * lam.sin(), phi.sin()]
}

fn to_lat_lon(v: [f64; 3]) -> [f64; 2] {
    let hyp = (v[0] * v[0] + v[1] * v[1]).sqrt();
    [v[2].atan2(hyp).to_degrees(), v[1].atan2(v[0]).to_degrees()]
}
