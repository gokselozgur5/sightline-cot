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
/// Strongly-typed Cursor-on-Target 2.0 event payload.
#[derive(Debug, Clone, PartialEq)]
pub struct CotEvent {
    pub uid: String,
    pub cot_type: String,
    pub how: String,
    pub time_ms: i64,
    pub stale_ms: i64,
    pub lat_deg: f64,
    pub lon_deg: f64,
    pub hae_m: f64,
    pub ce_m: f64,
    pub le_m: f64,
    pub course_deg: f64,
    pub speed_mps: f64,
}
/// Escape XML attribute special characters to prevent XML injection.
pub fn escape_xml_attr(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 8);
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(c),
        }
    }
    out
}
/// Unescape XML attribute entities in a single pass.
pub fn unescape_xml_attr(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(idx) = rest.find('&') {
        out.push_str(&rest[..idx]);
        let rem = &rest[idx..];
        if let Some(semi) = rem.find(';') {
            match &rem[..=semi] {
                "&quot;" => out.push('"'),
                "&apos;" => out.push('\''),
                "&lt;" => out.push('<'),
                "&gt;" => out.push('>'),
                "&amp;" => out.push('&'),
                other => out.push_str(other),
            }
            rest = &rem[semi + 1..];
        } else {
            out.push_str(rem);
            return out;
        }
    }
    out.push_str(rest);
    out
}
fn days_to_ymd(days_since_epoch: i64) -> (i64, u8, u8) {
    let z = days_since_epoch + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u8;
    let m = (if mp < 10 { mp + 3 } else { mp - 9 }) as u8;
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}
