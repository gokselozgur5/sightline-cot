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
            Self::NonFiniteNumber(field) => write!(f, "non-finite number for '{field}'"),
            Self::DuplicateAttribute { element, attribute } => {
                write!(f, "duplicate attribute '{attribute}' on element <{element}>")
            }
            Self::UnclosedTag(tag) => write!(f, "unclosed tag: <{tag}>"),
            Self::MismatchedClosingTag { expected, found } => {
                write!(f, "mismatched closing tag: expected </{expected}>, found </{found}>")
            }
        }
    }
}

impl std::error::Error for CotValidationError {}

/// Parsed XML attribute representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XmlAttribute {
    pub name: String,
    pub value: String,
}

/// Options controlling CoT XML canonicalization behavior.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalizeOptions {
    /// Sort attributes lexicographically (namespaces first, then names).
    pub sort_attributes: bool,
    /// Normalize surrounding whitespace.
    pub normalize_whitespace: bool,
    /// Strip XML comments.
    pub remove_comments: bool,
    /// Format empty elements as self-closing tags (<tag />).
    pub self_closing_tags: bool,
    /// Strip redundant namespace declarations already declared in ancestor scopes.
    pub strip_redundant_namespaces: bool,
}

impl Default for CanonicalizeOptions {
    fn default() -> Self {
        Self {
            sort_attributes: true,
            normalize_whitespace: true,
            remove_comments: true,
            self_closing_tags: true,
            strip_redundant_namespaces: true,
        }
    }
}

/// Internal XML Node representation for canonical tree formatting.
#[derive(Debug, Clone, PartialEq)]
enum XmlNode {
    Element {
        name: String,
        attributes: Vec<XmlAttribute>,
        children: Vec<XmlNode>,
    },
    Text(String),
}

/// Tokenizer token produced when scanning XML stream.
#[derive(Debug, Clone, PartialEq)]
enum RawXmlToken {
    StartTag {
        name: String,
        attributes: Vec<XmlAttribute>,
        self_closing: bool,
    },
    EndTag {
        name: String,
    },
    Text(String),
    Comment,
