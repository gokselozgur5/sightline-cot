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
