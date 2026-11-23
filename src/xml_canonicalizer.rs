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
    MissingRequiredAttribute {
        element: &'static str,
        attribute: &'static str,
    },
    InvalidVersion(String),
    InvalidCotType(String),
    InvalidTimestamp {
        attribute: &'static str,
        value: String,
    },
    TimestampOrderViolation {
        earlier: &'static str,
        later: &'static str,
    },
    CoordinateOutOfBounds {
        coordinate: &'static str,
        value: f64,
    },
    NegativeValueError {
        field: &'static str,
        value: f64,
    },
    NonFiniteNumber(&'static str),
    DuplicateAttribute {
        element: String,
        attribute: String,
    },
    UnclosedTag(String),
    MismatchedClosingTag {
        expected: String,
        found: String,
    },
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
                write!(
                    f,
                    "timestamp order violation: '{earlier}' must be <= '{later}'"
                )
            }
            Self::CoordinateOutOfBounds { coordinate, value } => {
                write!(f, "coordinate out of bounds for '{coordinate}': {value}")
            }
            Self::NegativeValueError { field, value } => {
                write!(f, "value must be non-negative for '{field}': {value}")
            }
            Self::NonFiniteNumber(field) => write!(f, "non-finite number for '{field}'"),
            Self::DuplicateAttribute { element, attribute } => {
                write!(
                    f,
                    "duplicate attribute '{attribute}' on element <{element}>"
                )
            }
            Self::UnclosedTag(tag) => write!(f, "unclosed tag: <{tag}>"),
            Self::MismatchedClosingTag { expected, found } => {
                write!(
                    f,
                    "mismatched closing tag: expected </{expected}>, found </{found}>"
                )
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
    /// Format empty elements as self-closing tags (`<tag />`).
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
    Comment(String),
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
    Comment(String),
    ProcessingInstruction,
}

/// Cursor-on-Target XML schema validator.
pub struct CotSchemaValidator;

impl CotSchemaValidator {
    /// Maximum allowed element nesting depth to prevent stack overflow.
    pub const MAX_DEPTH: usize = 256;

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

        for token in tokens {
            match token {
                RawXmlToken::Comment(_) | RawXmlToken::ProcessingInstruction => {}
                RawXmlToken::Text(text) => {
                    if text.chars().any(|c| !c.is_whitespace()) && tag_stack.is_empty() {
                        return Err(CotValidationError::MalformedXml(
                            "non-whitespace text outside root element".into(),
                        ));
                        // Non-whitespace text inside elements is acceptable in CoT (e.g. remarks)
                    }
                }
                RawXmlToken::StartTag {
                    name,
                    attributes,
                    self_closing,
                } => {
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
                    } else if local_name == "point" {
                        let parent = tag_stack.last().map(|s| strip_prefix(s)).unwrap_or("");
                        if parent == "event" {
                            point_seen = true;
                            Self::validate_point_attributes(&attributes)?;
                        }
                    }

                    if !self_closing {
                        if tag_stack.len() >= Self::MAX_DEPTH {
                            return Err(CotValidationError::MalformedXml(format!(
                                "element nesting depth exceeds limit of {}",
                                Self::MAX_DEPTH
                            )));
                        }
                        tag_stack.push(name);
                    }
                }
                RawXmlToken::EndTag { name } => match tag_stack.pop() {
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
                },
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

        Ok(())
    }

