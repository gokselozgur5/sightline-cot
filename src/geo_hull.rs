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
