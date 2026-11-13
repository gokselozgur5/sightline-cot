//! Geodesic perimeter and segment bearings.

const EARTH_RADIUS_M: f64 = 6_371_000.0;

/// Great-circle distance between two `[lat_deg, lon_deg]` points in meters.
pub fn segment_distance_m(p0: [f64; 2], p1: [f64; 2]) -> f64 {
    let (lat0, lat1) = (p0[0].to_radians(), p1[0].to_radians());
    let (dlat, dlon) = ((p1[0] - p0[0]).to_radians(), (p1[1] - p0[1]).to_radians());
    let a = (dlat * 0.5).sin().powi(2) + lat0.cos() * lat1.cos() * (dlon * 0.5).sin().powi(2);
    2.0 * EARTH_RADIUS_M
        * a.clamp(0.0, 1.0)
            .sqrt()
            .atan2((1.0 - a).clamp(0.0, 1.0).sqrt())
}
