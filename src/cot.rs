//! Zero-dependency Cursor-on-Target 2.0 codec.

use std::fmt;
use std::net::{SocketAddr, UdpSocket};

/// Errors produced during CoT parsing, validation, or dispatch.
#[derive(Debug, Clone, PartialEq)]
pub enum CotError {
    InvalidXml(String),
    MissingAttribute(&'static str),
    InvalidNumber(&'static str),
    CoordinateOutOfBounds(&'static str),
    NetworkError(String),
}
impl fmt::Display for CotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidXml(m) => write!(f, "invalid XML: {m}"),
            Self::MissingAttribute(a) => write!(f, "missing attribute: {a}"),
            Self::InvalidNumber(a) => write!(f, "invalid number: {a}"),
            Self::CoordinateOutOfBounds(a) => write!(f, "out of bounds: {a}"),
            Self::NetworkError(m) => write!(f, "network error: {m}"),
        }
    }
}

impl std::error::Error for CotError {}
