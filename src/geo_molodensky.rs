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

impl MolodenskyShift {
    /// Shifts (lat deg, lon deg, height m) to the target datum.
    pub fn apply(&self, lat_deg: f64, lon_deg: f64, h: f64) -> (f64, f64, f64) {
        let (a, f) = (self.a, self.f);
        let [dx, dy, dz] = self.translation;
        let (phi, lam) = (lat_deg.to_radians(), lon_deg.to_radians());
        let (sp, cp, sl, cl) = (phi.sin(), phi.cos(), lam.sin(), lam.cos());
        let e2 = f * (2.0 - f);
        let b = a * (1.0 - f);
        let w = 1.0 - e2 * sp * sp;
        let rn = a / w.sqrt();
        let rm = a * (1.0 - e2) / w.powf(1.5);
        let dphi = (-dx * sp * cl - dy * sp * sl
            + dz * cp
            + self.da * rn * e2 * sp * cp / a
            + self.df * (rm * a / b + rn * b / a) * sp * cp)
            / (rm + h);
        let dlam = (-dx * sl + dy * cl) / ((rn + h) * cp);
        let dh = dx * cp * cl + dy * cp * sl + dz * sp - self.da * a / rn
            + self.df * (b / a) * rn * sp * sp;
        (
            lat_deg + dphi.to_degrees(),
            lon_deg + dlam.to_degrees(),
            h + dh,
        )
    }
}
