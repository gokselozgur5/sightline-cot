//! Reference ellipsoid models and geometric parameters.

/// Geodetic reference ellipsoid parameters.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ellipsoid {
    pub a: f64,
    pub b: f64,
    pub f: f64,
    pub inv_f: f64,
}

impl Ellipsoid {
    /// Create an ellipsoid from semi-major axis and inverse flattening.
    pub const fn from_a_inv_f(a: f64, inv_f: f64) -> Self {
        let f = 1.0 / inv_f;
        let b = a * (1.0 - f);
        Self { a, b, f, inv_f }
    }

    pub const WGS84: Self = Self::from_a_inv_f(6_378_137.0, 298.257_223_563);
    pub const GRS80: Self = Self::from_a_inv_f(6_378_137.0, 298.257_222_101);
    pub const WGS72: Self = Self::from_a_inv_f(6_378_135.0, 298.26);

    pub const fn wgs84() -> Self { Self::WGS84 }
    pub const fn grs80() -> Self { Self::GRS80 }
    pub const fn wgs72() -> Self { Self::WGS72 }
    pub const fn new(a: f64, inv_f: f64) -> Self { Self::from_a_inv_f(a, inv_f) }
}

pub const WGS84: Ellipsoid = Ellipsoid::WGS84;
pub const GRS80: Ellipsoid = Ellipsoid::GRS80;
pub const WGS72: Ellipsoid = Ellipsoid::WGS72;

impl Default for Ellipsoid {
    fn default() -> Self {
        Self::WGS84
    }
}
