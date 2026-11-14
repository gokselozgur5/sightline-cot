//! Geodetic polyline self-intersection detector.

/// Represents an intersection between two polyline segments.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SegmentIntersection {
    pub segment_a: usize,
    pub segment_b: usize,
    pub point: [f64; 2],
}

fn unwrap_lon(lon: f64, reference: f64) -> f64 {
    reference + ((lon - reference + 540.0).rem_euclid(360.0) - 180.0)
}

/// Computes the intersection point between two line segments if they intersect.
pub fn segments_intersect(
    p1: [f64; 2],
    p2: [f64; 2],
    p3: [f64; 2],
    p4: [f64; 2],
) -> Option<[f64; 2]> {
    let (lat1, lon1) = (p1[0], p1[1]);
    let (lat2, lon2) = (p2[0], unwrap_lon(p2[1], lon1));
    let (lat3, lon3) = (p3[0], unwrap_lon(p3[1], lon1));
    let (lat4, lon4) = (p4[0], unwrap_lon(p4[1], lon1));
    let (dx12, dy12) = (lon2 - lon1, lat2 - lat1);
    let (dx34, dy34) = (lon4 - lon3, lat4 - lat3);
    let denom = dx12 * dy34 - dy12 * dx34;
    if denom.abs() < 1e-12 {
        return None;
    }
    let (dx31, dy31) = (lon3 - lon1, lat3 - lat1);
    let t = (dx31 * dy34 - dy31 * dx34) / denom;
    let u = (dx31 * dy12 - dy31 * dx12) / denom;
    if (0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u) {
        let lat = lat1 + t * dy12;
        let lon = lon1 + t * dx12;
        Some([lat, ((lon + 180.0).rem_euclid(360.0) - 180.0)])
    } else {
        None
    }
}
