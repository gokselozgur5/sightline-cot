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

fn to_unit_cartesian(lat: f64, lon: f64) -> [f64; 3] {
    let (phi, lam) = (lat.to_radians(), lon.to_radians());
    [phi.cos() * lam.cos(), phi.cos() * lam.sin(), phi.sin()]
}

fn cross_prod(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn dot_prod(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// Compute spherical excess in steradians using Girard's theorem.
pub fn spherical_polygon_excess(poly: &[[f64; 2]]) -> Option<f64> {
    let n = poly.len();
    if n < 3 || poly.iter().flatten().any(|c| !c.is_finite()) {
        return None;
    }
    let vs: Vec<[f64; 3]> = poly.iter().map(|p| to_unit_cartesian(p[0], p[1])).collect();
    let mut normals = Vec::with_capacity(n);
    for i in 0..n {
        let c = cross_prod(vs[i], vs[(i + 1) % n]);
        let norm = dot_prod(c, c).sqrt();
        if norm < 1e-15 {
            return None;
        }
        normals.push([c[0] / norm, c[1] / norm, c[2] / norm]);
    }
    let mut sum_alpha = 0.0;
    for i in 0..n {
        let (np, nc, v) = (normals[(i + n - 1) % n], normals[i], vs[i]);
        sum_alpha += dot_prod(cross_prod(np, nc), v).atan2(dot_prod(np, nc));
    }
    Some((2.0 * std::f64::consts::PI - sum_alpha.abs()).abs())
}

/// Mean spherical Earth radius in meters (IUGG standard).
pub const EARTH_RADIUS_M: f64 = 6_371_008.8;

/// Compute spherical polygon surface area with a custom sphere radius.
pub fn spherical_polygon_area_with_radius(poly: &[[f64; 2]], radius_m: f64) -> Option<f64> {
    if !radius_m.is_finite() || radius_m <= 0.0 {
        return None;
    }
    let excess = spherical_polygon_excess(poly)?;
    Some(excess * radius_m * radius_m)
}

/// Compute spherical polygon surface area in square meters.
pub fn spherical_polygon_area(poly: &[[f64; 2]]) -> Option<f64> {
    spherical_polygon_area_with_radius(poly, EARTH_RADIUS_M)
}

/// Compute spherical polygon perimeter with a custom sphere radius.
pub fn spherical_polygon_perimeter_with_radius(poly: &[[f64; 2]], radius_m: f64) -> Option<f64> {
    if !radius_m.is_finite() || radius_m <= 0.0 || poly.len() < 3 {
        return None;
    }
    if poly.iter().flatten().any(|c| !c.is_finite()) {
        return None;
    }
    let vs: Vec<[f64; 3]> = poly.iter().map(|p| to_unit_cartesian(p[0], p[1])).collect();
    let mut total_rad = 0.0;
    for i in 0..poly.len() {
        let (v1, v2) = (vs[i], vs[(i + 1) % poly.len()]);
        let c = cross_prod(v1, v2);
        let sin_s = dot_prod(c, c).sqrt();
        let cos_s = dot_prod(v1, v2);
        total_rad += sin_s.atan2(cos_s);
    }
    Some(total_rad * radius_m)
}

/// Compute spherical polygon perimeter in meters along great-circle arcs.
pub fn spherical_polygon_perimeter(poly: &[[f64; 2]]) -> Option<f64> {
    spherical_polygon_perimeter_with_radius(poly, EARTH_RADIUS_M)
}
