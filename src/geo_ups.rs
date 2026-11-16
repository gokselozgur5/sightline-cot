//! Universal Polar Stereographic (UPS) projection for polar regions.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpsHemisphere {
    North,
    South,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UpsCoord {
    pub easting: f64,
    pub northing: f64,
    pub hemisphere: UpsHemisphere,
}

impl UpsCoord {
    pub fn new(easting: f64, northing: f64, hemisphere: UpsHemisphere) -> Self {
        Self { easting, northing, hemisphere }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpsError {
    InvalidLatitude,
    InvalidLongitude,
    NonFinite,
}
