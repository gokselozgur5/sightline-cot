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
