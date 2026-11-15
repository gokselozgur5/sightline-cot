//! UTM projection forced into a chosen zone across boundaries.

use crate::mgrs::{GridError, UtmCoord};

const WGS84_A: f64 = 6378137.0;
const WGS84_F: f64 = 1.0 / 298.257223563;
const UTM_K0: f64 = 0.9996;
const FALSE_EASTING: f64 = 500000.0;
const FALSE_NORTHING_SOUTH: f64 = 10000000.0;

/// Return central meridian for a given UTM zone in radians.
#[inline]
pub fn zone_central_meridian_rad(zone: u8) -> f64 {
    ((zone as f64 - 1.0) * 6.0 - 180.0 + 3.0).to_radians()
}

fn normalize_dlon_rad(mut dlon: f64) -> f64 {
    while dlon > std::f64::consts::PI {
        dlon -= 2.0 * std::f64::consts::PI;
    }
    while dlon < -std::f64::consts::PI {
        dlon += 2.0 * std::f64::consts::PI;
    }
    dlon
}

fn meridian_arc(phi: f64, e2: f64) -> f64 {
    let e4 = e2 * e2;
    let e6 = e4 * e2;
    let a0 = 1.0 - e2 / 4.0 - 3.0 * e4 / 64.0 - 5.0 * e6 / 256.0;
    let a2 = 3.0 * e2 / 8.0 + 3.0 * e4 / 32.0 + 45.0 * e6 / 1024.0;
    let a4 = 15.0 * e4 / 256.0 + 45.0 * e6 / 1024.0;
    let a6 = 35.0 * e6 / 3072.0;
    WGS84_A * (a0 * phi - a2 * (2.0 * phi).sin() + a4 * (4.0 * phi).sin() - a6 * (6.0 * phi).sin())
}
