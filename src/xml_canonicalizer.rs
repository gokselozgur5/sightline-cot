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
    ProcessingInstruction,
}

/// Cursor-on-Target XML schema validator.
pub struct CotSchemaValidator;

impl CotSchemaValidator {
    /// Validate that an XML document conforms to CoT 2.0 schema rules.
    pub fn validate(xml: &str) -> Result<(), CotValidationError> {
        let trimmed = xml.trim();
        if trimmed.is_empty() {
            return Err(CotValidationError::EmptyXml);
        }

        let tokens = tokenize_xml(trimmed)?;
        let mut tag_stack: Vec<String> = Vec::new();
        let mut root_seen = false;
        let mut point_seen = false;
        let mut event_attrs: Option<Vec<XmlAttribute>> = None;

        for token in tokens {
            match token {
                RawXmlToken::Comment | RawXmlToken::ProcessingInstruction => {}
                RawXmlToken::Text(text) => {
                    if !tag_stack.is_empty() && text.chars().any(|c| !c.is_whitespace()) {
                        // Non-whitespace text inside elements is acceptable in CoT (e.g. remarks)
                    }
                }
                RawXmlToken::StartTag { name, attributes, self_closing } => {
                    let local_name = strip_prefix(&name);
                    if tag_stack.is_empty() {
                        if root_seen {
                            return Err(CotValidationError::MalformedXml(
                                "multiple root elements detected".into(),
                            ));
                        }
                        if local_name != "event" {
                            return Err(CotValidationError::MalformedXml(format!(
                                "root element must be <event>, found <{name}>"
                            )));
                        }
                        root_seen = true;
                        Self::validate_event_attributes(&attributes)?;
                        event_attrs = Some(attributes.clone());
                    } else if local_name == "point" {
                        let parent = tag_stack.last().map(|s| strip_prefix(s)).unwrap_or("");
                        if parent == "event" {
                            point_seen = true;
                            Self::validate_point_attributes(&attributes)?;
                        }
                    }

                    if !self_closing {
                        tag_stack.push(name);
                    }
                }
                RawXmlToken::EndTag { name } => {
                    match tag_stack.pop() {
                        Some(expected) => {
                            if expected != name {
                                return Err(CotValidationError::MismatchedClosingTag {
                                    expected,
                                    found: name,
                                });
                            }
                        }
                        None => {
                            return Err(CotValidationError::MalformedXml(format!(
                                "unexpected closing tag </{name}> without matching open tag"
                            )));
                        }
                    }
                }
            }
        }

        if let Some(unclosed) = tag_stack.pop() {
            return Err(CotValidationError::UnclosedTag(unclosed));
        }

        if !root_seen {
            return Err(CotValidationError::MissingRequiredElement("event"));
        }
        if !point_seen {
            return Err(CotValidationError::MissingRequiredElement("point"));
        }

        let _ = event_attrs;
        Ok(())
    }

    /// Validate required and optional attributes of <event> element.
    pub fn validate_event_attributes(attrs: &[XmlAttribute]) -> Result<(), CotValidationError> {
        let mut map: HashMap<&str, &str> = HashMap::new();
        for a in attrs {
            map.insert(a.name.as_str(), a.value.as_str());
