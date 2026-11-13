//! Degrees decimal-minutes (DDM) coordinate parser.

/// Errors that can occur during DDM coordinate parsing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DdmError {
    EmptyInput,
    InvalidFormat,
    InvalidDegrees,
    InvalidMinutes,
    MinutesOutOfRange,
    LatitudeOutOfRange,
    LongitudeOutOfRange,
    ConflictingHemisphere,
}