    /// Validate required and optional attributes of `<event>` element.
    pub fn validate_event_attributes(attrs: &[XmlAttribute]) -> Result<(), CotValidationError> {
        let mut map: HashMap<&str, &str> = HashMap::new();
        for a in attrs {
            map.insert(a.name.as_str(), a.value.as_str());
        }

        let version = map
            .get("version")
            .ok_or(CotValidationError::MissingRequiredAttribute {
                element: "event",
                attribute: "version",
            })?;
        if *version != "2.0" {
            return Err(CotValidationError::InvalidVersion((*version).to_string()));
        }

        let uid = map
            .get("uid")
            .ok_or(CotValidationError::MissingRequiredAttribute {
                element: "event",
                attribute: "uid",
            })?;
        if uid.trim().is_empty() {
            return Err(CotValidationError::MalformedXml(
                "event uid cannot be empty".into(),
            ));
        }

        let cot_type = map
            .get("type")
            .ok_or(CotValidationError::MissingRequiredAttribute {
                element: "event",
                attribute: "type",
            })?;
        Self::validate_cot_type(cot_type)?;

        let how = map
            .get("how")
            .ok_or(CotValidationError::MissingRequiredAttribute {
                element: "event",
                attribute: "how",
            })?;
        if how.trim().is_empty() {
            return Err(CotValidationError::MalformedXml(
                "event how cannot be empty".into(),
            ));
        }

        let time_str = map
            .get("time")
            .ok_or(CotValidationError::MissingRequiredAttribute {
                element: "event",
                attribute: "time",
            })?;
        let time_ms =
            parse_iso8601_utc(time_str).map_err(|_| CotValidationError::InvalidTimestamp {
                attribute: "time",
                value: (*time_str).to_string(),
            })?;

        let start_str = map
            .get("start")
            .ok_or(CotValidationError::MissingRequiredAttribute {
                element: "event",
                attribute: "start",
            })?;
        let start_ms =
            parse_iso8601_utc(start_str).map_err(|_| CotValidationError::InvalidTimestamp {
                attribute: "start",
                value: (*start_str).to_string(),
            })?;

        let stale_str = map
            .get("stale")
            .ok_or(CotValidationError::MissingRequiredAttribute {
                element: "event",
                attribute: "stale",
            })?;
        let stale_ms =
            parse_iso8601_utc(stale_str).map_err(|_| CotValidationError::InvalidTimestamp {
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

    /// Validate required and optional attributes of `<point>` element.
    pub fn validate_point_attributes(attrs: &[XmlAttribute]) -> Result<(), CotValidationError> {
        let mut map: HashMap<&str, &str> = HashMap::new();
        for a in attrs {
            map.insert(a.name.as_str(), a.value.as_str());
        }

        let lat_str = map
            .get("lat")
            .ok_or(CotValidationError::MissingRequiredAttribute {
                element: "point",
                attribute: "lat",
            })?;
        let lat: f64 = lat_str.parse().map_err(|_| {
            CotValidationError::MalformedXml(format!("invalid float for point lat: '{lat_str}'"))
        })?;
        if !lat.is_finite() {
            return Err(CotValidationError::NonFiniteNumber("lat"));
        }
        if !(-90.0..=90.0).contains(&lat) {
            return Err(CotValidationError::CoordinateOutOfBounds {
                coordinate: "lat",
                value: lat,
            });
        }

        let lon_str = map
            .get("lon")
            .ok_or(CotValidationError::MissingRequiredAttribute {
                element: "point",
                attribute: "lon",
            })?;
        let lon: f64 = lon_str.parse().map_err(|_| {
            CotValidationError::MalformedXml(format!("invalid float for point lon: '{lon_str}'"))
        })?;
        if !lon.is_finite() {
            return Err(CotValidationError::NonFiniteNumber("lon"));
        }
        if !(-180.0..=180.0).contains(&lon) {
            return Err(CotValidationError::CoordinateOutOfBounds {
                coordinate: "lon",
                value: lon,
            });
        }

        let hae_str = map
            .get("hae")
            .ok_or(CotValidationError::MissingRequiredAttribute {
                element: "point",
                attribute: "hae",
            })?;
        let hae: f64 = hae_str.parse().map_err(|_| {
            CotValidationError::MalformedXml(format!("invalid float for point hae: '{hae_str}'"))
        })?;
        if !hae.is_finite() {
            return Err(CotValidationError::NonFiniteNumber("hae"));
        }

        let ce_str = map
            .get("ce")
            .ok_or(CotValidationError::MissingRequiredAttribute {
                element: "point",
                attribute: "ce",
            })?;
        let ce: f64 = ce_str.parse().map_err(|_| {
            CotValidationError::MalformedXml(format!("invalid float for point ce: '{ce_str}'"))
        })?;
        if !ce.is_finite() {
            return Err(CotValidationError::NonFiniteNumber("ce"));
        }
        if ce < 0.0 {
            return Err(CotValidationError::NegativeValueError {
                field: "ce",
                value: ce,
            });
        }

        let le_str = map
            .get("le")
            .ok_or(CotValidationError::MissingRequiredAttribute {
                element: "point",
                attribute: "le",
            })?;
        let le: f64 = le_str.parse().map_err(|_| {
            CotValidationError::MalformedXml(format!("invalid float for point le: '{le_str}'"))
        })?;
        if !le.is_finite() {
            return Err(CotValidationError::NonFiniteNumber("le"));
        }
        if le < 0.0 {
            return Err(CotValidationError::NegativeValueError {
                field: "le",
                value: le,
            });
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

/// CoT XML canonicalizer producing crate-specific deterministic normal forms.
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

/// Canonicalize CoT XML into a crate-specific deterministic normal form.
///
/// Documents with element nesting depth exceeding [`CotSchemaValidator::MAX_DEPTH`] (256)
/// are rejected with [`CotValidationError::MalformedXml`] to prevent stack overflow.
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
            let end_idx = rest
                .find("-->")
                .ok_or_else(|| CotValidationError::MalformedXml("unclosed comment <!--".into()))?;
            let comment_text = &rest[4..end_idx];
            tokens.push(RawXmlToken::Comment(comment_text.to_string()));
            rest = &rest[end_idx + 3..];
        } else if rest.starts_with("<?") {
            let end_idx = rest.find("?>").ok_or_else(|| {
                CotValidationError::MalformedXml("unclosed processing instruction <?".into())
            })?;
            tokens.push(RawXmlToken::ProcessingInstruction);
            rest = &rest[end_idx + 2..];
        } else if rest.starts_with("<![CDATA[") {
            let end_idx = rest
                .find("]]>")
                .ok_or_else(|| CotValidationError::MalformedXml("unclosed CDATA block".into()))?;
            let cdata_content = &rest[9..end_idx];
            tokens.push(RawXmlToken::Text(cdata_content.to_string()));
            rest = &rest[end_idx + 3..];
        } else if rest.starts_with("<!") {
            let end_idx = rest.find('>').ok_or_else(|| {
                CotValidationError::MalformedXml("unclosed DOCTYPE or declaration".into())
            })?;
            rest = &rest[end_idx + 1..];
        } else if rest.starts_with("</") {
            let end_idx = rest
                .find('>')
                .ok_or_else(|| CotValidationError::MalformedXml("unclosed end tag </".into()))?;
            let tag_content = rest[2..end_idx].trim();
            if tag_content.is_empty() {
                return Err(CotValidationError::MalformedXml("empty end tag </>".into()));
            }
            tokens.push(RawXmlToken::EndTag {
                name: tag_content.to_string(),
            });
            rest = &rest[end_idx + 1..];
        } else if rest.starts_with('<') {
            let end_idx = find_tag_closing(rest)?;
            let tag_content = &rest[1..end_idx];
            let is_self_closing = tag_content.ends_with('/');
            let inner = if is_self_closing {
                tag_content[..tag_content.len() - 1].trim()
            } else {
                tag_content.trim()
            };

            let (name, attributes) = parse_tag_open_and_attrs(inner)?;
            tokens.push(RawXmlToken::StartTag {
                name,
                attributes,
                self_closing: is_self_closing,
            });
            rest = &rest[end_idx + 1..];
        } else {
            let next_tag = rest.find('<').unwrap_or(rest.len());
            let text = &rest[..next_tag];
            if !text.is_empty() {
                tokens.push(RawXmlToken::Text(unescape_xml_entities(text)));
            }
            rest = &rest[next_tag..];
        }
    }

    Ok(tokens)
}

fn find_tag_closing(slice: &str) -> Result<usize, CotValidationError> {
    let mut in_quote = None;
    for (i, c) in slice.char_indices().skip(1) {
        match c {
            '"' | '\'' => {
                if in_quote == Some(c) {
                    in_quote = None;
                } else if in_quote.is_none() {
                    in_quote = Some(c);
                }
            }
            '>' if in_quote.is_none() => return Ok(i),
            _ => {}
        }
    }
    Err(CotValidationError::MalformedXml(
        "unclosed tag < without matching >".into(),
    ))
}

fn parse_tag_open_and_attrs(
    inner: &str,
) -> Result<(String, Vec<XmlAttribute>), CotValidationError> {
    let mut parts = inner.split_whitespace();
    let name = parts
        .next()
        .ok_or_else(|| CotValidationError::MalformedXml("empty tag name".into()))?
        .to_string();

    let after_name = inner[name.len()..].trim_start();
    let attributes = parse_attributes_list(after_name, &name)?;
    Ok((name, attributes))
}

fn parse_attributes_list(
    mut s: &str,
    el_name: &str,
) -> Result<Vec<XmlAttribute>, CotValidationError> {
    let mut attrs = Vec::new();
    let mut seen_names = HashSet::new();

    while !s.is_empty() {
        s = s.trim_start();
        if s.is_empty() {
            break;
        }

        let eq_idx = match s.find('=') {
            Some(idx) => idx,
            None => {
                return Err(CotValidationError::MalformedXml(format!(
                    "attribute without '=' found on <{el_name}>: '{s}'"
                )));
            }
        };

        let attr_name = s[..eq_idx].trim();
        if attr_name.is_empty() {
            return Err(CotValidationError::MalformedXml(format!(
                "empty attribute name on <{el_name}>"
            )));
        }

        if !seen_names.insert(attr_name.to_string()) {
            return Err(CotValidationError::DuplicateAttribute {
                element: el_name.to_string(),
                attribute: attr_name.to_string(),
            });
        }

        let after_eq = s[eq_idx + 1..].trim_start();
        let quote_char = after_eq.chars().next().ok_or_else(|| {
            CotValidationError::MalformedXml(format!(
                "missing attribute value quote for '{attr_name}' on <{el_name}>"
            ))
        })?;

        if quote_char != '"' && quote_char != '\'' {
            return Err(CotValidationError::MalformedXml(format!(
                "unquoted attribute value for '{attr_name}' on <{el_name}>"
            )));
        }

        let value_start = quote_char.len_utf8();
        let end_quote = after_eq[value_start..].find(quote_char).ok_or_else(|| {
            CotValidationError::MalformedXml(format!(
                "unclosed attribute quote for '{attr_name}' on <{el_name}>"
            ))
        })? + value_start;

        let raw_val = &after_eq[value_start..end_quote];
        attrs.push(XmlAttribute {
            name: attr_name.to_string(),
            value: unescape_xml_entities(raw_val),
        });

        s = &after_eq[end_quote + quote_char.len_utf8()..];
    }

    Ok(attrs)
}

fn unescape_xml_entities(s: &str) -> String {
    crate::cot::unescape_xml_attr(s)
}

fn escape_xml_attr_val(s: &str) -> String {
    crate::cot::escape_xml_attr(s)
}

fn escape_xml_text(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 8);
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            other => out.push(other),
        }
    }
    out
}

