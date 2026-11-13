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

    /// Exactly inverts [`HelmertParams::apply`] by solving the 3x3 system.
    pub fn apply_inverse(&self, q: [f64; 3]) -> [f64; 3] {
        let m = self.matrix();
        let d: [f64; 3] = std::array::from_fn(|i| q[i] - self.translation[i]);
        let det = m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
            - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
            + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0]);
        let solve = |col: usize| {
            let mut a = m;
            for (row, v) in a.iter_mut().zip(d) {
                row[col] = v;
            }
            a[0][0] * (a[1][1] * a[2][2] - a[1][2] * a[2][1])
                - a[0][1] * (a[1][0] * a[2][2] - a[1][2] * a[2][0])
                + a[0][2] * (a[1][0] * a[2][1] - a[1][1] * a[2][0])
        };
        [solve(0) / det, solve(1) / det, solve(2) / det]
    }
}
