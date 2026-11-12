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
    Ok(UtmCoord { zone, hemisphere, easting: east, northing: north })
}

fn utm_project_point(lat_deg: f64, lon_deg: f64, zone: u8, hemi: char) -> (f64, f64) {
    let lat = lat_deg.to_radians();
    let lon0 = ((zone as f64 - 1.0) * 6.0 - 180.0 + 3.0).to_radians();
    let dlon = lon_deg.to_radians() - lon0;
    let (sin_lat, cos_lat) = (lat.sin(), lat.cos());
    let tan_lat = lat.tan();
    let e2 = WGS84_F * (2.0 - WGS84_F);
    let ep2 = e2 / (1.0 - e2);
    let nu = WGS84_A / (1.0 - e2 * sin_lat * sin_lat).sqrt();
    let m = meridian_arc(lat, e2);
    let eta2 = ep2 * cos_lat * cos_lat;
    let t = tan_lat * tan_lat;
    let p = dlon * cos_lat;
    let p2 = p * p;
    let p3 = p2 * p;
    let p4 = p2 * p2;
    let p5 = p4 * p;
    let east = FALSE_EASTING + UTM_K0 * nu * (p + (1.0 - t + eta2) * p3 / 6.0
        + (5.0 - 18.0 * t + t * t + 72.0 * eta2) * p5 / 120.0);
    let north_val = UTM_K0 * (m + nu * tan_lat * (p2 / 2.0
        + (5.0 - t + 9.0 * eta2 + 4.0 * eta2 * eta2) * p4 / 24.0));
    let north = if hemi == 'S' { FALSE_NORTHING_SOUTH + north_val } else { north_val };
    (east, north)
}
