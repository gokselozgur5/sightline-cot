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
