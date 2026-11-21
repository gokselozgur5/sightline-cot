//! 2D convex hull of geodetic points in a local tangent frame.

/// Earth semi-major axis in meters (WGS84).
const EARTH_RADIUS_M: f64 = 6_378_137.0;

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
        let finite_points: Vec<(usize, &[f64; 2])> = points
            .iter()
            .enumerate()
            .filter(|(_, p)| p[0].is_finite() && p[1].is_finite())
            .collect();

        if finite_points.is_empty() {
            return Vec::new();
        }

        let (lat0, lon0) = (finite_points[0].1[0], finite_points[0].1[1]);
        let cos0 = lat0.to_radians().cos().max(1e-6);
        finite_points
            .into_iter()
            .map(|(idx, p)| {
                let mut dlon = p[1] - lon0;
                if dlon > 180.0 {
                    dlon -= 360.0;
                } else if dlon < -180.0 {
                    dlon += 360.0;
                }
                let x = dlon.to_radians() * EARTH_RADIUS_M * cos0;
                let y = (p[0] - lat0).to_radians() * EARTH_RADIUS_M;
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
///
/// Non-finite coordinate pairs are rejected and omitted from the resulting hull.
pub fn convex_hull(points: &[[f64; 2]]) -> Vec<[f64; 2]> {
    let finite_points: Vec<[f64; 2]> = points
        .iter()
        .filter(|p| p[0].is_finite() && p[1].is_finite())
        .copied()
        .collect();

    if finite_points.len() <= 2 {
        return finite_points;
    }
    let mut pts = LocalPoint::project_all(&finite_points);
    pts.sort_by(|a, b| a.x.total_cmp(&b.x).then_with(|| a.y.total_cmp(&b.y)));
    pts.dedup_by(|a, b| (a.x - b.x).hypot(a.y - b.y) < 1e-6);
    if pts.len() <= 2 {
        return pts.iter().map(|p| finite_points[p.idx]).collect();
    }
    let (mut lower, mut upper) = (scan_half(pts.iter()), scan_half(pts.iter().rev()));
    lower.pop();
    upper.pop();
    lower.extend(upper);
    lower.iter().map(|p| finite_points[p.idx]).collect()
}