fn parse_xml_to_tree(xml: &str, opts: &CanonicalizeOptions) -> Result<XmlNode, CotValidationError> {
    let tokens = tokenize_xml(xml)?;
    let mut stack: Vec<(String, Vec<XmlAttribute>, Vec<XmlNode>)> = Vec::new();
    let mut root: Option<XmlNode> = None;

    for tok in tokens {
        match tok {
            RawXmlToken::ProcessingInstruction => {}
            RawXmlToken::Comment(comment) => {
                if !opts.remove_comments {
                    if let Some((_, _, ref mut children)) = stack.last_mut() {
                        children.push(XmlNode::Comment(comment));
                    }
                }
            }
            RawXmlToken::Text(txt) => {
                let text_val = if opts.normalize_whitespace {
                    txt.trim().to_string()
                } else {
                    txt
                };
                if !text_val.is_empty() {
                    if let Some((_, _, ref mut children)) = stack.last_mut() {
                        children.push(XmlNode::Text(text_val));
                    }
                }
            }
            RawXmlToken::StartTag {
                name,
                attributes,
                self_closing,
            } => {
                if self_closing {
                    let node = XmlNode::Element {
                        name,
                        attributes,
                        children: Vec::new(),
                    };
                    if let Some((_, _, ref mut children)) = stack.last_mut() {
                        children.push(node);
                    } else if root.is_none() {
                        root = Some(node);
                    }
                } else {
                    if stack.len() >= CotSchemaValidator::MAX_DEPTH {
                        return Err(CotValidationError::MalformedXml(format!(
                            "element nesting depth exceeds limit of {}",
                            CotSchemaValidator::MAX_DEPTH
                        )));
                    }
                    stack.push((name, attributes, Vec::new()));
                }
            }
            RawXmlToken::EndTag { name } => {
                if let Some((open_name, attrs, children)) = stack.pop() {
                    if open_name != name {
                        return Err(CotValidationError::MismatchedClosingTag {
                            expected: open_name,
                            found: name,
                        });
                    }
                    let node = XmlNode::Element {
                        name: open_name,
                        attributes: attrs,
                        children,
                    };
                    if let Some((_, _, ref mut parent_children)) = stack.last_mut() {
                        parent_children.push(node);
                    } else if root.is_none() {
                        root = Some(node);
                    }
                }
            }
        }
    }

    root.ok_or(CotValidationError::MissingRequiredElement("event"))
}

