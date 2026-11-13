//! Seven-parameter Helmert datum transform between ECEF frames.

/// Translation in metres, rotation in arc seconds and scale in parts per million.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct HelmertParams {
    pub translation: [f64; 3],
    pub rotation_arcsec: [f64; 3],
    pub scale_ppm: f64,
}

impl HelmertParams {
    fn matrix(&self) -> [[f64; 3]; 3] {
        let k = std::f64::consts::PI / (180.0 * 3600.0);
        let [rx, ry, rz] = self.rotation_arcsec.map(|r| r * k);
        let s = 1.0 + self.scale_ppm * 1e-6;
        [
            [s, -s * rz, s * ry],
            [s * rz, s, -s * rx],
            [-s * ry, s * rx, s],
        ]
    }

    /// Applies the position-vector transform to an ECEF point.
    pub fn apply(&self, p: [f64; 3]) -> [f64; 3] {
        let m = self.matrix();
        let t = self.translation;
        std::array::from_fn(|i| t[i] + m[i][0] * p[0] + m[i][1] * p[1] + m[i][2] * p[2])
    }
}
