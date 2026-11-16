//! Unified UTM / UPS coordinate transformations.

use crate::mgrs::{utm_to_wgs84, wgs84_to_utm, GridError, UtmCoord};

/// Active grid coordinate system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridSystem {
    /// Universal Transverse Mercator (80S to 84N).
    Utm,
    /// Universal Polar Stereographic (polar caps).
    Ups,
}

/// UPS coordinate representation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UpsCoord {
    pub hemisphere: char,
    pub easting: f64,
    pub northing: f64,
}

/// Unified UTM or UPS coordinate.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnifiedCoord {
    /// Standard UTM projection coordinate.
    Utm(UtmCoord),
    /// Universal Polar Stereographic coordinate.
    Ups(UpsCoord),
}

impl UnifiedCoord {
    /// Return easting in meters.
    pub fn easting(&self) -> f64 {
        match self {
            Self::Utm(c) => c.easting,
            Self::Ups(c) => c.easting,
        }
    }

    /// Return northing in meters.
    pub fn northing(&self) -> f64 {
        match self {
            Self::Utm(c) => c.northing,
            Self::Ups(c) => c.northing,
        }
    }

    /// Return hemisphere character ('N' or 'S').
    pub fn hemisphere(&self) -> char {
        match self {
            Self::Utm(c) => c.hemisphere,
            Self::Ups(c) => c.hemisphere,
        }
    }
}

impl UnifiedCoord {
    /// Return the active grid system.
    pub fn grid_system(&self) -> GridSystem {
        match self {
            Self::Utm(_) => GridSystem::Utm,
            Self::Ups(_) => GridSystem::Ups,
        }
    }

    /// Return UTM zone or None for UPS.
    pub fn zone(&self) -> Option<u8> {
        match self {
            Self::Utm(c) => Some(c.zone),
            Self::Ups(_) => None,
        }
    }

    /// Return true if coordinate is UTM.
    pub fn is_utm(&self) -> bool {
        matches!(self, Self::Utm(_))
    }

    /// Return true if coordinate is UPS.
    pub fn is_ups(&self) -> bool {
        matches!(self, Self::Ups(_))
    }
}

const WGS84_A: f64 = 6378137.0;
const WGS84_F: f64 = 1.0 / 298.257223563;
const UPS_K0: f64 = 0.994;
const UPS_FALSE_ORIGIN: f64 = 2000000.0;

#[inline]
fn ups_eccentricity() -> (f64, f64) {
    let e2 = WGS84_F * (2.0 - WGS84_F);
    (e2, e2.sqrt())
}

/// Return true if the latitude falls in polar UPS coverage.
#[inline]
pub fn is_polar_latitude(lat_deg: f64) -> bool {
    lat_deg > 84.0 || lat_deg < -80.0
}

/// Return true if the latitude falls in standard UTM coverage.
#[inline]
pub fn is_utm_latitude(lat_deg: f64) -> bool {
    (-80.0..=84.0).contains(&lat_deg)
}

fn ups_c0(e: f64) -> f64 {
    let term1 = (1.0 + e).powf(1.0 + e);
    let term2 = (1.0 - e).powf(1.0 - e);
    2.0 * WGS84_A * UPS_K0 / (term1 * term2).sqrt()
}

fn ups_t(phi: f64, e: f64) -> f64 {
    let con = e * phi.sin();
    let ep = ((1.0 + con) / (1.0 - con)).powf(e / 2.0);
    (std::f64::consts::FRAC_PI_4 - phi / 2.0).tan() * ep
}

fn ups_project_north(phi: f64, lam: f64, e: f64) -> (f64, f64) {
    let rho = ups_c0(e) * ups_t(phi, e);
    let x = UPS_FALSE_ORIGIN + rho * lam.sin();
    let y = UPS_FALSE_ORIGIN - rho * lam.cos();
    (x, y)
}

fn ups_project_south(phi_s: f64, lam: f64, e: f64) -> (f64, f64) {
    let rho = ups_c0(e) * ups_t(phi_s, e);
    let x = UPS_FALSE_ORIGIN + rho * lam.sin();
    let y = UPS_FALSE_ORIGIN + rho * lam.cos();
    (x, y)
}