fn serialize_node(
    node: &XmlNode,
    opts: &CanonicalizeOptions,
    active_ns: &mut Vec<HashMap<String, String>>,
    out: &mut String,
) {
    match node {
        XmlNode::Comment(comment) => {
            out.push_str("<!--");
            out.push_str(comment);
            out.push_str("-->");
        }
        XmlNode::Text(txt) => {
            out.push_str(&escape_xml_text(txt));
        }
        XmlNode::Element {
            name,
            attributes,
            children,
        } => {
            out.push('<');
            out.push_str(name);

            let mut scope_ns: HashMap<String, String> = HashMap::new();
            let mut ns_attrs: Vec<&XmlAttribute> = Vec::new();
            let mut standard_attrs: Vec<&XmlAttribute> = Vec::new();

            for attr in attributes {
                if attr.name == "xmlns" {
                    let prefix = "";
                    let is_redundant = opts.strip_redundant_namespaces
                        && is_ns_declared(active_ns, prefix, &attr.value);
                    if !is_redundant {
                        scope_ns.insert(prefix.to_string(), attr.value.clone());
                        ns_attrs.push(attr);
                    }
                } else if let Some(pfx) = attr.name.strip_prefix("xmlns:") {
                    let is_redundant = opts.strip_redundant_namespaces
                        && is_ns_declared(active_ns, pfx, &attr.value);
                    if !is_redundant {
                        scope_ns.insert(pfx.to_string(), attr.value.clone());
                        ns_attrs.push(attr);
                    }
                } else {
                    standard_attrs.push(attr);
                }
            }

            if opts.sort_attributes {
                ns_attrs.sort_by(|a, b| a.name.cmp(&b.name));
                standard_attrs.sort_by(|a, b| a.name.cmp(&b.name));
            }

            for a in ns_attrs {
                out.push(' ');
                out.push_str(&a.name);
                out.push_str("=\"");
                out.push_str(&escape_xml_attr_val(&a.value));
                out.push('"');
            }

            for a in standard_attrs {
                out.push(' ');
                out.push_str(&a.name);
                out.push_str("=\"");
                out.push_str(&escape_xml_attr_val(&a.value));
                out.push('"');
            }

            active_ns.push(scope_ns);

            if children.is_empty() {
                if opts.self_closing_tags {
                    out.push_str("/>");
                } else {
                    out.push_str("></");
                    out.push_str(name);
                    out.push('>');
                }
            } else {
                out.push('>');
                for child in children {
                    serialize_node(child, opts, active_ns, out);
                }
                out.push_str("</");
                out.push_str(name);
                out.push('>');
            }

            active_ns.pop();
        }
    }
}

