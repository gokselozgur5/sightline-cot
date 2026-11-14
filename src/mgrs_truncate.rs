//! MGRS precision truncation module.
use crate::mgrs::GridError;

/// Target precision level for MGRS truncation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MgrsPrecision {
    OneMeter,
    TenMeters,
    HundredMeters,
    OneKilometer,
    TenKilometers,
}

impl MgrsPrecision {
    /// Number of digits per coordinate axis (easting and northing).
    pub fn digits_per_axis(&self) -> usize {
        match self {
            Self::OneMeter => 5,
            Self::TenMeters => 4,
            Self::HundredMeters => 3,
            Self::OneKilometer => 2,
            Self::TenKilometers => 1,
        }
    }
}
