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
