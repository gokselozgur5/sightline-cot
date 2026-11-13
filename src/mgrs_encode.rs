//! MGRS 10-digit grid reference encoder.

use crate::mgrs::GridError;

/// A 10-digit MGRS grid reference with 1-meter precision.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mgrs10Digit {
    pub zone: u8,
    pub band: char,
    pub col: char,
    pub row: char,
    pub easting: u32,
    pub northing: u32,
}

impl Mgrs10Digit {
    /// Encode a WGS84 coordinate into a 10-digit MGRS reference.
    pub fn from_lat_lon(lat_deg: f64, lon_deg: f64) -> Result<Self, GridError> {
        let mgrs = crate::mgrs::format_mgrs(lat_deg, lon_deg, 5)?;
        let s = mgrs.as_str();
        let zone: u8 = s[0..2].parse().map_err(|_| GridError::InvalidZone)?;
        let mut chars = s[2..5].chars();
        let band = chars.next().ok_or(GridError::InvalidLatitude)?;
        let col = chars.next().ok_or(GridError::InvalidLongitude)?;
        let row = chars.next().ok_or(GridError::InvalidLatitude)?;
        let easting: u32 = s[5..10].parse().map_err(|_| GridError::InvalidEasting)?;
        let northing: u32 = s[10..15].parse().map_err(|_| GridError::InvalidNorthing)?;
        Ok(Self { zone, band, col, row, easting, northing })
    }
}

/// Encode a WGS84 coordinate into a 10-digit MGRS reference.
pub fn encode_mgrs_10digit(lat_deg: f64, lon_deg: f64) -> Result<Mgrs10Digit, GridError> {
    Mgrs10Digit::from_lat_lon(lat_deg, lon_deg)
}
