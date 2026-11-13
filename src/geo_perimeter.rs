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

/// Initial forward bearing in degrees in [0, 360) from `p0` to `p1`.
pub fn segment_bearing_deg(p0: [f64; 2], p1: [f64; 2]) -> f64 {
    let (lat0, lat1) = (p0[0].to_radians(), p1[0].to_radians());
    let dlon = (p1[1] - p0[1]).to_radians();
    let y = dlon.sin() * lat1.cos();
    let x = lat0.cos() * lat1.sin() - lat0.sin() * lat1.cos() * dlon.cos();
    let deg = y.atan2(x).to_degrees();
    let b = deg % 360.0;
    if b < 0.0 {
        b + 360.0
    } else {
        b
    }
}
