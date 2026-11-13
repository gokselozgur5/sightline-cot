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
