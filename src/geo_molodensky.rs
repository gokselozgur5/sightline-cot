//! Abridged Molodensky datum shift on geodetic coordinates.

/// Source ellipsoid and the shift to the target datum.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MolodenskyShift {
    /// Source semi-major axis in metres.
    pub a: f64,
    /// Source flattening.
    pub f: f64,
    /// Geocentric translation (dx, dy, dz) in metres.
    pub translation: [f64; 3],
    /// Target minus source semi-major axis in metres.
    pub da: f64,
    /// Target minus source flattening.
    pub df: f64,
}
