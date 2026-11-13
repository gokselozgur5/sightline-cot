//! Compass point names for 8 and 16 point roses.

const POINTS_16: [&str; 16] = [
    "N", "NNE", "NE", "ENE", "E", "ESE", "SE", "SSE", "S", "SSW", "SW", "WSW", "W", "WNW", "NW",
    "NNW",
];

/// Name of the compass point nearest to `bearing_deg` on a rose of `points` (8 or 16).
///
/// Returns `None` for a non-finite bearing or an unsupported point count.
pub fn compass_name(bearing_deg: f64, points: u8) -> Option<&'static str> {
    if !bearing_deg.is_finite() || (points != 8 && points != 16) {
        return None;
    }
    let step = 360.0 / f64::from(points);
    let idx = (bearing_deg.rem_euclid(360.0) / step).round() as usize % usize::from(points);
    Some(POINTS_16[idx * (16 / usize::from(points))])
}
