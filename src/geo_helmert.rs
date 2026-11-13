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

impl HelmertParams {
    /// Transform an ECEF point in meters.
    pub fn apply(&self, p: [f64; 3]) -> [f64; 3] {
        let [rx, ry, rz] = self.rotation_arcsec.map(|a| a * ARCSEC_TO_RAD);
        let k = 1.0 + self.scale_ppm * 1e-6;
        let r = [
            p[0] - rz * p[1] + ry * p[2],
            rz * p[0] + p[1] - rx * p[2],
            -ry * p[0] + rx * p[1] + p[2],
        ];
        let t = self.translation_m;
        [t[0] + k * r[0], t[1] + k * r[1], t[2] + k * r[2]]
    }
}
