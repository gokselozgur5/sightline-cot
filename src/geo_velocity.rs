//! Rotation of velocity vectors between ECEF and local ENU axes.

fn basis(lat_deg: f64, lon_deg: f64) -> [[f64; 3]; 3] {
    let (sp, cp) = lat_deg.to_radians().sin_cos();
    let (sl, cl) = lon_deg.to_radians().sin_cos();
    [
        [-sl, cl, 0.0],
        [-sp * cl, -sp * sl, cp],
        [cp * cl, cp * sl, sp],
    ]
}
