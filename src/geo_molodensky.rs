//! Abridged Molodensky datum shift on geodetic coordinates.

/// Source ellipsoid and the shift to the target datum.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MolodenskyShift {
    /// Source semi-major axis in meters.
    pub a: f64,
    /// Source flattening.
    pub f: f64,
    /// Target minus source semi-major axis in meters.
    pub da: f64,
    /// Target minus source flattening.
    pub df: f64,
    /// Geocentric translations dX, dY, dZ in meters.
    pub dxyz: [f64; 3],
}
