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
        }

        let version = map.get("version").ok_or(CotValidationError::MissingRequiredAttribute {
            element: "event",
            attribute: "version",
        })?;
        if *version != "2.0" {
            return Err(CotValidationError::InvalidVersion((*version).to_string()));
        }

        let uid = map.get("uid").ok_or(CotValidationError::MissingRequiredAttribute {
            element: "event",
            attribute: "uid",
        })?;
        if uid.trim().is_empty() {
            return Err(CotValidationError::MalformedXml("event uid cannot be empty".into()));
        }

        let cot_type = map.get("type").ok_or(CotValidationError::MissingRequiredAttribute {
            element: "event",
            attribute: "type",
        })?;
        Self::validate_cot_type(cot_type)?;

        let how = map.get("how").ok_or(CotValidationError::MissingRequiredAttribute {
            element: "event",
            attribute: "how",
        })?;
        if how.trim().is_empty() {
            return Err(CotValidationError::MalformedXml("event how cannot be empty".into()));
        }

        let time_str = map.get("time").ok_or(CotValidationError::MissingRequiredAttribute {
            element: "event",
            attribute: "time",
        })?;
        let time_ms = parse_iso8601_utc(time_str).map_err(|_| CotValidationError::InvalidTimestamp {
            attribute: "time",
            value: (*time_str).to_string(),
        })?;

        let start_str = map.get("start").ok_or(CotValidationError::MissingRequiredAttribute {
            element: "event",
            attribute: "start",
        })?;
        let start_ms = parse_iso8601_utc(start_str).map_err(|_| CotValidationError::InvalidTimestamp {
            attribute: "start",
            value: (*start_str).to_string(),
        })?;

        let stale_str = map.get("stale").ok_or(CotValidationError::MissingRequiredAttribute {
            element: "event",
            attribute: "stale",
        })?;
        let stale_ms = parse_iso8601_utc(stale_str).map_err(|_| CotValidationError::InvalidTimestamp {
            attribute: "stale",
            value: (*stale_str).to_string(),
        })?;

        if stale_ms < time_ms {
            return Err(CotValidationError::TimestampOrderViolation {
                earlier: "time",
                later: "stale",
            });
        }
        if stale_ms < start_ms {
            return Err(CotValidationError::TimestampOrderViolation {
                earlier: "start",
                later: "stale",
            });
        }

        Ok(())
    }

    /// Validate required and optional attributes of <point> element.
    pub fn validate_point_attributes(attrs: &[XmlAttribute]) -> Result<(), CotValidationError> {
        let mut map: HashMap<&str, &str> = HashMap::new();
        for a in attrs {
            map.insert(a.name.as_str(), a.value.as_str());
        }

        let lat_str = map.get("lat").ok_or(CotValidationError::MissingRequiredAttribute {
            element: "point",
            attribute: "lat",
        })?;
        let lat: f64 = lat_str.parse().map_err(|_| CotValidationError::MalformedXml(
            format!("invalid float for point lat: '{lat_str}'"),
        ))?;
        if !lat.is_finite() {
            return Err(CotValidationError::NonFiniteNumber("lat"));
        }
        if !(-90.0..=90.0).contains(&lat) {
            return Err(CotValidationError::CoordinateOutOfBounds {
                coordinate: "lat",
                value: lat,
            });
        }

        let lon_str = map.get("lon").ok_or(CotValidationError::MissingRequiredAttribute {
            element: "point",
            attribute: "lon",
        })?;
        let lon: f64 = lon_str.parse().map_err(|_| CotValidationError::MalformedXml(
            format!("invalid float for point lon: '{lon_str}'"),
        ))?;
        if !lon.is_finite() {
            return Err(CotValidationError::NonFiniteNumber("lon"));
        }
        if !(-180.0..=180.0).contains(&lon) {
            return Err(CotValidationError::CoordinateOutOfBounds {
                coordinate: "lon",
                value: lon,
            });
        }

        let hae_str = map.get("hae").ok_or(CotValidationError::MissingRequiredAttribute {
            element: "point",
            attribute: "hae",
        })?;
        let hae: f64 = hae_str.parse().map_err(|_| CotValidationError::MalformedXml(
            format!("invalid float for point hae: '{hae_str}'"),
        ))?;
        if !hae.is_finite() {
            return Err(CotValidationError::NonFiniteNumber("hae"));
        }

        let ce_str = map.get("ce").ok_or(CotValidationError::MissingRequiredAttribute {
            element: "point",
            attribute: "ce",
        })?;
        let ce: f64 = ce_str.parse().map_err(|_| CotValidationError::MalformedXml(
            format!("invalid float for point ce: '{ce_str}'"),
        ))?;
        if !ce.is_finite() {
            return Err(CotValidationError::NonFiniteNumber("ce"));
        }
        if ce < 0.0 {
            return Err(CotValidationError::NegativeValueError { field: "ce", value: ce });
        }

        let le_str = map.get("le").ok_or(CotValidationError::MissingRequiredAttribute {
            element: "point",
            attribute: "le",
        })?;
        let le: f64 = le_str.parse().map_err(|_| CotValidationError::MalformedXml(
            format!("invalid float for point le: '{le_str}'"),
        ))?;
        if !le.is_finite() {
            return Err(CotValidationError::NonFiniteNumber("le"));
        }
        if le < 0.0 {
            return Err(CotValidationError::NegativeValueError { field: "le", value: le });
        }

        Ok(())
    }

    /// Validate CoT hierarchical type string format.
    pub fn validate_cot_type(cot_type: &str) -> Result<(), CotValidationError> {
        let trimmed = cot_type.trim();
        if trimmed.is_empty() {
            return Err(CotValidationError::InvalidCotType("empty type".into()));
        }
        for c in trimmed.chars() {
            if !c.is_ascii_alphanumeric() && c != '-' && c != '.' && c != '_' {
                return Err(CotValidationError::InvalidCotType(format!(
                    "invalid character '{c}' in CoT type '{trimmed}'"
                )));
            }
        }
        Ok(())
    }
}

