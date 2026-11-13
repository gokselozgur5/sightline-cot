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

impl HelmertParams {
    /// Exact inverse of [`HelmertParams::apply`], solved by fixed-point iteration.
    pub fn invert(&self, q: [f64; 3]) -> [f64; 3] {
        let k = 1.0 + self.scale_ppm * 1e-6;
        let t = self.translation_m;
        let base = [(q[0] - t[0]) / k, (q[1] - t[1]) / k, (q[2] - t[2]) / k];
        let rot_only = Self {
            translation_m: [0.0; 3],
            scale_ppm: 0.0,
            ..*self
        };
        let mut p = base;
        for _ in 0..8 {
            let rp = rot_only.apply(p);
            p = [
                base[0] - (rp[0] - p[0]),
                base[1] - (rp[1] - p[1]),
                base[2] - (rp[2] - p[2]),
            ];
        }
        p
    }
}
