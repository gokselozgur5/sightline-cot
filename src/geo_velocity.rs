//! Rotation of velocity vectors between ECEF and local ENU frames.

fn basis(lat_deg: f64, lon_deg: f64) -> [[f64; 3]; 3] {
    let (p, l) = (lat_deg.to_radians(), lon_deg.to_radians());
    let (sp, cp, sl, cl) = (p.sin(), p.cos(), l.sin(), l.cos());
    [[-sl, cl, 0.0], [-sp * cl, -sp * sl, cp], [cp * cl, cp * sl, sp]]
}
