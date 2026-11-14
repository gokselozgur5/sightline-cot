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

/// Compute great-circle intermediate point at fraction `f` in [0, 1].
pub fn great_circle_intermediate(
    lat1: f64,
    lon1: f64,
    lat2: f64,
    lon2: f64,
    f: f64,
) -> Result<(f64, f64), InterpError> {
    validate_coords(lat1, lon1)?;
    validate_coords(lat2, lon2)?;
    validate_fraction(f)?;
    let (phi1, phi2) = (lat1.to_radians(), lat2.to_radians());
    let (lam1, lam2) = (lon1.to_radians(), lon2.to_radians());
    let (c1, c2) = (phi1.cos(), phi2.cos());
    let v1 = [c1 * lam1.cos(), c1 * lam1.sin(), phi1.sin()];
    let v2 = [c2 * lam2.cos(), c2 * lam2.sin(), phi2.sin()];
    let dot = (v1[0] * v2[0] + v1[1] * v2[1] + v1[2] * v2[2]).clamp(-1.0, 1.0);
    if dot <= -1.0 + 1e-12 {
        return Err(InterpError::AntipodalPoints);
    }
    let d = dot.acos();
    if d < 1e-15 {
        return Ok((lat1, lon1));
    }
    let (a, b) = (((1.0 - f) * d).sin() / d.sin(), (f * d).sin() / d.sin());
    let (x, y, z) = (
        a * v1[0] + b * v2[0],
        a * v1[1] + b * v2[1],
        a * v1[2] + b * v2[2],
    );
    let lat = z.atan2((x * x + y * y).sqrt()).to_degrees();
    let lon = (y.atan2(x).to_degrees() + 180.0).rem_euclid(360.0) - 180.0;
    Ok((lat, lon))
}

/// Compute great-circle midpoint between two geodetic coordinates.
pub fn great_circle_midpoint(
    lat1: f64,
    lon1: f64,
    lat2: f64,
    lon2: f64,
) -> Result<(f64, f64), InterpError> {
    great_circle_intermediate(lat1, lon1, lat2, lon2, 0.5)
}