fn is_ns_declared(active_ns: &[HashMap<String, String>], prefix: &str, uri: &str) -> bool {
    for scope in active_ns.iter().rev() {
        if let Some(existing_uri) = scope.get(prefix) {
            return existing_uri == uri;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_xml_rejected() {
        assert_eq!(validate_cot_xml(""), Err(CotValidationError::EmptyXml));
        assert_eq!(
            validate_cot_xml("   \n\t "),
            Err(CotValidationError::EmptyXml)
        );
    }

    #[test]
    fn test_non_event_root_rejected() {
        let xml = "<not_event><point lat=\"0.0\" lon=\"0.0\" hae=\"0.0\" ce=\"0.0\" le=\"0.0\"/></not_event>";
        assert!(matches!(
            validate_cot_xml(xml),
            Err(CotValidationError::MalformedXml(_))
        ));
    }

    #[test]
    fn test_entity_unescaping_and_escaping() {
        assert_eq!(
            unescape_xml_entities("&quot;&apos;&lt;&gt;&amp;&#65;&#x42;"),
            "\"'<>&AB"
        );
        assert_eq!(
            escape_xml_attr_val("\"hello & world\""),
            "&quot;hello &amp; world&quot;"
        );
    }

    #[test]
    fn test_cdata_and_processing_instruction() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
        <event version="2.0" uid="U1" type="a-f" how="m-g" time="2026-11-17T04:30:00.000Z" start="2026-11-17T04:30:00.000Z" stale="2026-11-17T04:35:00.000Z">
            <point lat="12.34" lon="-56.78" hae="100.0" ce="10.0" le="5.0"/>
            <detail><remarks><![CDATA[Tactical Note: Alpha & Bravo]]></remarks></detail>
        </event>"#;
        assert!(validate_cot_xml(xml).is_ok());
        let canon = canonicalize_cot_xml(xml).unwrap();
        assert!(canon.contains("Tactical Note: Alpha &amp; Bravo"));
    }

    #[test]
    fn test_canonicalize_non_self_closing() {
        let xml = r#"<event version="2.0" uid="U" type="a-f" how="m-g" time="2026-11-17T04:30:00.000Z" start="2026-11-17T04:30:00.000Z" stale="2026-11-17T04:35:00.000Z"><point lat="0.0" lon="0.0" hae="0.0" ce="0.0" le="0.0"/></event>"#;
        let opts = CanonicalizeOptions {
            self_closing_tags: false,
            ..Default::default()
        };
        let canon = CotXmlCanonicalizer::canonicalize_with_options(xml, &opts).unwrap();
        assert!(canon
            .contains("<point ce=\"0.0\" hae=\"0.0\" lat=\"0.0\" le=\"0.0\" lon=\"0.0\"></point>"));
    }

    #[test]
    fn test_invalid_type_characters() {
        assert_eq!(CotSchemaValidator::validate_cot_type("a-f-G-U-C"), Ok(()));
        assert!(CotSchemaValidator::validate_cot_type("a-f-G-U-C").is_ok());
        assert_eq!(CotSchemaValidator::validate_cot_type("a-f-G.1_2"), Ok(()));
        assert!(CotSchemaValidator::validate_cot_type("a-f-G.1_2").is_ok());
        assert_eq!(
            CotSchemaValidator::validate_cot_type("bad type with spaces"),
            Err(CotValidationError::InvalidCotType(
                "invalid character ' ' in CoT type 'bad type with spaces'".into()
            ))
        );
        assert!(CotSchemaValidator::validate_cot_type("bad type with spaces").is_err());
        assert_eq!(
            CotSchemaValidator::validate_cot_type("bad$type"),
            Err(CotValidationError::InvalidCotType(
                "invalid character '$' in CoT type 'bad$type'".into()
            ))
        );
        assert!(CotSchemaValidator::validate_cot_type("bad$type").is_err());
    }
}
