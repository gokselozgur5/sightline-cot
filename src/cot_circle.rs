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
            return Err(CotCircleError::InvalidCoordinate(
                "longitude in [-180, 180]",
            ));
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
            return Err(CotCircleError::InvalidParameter(
                "stroke weight must be positive",
            ));
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
            return Err(CotCircleError::InvalidCoordinate(
                "longitude in [-180, 180]",
            ));
        }
        let dist = crate::geo_haversine::haversine_distance_m([self.lat, self.lon], [lat, lon]);
        Ok(dist <= self.radius_m)
    }
}

impl CotCircle {
    /// Generate an approximation polygon of `num_points` geodetic boundary vertices.
    pub fn to_polygon_vertices(&self, num_points: usize) -> Result<Vec<[f64; 2]>, CotCircleError> {
        if num_points < 3 {
            return Err(CotCircleError::InvalidParameter(
                "num_points must be at least 3",
            ));
        }
        let mut vertices = Vec::with_capacity(num_points);
        let step_deg = 360.0 / num_points as f64;
        for i in 0..num_points {
            let bearing = i as f64 * step_deg;
            let (p_lat, p_lon) = crate::geo_destination::destination_point(
                self.lat,
                self.lon,
                bearing,
                self.radius_m,
            );
            vertices.push([p_lat, p_lon]);
        }
        Ok(vertices)
    }
}

impl CotCircle {
    /// Render inner ATAK `<shape><ellipse .../></shape>` XML snippet.
    pub fn to_shape_xml(&self) -> String {
        format!(
            "<shape><ellipse major=\"{:.2}\" minor=\"{:.2}\" angle=\"0.0\"/></shape>",
            self.radius_m, self.radius_m
        )
    }
}

impl CotCircle {
    /// Render the complete `<detail>` block including contact, styles, and shape.
    pub fn to_detail_xml(&self) -> String {
        let mut inner = String::new();
        if let Some(ref cs) = self.callsign {
            inner.push_str(&format!(
                "<contact callsign=\"{}\"/>",
                crate::cot::escape_xml_attr(cs)
            ));
        }
        if let Some(sc) = self.stroke_color {
            inner.push_str(&format!("<strokeColor value=\"{sc}\"/>"));
        }
        if let Some(sw) = self.stroke_weight {
            inner.push_str(&format!("<strokeWeight value=\"{sw:.2}\"/>"));
        }
        if let Some(fc) = self.fill_color {
            inner.push_str(&format!("<fillColor value=\"{fc}\"/>"));
        }
        inner.push_str(&self.to_shape_xml());
        if let Some(ref rem) = self.remarks {
            inner.push_str(&format!(
                "<remarks>{}</remarks>",
                crate::cot::escape_xml_attr(rem)
            ));
        }
        format!("<detail>{inner}</detail>")
    }
}

impl CotCircle {
    /// Render complete standalone CoT 2.0 XML event for this circle.
    pub fn to_cot_event_xml(&self, uid: &str, time_ms: i64, stale_ms: i64) -> String {
        let time_str = crate::cot::format_iso8601_utc(time_ms);
        let stale_str = crate::cot::format_iso8601_utc(stale_ms);
        let uid_esc = crate::cot::escape_xml_attr(uid);
        let hae_val = self.hae.unwrap_or(0.0);
        let detail_xml = self.to_detail_xml();
        format!(
            "<event version=\"2.0\" uid=\"{uid_esc}\" type=\"u-d-c-c\" how=\"h-e\" time=\"{time_str}\" start=\"{time_str}\" stale=\"{stale_str}\">\
<point lat=\"{:.7}\" lon=\"{:.7}\" hae=\"{hae_val:.2}\" ce=\"9999999.00\" le=\"9999999.00\"/>\
{detail_xml}</event>",
            self.lat, self.lon
        )
    }
}

fn extract_attr(tag: &str, attr: &str) -> Option<String> {
    crate::cot::find_attr_in_tag(tag, attr).map(|s| s.to_string())
}

fn extract_tag_content<'a>(xml: &'a str, tag: &str) -> Option<&'a str> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let start = xml.find(&open)? + open.len();
    let end = xml[start..].find(&close)?;
    Some(&xml[start..start + end])
}

