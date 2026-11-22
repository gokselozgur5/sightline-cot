//! WGS84 radii of curvature as functions of geodetic latitude.

use crate::ellipsoid::Ellipsoid;

/// Meridian radius of curvature M in metres at `lat_deg` degrees.
pub fn meridian_radius(lat_deg: f64) -> f64 {
    Ellipsoid::WGS84.meridian_radius(lat_deg)
}

/// Prime-vertical radius of curvature N in metres at `lat_deg` degrees.
pub fn prime_vertical_radius(lat_deg: f64) -> f64 {
    Ellipsoid::WGS84.prime_vertical_radius(lat_deg)
}
