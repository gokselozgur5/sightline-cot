//! CoT Cursor-on-Target XML schema validator and namespace canonicalizer.

use crate::cot::parse_iso8601_utc;
use std::collections::{HashMap, HashSet};
use std::fmt;

/// Errors produced during CoT XML schema validation or canonicalization.
#[derive(Debug, Clone, PartialEq)]
pub enum CotValidationError {
    EmptyXml,
    MalformedXml(String),
    MissingRequiredElement(&'static str),
    MissingRequiredAttribute { element: &'static str, attribute: &'static str },
    InvalidVersion(String),
    InvalidCotType(String),
    InvalidTimestamp { attribute: &'static str, value: String },
    TimestampOrderViolation { earlier: &'static str, later: &'static str },
    CoordinateOutOfBounds { coordinate: &'static str, value: f64 },
    NegativeValueError { field: &'static str, value: f64 },
    NonFiniteNumber(&'static str),
    DuplicateAttribute { element: String, attribute: String },
    UnclosedTag(String),
    MismatchedClosingTag { expected: String, found: String },
}

impl fmt::Display for CotValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyXml => write!(f, "XML input is empty"),
            Self::MalformedXml(msg) => write!(f, "malformed XML: {msg}"),
            Self::MissingRequiredElement(el) => write!(f, "missing required element: <{el}>"),
            Self::MissingRequiredAttribute { element, attribute } => {
                write!(f, "missing required attribute '{attribute}' on <{element}>")
            }
            Self::InvalidVersion(v) => write!(f, "unsupported CoT version: '{v}', expected '2.0'"),
            Self::InvalidCotType(t) => write!(f, "invalid CoT type: '{t}'"),
            Self::InvalidTimestamp { attribute, value } => {
                write!(f, "invalid ISO-8601 timestamp in '{attribute}': '{value}'")
            }
            Self::TimestampOrderViolation { earlier, later } => {
                write!(f, "timestamp order violation: '{earlier}' must be <= '{later}'")
            }
            Self::CoordinateOutOfBounds { coordinate, value } => {
                write!(f, "coordinate out of bounds for '{coordinate}': {value}")
            }
            Self::NegativeValueError { field, value } => {
                write!(f, "value must be non-negative for '{field}': {value}")
            }
