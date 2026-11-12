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
const WGS84_F: f64 = 1.0 / 298.257223563;
const UTM_K0: f64 = 0.9996;
const FALSE_EASTING: f64 = 500000.0;
const FALSE_NORTHING_SOUTH: f64 = 10000000.0;

/// Determine standard UTM zone from longitude in degrees.
pub fn lon_to_utm_zone(lon_deg: f64) -> Result<u8, GridError> {
    if !(-180.0..=180.0).contains(&lon_deg) {
        return Err(GridError::InvalidLongitude);
    }
    let zone = ((lon_deg + 180.0) / 6.0).floor() as u8 + 1;
    Ok(zone.clamp(1, 60))
}

/// Forward UTM projection from WGS84 lat/lon.
pub fn wgs84_to_utm(lat_deg: f64, lon_deg: f64) -> Result<UtmCoord, GridError> {
    if !(-80.0..=84.0).contains(&lat_deg) {
        return Err(GridError::InvalidLatitude);
    }
    let zone = lon_to_utm_zone(lon_deg)?;
    let hemisphere = if lat_deg >= 0.0 { 'N' } else { 'S' };
    let (east, north) = utm_project_point(lat_deg, lon_deg, zone, hemisphere);
    Ok(UtmCoord {
