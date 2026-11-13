//! Seven-parameter Helmert datum transform on ECEF coordinates.

const ARCSEC_TO_RAD: f64 = std::f64::consts::PI / 648_000.0;

/// Position-vector Helmert parameters: translations in meters,
/// rotations in arcseconds and scale in parts per million.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct HelmertParams {
    pub translation_m: [f64; 3],
    pub rotation_arcsec: [f64; 3],
    pub scale_ppm: f64,
}

impl HelmertParams {
    /// Parameters that leave every point unchanged.
    pub fn identity() -> Self {
        Self::default()
    }
}
