//! Geodesic cross-track and along-track error to a great circle.

pub const MEAN_EARTH_RADIUS_M: f64 = 6_371_008.8;

/// Cross-track and along-track distances in metres.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CrossTrackResult {
    pub cross_track_m: f64,
    pub along_track_m: f64,
}