impl CotCircle {
    fn parse_point_and_radius(xml: &str) -> Result<(f64, f64, Option<f64>, f64), CotCircleError> {
        let p_start = xml
            .find("<point")
            .ok_or_else(|| CotCircleError::XmlParseError("missing <point".into()))?;
        let p_end = xml[p_start..]
            .find("/>")
            .ok_or_else(|| CotCircleError::XmlParseError("unclosed <point".into()))?;
        let tag = &xml[p_start..p_start + p_end];
        let lat = extract_attr(tag, "lat")
            .ok_or(CotCircleError::InvalidCoordinate("missing lat"))?
            .parse::<f64>()
            .map_err(|_| CotCircleError::InvalidCoordinate("non-numeric lat"))?;
        let lon = extract_attr(tag, "lon")
            .ok_or(CotCircleError::InvalidCoordinate("missing lon"))?
            .parse::<f64>()
            .map_err(|_| CotCircleError::InvalidCoordinate("non-numeric lon"))?;
        let hae = extract_attr(tag, "hae").and_then(|h| h.parse::<f64>().ok());
        let e_start = xml
            .find("<ellipse")
            .ok_or_else(|| CotCircleError::XmlParseError("missing <ellipse".into()))?;
        let e_end = xml[e_start..]
            .find("/>")
            .ok_or_else(|| CotCircleError::XmlParseError("unclosed <ellipse".into()))?;
        let el_tag = &xml[e_start..e_start + e_end];
        let major = extract_attr(el_tag, "major")
            .ok_or(CotCircleError::InvalidRadius("missing major radius"))?
            .parse::<f64>()
            .map_err(|_| CotCircleError::InvalidRadius("non-numeric major radius"))?;
        Ok((lat, lon, hae, major))
    }
}

impl CotCircle {
    /// Parse a CoT circle from an XML event.
    pub fn parse(xml: &str) -> Result<Self, CotCircleError> {
        let (lat, lon, hae, radius) = Self::parse_point_and_radius(xml)?;
        let mut circle = Self::new(lat, lon, radius)?;
        circle.hae = hae;
        if let Some(c) = xml
            .find("<contact")
            .and_then(|i| xml[i..].find("/>").map(|j| &xml[i..i + j]))
        {
            circle.callsign =
                extract_attr(c, "callsign").map(|s| crate::cot::unescape_xml_attr(&s));
        }
        if let Some(s) = xml
            .find("<strokeColor")
            .and_then(|i| xml[i..].find("/>").map(|j| &xml[i..i + j]))
        {
            circle.stroke_color = extract_attr(s, "value").and_then(|v| v.parse().ok());
        }
        if let Some(w) = xml
            .find("<strokeWeight")
            .and_then(|i| xml[i..].find("/>").map(|j| &xml[i..i + j]))
        {
            circle.stroke_weight = extract_attr(w, "value").and_then(|v| v.parse().ok());
        }
        if let Some(f) = xml
            .find("<fillColor")
            .and_then(|i| xml[i..].find("/>").map(|j| &xml[i..i + j]))
        {
            circle.fill_color = extract_attr(f, "value").and_then(|v| v.parse().ok());
        }
        circle.remarks = extract_tag_content(xml, "remarks").map(crate::cot::unescape_xml_attr);
        Ok(circle)
    }
}

/// Concentric range rings sharing a common center coordinate.
#[derive(Debug, Clone, PartialEq)]
pub struct CotRangeRings {
    pub center_lat: f64,
    pub center_lon: f64,
    pub hae: Option<f64>,
    pub radii_m: Vec<f64>,
    pub stroke_color: Option<i32>,
    pub stroke_weight: Option<f64>,
    pub callsign_prefix: Option<String>,
}

