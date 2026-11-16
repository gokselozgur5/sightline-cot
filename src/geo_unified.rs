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
