//! Rhumb line (loxodrome) distance, bearing, and interpolation.

use std::f64::consts::{FRAC_PI_2, FRAC_PI_4};

pub const WGS84_EARTH_RADIUS: f64 = 6_371_008.8;

fn unwrap_lon_diff(lon1_deg: f64, lon2_deg: f64) -> f64 {
    ((lon2_deg - lon1_deg + 540.0).rem_euclid(360.0) - 180.0).to_radians()
}

fn mercator_lat(lat_rad: f64) -> f64 {
    (FRAC_PI_4 + lat_rad / 2.0).tan().ln()
}
