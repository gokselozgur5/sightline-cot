//! Polygon area and perimeter in a local tangent frame.

const METERS_PER_DEG: f64 = 111_194.926_6;

/// Planar area of a 2D polygon in square units.
pub fn planar_polygon_area(pts: &[[f64; 2]]) -> Option<f64> {
    let n = pts.len();
    if n < 3 || pts.iter().flatten().any(|c| !c.is_finite()) {
        return None;
    }
    let mut area2 = 0.0;
    for i in 0..n {
        let (p0, p1) = (pts[i], pts[(i + 1) % n]);
        area2 += p0[0] * p1[1] - p1[0] * p0[1];
    }
    if !area2.is_finite() {
        return None;
    }
    Some((area2 * 0.5).abs())
}
