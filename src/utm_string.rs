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

impl UtmCoordinate {
    /// Create and validate a new UTM coordinate.
    pub fn new(zone: u8, band: char, easting: f64, northing: f64) -> Result<Self, UtmStringError> {
        if !(1..=60).contains(&zone) {
            return Err(UtmStringError::InvalidZone);
        }
        let band = band.to_ascii_uppercase();
        if !b"CDEFGHJKLMNPQRSTUVWX".contains(&(band as u8)) {
            return Err(UtmStringError::InvalidBand);
        }
        if !easting.is_finite() || easting < 0.0 {
            return Err(UtmStringError::InvalidEasting);
        }
        if !northing.is_finite() || northing < 0.0 {
            return Err(UtmStringError::InvalidNorthing);
        }
        Ok(Self { zone, band, easting, northing })
    }
}
