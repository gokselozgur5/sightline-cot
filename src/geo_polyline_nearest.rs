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
