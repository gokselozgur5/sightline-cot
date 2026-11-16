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

impl CotShape {
    /// Format shape as `<shape><polyline ...>...</polyline></shape>` XML snippet.
    pub fn to_xml(&self) -> String {
        let mut attrs = format!("closed=\"{}\"", if self.is_closed() { "true" } else { "false" });
        if let Some(c) = self.color {
            attrs.push_str(&format!(" color=\"{c}\""));
        }
        if let Some(fc) = self.fill_color {
            attrs.push_str(&format!(" fillColor=\"{fc}\""));
        }
        if let Some(w) = self.stroke_weight {
            attrs.push_str(&format!(" strokeWeight=\"{w:.2}\""));
        }
        let inner: String = self.vertices.iter().map(|v| v.to_xml()).collect();
        format!("<shape><polyline {attrs}>{inner}</polyline></shape>")
    }

    /// Calculate centroid of the shape vertices as [lat, lon].
    pub fn centroid(&self) -> [f64; 2] {
        let n = self.vertices.len() as f64;
        let lat = self.vertices.iter().map(|v| v.lat).sum::<f64>() / n;
        let lon = self.vertices.iter().map(|v| v.lon).sum::<f64>() / n;
        [lat, lon]
    }
}

impl CotShape {
    /// Render complete standalone CoT 2.0 XML event containing this shape.
    pub fn to_cot_event_xml(&self, uid: &str, callsign: &str, time_ms: i64, stale_ms: i64) -> String {
        let [c_lat, c_lon] = self.centroid();
        let time_str = crate::cot::format_iso8601_utc(time_ms);
        let stale_str = crate::cot::format_iso8601_utc(stale_ms);
        let shape_xml = self.to_xml();
        let cs_esc = crate::cot::escape_xml_attr(callsign);
        let uid_esc = crate::cot::escape_xml_attr(uid);
        format!(
            "<event version=\"2.0\" uid=\"{uid_esc}\" type=\"u-d-f\" how=\"h-e\" time=\"{time_str}\" start=\"{time_str}\" stale=\"{stale_str}\">\
<point lat=\"{c_lat:.7}\" lon=\"{c_lon:.7}\" hae=\"0.00\" ce=\"9999999.00\" le=\"9999999.00\"/>\
<detail><contact callsign=\"{cs_esc}\"/>{shape_xml}</detail></event>"
        )
    }
}

fn extract_attr(tag: &str, attr: &str) -> Option<String> {
    let needle = format!("{attr}=\"");
    let start = tag.find(&needle)? + needle.len();
    let end = tag[start..].find('"')?;
    Some(tag[start..start + end].to_string())
}

impl CotVertex {
    /// Parse `<vertex lat="..." lon="..." [hae="..."]/>` XML element.
    pub fn parse(xml: &str) -> Result<Self, CotShapeError> {
        let lat_str = extract_attr(xml, "lat").ok_or(CotShapeError::InvalidCoordinate("missing lat"))?;
        let lon_str = extract_attr(xml, "lon").ok_or(CotShapeError::InvalidCoordinate("missing lon"))?;
        let lat = lat_str.parse::<f64>().map_err(|_| CotShapeError::InvalidCoordinate("non-numeric lat"))?;
        let lon = lon_str.parse::<f64>().map_err(|_| CotShapeError::InvalidCoordinate("non-numeric lon"))?;
        let hae = extract_attr(xml, "hae").and_then(|h| h.parse::<f64>().ok());
        let mut v = Self::new(lat, lon)?;
        v.hae = hae;
        Ok(v)
    }
}

impl CotShape {
    /// Parse a `<shape>...</shape>` or `<polyline ...>...</polyline>` XML snippet.
    pub fn parse(xml: &str) -> Result<Self, CotShapeError> {
        let p_start = xml.find("<polyline").ok_or_else(|| CotShapeError::XmlParseError("missing <polyline".into()))?;
        let header_end = xml[p_start..].find('>').ok_or_else(|| CotShapeError::XmlParseError("unclosed <polyline".into()))?;
        let header = &xml[p_start..p_start + header_end];
        let closed = extract_attr(header, "closed").map(|s| s == "true").unwrap_or(false);
        let color = extract_attr(header, "color").and_then(|s| s.parse::<i32>().ok());
        let fill_color = extract_attr(header, "fillColor").and_then(|s| s.parse::<i32>().ok());
        let stroke_weight = extract_attr(header, "strokeWeight").and_then(|s| s.parse::<f64>().ok());

        let mut vertices = Vec::new();
        let mut rest = &xml[p_start + header_end..];
        while let Some(v_start) = rest.find("<vertex") {
            let v_end = rest[v_start..].find("/>").ok_or_else(|| CotShapeError::XmlParseError("unclosed <vertex".into()))?;
            let v_xml = &rest[v_start..v_start + v_end + 2];
            vertices.push(CotVertex::parse(v_xml)?);
            rest = &rest[v_start + v_end + 2..];
        }

        let mut shape = if closed { Self::new_polygon(vertices)? } else { Self::new_polyline(vertices)? };
        shape.color = color;
        shape.fill_color = fill_color;
        shape.stroke_weight = stroke_weight;
        Ok(shape)
    }
}
