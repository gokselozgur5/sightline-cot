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
