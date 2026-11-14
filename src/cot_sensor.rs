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
