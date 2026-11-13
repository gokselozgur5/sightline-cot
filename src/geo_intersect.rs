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
