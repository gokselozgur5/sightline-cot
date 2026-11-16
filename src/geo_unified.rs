//! Unified UTM / UPS coordinate transformations.

use crate::mgrs::{GridError, UtmCoord};

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
