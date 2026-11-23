//! Sensor field-of-view CoT detail element.

use crate::cot::CotError;

/// Sensor FOV detail element in CoT 2.0 XML.
#[derive(Debug, Clone, PartialEq)]
pub struct CotSensor {
    pub azimuth_deg: f64,
    pub fov_deg: f64,
    pub range_m: f64,
    pub elevation_deg: f64,
}

impl CotSensor {
    /// Create a new CotSensor after validating FOV and parameters.
    pub fn new(az: f64, fov: f64, rng: f64, el: f64) -> Result<Self, CotError> {
        if !az.is_finite() || !fov.is_finite() || !rng.is_finite() || !el.is_finite() {
            return Err(CotError::InvalidNumber("sensor parameter"));
        }
        if !(0.0..=360.0).contains(&fov) {
            return Err(CotError::CoordinateOutOfBounds("fov in [0.0, 360.0]"));
        }
        if !(0.0..=360.0).contains(&az) {
            return Err(CotError::CoordinateOutOfBounds("azimuth in [0.0, 360.0]"));
        }
        if !(-90.0..=90.0).contains(&el) {
            return Err(CotError::CoordinateOutOfBounds(
                "elevation in [-90.0, 90.0]",
            ));
        }
        if rng < 0.0 {
            return Err(CotError::CoordinateOutOfBounds("range >= 0.0"));
        }
        Ok(Self {
            azimuth_deg: az,
            fov_deg: fov,
            range_m: rng,
            elevation_deg: el,
        })
    }

    /// Render CotSensor as an XML `<sensor>` element string.
    pub fn to_xml(&self) -> String {
        format!(
            "<sensor azimuth=\"{:.2}\" fov=\"{:.2}\" range=\"{:.2}\" elevation=\"{:.2}\"/>",
            self.azimuth_deg, self.fov_deg, self.range_m, self.elevation_deg
        )
    }

    /// Parse a `<sensor .../>` element string into a CotSensor.
    pub fn parse(xml: &str) -> Result<Self, CotError> {
        let tag = extract_sensor_tag(xml)?;
        let az = parse_attr(tag, "azimuth")?;
        let fov = parse_attr(tag, "fov")?;
        let rng = parse_attr(tag, "range")?;
        let el = parse_attr(tag, "elevation")?;
        Self::new(az, fov, rng, el)
    }
}

fn extract_sensor_tag(xml: &str) -> Result<&str, CotError> {
    let start = xml
        .find("<sensor")
        .ok_or_else(|| CotError::InvalidXml("missing <sensor tag".into()))?;
    let end = xml[start..]
        .find('>')
        .ok_or_else(|| CotError::InvalidXml("unclosed <sensor tag".into()))?;
    Ok(&xml[start..=start + end])
}

fn parse_attr(tag: &str, attr: &'static str) -> Result<f64, CotError> {
    let val = crate::cot::find_attr_in_tag(tag, attr).ok_or(CotError::MissingAttribute(attr))?;
    val.parse::<f64>()
        .map_err(|_| CotError::InvalidNumber(attr))
}
