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

/// Rotates an ECEF velocity (m/s) into east, north, up at the given position.
pub fn ecef_to_enu_velocity(v: [f64; 3], lat_deg: f64, lon_deg: f64) -> [f64; 3] {
    let m = basis(lat_deg, lon_deg);
    std::array::from_fn(|i| m[i][0] * v[0] + m[i][1] * v[1] + m[i][2] * v[2])
}

/// Rotates an east, north, up velocity (m/s) into ECEF axes.
pub fn enu_to_ecef_velocity(v: [f64; 3], lat_deg: f64, lon_deg: f64) -> [f64; 3] {
    let m = basis(lat_deg, lon_deg);
    std::array::from_fn(|i| m[0][i] * v[0] + m[1][i] * v[1] + m[2][i] * v[2])
}
