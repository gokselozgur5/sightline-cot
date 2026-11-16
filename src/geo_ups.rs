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

const WGS84_A: f64 = 6378137.0;
const WGS84_F: f64 = 1.0 / 298.257223563;
const UPS_K0: f64 = 0.994;
const FALSE_EASTING_NORTH: f64 = 2000000.0;
const FALSE_NORTHING_NORTH: f64 = 2000000.0;
const FALSE_EASTING_SOUTH: f64 = 2000000.0;
const FALSE_NORTHING_SOUTH: f64 = 2000000.0;
