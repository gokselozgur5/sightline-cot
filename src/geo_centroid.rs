//! Polygon centroid in a local tangent frame.

/// Centroid of a 2D planar polygon from its vertices in meters or local units.
pub fn planar_centroid(pts: &[[f64; 2]]) -> Option<[f64; 2]> {
    let n = pts.len();
    if n < 3 || pts.iter().flatten().any(|c| !c.is_finite()) {
        return None;
    }
    let mut area2 = 0.0;
    let (mut cx, mut cy) = (0.0, 0.0);
    for i in 0..n {
        let (p0, p1) = (pts[i], pts[(i + 1) % n]);
        let cross = p0[0] * p1[1] - p1[0] * p0[1];
        area2 += cross;
        cx += (p0[0] + p1[0]) * cross;
        cy += (p0[1] + p1[1]) * cross;
    }
    if area2.abs() < 1e-12 || !area2.is_finite() {
        return None;
    }
    let inv = 1.0 / (3.0 * area2);
    Some([cx * inv, cy * inv])
}

const METERS_PER_DEG: f64 = 111_194.926_6;

fn unwrap_lon(lon: f64, ref_lon: f64) -> f64 {
    let mut d = lon - ref_lon;
    while d > 180.0 {
        d -= 360.0;
    }
    while d < -180.0 {
        d += 360.0;
    }
    d
}

/// Compute the geodetic centroid `[lat_deg, lon_deg]` of a geographic polygon.
pub fn polygon_centroid(poly: &[[f64; 2]]) -> Option<[f64; 2]> {
    let n = poly.len();
    if n < 3 || poly.iter().flatten().any(|c| !c.is_finite()) {
        return None;
    }
    let lat0 = poly.iter().map(|p| p[0]).sum::<f64>() / n as f64;
    let lon0 = poly.iter().map(|p| p[1]).sum::<f64>() / n as f64;
    let cos_lat = lat0.to_radians().cos().max(1e-12);
    let pts: Vec<[f64; 2]> = poly
        .iter()
        .map(|p| {
            let dx = unwrap_lon(p[1], lon0) * cos_lat * METERS_PER_DEG;
            let dy = (p[0] - lat0) * METERS_PER_DEG;
            [dx, dy]
        })
        .collect();
    let [cx, cy] = planar_centroid(&pts)?;
    let lat = lat0 + cy / METERS_PER_DEG;
    let lon = lon0 + cx / (cos_lat * METERS_PER_DEG);
    Some([lat, lon])
}
