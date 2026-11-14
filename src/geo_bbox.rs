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

    /// Computes the tight bounding box enclosing a slice of points.
    pub fn from_points(points: &[[f64; 2]]) -> Option<Self> {
        let first = points.first()?;
        let (mut min_lat, mut max_lat) = (first[0], first[0]);
        let (mut min_lon, mut max_lon) = (first[1], first[1]);
        for p in &points[1..] {
            min_lat = min_lat.min(p[0]);
            max_lat = max_lat.max(p[0]);
            min_lon = min_lon.min(p[1]);
            max_lon = max_lon.max(p[1]);
        }
        Some(Self::new(min_lat, min_lon, max_lat, max_lon))
    }

    /// Checks if a geodetic point lies inside or on the boundary of the box.
    pub fn contains(&self, lat: f64, lon: f64) -> bool {
        lat >= self.min_lat && lat <= self.max_lat && lon >= self.min_lon && lon <= self.max_lon
    }

    /// Computes the center coordinates of the bounding box.
    pub fn center(&self) -> [f64; 2] {
        [
            (self.min_lat + self.max_lat) * 0.5,
            (self.min_lon + self.max_lon) * 0.5,
        ]
    }

    /// Returns the four corners in counter-clockwise order starting from southwest.
    pub fn corners(&self) -> [[f64; 2]; 4] {
        [
            [self.min_lat, self.min_lon],
            [self.min_lat, self.max_lon],
            [self.max_lat, self.max_lon],
            [self.max_lat, self.min_lon],
        ]
    }

    /// Checks if this bounding box intersects another bounding box.
    pub fn intersects(&self, other: &Self) -> bool {
        self.min_lat <= other.max_lat
            && self.max_lat >= other.min_lat
            && self.min_lon <= other.max_lon
            && self.max_lon >= other.min_lon
    }

    /// Expands this bounding box to include a given point.
    pub fn expand(&mut self, lat: f64, lon: f64) {
        self.min_lat = self.min_lat.min(lat);
        self.max_lat = self.max_lat.max(lat);
        self.min_lon = self.min_lon.min(lon);
        self.max_lon = self.max_lon.max(lon);
    }
}
