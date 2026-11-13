//! Rotation of velocity vectors between ECEF and local ENU frames.

fn basis(lat_deg: f64, lon_deg: f64) -> [[f64; 3]; 3] {
    let (p, l) = (lat_deg.to_radians(), lon_deg.to_radians());
    let (sp, cp, sl, cl) = (p.sin(), p.cos(), l.sin(), l.cos());
    [[-sl, cl, 0.0], [-sp * cl, -sp * sl, cp], [cp * cl, cp * sl, sp]]
}

/// Rotate an ECEF velocity (m/s) into east, north, up at the given point.
pub fn ecef_to_enu_velocity(v: [f64; 3], lat_deg: f64, lon_deg: f64) -> [f64; 3] {
    let r = basis(lat_deg, lon_deg);
    r.map(|row| row[0] * v[0] + row[1] * v[1] + row[2] * v[2])
}
