//! Great-circle interpolation and midpoint calculations on a sphere.

/// Error returned when great-circle interpolation fails.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterpError {
    InvalidLatitude,
    InvalidLongitude,
    InvalidFraction,
    AntipodalPoints,
}

pub(crate) fn validate_coords(lat: f64, lon: f64) -> Result<(), InterpError> {
    if !lat.is_finite() || !(-90.0..=90.0).contains(&lat) {
        return Err(InterpError::InvalidLatitude);
    }
    if !lon.is_finite() || !(-180.0..=180.0).contains(&lon) {
        return Err(InterpError::InvalidLongitude);
    }
    Ok(())
}

pub(crate) fn validate_fraction(f: f64) -> Result<(), InterpError> {
    if !f.is_finite() || !(0.0..=1.0).contains(&f) {
        return Err(InterpError::InvalidFraction);
    }
    Ok(())
}
