//! NATO Military Grid Reference System (MGRS) & UTM conversions.

/// Error conditions in UTM and MGRS transformations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridError {
    InvalidLatitude,
    InvalidLongitude,
    InvalidZone,
    InvalidNorthing,
    InvalidEasting,
    BufferTooSmall,
}

/// UTM coordinate representation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UtmCoord {
    pub zone: u8,
    pub hemisphere: char,
    pub easting: f64,
    pub northing: f64,
}

const WGS84_A: f64 = 6378137.0;