/// Project WGS84 latitude and longitude to UPS.
pub fn wgs84_to_ups(lat_deg: f64, lon_deg: f64) -> Result<UpsCoord, GridError> {
    if !(-90.0..=90.0).contains(&lat_deg) {
        return Err(GridError::InvalidLatitude);
    }
    if !(-180.0..=180.0).contains(&lon_deg) {
        return Err(GridError::InvalidLongitude);
    }
    if !is_polar_latitude(lat_deg) && lat_deg != 84.0 && lat_deg != -80.0 {
        return Err(GridError::InvalidLatitude);
    }
    let (_, e) = ups_eccentricity();
    let lam = lon_deg.to_radians();
    let (hemisphere, easting, northing) = if lat_deg >= 0.0 {
        let (x, y) = ups_project_north(lat_deg.to_radians(), lam, e);
        ('N', x, y)
    } else {
        let (x, y) = ups_project_south((-lat_deg).to_radians(), lam, e);
        ('S', x, y)
    };
    Ok(UpsCoord { hemisphere, easting, northing })
}

fn ups_solve_phi(t: f64, e: f64) -> f64 {
    let mut phi = std::f64::consts::FRAC_PI_2 - 2.0 * t.atan();
    for _ in 0..6 {
        let con = e * phi.sin();
        let ep = ((1.0 - con) / (1.0 + con)).powf(e / 2.0);
        phi = std::f64::consts::FRAC_PI_2 - 2.0 * (t * ep).atan();
    }
    phi
}

fn ups_unproject_north(easting: f64, northing: f64, e: f64) -> (f64, f64) {
    let (dx, dy) = (easting - UPS_FALSE_ORIGIN, northing - UPS_FALSE_ORIGIN);
    let rho = dx.hypot(dy);
    if rho < 1e-10 {
        return (90.0, 0.0);
    }
    let lam = dx.atan2(-dy).to_degrees();
    let phi = ups_solve_phi(rho / ups_c0(e), e).to_degrees();
    (phi, lam)
}

fn ups_unproject_south(easting: f64, northing: f64, e: f64) -> (f64, f64) {
    let (dx, dy) = (easting - UPS_FALSE_ORIGIN, northing - UPS_FALSE_ORIGIN);
    let rho = dx.hypot(dy);
    if rho < 1e-10 {
        return (-90.0, 0.0);
    }
    let lam = dx.atan2(dy).to_degrees();
    let phi_s = ups_solve_phi(rho / ups_c0(e), e).to_degrees();
    (-phi_s, lam)
}

/// Unproject UPS coordinate back to WGS84 (lat_deg, lon_deg).
pub fn ups_to_wgs84(coord: &UpsCoord) -> Result<(f64, f64), GridError> {
    if coord.hemisphere != 'N' && coord.hemisphere != 'S' {
        return Err(GridError::InvalidLatitude);
    }
    if coord.easting <= 0.0 || coord.northing <= 0.0 {
        return Err(GridError::InvalidEasting);
    }
    let (_, e) = ups_eccentricity();
    let (lat, lon) = if coord.hemisphere == 'N' {
        ups_unproject_north(coord.easting, coord.northing, e)
    } else {
        ups_unproject_south(coord.easting, coord.northing, e)
    };
    if !(-90.0..=90.0).contains(&lat) {
        return Err(GridError::InvalidLatitude);
    }
    Ok((lat, lon))
}

/// Transform WGS84 geographic coordinate to unified UTM/UPS representation.
pub fn wgs84_to_unified(lat_deg: f64, lon_deg: f64) -> Result<UnifiedCoord, GridError> {
    if !(-90.0..=90.0).contains(&lat_deg) {
        return Err(GridError::InvalidLatitude);
    }
    if !(-180.0..=180.0).contains(&lon_deg) {
        return Err(GridError::InvalidLongitude);
    }
    if is_polar_latitude(lat_deg) {
        wgs84_to_ups(lat_deg, lon_deg).map(UnifiedCoord::Ups)
    } else {
        wgs84_to_utm(lat_deg, lon_deg).map(UnifiedCoord::Utm)
    }
}

/// Transform unified coordinate back to WGS84 geographic coordinate (lat_deg, lon_deg).
pub fn unified_to_wgs84(coord: &UnifiedCoord) -> Result<(f64, f64), GridError> {
    match coord {
        UnifiedCoord::Utm(utm) => utm_to_wgs84(utm),
        UnifiedCoord::Ups(ups) => ups_to_wgs84(ups),
    }
}
