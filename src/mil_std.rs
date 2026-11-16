//! MIL-STD-2525 tactical symbol parser and generator.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MilStdError {
    InvalidLength,
    InvalidCharacter,
    InvalidAffiliation,
    InvalidDimension,
    InvalidStatus,
    InvalidEchelon,
}
