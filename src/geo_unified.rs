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