impl CotRangeRings {
    /// Create concentric range rings from explicit list of radii in meters.
    pub fn new(
        center_lat: f64,
        center_lon: f64,
        radii_m: Vec<f64>,
    ) -> Result<Self, CotCircleError> {
        if !center_lat.is_finite() || !(-90.0..=90.0).contains(&center_lat) {
            return Err(CotCircleError::InvalidCoordinate("latitude in [-90, 90]"));
        }
        if !center_lon.is_finite() || !(-180.0..=180.0).contains(&center_lon) {
            return Err(CotCircleError::InvalidCoordinate(
                "longitude in [-180, 180]",
            ));
        }
        if radii_m.is_empty() {
            return Err(CotCircleError::InvalidParameter(
                "radii_m must not be empty",
            ));
        }
        for &r in &radii_m {
            if !r.is_finite() || r <= 0.0 {
                return Err(CotCircleError::InvalidRadius(
                    "each radius must be positive",
                ));
            }
        }
        for w in radii_m.windows(2) {
            if w[0] >= w[1] {
                return Err(CotCircleError::InvalidParameter(
                    "radii_m must be strictly ascending",
                ));
            }
        }
        Ok(Self {
            center_lat,
            center_lon,
            hae: None,
            radii_m,
            stroke_color: None,
            stroke_weight: None,
            callsign_prefix: None,
        })
    }
}

impl CotRangeRings {
    /// Create uniform concentric range rings spaced by step_m.
    pub fn from_interval(
        center_lat: f64,
        center_lon: f64,
        step_m: f64,
        count: usize,
    ) -> Result<Self, CotCircleError> {
        if count == 0 {
            return Err(CotCircleError::InvalidParameter(
                "ring count must be at least 1",
            ));
        }
        if !step_m.is_finite() || step_m <= 0.0 {
            return Err(CotCircleError::InvalidRadius(
                "interval step must be positive",
            ));
        }
        let radii: Vec<f64> = (1..=count).map(|i| i as f64 * step_m).collect();
        Self::new(center_lat, center_lon, radii)
    }

    /// Set height above ellipsoid in meters.
    pub fn with_hae(mut self, hae: f64) -> Result<Self, CotCircleError> {
        if !hae.is_finite() {
            return Err(CotCircleError::InvalidParameter("hae must be finite"));
        }
        self.hae = Some(hae);
        Ok(self)
    }
}

impl CotRangeRings {
    /// Set stroke line color (signed 32-bit ARGB integer).
    pub fn with_stroke_color(mut self, color: i32) -> Self {
        self.stroke_color = Some(color);
        self
    }

    /// Set stroke line thickness in pixels.
    pub fn with_stroke_weight(mut self, weight: f64) -> Result<Self, CotCircleError> {
        if !weight.is_finite() || weight <= 0.0 {
            return Err(CotCircleError::InvalidParameter(
                "stroke weight must be positive",
            ));
        }
        self.stroke_weight = Some(weight);
        Ok(self)
    }

    /// Set callsign prefix for individual rings.
    pub fn with_callsign_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.callsign_prefix = Some(prefix.into());
        self
    }
}

impl CotRangeRings {
    /// Determine which concentric ring index (0-based) a distance falls within.
    pub fn ring_index_for_distance(&self, distance_m: f64) -> Option<usize> {
        if !distance_m.is_finite() || distance_m < 0.0 {
            return None;
        }
        self.radii_m.iter().position(|&r| distance_m <= r)
    }

    /// Convert into a list of individual CotCircle instances.
    pub fn to_circles(&self) -> Vec<CotCircle> {
        self.radii_m
            .iter()
            .enumerate()
            .map(|(i, &r)| {
                let mut c = CotCircle::new(self.center_lat, self.center_lon, r).expect("valid");
                c.hae = self.hae;
                c.stroke_color = self.stroke_color;
                c.stroke_weight = self.stroke_weight;
                if let Some(ref pfx) = self.callsign_prefix {
                    c.callsign = Some(format!("{pfx} Ring {}", i + 1));
                }
                c
            })
            .collect()
    }
}

impl CotRangeRings {
    /// Generate complete CoT 2.0 XML events for all concentric rings.
    pub fn to_cot_events(&self, base_uid: &str, time_ms: i64, stale_ms: i64) -> Vec<String> {
        self.to_circles()
            .into_iter()
            .enumerate()
            .map(|(i, circle)| {
                let ring_uid = format!("{base_uid}-ring-{}", i + 1);
                circle.to_cot_event_xml(&ring_uid, time_ms, stale_ms)
            })
            .collect()
    }
}
