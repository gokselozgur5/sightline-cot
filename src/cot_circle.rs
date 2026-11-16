//! CoT circle and range ring XML generator and validator.

/// Errors produced during CoT circle validation, parsing, or generation.
#[derive(Debug, Clone, PartialEq)]
pub enum CotCircleError {
    InvalidCoordinate(&'static str),
    InvalidRadius(&'static str),
    InvalidParameter(&'static str),
    XmlParseError(String),
}

impl std::fmt::Display for CotCircleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCoordinate(m) => write!(f, "invalid coordinate: {m}"),
            Self::InvalidRadius(m) => write!(f, "invalid radius: {m}"),
            Self::InvalidParameter(m) => write!(f, "invalid parameter: {m}"),
            Self::XmlParseError(m) => write!(f, "XML parse error: {m}"),
        }
    }
}

impl std::error::Error for CotCircleError {}

/// Cursor-on-Target circle or range ring shape representation.
#[derive(Debug, Clone, PartialEq)]
pub struct CotCircle {
    pub lat: f64,
    pub lon: f64,
    pub hae: Option<f64>,
    pub radius_m: f64,
    pub stroke_color: Option<i32>,
    pub fill_color: Option<i32>,
    pub stroke_weight: Option<f64>,
    pub callsign: Option<String>,
    pub remarks: Option<String>,
}

impl CotCircle {
    /// Create a new circle shape with center coordinate and radius in meters.
    pub fn new(lat: f64, lon: f64, radius_m: f64) -> Result<Self, CotCircleError> {
        if !lat.is_finite() || !(-90.0..=90.0).contains(&lat) {
            return Err(CotCircleError::InvalidCoordinate("latitude in [-90, 90]"));
        }
        if !lon.is_finite() || !(-180.0..=180.0).contains(&lon) {
            return Err(CotCircleError::InvalidCoordinate("longitude in [-180, 180]"));
        }
        if !radius_m.is_finite() || radius_m <= 0.0 {
            return Err(CotCircleError::InvalidRadius("radius must be positive"));
        }
        Ok(Self {
            lat,
            lon,
            hae: None,
            radius_m,
            stroke_color: None,
            fill_color: None,
            stroke_weight: None,
            callsign: None,
            remarks: None,
        })
    }
}

impl CotCircle {
    /// Set height above ellipsoid in meters.
    pub fn with_hae(mut self, hae: f64) -> Result<Self, CotCircleError> {
        if !hae.is_finite() {
            return Err(CotCircleError::InvalidParameter("hae must be finite"));
        }
        self.hae = Some(hae);
        Ok(self)
    }

    /// Set line/stroke color (signed 32-bit ARGB integer).
    pub fn with_stroke_color(mut self, color: i32) -> Self {
        self.stroke_color = Some(color);
        self
    }

    /// Set interior fill color (signed 32-bit ARGB integer).
    pub fn with_fill_color(mut self, color: i32) -> Self {
        self.fill_color = Some(color);
        self
    }
}

impl CotCircle {
    /// Set stroke line thickness in pixels.
    pub fn with_stroke_weight(mut self, weight: f64) -> Result<Self, CotCircleError> {
        if !weight.is_finite() || weight <= 0.0 {
            return Err(CotCircleError::InvalidParameter("stroke weight must be positive"));
        }
        self.stroke_weight = Some(weight);
        Ok(self)
    }

    /// Set contact callsign for map label rendering.
    pub fn with_callsign(mut self, callsign: impl Into<String>) -> Self {
        self.callsign = Some(callsign.into());
        self
    }

    /// Set optional remarks description.
    pub fn with_remarks(mut self, remarks: impl Into<String>) -> Self {
        self.remarks = Some(remarks.into());
        self
    }
}

impl CotCircle {
    /// Compute planar circle area in square meters.
    pub fn area_m2(&self) -> f64 {
        std::f64::consts::PI * self.radius_m * self.radius_m
    }

    /// Compute circle circumference in meters.
    pub fn circumference_m(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius_m
    }
}

impl CotCircle {
    /// Test whether a geodetic point (lat, lon) falls within this circle.
    pub fn contains_point(&self, lat: f64, lon: f64) -> Result<bool, CotCircleError> {
        if !lat.is_finite() || !(-90.0..=90.0).contains(&lat) {
            return Err(CotCircleError::InvalidCoordinate("latitude in [-90, 90]"));
        }
        if !lon.is_finite() || !(-180.0..=180.0).contains(&lon) {
            return Err(CotCircleError::InvalidCoordinate("longitude in [-180, 180]"));
        }
        let dist = crate::geo_haversine::haversine_distance_m([self.lat, self.lon], [lat, lon]);
        Ok(dist <= self.radius_m)
    }
}

impl CotCircle {
    /// Generate an approximation polygon of `num_points` geodetic boundary vertices.
    pub fn to_polygon_vertices(&self, num_points: usize) -> Result<Vec<[f64; 2]>, CotCircleError> {
        if num_points < 3 {
            return Err(CotCircleError::InvalidParameter("num_points must be at least 3"));
        }
        let mut vertices = Vec::with_capacity(num_points);
        let step_deg = 360.0 / num_points as f64;
        for i in 0..num_points {
            let bearing = i as f64 * step_deg;
            let (p_lat, p_lon) = crate::geo_destination::destination_point(
                self.lat, self.lon, bearing, self.radius_m,
            );
            vertices.push([p_lat, p_lon]);
        }
        Ok(vertices)
    }
}
