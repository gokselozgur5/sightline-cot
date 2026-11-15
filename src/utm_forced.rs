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
