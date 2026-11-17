//! High-throughput zero-copy SAX (Simple API for XML) streaming parser for Cursor-on-Target.

use std::fmt;

/// Errors produced during SAX tokenization and CoT event extraction.
#[derive(Debug, Clone, PartialEq)]
pub enum SaxError {
    UnexpectedEof,
    InvalidSyntax(String),
    UnclosedElement(String),
    MismatchedTag { expected: String, found: String },
    InvalidEntity(String),
    InvalidNumericValue(&'static str),
}

impl fmt::Display for SaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedEof => write!(f, "unexpected end of XML stream"),
            Self::InvalidSyntax(s) => write!(f, "invalid XML syntax: {s}"),
            Self::UnclosedElement(tag) => write!(f, "unclosed XML tag: <{tag}>"),
            Self::MismatchedTag { expected, found } => {
                write!(
                    f,
                    "mismatched closing tag: expected </{expected}>, found </{found}>"
                )
            }
            Self::InvalidEntity(e) => write!(f, "invalid XML entity: {e}"),
            Self::InvalidNumericValue(v) => write!(f, "invalid numeric value for field: {v}"),
        }
    }
}

impl std::error::Error for SaxError {}

/// Single XML attribute borrowed directly from the input stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SaxAttribute<'a> {
    pub name: &'a str,
    pub value: &'a str,
}

/// Zero-copy iterator and lookup over an element's attribute string.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SaxAttributes<'a> {
    raw: &'a str,
}

impl<'a> SaxAttributes<'a> {
    /// Construct a new attribute parser from the raw attribute string slice.
    pub fn new(raw: &'a str) -> Self {
        Self { raw }
    }

    /// Retrieve the raw attribute slice.
    pub fn as_raw(&self) -> &'a str {
        self.raw
    }

    /// Lookup an attribute value by name.
    pub fn get(&self, target_name: &str) -> Option<&'a str> {
        let mut iter = self.iter();
        while let Some(attr) = iter.next() {
            if let Ok(a) = attr {
                if a.name == target_name {
                    return Some(a.value);
                }
            }
        }
        None
    }

    /// Create an iterator over all attributes.
    pub fn iter(&self) -> SaxAttributesIter<'a> {
        SaxAttributesIter {
            input: self.raw,
            cursor: 0,
        }
    }
}

/// Iterator over raw XML attributes.
#[derive(Debug, Clone)]
pub struct SaxAttributesIter<'a> {
    input: &'a str,
    cursor: usize,
}

impl<'a> Iterator for SaxAttributesIter<'a> {
    type Item = Result<SaxAttribute<'a>, SaxError>;

    fn next(&mut self) -> Option<Self::Item> {
        let bytes = self.input.as_bytes();
        let len = bytes.len();

        // Skip whitespace
        while self.cursor < len && bytes[self.cursor].is_ascii_whitespace() {
            self.cursor += 1;
        }

        if self.cursor >= len {
            return None;
        }

        // Attribute name
        let name_start = self.cursor;
        while self.cursor < len
            && !bytes[self.cursor].is_ascii_whitespace()
            && bytes[self.cursor] != b'='
            && bytes[self.cursor] != b'>'
            && bytes[self.cursor] != b'/'
        {
            self.cursor += 1;
        }

        let name = &self.input[name_start..self.cursor];
        if name.is_empty() {
            return None;
        }

        // Skip whitespace before '='
        while self.cursor < len && bytes[self.cursor].is_ascii_whitespace() {
            self.cursor += 1;
        }

        if self.cursor >= len || bytes[self.cursor] != b'=' {
            return Some(Err(SaxError::InvalidSyntax(format!(
                "missing '=' after attribute '{name}'"
            ))));
        }
        self.cursor += 1; // Consume '='

        // Skip whitespace after '='
        while self.cursor < len && bytes[self.cursor].is_ascii_whitespace() {
            self.cursor += 1;
        }

        if self.cursor >= len {
            return Some(Err(SaxError::UnexpectedEof));
        }

        let quote = bytes[self.cursor];
        if quote != b'"' && quote != b'\'' {
            return Some(Err(SaxError::InvalidSyntax(format!(
                "attribute '{name}' value not quoted"
            ))));
        }
        self.cursor += 1; // Consume opening quote

        let val_start = self.cursor;
        while self.cursor < len && bytes[self.cursor] != quote {
            self.cursor += 1;
        }

        if self.cursor >= len {
            return Some(Err(SaxError::UnexpectedEof));
        }

        let val = &self.input[val_start..self.cursor];
        self.cursor += 1; // Consume closing quote

        Some(Ok(SaxAttribute { name, value: val }))
    }
}

