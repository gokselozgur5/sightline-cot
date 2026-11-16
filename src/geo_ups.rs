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

fn conformal_radius(lat_rad: f64, e: f64) -> f64 {
    let half_pi = std::f64::consts::FRAC_PI_2;
    let s = lat_rad.sin();
    let es = e * s;
    let t = ((half_pi - lat_rad) / 2.0).tan() * ((1.0 + es) / (1.0 - es)).powf(e / 2.0);
    t
}

pub fn wgs84_to_ups(lat_deg: f64, lon_deg: f64) -> Result<UpsCoord, UpsError> {
    if !lat_deg.is_finite() || !lon_deg.is_finite() {
        return Err(UpsError::NonFinite);
    }
    let e2 = WGS84_F * (2.0 - WGS84_F);
    let e = e2.sqrt();
    if lat_deg >= 83.5 {
        let lat_rad = lat_deg.to_radians();
        let lon_rad = lon_deg.to_radians();
        let t = conformal_radius(lat_rad, e);
        let m1 = UPS_K0 * 2.0 * WGS84_A / ((1.0 - e2).sqrt());
        let rho = m1 * t / 2.0;
        let easting = FALSE_EASTING_NORTH + rho * lon_rad.sin();
        let northing = FALSE_NORTHING_NORTH - rho * lon_rad.cos();
        Ok(UpsCoord::new(easting, northing, UpsHemisphere::North))
    } else if lat_deg <= -79.5 {
        let lat_rad = (-lat_deg).to_radians();
        let lon_rad = lon_deg.to_radians();
        let t = conformal_radius(lat_rad, e);
        let m1 = UPS_K0 * 2.0 * WGS84_A / ((1.0 - e2).sqrt());
        let rho = m1 * t / 2.0;
        let easting = FALSE_EASTING_SOUTH + rho * lon_rad.sin();
        let northing = FALSE_NORTHING_SOUTH + rho * lon_rad.cos();
        Ok(UpsCoord::new(easting, northing, UpsHemisphere::South))
    } else {
        Err(UpsError::InvalidLatitude)
    }
}

pub fn ups_to_wgs84(coord: UpsCoord) -> Result<(f64, f64), UpsError> {
    if !coord.easting.is_finite() || !coord.northing.is_finite() {
        return Err(UpsError::NonFinite);
    }
    let (dx, dy) = match coord.hemisphere {
        UpsHemisphere::North => (coord.easting - FALSE_EASTING_NORTH, -(coord.northing - FALSE_NORTHING_NORTH)),
        UpsHemisphere::South => (coord.easting - FALSE_EASTING_SOUTH, coord.northing - FALSE_NORTHING_SOUTH),
    };
    let rho = (dx * dx + dy * dy).sqrt();
    if rho < 1e-12 {
        let lat = if coord.hemisphere == UpsHemisphere::North { 90.0 } else { -90.0 };
        return Ok((lat, 0.0));
    }
    let lon_rad = dx.atan2(dy);
    let lon_deg = lon_rad.to_degrees();
    let e2 = WGS84_F * (2.0 - WGS84_F);
    let m1 = UPS_K0 * 2.0 * WGS84_A / ((1.0 - e2).sqrt());
    let t = 2.0 * rho / m1;
    let mut phi = std::f64::consts::FRAC_PI_2 - 2.0 * t.atan();
    let e = e2.sqrt();
    for _ in 0..5 {
        let es = e * phi.sin();
        let next_phi = std::f64::consts::FRAC_PI_2 - 2.0 * (t * ((1.0 - es) / (1.0 + es)).powf(e / 2.0)).atan();
        phi = next_phi;
    }
    let lat_deg = if coord.hemisphere == UpsHemisphere::North { phi.to_degrees() } else { -phi.to_degrees() };
    Ok((lat_deg, lon_deg))
}
