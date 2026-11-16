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
