//! Geodetic bounding box for lat/lon coordinates.

/// 2D axis-aligned geodetic bounding box.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeoBbox {
    pub min_lat: f64,
    pub min_lon: f64,
    pub max_lat: f64,
    pub max_lon: f64,
}

impl GeoBbox {
    /// Creates a new bounding box with explicit bounds.
    pub fn new(min_lat: f64, min_lon: f64, max_lat: f64, max_lon: f64) -> Self {
        Self {
            min_lat,
            min_lon,
            max_lat,
            max_lon,
        }
    }
}