/// CoT XML canonicalizer for standard C14N attribute sorting and namespace management.
pub struct CotXmlCanonicalizer;

impl CotXmlCanonicalizer {
    /// Canonicalize an XML string using default canonicalization options.
    pub fn canonicalize(xml: &str) -> Result<String, CotValidationError> {
        Self::canonicalize_with_options(xml, &CanonicalizeOptions::default())
    }

    /// Canonicalize an XML string with explicit options.
    pub fn canonicalize_with_options(
        xml: &str,
        opts: &CanonicalizeOptions,
    ) -> Result<String, CotValidationError> {
        CotSchemaValidator::validate(xml)?;
        let tree = parse_xml_to_tree(xml, opts)?;
        let mut out = String::new();
        let mut active_ns: Vec<HashMap<String, String>> = Vec::new();
        serialize_node(&tree, opts, &mut active_ns, &mut out);
        Ok(out)
    }
}

/// Validate CoT XML string against CoT 2.0 schema rules.
pub fn validate_cot_xml(xml: &str) -> Result<(), CotValidationError> {
    CotSchemaValidator::validate(xml)
}

/// Canonicalize CoT XML into deterministic C14N attribute order and namespace normalization.
pub fn canonicalize_cot_xml(xml: &str) -> Result<String, CotValidationError> {
    CotXmlCanonicalizer::canonicalize(xml)
}

fn strip_prefix(name: &str) -> &str {
    if let Some(idx) = name.find(':') {
        &name[idx + 1..]
    } else {
        name
    }
}

fn tokenize_xml(xml: &str) -> Result<Vec<RawXmlToken>, CotValidationError> {
    let mut tokens = Vec::new();
    let mut rest = xml;

    while !rest.is_empty() {
        if rest.starts_with("<!--") {
            let end_idx = rest.find("-->").ok_or_else(|| {
                CotValidationError::MalformedXml("unclosed comment <!--".into())
            })?;
            tokens.push(RawXmlToken::Comment);
            rest = &rest[end_idx + 3..];
        } else if rest.starts_with("<?") {
            let end_idx = rest.find("?>").ok_or_else(|| {
                CotValidationError::MalformedXml("unclosed processing instruction <?".into())
            })?;
            tokens.push(RawXmlToken::ProcessingInstruction);
            rest = &rest[end_idx + 2..];
        } else if rest.starts_with("<![CDATA[") {
            let end_idx = rest.find("]]>").ok_or_else(|| {
                CotValidationError::MalformedXml("unclosed CDATA block".into())
            })?;
            let cdata_content = &rest[9..end_idx];
            tokens.push(RawXmlToken::Text(cdata_content.to_string()));
            rest = &rest[end_idx + 3..];
