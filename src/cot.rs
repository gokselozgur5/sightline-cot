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
