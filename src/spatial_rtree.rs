//! Geodesic polygon bounding box and spatial indexing R-Tree helper.

/// Earth's mean radius in meters according to WGS-84 spherical approximation.
pub const EARTH_RADIUS_M: f64 = 6_371_008.8;

/// Normalizes a longitude coordinate into the standard [-180.0, 180.0) range.
pub fn normalize_lon(mut lon: f64) -> f64 {
    while lon >= 180.0 {
        lon -= 360.0;
    }
    while lon < -180.0 {
        lon += 360.0;
    }
    lon
}

/// Great-circle distance between two geodetic coordinates in meters.
pub fn haversine_distance_m(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let d_lat = (lat2 - lat1).to_radians();
    let d_lon = (normalize_lon(lon2) - normalize_lon(lon1)).to_radians();
    let lat1_r = lat1.to_radians();
    let lat2_r = lat2.to_radians();

    let a = (d_lat * 0.5).sin().powi(2)
        + lat1_r.cos() * lat2_r.cos() * (d_lon * 0.5).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    EARTH_RADIUS_M * c
}

/// 2D axis-aligned geodetic spatial bounding box.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpatialEnvelope {
    pub min_lat: f64,
    pub min_lon: f64,
    pub max_lat: f64,
    pub max_lon: f64,
}

impl SpatialEnvelope {
    /// Creates a new geodetic envelope with explicit boundaries.
    pub fn new(min_lat: f64, min_lon: f64, max_lat: f64, max_lon: f64) -> Self {
        Self {
            min_lat,
            min_lon,
