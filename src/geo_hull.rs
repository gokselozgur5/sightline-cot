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
}
