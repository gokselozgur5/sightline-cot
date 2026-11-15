//! Degrees-minutes-seconds (DMS) coordinate parsing and formatting.

/// Errors that can occur during DMS coordinate parsing and formatting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmsError {
    EmptyInput,
    InvalidFormat,
    InvalidDegrees,
    InvalidMinutes,
    InvalidSeconds,
    MinutesOutOfRange,
    SecondsOutOfRange,
    LatitudeOutOfRange,
    LongitudeOutOfRange,
    ConflictingHemisphere,
}
