//! Rhumb line (loxodrome) distance, bearing, and interpolation.

use std::f64::consts::{FRAC_PI_2, FRAC_PI_4};

pub const WGS84_EARTH_RADIUS: f64 = 6_371_008.8;

fn unwrap_lon_diff(lon1_deg: f64, lon2_deg: f64) -> f64 {
    ((lon2_deg - lon1_deg + 540.0).rem_euclid(360.0) - 180.0).to_radians()
}

fn mercator_lat(lat_rad: f64) -> f64 {
    (FRAC_PI_4 + lat_rad / 2.0).tan().ln()
}

/// Computes the rhumb line distance between two geodetic points with a custom sphere radius.
pub fn rhumb_distance_radius(p1: [f64; 2], p2: [f64; 2], radius: f64) -> f64 {
    let (lat1, lat2) = (p1[0].to_radians(), p2[0].to_radians());
    let dlat = lat2 - lat1;
    let dlon = unwrap_lon_diff(p1[1], p2[1]);
    let dpsi = mercator_lat(lat2) - mercator_lat(lat1);
    let q = if dpsi.abs() < 1e-12 {
        lat1.cos()
    } else {
        dlat / dpsi
    };
    radius * (dlat * dlat + q * q * dlon * dlon).sqrt()
}

/// Computes the rhumb line distance using WGS84 mean earth radius.
pub fn rhumb_distance(p1: [f64; 2], p2: [f64; 2]) -> f64 {
    rhumb_distance_radius(p1, p2, WGS84_EARTH_RADIUS)
}

/// Computes the constant rhumb line bearing (course) in degrees from `p1` to `p2` in [0, 360).
pub fn rhumb_bearing(p1: [f64; 2], p2: [f64; 2]) -> f64 {
    let (lat1, lat2) = (p1[0].to_radians(), p2[0].to_radians());
    let dlon = unwrap_lon_diff(p1[1], p2[1]);
    let dpsi = mercator_lat(lat2) - mercator_lat(lat1);
    let bearing = dlon.atan2(dpsi).to_degrees();
    bearing.rem_euclid(360.0)
}

/// Computes the destination point along a rhumb line with custom radius.
pub fn rhumb_destination_radius(
    start: [f64; 2],
    bearing_deg: f64,
    distance_m: f64,
    radius: f64,
) -> [f64; 2] {
    let lat1 = start[0].to_radians();
    let lon1 = start[1].to_radians();
    let theta = bearing_deg.to_radians();
    let delta = distance_m / radius;
    let dlat = delta * theta.cos();
    let lat2 = (lat1 + dlat).clamp(-FRAC_PI_2, FRAC_PI_2);
    let dpsi = mercator_lat(lat2) - mercator_lat(lat1);
    let q = if dpsi.abs() < 1e-12 {
        lat1.cos()
    } else {
        (lat2 - lat1) / dpsi
    };
    let dlon = if q.abs() < 1e-12 {
        0.0
    } else {
        (delta * theta.sin()) / q
    };
    let lon2 = ((lon1 + dlon).to_degrees() + 180.0).rem_euclid(360.0) - 180.0;
    [lat2.to_degrees(), lon2]
}

/// Computes the destination point along a rhumb line using WGS84 mean earth radius.
pub fn rhumb_destination(start: [f64; 2], bearing_deg: f64, distance_m: f64) -> [f64; 2] {
    rhumb_destination_radius(start, bearing_deg, distance_m, WGS84_EARTH_RADIUS)
}

/// Computes an intermediate point along a rhumb line at fraction `f` in [0, 1].
pub fn rhumb_intermediate(p1: [f64; 2], p2: [f64; 2], f: f64) -> [f64; 2] {
    if f <= 0.0 {
        return p1;
    }
    if f >= 1.0 {
        return p2;
    }
    let lat1 = p1[0].to_radians();
    let lat2 = p2[0].to_radians();
    let dlat = lat2 - lat1;
    let dlon = unwrap_lon_diff(p1[1], p2[1]);
    let lat_f = lat1 + f * dlat;
    let dpsi = mercator_lat(lat2) - mercator_lat(lat1);
    let dlon_f = if dpsi.abs() < 1e-12 {
        f * dlon
    } else {
        (mercator_lat(lat_f) - mercator_lat(lat1)) * (dlon / dpsi)
    };
    let lon_f = (p1[1] + dlon_f.to_degrees() + 180.0).rem_euclid(360.0) - 180.0;
    [lat_f.to_degrees(), lon_f]
}