/// Token events emitted by `SaxReader`.
#[derive(Debug, Clone, PartialEq)]
pub enum SaxEvent<'a> {
    StartDocument,
    EndDocument,
    StartElement {
        name: &'a str,
        attributes: SaxAttributes<'a>,
    },
    EndElement {
        name: &'a str,
    },
    EmptyElement {
        name: &'a str,
        attributes: SaxAttributes<'a>,
    },
    Characters(&'a str),
    Comment(&'a str),
    ProcessingInstruction {
        target: &'a str,
        data: &'a str,
    },
    CData(&'a str),
}

/// Zero-copy streaming SAX parser over a borrowed XML string slice.
#[derive(Debug, Clone)]
pub struct SaxReader<'a> {
    input: &'a str,
    cursor: usize,
    tag_stack: Vec<&'a str>,
    started: bool,
    ended: bool,
}

impl<'a> SaxReader<'a> {
    /// Create a new SAX reader over an input string slice.
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            cursor: 0,
            tag_stack: Vec::new(),
            started: false,
            ended: false,
        }
    }

    /// Current cursor byte position in the stream.
    pub fn position(&self) -> usize {
        self.cursor
    }

    /// Read the next SAX event from the stream.
    pub fn next_event(&mut self) -> Result<Option<SaxEvent<'a>>, SaxError> {
        if !self.started {
            self.started = true;
            return Ok(Some(SaxEvent::StartDocument));
        }

        if self.ended {
            return Ok(None);
        }

        let bytes = self.input.as_bytes();
        let len = bytes.len();

        if self.cursor >= len {
            if let Some(unclosed) = self.tag_stack.pop() {
                return Err(SaxError::UnclosedElement(unclosed.to_string()));
            }
            self.ended = true;
            return Ok(Some(SaxEvent::EndDocument));
        }

        if bytes[self.cursor] == b'<' {
            self.parse_tag_or_markup()
        } else {
            self.parse_characters()
        }
    }

    fn parse_characters(&mut self) -> Result<Option<SaxEvent<'a>>, SaxError> {
        let start = self.cursor;
        let bytes = self.input.as_bytes();
        let len = bytes.len();

        while self.cursor < len && bytes[self.cursor] != b'<' {
            self.cursor += 1;
        }

        let text = &self.input[start..self.cursor];
        Ok(Some(SaxEvent::Characters(text)))
    }

    fn parse_tag_or_markup(&mut self) -> Result<Option<SaxEvent<'a>>, SaxError> {
        let bytes = self.input.as_bytes();
        let len = bytes.len();

        // Must start with '<'
        self.cursor += 1;
        if self.cursor >= len {
            return Err(SaxError::UnexpectedEof);
        }

        // Check for comment <!-- or CDATA <![CDATA[
        if bytes[self.cursor] == b'!' {
            self.cursor += 1;
            if self.cursor + 1 < len && bytes[self.cursor] == b'-' && bytes[self.cursor + 1] == b'-'
            {
                // Comment
                self.cursor += 2;
                let start = self.cursor;
                if let Some(end_offset) = self.input[self.cursor..].find("-->") {
                    let comment_text = &self.input[start..self.cursor + end_offset];
                    self.cursor += end_offset + 3;
                    return Ok(Some(SaxEvent::Comment(comment_text)));
                } else {
                    return Err(SaxError::UnexpectedEof);
                }
            } else if self.cursor + 6 < len
                && &self.input[self.cursor..self.cursor + 7] == "[CDATA["
            {
                // CDATA
                self.cursor += 7;
                let start = self.cursor;
                if let Some(end_offset) = self.input[self.cursor..].find("]]>") {
                    let cdata_text = &self.input[start..self.cursor + end_offset];
                    self.cursor += end_offset + 3;
                    return Ok(Some(SaxEvent::CData(cdata_text)));
                } else {
                    return Err(SaxError::UnexpectedEof);
                }
            } else {
                return Err(SaxError::InvalidSyntax(
                    "malformed markup declaration".to_string(),
                ));
            }
        }

        // Check for Processing Instruction <? ... ?>
        if bytes[self.cursor] == b'?' {
            self.cursor += 1;
            let start = self.cursor;
            if let Some(end_offset) = self.input[self.cursor..].find("?>") {
                let pi_content = &self.input[start..self.cursor + end_offset];
                self.cursor += end_offset + 2;
                let mut parts = pi_content.splitn(2, |c: char| c.is_ascii_whitespace());
                let target = parts.next().unwrap_or("").trim();
                let data = parts.next().unwrap_or("").trim();
                return Ok(Some(SaxEvent::ProcessingInstruction { target, data }));
            } else {
                return Err(SaxError::UnexpectedEof);
            }
        }

        // Check for closing tag </tag>
        if bytes[self.cursor] == b'/' {
            self.cursor += 1;
            let start = self.cursor;
            while self.cursor < len
                && bytes[self.cursor] != b'>'
                && !bytes[self.cursor].is_ascii_whitespace()
            {
                self.cursor += 1;
            }
            let name = &self.input[start..self.cursor];
            while self.cursor < len && bytes[self.cursor] != b'>' {
                self.cursor += 1;
            }
            if self.cursor >= len {
                return Err(SaxError::UnexpectedEof);
            }
            self.cursor += 1; // Consume '>'

            if let Some(expected) = self.tag_stack.pop() {
                if expected != name {
                    return Err(SaxError::MismatchedTag {
                        expected: expected.to_string(),
                        found: name.to_string(),
                    });
                }
            } else {
                return Err(SaxError::InvalidSyntax(format!(
                    "unexpected closing tag </{name}>"
                )));
            }

            return Ok(Some(SaxEvent::EndElement { name }));
        }

        // Start tag or self-closing tag <tag ... > or <tag ... />
        let name_start = self.cursor;
        while self.cursor < len
            && !bytes[self.cursor].is_ascii_whitespace()
            && bytes[self.cursor] != b'>'
            && bytes[self.cursor] != b'/'
        {
            self.cursor += 1;
        }

        let name = &self.input[name_start..self.cursor];
        if name.is_empty() {
            return Err(SaxError::InvalidSyntax("empty tag name".to_string()));
        }

        let attr_start = self.cursor;
        let mut self_closing = false;

        while self.cursor < len {
            if bytes[self.cursor] == b'/' && self.cursor + 1 < len && bytes[self.cursor + 1] == b'>'
            {
                self_closing = true;
                break;
            } else if bytes[self.cursor] == b'>' {
                break;
            } else if bytes[self.cursor] == b'"' || bytes[self.cursor] == b'\'' {
                let q = bytes[self.cursor];
                self.cursor += 1;
                while self.cursor < len && bytes[self.cursor] != q {
                    self.cursor += 1;
                }
                if self.cursor < len {
                    self.cursor += 1;
                }
            } else {
                self.cursor += 1;
            }
        }

        if self.cursor >= len {
            return Err(SaxError::UnexpectedEof);
        }

        let raw_attrs = &self.input[attr_start..self.cursor].trim();
        let attributes = SaxAttributes::new(raw_attrs);

        if self_closing {
            self.cursor += 2; // Consume '/>'
            Ok(Some(SaxEvent::EmptyElement { name, attributes }))
        } else {
            self.cursor += 1; // Consume '>'
            self.tag_stack.push(name);
            Ok(Some(SaxEvent::StartElement { name, attributes }))
        }
    }
}

/// Zero-copy point data extracted from a `<point>` element.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CotPointView {
    pub lat: f64,
    pub lon: f64,
    pub hae: f64,
    pub ce: f64,
    pub le: f64,
}

impl CotPointView {
    /// Parse from SaxAttributes of a point element.
    pub fn from_attributes(attrs: &SaxAttributes<'_>) -> Result<Self, SaxError> {
        let lat_str = attrs
            .get("lat")
            .ok_or(SaxError::InvalidNumericValue("lat"))?;
        let lon_str = attrs
            .get("lon")
            .ok_or(SaxError::InvalidNumericValue("lon"))?;
        let hae_str = attrs.get("hae").unwrap_or("0.0");
