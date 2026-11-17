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
            max_lat,
            max_lon,
        }
    }

    /// Creates an envelope enclosing a single geodetic point.
    pub fn from_point(lat: f64, lon: f64) -> Self {
        let norm_lon = normalize_lon(lon);
        Self::new(lat, norm_lon, lat, norm_lon)
    }

    /// Computes the bounding envelope from a slice of polygon vertices [lat, lon].
    pub fn from_geodesic_polygon(coords: &[[f64; 2]]) -> Option<Self> {
        let first = coords.first()?;
        let mut min_lat = first[0];
        let mut max_lat = first[0];
        let mut min_lon = normalize_lon(first[1]);
        let mut max_lon = min_lon;

        for pt in &coords[1..] {
            min_lat = min_lat.min(pt[0]);
            max_lat = max_lat.max(pt[0]);
            let n_lon = normalize_lon(pt[1]);
            min_lon = min_lon.min(n_lon);
            max_lon = max_lon.max(n_lon);
        }

        Some(Self::new(min_lat, min_lon, max_lat, max_lon))
    }

    /// Returns true if this envelope contains the given point.
    pub fn contains_point(&self, lat: f64, lon: f64) -> bool {
        let n_lon = normalize_lon(lon);
        lat >= self.min_lat && lat <= self.max_lat && n_lon >= self.min_lon && n_lon <= self.max_lon
    }

    /// Returns true if this envelope fully encloses another envelope.
    pub fn contains_envelope(&self, other: &Self) -> bool {
        self.min_lat <= other.min_lat
            && self.max_lat >= other.max_lat
            && self.min_lon <= other.min_lon
            && self.max_lon >= other.max_lon
    }

    /// Returns true if this envelope intersects with another envelope.
    pub fn intersects(&self, other: &Self) -> bool {
        self.min_lat <= other.max_lat
            && self.max_lat >= other.min_lat
            && self.min_lon <= other.max_lon
            && self.max_lon >= other.min_lon
    }

    /// Computes the union envelope of this and another envelope.
    pub fn merge(&self, other: &Self) -> Self {
        Self::new(
            self.min_lat.min(other.min_lat),
            self.min_lon.min(other.min_lon),
            self.max_lat.max(other.max_lat),
            self.max_lon.max(other.max_lon),
        )
    }

    /// Computes the area metric of the bounding box in degrees squared.
    pub fn area(&self) -> f64 {
        let d_lat = (self.max_lat - self.min_lat).max(0.0);
        let d_lon = (self.max_lon - self.min_lon).max(0.0);
