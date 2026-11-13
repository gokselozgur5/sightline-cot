//! 2D convex hull of geodetic points in a local tangent frame.

/// Point in a local tangent projection with its original index.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct LocalPoint {
    pub x: f64,
    pub y: f64,
    pub idx: usize,
}

impl LocalPoint {
    /// 2D cross product of vectors OA and OB.
    pub fn cross(o: &Self, a: &Self, b: &Self) -> f64 {
        (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x)
    }

    /// Projects geodetic points into a local tangent plane.
    pub fn project_all(points: &[[f64; 2]]) -> Vec<Self> {
        if points.is_empty() {
            return Vec::new();
        }
        let (lat0, lon0) = (points[0][0], points[0][1]);
        let cos0 = lat0.to_radians().cos().max(1e-6);
        points
            .iter()
            .enumerate()
            .map(|(idx, p)| {
                let mut dlon = p[1] - lon0;
                if dlon > 180.0 {
                    dlon -= 360.0;
                } else if dlon < -180.0 {
                    dlon += 360.0;
                }
                let x = dlon.to_radians() * 6_378_137.0 * cos0;
                let y = (p[0] - lat0).to_radians() * 6_378_137.0;
                Self { x, y, idx }
            })
            .collect()
    }
}

fn scan_half<'a>(iter: impl Iterator<Item = &'a LocalPoint>) -> Vec<LocalPoint> {
    let mut h: Vec<LocalPoint> = Vec::new();
    for p in iter {
        while h.len() >= 2 && LocalPoint::cross(&h[h.len() - 2], &h[h.len() - 1], p) <= 1e-9 {
            h.pop();
        }
        h.push(*p);
    }
    h
}

/// Computes the 2D convex hull of geodetic points in a local tangent frame.
pub fn convex_hull(points: &[[f64; 2]]) -> Vec<[f64; 2]> {
    if points.len() <= 2 {
        return points.to_vec();
    }
    let mut pts = LocalPoint::project_all(points);
    pts.sort_by(|a, b| a.x.total_cmp(&b.x).then_with(|| a.y.total_cmp(&b.y)));
    pts.dedup_by(|a, b| (a.x - b.x).hypot(a.y - b.y) < 1e-6);
    if pts.len() <= 2 {
        return pts.iter().map(|p| points[p.idx]).collect();
    }
    let (mut lower, mut upper) = (scan_half(pts.iter()), scan_half(pts.iter().rev()));
    lower.pop();
    upper.pop();
    lower.extend(upper);
    lower.iter().map(|p| points[p.idx]).collect()
}
