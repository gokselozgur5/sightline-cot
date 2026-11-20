use crate::geo_greatcircle::EARTH_RADIUS_M;

/// Destination (lat, lon) in degrees with a custom sphere radius in meters.
pub fn destination_point_with_radius(
    lat: f64,
    lon: f64,
    bearing_deg: f64,
    dist_m: f64,
    radius_m: f64,
) -> (f64, f64) {
    let d = dist_m / radius_m;
    let (p1, l1, b) = (lat.to_radians(), lon.to_radians(), bearing_deg.to_radians());
    let p2 = (p1.sin() * d.cos() + p1.cos() * d.sin() * b.cos())
        .clamp(-1.0, 1.0)
        .asin();
    let lon2 = if 90.0 - lat.abs() < 1e-12 {
        let l = if lat > 0.0 {
            lon + 180.0 - bearing_deg
        } else {
            lon + bearing_deg
        };
        (l + 540.0).rem_euclid(360.0) - 180.0
    } else {
        let l2 = l1 + (b.sin() * d.sin() * p1.cos()).atan2(d.cos() - p1.sin() * p2.sin());
        (l2.to_degrees() + 540.0).rem_euclid(360.0) - 180.0
    };
    (p2.to_degrees(), lon2)
}

/// Destination (lat, lon) in degrees from a start point, bearing and distance in meters.
pub fn destination_point(lat: f64, lon: f64, bearing_deg: f64, dist_m: f64) -> (f64, f64) {
    destination_point_with_radius(lat, lon, bearing_deg, dist_m, EARTH_RADIUS_M)
}

/// Final bearing in degrees [0, 360) with custom sphere radius in meters.
pub fn final_bearing_with_radius(
    lat: f64,
    _lon: f64,
    bearing_deg: f64,
    dist_m: f64,
    radius_m: f64,
) -> f64 {
    let d = dist_m / radius_m;
    let (p1, b) = (lat.to_radians(), bearing_deg.to_radians());
    let y = b.sin() * p1.cos();
    let x = d.cos() * b.cos() * p1.cos() - p1.sin() * d.sin();
    (y.atan2(x).to_degrees() + 360.0).rem_euclid(360.0)
}

/// Final bearing in degrees [0, 360) at destination point.
pub fn final_bearing(lat: f64, lon: f64, bearing_deg: f64, dist_m: f64) -> f64 {
    final_bearing_with_radius(lat, lon, bearing_deg, dist_m, EARTH_RADIUS_M)
}
