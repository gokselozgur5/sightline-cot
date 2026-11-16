//! CoT shape polyline and polygon XML builder and parser.

#[derive(Debug, Clone, PartialEq)]
pub enum CotShapeError {
    InvalidCoordinate(&'static str),
    InsufficientVertices { expected: usize, found: usize },
    InvalidParameter(&'static str),
    XmlParseError(String),
}

impl std::fmt::Display for CotShapeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCoordinate(m) => write!(f, "invalid coordinate: {m}"),
            Self::InsufficientVertices { expected, found } => {
                write!(f, "insufficient vertices: expected {expected}, got {found}")
            }
            Self::InvalidParameter(m) => write!(f, "invalid parameter: {m}"),
            Self::XmlParseError(m) => write!(f, "XML parse error: {m}"),
        }
    }
}

impl std::error::Error for CotShapeError {}

/// Geographic vertex with optional height above ellipsoid (HAE).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CotVertex {
    pub lat: f64,
    pub lon: f64,
    pub hae: Option<f64>,
}

impl CotVertex {
    /// Create a 2D vertex validating latitude and longitude ranges.
    pub fn new(lat: f64, lon: f64) -> Result<Self, CotShapeError> {
        if !lat.is_finite() || !(-90.0..=90.0).contains(&lat) {
            return Err(CotShapeError::InvalidCoordinate("latitude in [-90, 90]"));
        }
        if !lon.is_finite() || !(-180.0..=180.0).contains(&lon) {
            return Err(CotShapeError::InvalidCoordinate("longitude in [-180, 180]"));
        }
        Ok(Self { lat, lon, hae: None })
    }
}

impl CotVertex {
    /// Create a 3D vertex with height above ellipsoid in meters.
    pub fn with_hae(lat: f64, lon: f64, hae: f64) -> Result<Self, CotShapeError> {
        let mut v = Self::new(lat, lon)?;
        if !hae.is_finite() {
            return Err(CotShapeError::InvalidParameter("hae must be finite"));
        }
        v.hae = Some(hae);
        Ok(v)
    }

    /// Format vertex as XML `<vertex .../>` tag.
    pub fn to_xml(&self) -> String {
        match self.hae {
            Some(h) => format!("<vertex lat=\"{:.6}\" lon=\"{:.6}\" hae=\"{:.2}\"/>", self.lat, self.lon, h),
            None => format!("<vertex lat=\"{:.6}\" lon=\"{:.6}\"/>", self.lat, self.lon),
        }
    }
}

/// Geometric shape category in CoT detail.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CotShapeType {
    Polyline,
    Polygon,
}

/// CoT shape representation for polylines and polygons.
#[derive(Debug, Clone, PartialEq)]
pub struct CotShape {
    pub shape_type: CotShapeType,
    pub color: Option<i32>,
    pub fill_color: Option<i32>,
    pub stroke_weight: Option<f64>,
    pub vertices: Vec<CotVertex>,
}

impl CotShape {
    /// Create a new open polyline shape (minimum 2 vertices).
    pub fn new_polyline(vertices: Vec<CotVertex>) -> Result<Self, CotShapeError> {
        if vertices.len() < 2 {
            return Err(CotShapeError::InsufficientVertices { expected: 2, found: vertices.len() });
        }
        Ok(Self { shape_type: CotShapeType::Polyline, color: None, fill_color: None, stroke_weight: None, vertices })
    }

    /// Create a new closed polygon shape (minimum 3 vertices).
    pub fn new_polygon(vertices: Vec<CotVertex>) -> Result<Self, CotShapeError> {
        if vertices.len() < 3 {
            return Err(CotShapeError::InsufficientVertices { expected: 3, found: vertices.len() });
        }
        Ok(Self { shape_type: CotShapeType::Polygon, color: None, fill_color: None, stroke_weight: None, vertices })
    }
}

impl CotShape {
    /// Set line/stroke color (ARGB signed 32-bit integer).
    pub fn with_color(mut self, color: i32) -> Self {
        self.color = Some(color);
        self
    }

    /// Set interior fill color (ARGB signed 32-bit integer).
    pub fn with_fill_color(mut self, color: i32) -> Self {
        self.fill_color = Some(color);
        self
    }

    /// Set stroke line thickness in pixels.
    pub fn with_stroke_weight(mut self, weight: f64) -> Result<Self, CotShapeError> {
        if !weight.is_finite() || weight <= 0.0 {
            return Err(CotShapeError::InvalidParameter("stroke weight must be positive"));
        }
        self.stroke_weight = Some(weight);
        Ok(self)
    }

    /// Check if shape is a closed polygon.
    pub fn is_closed(&self) -> bool {
        self.shape_type == CotShapeType::Polygon
    }
}
