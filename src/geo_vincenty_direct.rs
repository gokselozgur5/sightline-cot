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

fn vincenty_ab(u2: f64) -> (f64, f64) {
    let a = 1.0 + u2 / 16384.0 * (4096.0 + u2 * (-768.0 + u2 * (320.0 - 175.0 * u2)));
    let b = u2 / 1024.0 * (256.0 + u2 * (-128.0 + u2 * (74.0 - 47.0 * u2)));
    (a, b)
}

fn delta_sigma(b: f64, sin_sig: f64, cos_sig: f64, cos_2sm: f64) -> f64 {
    let cos2_2sm = cos_2sm * cos_2sm;
    b * sin_sig
        * (cos_2sm
            + b / 4.0
                * (cos_sig * (-1.0 + 2.0 * cos2_2sm)
                    - b / 6.0
                        * cos_2sm
                        * (-3.0 + 4.0 * sin_sig * sin_sig)
                        * (-3.0 + 4.0 * cos2_2sm)))
}
