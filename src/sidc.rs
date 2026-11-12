//! MIL-STD-2525D and APP-6 tactical Symbol Identification Code (SIDC).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SidcError {
    InvalidLength,
    InvalidCharacter,
    UnknownStandard,
    InvalidAffiliation,
    InvalidDomain,
}
