//! Vincenty direct geodesic algorithm on reference ellipsoids.

pub use crate::ellipsoid::{Ellipsoid, WGS84};
pub use crate::geo_vincenty::{VincentyDirect, VincentyError};

fn validate_direct_inputs(
    lat: f64,
    lon: f64,
    bearing: f64,
    dist: f64,
) -> Result<(), VincentyError> {
    if !lat.is_finite() || !lon.is_finite() || !bearing.is_finite() || !dist.is_finite() {
        return Err(VincentyError::NonFiniteInput);
    }
    if !(-90.0..=90.0).contains(&lat) {
        return Err(VincentyError::InvalidLatitude);
    }
    if !(-180.0..=180.0).contains(&lon) {
        return Err(VincentyError::InvalidLongitude);
    }
    if dist < 0.0 {
        return Err(VincentyError::NegativeDistance);
    }
    Ok(())
}
