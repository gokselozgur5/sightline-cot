//! Angle normalization helpers in degrees.

/// Wraps an angle in degrees into the half-open range [0, 360).
pub fn wrap_360(deg: f64) -> f64 {
    let w = deg.rem_euclid(360.0);
    if w >= 360.0 {
        0.0
    } else {
        w
    }
}

/// Wraps an angle in degrees into the range (-180, 180].
pub fn wrap_180(deg: f64) -> f64 {
    let w = wrap_360(deg);
    if w > 180.0 {
        w - 360.0
    } else {
        w
    }
}
