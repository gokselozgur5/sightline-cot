use crate::geo_greatcircle::EARTH_RADIUS_M;

/// Destination (lat, lon) in degrees from a start point, bearing and distance in meters.
pub fn destination_point(lat: f64, lon: f64, bearing_deg: f64, dist_m: f64) -> (f64, f64) {
    let d = dist_m / EARTH_RADIUS_M;
    let (p1, l1, b) = (lat.to_radians(), lon.to_radians(), bearing_deg.to_radians());
    let p2 = (p1.sin() * d.cos() + p1.cos() * d.sin() * b.cos()).asin();
    let l2 = l1 + (b.sin() * d.sin() * p1.cos()).atan2(d.cos() - p1.sin() * p2.sin());
    let lon2 = (l2.to_degrees() + 540.0).rem_euclid(360.0) - 180.0;
    (p2.to_degrees(), lon2)
}
