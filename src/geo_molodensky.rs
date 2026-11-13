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

impl MolodenskyShift {
    /// Shift (lat_deg, lon_deg, h_m) into the target datum.
    pub fn apply(&self, lat_deg: f64, lon_deg: f64, h: f64) -> (f64, f64, f64) {
        let (phi, lam) = (lat_deg.to_radians(), lon_deg.to_radians());
        let (sp, cp, sl, cl) = (phi.sin(), phi.cos(), lam.sin(), lam.cos());
        let e2 = self.f * (2.0 - self.f);
        let w = 1.0 - e2 * sp * sp;
        let rn = self.a / w.sqrt();
        let rm = self.a * (1.0 - e2) / (w * w.sqrt());
        let [dx, dy, dz] = self.dxyz;
        let one_f = 1.0 - self.f;
        let dphi = (-dx * sp * cl - dy * sp * sl
            + dz * cp
            + self.da * rn * e2 * sp * cp / self.a
            + self.df * (rm / one_f + rn * one_f) * sp * cp)
            / (rm + h);
        let dlam = (-dx * sl + dy * cl) / ((rn + h) * cp);
        let dh = dx * cp * cl + dy * cp * sl + dz * sp - self.da * self.a / rn
            + self.df * one_f * rn * sp * sp;
        (
            lat_deg + dphi.to_degrees(),
            lon_deg + dlam.to_degrees(),
            h + dh,
        )
    }
}
