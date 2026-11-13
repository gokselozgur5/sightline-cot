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

/// Planar perimeter of a 2D polygon in linear units.
pub fn planar_polygon_perimeter(pts: &[[f64; 2]]) -> Option<f64> {
    let n = pts.len();
    if n < 3 || pts.iter().flatten().any(|c| !c.is_finite()) {
        return None;
    }
    let mut perim = 0.0;
    for i in 0..n {
        let (p0, p1) = (pts[i], pts[(i + 1) % n]);
        let (dx, dy) = (p1[0] - p0[0], p1[1] - p0[1]);
        perim += (dx * dx + dy * dy).sqrt();
    }
    if !perim.is_finite() {
        return None;
    }
    Some(perim)
}

fn project_poly_enu(poly: &[[f64; 2]]) -> Option<Vec<[f64; 2]>> {
    let n = poly.len();
    if n < 3 || poly.iter().flatten().any(|c| !c.is_finite()) {
        return None;
    }
    let lat0 = poly.iter().map(|p| p[0]).sum::<f64>() / n as f64;
    let lon0 = poly.iter().map(|p| p[1]).sum::<f64>() / n as f64;
    let cos_lat = lat0.to_radians().cos().max(1e-12);
    Some(
        poly.iter()
            .map(|p| {
                let mut dlon = p[1] - lon0;
                while dlon > 180.0 {
                    dlon -= 360.0;
                }
                while dlon < -180.0 {
                    dlon += 360.0;
                }
                [
                    dlon * cos_lat * METERS_PER_DEG,
                    (p[0] - lat0) * METERS_PER_DEG,
                ]
            })
            .collect(),
    )
}

/// Compute the geodetic polygon area in square meters.
pub fn polygon_area(poly: &[[f64; 2]]) -> Option<f64> {
    let pts = project_poly_enu(poly)?;
    planar_polygon_area(&pts)
}

/// Compute the geodetic polygon perimeter in meters.
pub fn polygon_perimeter(poly: &[[f64; 2]]) -> Option<f64> {
    let pts = project_poly_enu(poly)?;
    planar_polygon_perimeter(&pts)
}
