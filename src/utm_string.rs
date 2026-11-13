//! UTM coordinate string formatting and parsing.

/// Error returned when parsing a UTM coordinate string fails.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UtmStringError {
    InvalidFormat,
    InvalidZone,
    InvalidBand,
    InvalidEasting,
    InvalidNorthing,
}

/// A UTM coordinate with zone, latitude band letter, easting and northing.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UtmCoordinate {
    pub zone: u8,
    pub band: char,
    pub easting: f64,
    pub northing: f64,
}
