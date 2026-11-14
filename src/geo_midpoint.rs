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

/// Compute the great-circle midpoint between two (lat, lon) coordinates in degrees.
pub fn geodesic_midpoint(
    lat1: f64,
    lon1: f64,
    lat2: f64,
    lon2: f64,
) -> Result<(f64, f64), MidpointError> {
    validate_coords(lat1, lon1)?;
    validate_coords(lat2, lon2)?;
    let (phi1, phi2) = (lat1.to_radians(), lat2.to_radians());
    let mut dlambda = (lon2 - lon1).to_radians();
    let pi = core::f64::consts::PI;
    dlambda = (dlambda + pi).rem_euclid(2.0 * pi) - pi;
    let (sin_phi1, cos_phi1) = (phi1.sin(), phi1.cos());
    let (sin_phi2, cos_phi2) = (phi2.sin(), phi2.cos());
    let bx = cos_phi2 * dlambda.cos();
    let by = cos_phi2 * dlambda.sin();
    let denom = ((cos_phi1 + bx).powi(2) + by.powi(2)).sqrt();
    if denom < 1e-12 && (sin_phi1 + sin_phi2).abs() < 1e-12 {
        return Err(MidpointError::AntipodalPoints);
    }
    let phi_m = (sin_phi1 + sin_phi2).atan2(denom).to_degrees();
    let mut lambda_m = (lon1.to_radians() + by.atan2(cos_phi1 + bx)).to_degrees();
    lambda_m = (lambda_m + 180.0).rem_euclid(360.0) - 180.0;
    Ok((phi_m, lambda_m))
}

use crate::geo_vincenty::{vincenty_direct, vincenty_inverse, VincentyError};

impl From<VincentyError> for MidpointError {
    fn from(err: VincentyError) -> Self {
        match err {
            VincentyError::InvalidLatitude => MidpointError::InvalidLatitude,
            VincentyError::InvalidLongitude => MidpointError::InvalidLongitude,
            VincentyError::NoConvergence
            | VincentyError::NegativeDistance
            | VincentyError::NonFiniteInput => MidpointError::AntipodalPoints,
        }
    }
}

fn validate_fraction(f: f64) -> Result<(), MidpointError> {
    if !f.is_finite() || !(0.0..=1.0).contains(&f) {
        Err(MidpointError::AntipodalPoints)
    } else {
        Ok(())
    }
}
