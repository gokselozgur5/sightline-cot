//! Geodesic midpoint calculation between two geographic coordinates.

/// Error returned when calculating a geodesic midpoint fails.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MidpointError {
    InvalidLatitude,
    InvalidLongitude,
    AntipodalPoints,
}

fn validate_coords(lat: f64, lon: f64) -> Result<(), MidpointError> {
    if !lat.is_finite() || !(-90.0..=90.0).contains(&lat) {
        return Err(MidpointError::InvalidLatitude);
    }
    if !lon.is_finite() || !(-180.0..=180.0).contains(&lon) {
        return Err(MidpointError::InvalidLongitude);
    }
    Ok(())
}
