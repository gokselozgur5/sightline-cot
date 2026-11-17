//! MIL-STD-2525D tactical symbol SIDC 20-digit parser and generator.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SidcParseError {
    InvalidLength,
    NonNumeric,
    InvalidStandard,
    InvalidAffiliation,
    InvalidSymbolSet,
    InvalidStatus,
    InvalidEchelon,
    InvalidCotType,
}
