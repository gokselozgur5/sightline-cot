//! High-throughput zero-copy SAX (Simple API for XML) streaming parser for Cursor-on-Target.

use std::fmt;

/// Errors produced during SAX tokenization and CoT event extraction.
#[derive(Debug, Clone, PartialEq)]
pub enum SaxError {
    UnexpectedEof,
    InvalidSyntax(String),
    UnclosedElement(String),
    MismatchedTag { expected: String, found: String },
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
        let iter = self.iter();
        for a in iter.flatten() {
            if a.name == target_name {
                return Some(a.value);
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
            } else if self.input.as_bytes()[self.cursor..].starts_with(b"[CDATA[") {
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
        let ce_str = attrs.get("ce").unwrap_or("9999999.0");
        let le_str = attrs.get("le").unwrap_or("9999999.0");

        let lat: f64 = lat_str
            .parse()
            .map_err(|_| SaxError::InvalidNumericValue("lat"))?;
        let lon: f64 = lon_str
            .parse()
            .map_err(|_| SaxError::InvalidNumericValue("lon"))?;
        let hae: f64 = hae_str
            .parse()
            .map_err(|_| SaxError::InvalidNumericValue("hae"))?;
        let ce: f64 = ce_str
            .parse()
            .map_err(|_| SaxError::InvalidNumericValue("ce"))?;
        let le: f64 = le_str
            .parse()
            .map_err(|_| SaxError::InvalidNumericValue("le"))?;

        if !(-90.0..=90.0).contains(&lat)
            || !(-180.0..=180.0).contains(&lon)
            || ce.partial_cmp(&0.0) != Some(std::cmp::Ordering::Greater)
                && ce.partial_cmp(&0.0) != Some(std::cmp::Ordering::Equal)
            || le.partial_cmp(&0.0) != Some(std::cmp::Ordering::Greater)
                && le.partial_cmp(&0.0) != Some(std::cmp::Ordering::Equal)
            || !hae.is_finite()
        {
            return Err(SaxError::InvalidNumericValue("coordinate out of bounds"));
        }

        Ok(Self {
            lat,
            lon,
            hae,
            ce,
            le,
        })
    }
}

/// Zero-copy view into a Cursor-on-Target event parsed from an XML stream.
#[derive(Debug, Clone, PartialEq)]
pub struct CotEventView<'a> {
    pub version: &'a str,
    pub uid: &'a str,
    pub cot_type: &'a str,
    pub how: &'a str,
    pub time: &'a str,
    pub start: &'a str,
    pub stale: &'a str,
    pub point: Option<CotPointView>,
    pub callsign: Option<&'a str>,
}

/// Zero-copy CoT XML extractor using SAX stream processing.
pub struct CotSaxParser;

impl CotSaxParser {
    /// Parse all `<event>` elements from a zero-copy XML string slice.
    pub fn parse_events<'a>(xml: &'a str) -> Result<Vec<CotEventView<'a>>, SaxError> {
        let mut reader = SaxReader::new(xml);
        let mut events = Vec::new();
        let mut current_event: Option<CotEventView<'a>> = None;

        while let Some(ev) = reader.next_event()? {
            match ev {
                SaxEvent::EmptyElement {
                    name: "event",
                    attributes,
                }
                | SaxEvent::StartElement {
                    name: "event",
                    attributes,
                } => {
                    let version = attributes.get("version").unwrap_or("2.0");
                    let uid = attributes.get("uid").unwrap_or("");
                    let cot_type = attributes.get("type").unwrap_or("");
                    let how = attributes.get("how").unwrap_or("");
                    let time = attributes.get("time").unwrap_or("");
                    let start = attributes.get("start").unwrap_or("");
                    let stale = attributes.get("stale").unwrap_or("");

                    let evt = CotEventView {
                        version,
                        uid,
                        cot_type,
                        how,
                        time,
                        start,
                        stale,
                        point: None,
                        callsign: None,
                    };
                    if matches!(ev, SaxEvent::EmptyElement { .. }) {
                        events.push(evt);
                    } else {
                        current_event = Some(evt);
                    }
                }
                SaxEvent::EmptyElement { name, attributes }
                | SaxEvent::StartElement { name, attributes }
                    if name == "point" =>
                {
                    if let Some(ref mut evt) = current_event {
                        if let Ok(pt) = CotPointView::from_attributes(&attributes) {
                            evt.point = Some(pt);
                        }
                    }
                }
                SaxEvent::EmptyElement { name, attributes }
                | SaxEvent::StartElement { name, attributes }
                    if name == "contact" =>
                {
                    if let Some(ref mut evt) = current_event {
                        if let Some(cs) = attributes.get("callsign") {
                            evt.callsign = Some(cs);
                        }
                    }
                }
                SaxEvent::EndElement { name: "event" } => {
                    if let Some(evt) = current_event.take() {
                        events.push(evt);
                    }
                }
                _ => {}
            }
        }

        Ok(events)
    }
}

/// Default maximum size (1 MiB) for a single buffered CoT event.
pub const DEFAULT_MAX_EVENT_SIZE: usize = 1024 * 1024;

/// Buffer-based chunk streaming receiver for fragmented CoT data streams.
#[derive(Debug)]
pub struct CotChunkStreamer {
    buffer: Vec<u8>,
    max_event_size: usize,
}

impl Default for CotChunkStreamer {
    fn default() -> Self {
        Self::new()
    }
}

impl CotChunkStreamer {
    /// Create a new empty chunk streamer with the default size limit.
    pub fn new() -> Self {
        Self::with_max_event_size(DEFAULT_MAX_EVENT_SIZE)
    }

    /// Create a new chunk streamer with a custom maximum event size.
    pub fn with_max_event_size(max_event_size: usize) -> Self {
        Self {
            buffer: Vec::new(),
            max_event_size,
        }
    }

    /// Feed an incoming chunk of XML bytes into the streamer.
    ///
    /// Returns extracted complete CoT XML event strings.
    pub fn feed<B: AsRef<[u8]>>(&mut self, chunk: B) -> Vec<String> {
        self.buffer.extend_from_slice(chunk.as_ref());
        let mut completed = Vec::new();

        loop {
            let start_info = {
                let mut found = None;
                let mut i = 0;
                while i < self.buffer.len() {
                    let sub = &self.buffer[i..];
                    if let Some(pos) = sub.windows(6).position(|w| w == b"<event") {
                        let idx = i + pos;
                        let after = idx + 6;
                        if after < self.buffer.len() {
                            let b = self.buffer[after];
                            if b.is_ascii_whitespace() || b == b'/' || b == b'>' {
                                found = Some((idx, false));
                                break;
                            }
                            i = idx + 1;
                        } else {
                            found = Some((idx, true));
                            break;
                        }
                    } else {
                        break;
                    }
                }
                found
            };

            let (start_idx, partial) = match start_info {
                Some((idx, partial)) => (idx, partial),
                None => {
                    let prefixes: [&[u8]; 6] = [b"<event", b"<even", b"<eve", b"<ev", b"<e", b"<"];
                    let mut keep = 0;
                    for p in prefixes {
                        if self.buffer.ends_with(p) {
                            keep = p.len();
                            break;
                        }
                    }
                    if keep > 0 {
                        let drop = self.buffer.len() - keep;
                        self.buffer.drain(..drop);
                    } else {
                        self.buffer.clear();
                    }
                    break;
                }
            };

            if partial {
                if start_idx > 0 {
                    self.buffer.drain(..start_idx);
                }
                break;
            }

            let end_pattern = b"</event>";
            let end_idx = match self.buffer[start_idx..].iter().position(|&b| b == b'>') {
                Some(rel) if self.buffer[start_idx..start_idx + rel].ends_with(b"/") => {
                    start_idx + rel + 1
                }
                _ => {
                    let slice = &self.buffer[start_idx..];
                    match slice
                        .windows(end_pattern.len())
                        .position(|w| w == end_pattern)
                    {
                        Some(rel) => start_idx + rel + end_pattern.len(),
                        None => {
                            if start_idx > 0 {
                                self.buffer.drain(..start_idx);
                            }
                            if self.buffer.len() > self.max_event_size {
                                self.buffer
                                    .drain(..self.buffer.len() - (end_pattern.len() - 1));
                            }
                            break;
                        }
                    }
                }
            };

            if let Ok(event_xml) = std::str::from_utf8(&self.buffer[start_idx..end_idx]) {
                completed.push(event_xml.to_string());
            }
            self.buffer.drain(..end_idx);
        }

        completed
    }

    /// Retrieve the current pending buffer length.
    pub fn pending_len(&self) -> usize {
        self.buffer.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sax_tokens_basic() {
        let xml = "<root><child key=\"val\"/>text</root>";
        let mut reader = SaxReader::new(xml);

        assert_eq!(reader.next_event().unwrap(), Some(SaxEvent::StartDocument));
        match reader.next_event().unwrap().unwrap() {
            SaxEvent::StartElement { name, .. } => assert_eq!(name, "root"),
            other => panic!("expected root StartElement, got {other:?}"),
        }
        match reader.next_event().unwrap().unwrap() {
            SaxEvent::EmptyElement { name, attributes } => {
                assert_eq!(name, "child");
                assert_eq!(attributes.get("key"), Some("val"));
            }
            other => panic!("expected child EmptyElement, got {other:?}"),
        }
        match reader.next_event().unwrap().unwrap() {
            SaxEvent::Characters(text) => assert_eq!(text, "text"),
            other => panic!("expected Characters, got {other:?}"),
        }
        match reader.next_event().unwrap().unwrap() {
            SaxEvent::EndElement { name } => assert_eq!(name, "root"),
            other => panic!("expected root EndElement, got {other:?}"),
        }
        assert_eq!(reader.next_event().unwrap(), Some(SaxEvent::EndDocument));
        assert_eq!(reader.next_event().unwrap(), None);
    }

    #[test]
    fn test_attributes_iteration_and_get() {
        let raw = "foo=\"123\" bar='abc'   baz=\"xyz\"";
        let attrs = SaxAttributes::new(raw);
        assert_eq!(attrs.get("foo"), Some("123"));
        assert_eq!(attrs.get("bar"), Some("abc"));
        assert_eq!(attrs.get("baz"), Some("xyz"));
        assert_eq!(attrs.get("none"), None);

        let list: Result<Vec<_>, _> = attrs.iter().collect();
        let list = list.unwrap();
        assert_eq!(list.len(), 3);
        assert_eq!(list[0].name, "foo");
        assert_eq!(list[0].value, "123");
    }

    #[test]
    fn test_comment_cdata_and_processing_instruction() {
        let xml = "<?xml version=\"1.0\"?><!-- a comment --><![CDATA[raw content]]><node/>";
        let mut reader = SaxReader::new(xml);

        assert_eq!(reader.next_event().unwrap(), Some(SaxEvent::StartDocument));
        match reader.next_event().unwrap().unwrap() {
            SaxEvent::ProcessingInstruction { target, data } => {
                assert_eq!(target, "xml");
                assert_eq!(data, "version=\"1.0\"");
            }
            other => panic!("expected PI, got {other:?}"),
        }
        match reader.next_event().unwrap().unwrap() {
            SaxEvent::Comment(text) => assert_eq!(text, " a comment "),
            other => panic!("expected Comment, got {other:?}"),
        }
        match reader.next_event().unwrap().unwrap() {
            SaxEvent::CData(content) => assert_eq!(content, "raw content"),
            other => panic!("expected CData, got {other:?}"),
        }
        match reader.next_event().unwrap().unwrap() {
            SaxEvent::EmptyElement { name, .. } => assert_eq!(name, "node"),
            other => panic!("expected node EmptyElement, got {other:?}"),
        }
    }

    #[test]
    fn test_cot_sax_parser_zero_copy() {
        let xml = r#"<event version="2.0" uid="ALPHA-1" type="a-f-G-U-C" how="m-g" time="2026-02-16T12:00:00.000Z" start="2026-02-16T12:00:00.000Z" stale="2026-02-16T12:05:00.000Z">
            <point lat="34.0522" lon="-118.2437" hae="120.5" ce="15.0" le="25.0"/>
            <detail>
                <contact callsign="VIPER" />
            </detail>
        </event>"#;

        let events = CotSaxParser::parse_events(xml).expect("parse failed");
        assert_eq!(events.len(), 1);
        let evt = &events[0];
        assert_eq!(evt.uid, "ALPHA-1");
        assert_eq!(evt.cot_type, "a-f-G-U-C");
        assert_eq!(evt.how, "m-g");
        assert_eq!(evt.callsign, Some("VIPER"));

        let pt = evt.point.expect("missing point");
        assert!((pt.lat - 34.0522).abs() < 1e-5);
        assert!((pt.lon - -118.2437).abs() < 1e-5);
        assert!((pt.hae - 120.5).abs() < 1e-5);
    }

    #[test]
    fn test_mismatched_and_unclosed_tags() {
        let xml = "<a><b></c></a>";
        let mut reader = SaxReader::new(xml);
        assert_eq!(reader.next_event().unwrap(), Some(SaxEvent::StartDocument));
        let _ = reader.next_event().unwrap(); // <a>
        let _ = reader.next_event().unwrap(); // <b>
        match reader.next_event() {
            Err(SaxError::MismatchedTag { expected, found }) => {
                assert_eq!(expected, "b");
                assert_eq!(found, "c");
            }
            other => panic!("expected MismatchedTag, got {other:?}"),
        }

        let unclosed = "<start><nested>";
        let mut reader2 = SaxReader::new(unclosed);
        assert_eq!(reader2.next_event().unwrap(), Some(SaxEvent::StartDocument));
        let _ = reader2.next_event().unwrap(); // <start>
        let _ = reader2.next_event().unwrap(); // <nested>
        match reader2.next_event() {
            Err(SaxError::UnclosedElement(el)) => assert_eq!(el, "nested"),
            other => panic!("expected UnclosedElement, got {other:?}"),
        }
    }

    #[test]
    fn test_cot_chunk_streamer() {
        let mut streamer = CotChunkStreamer::new();
        let chunk1 = "<event uid=\"1\"><point lat=\"10.0\" lon=\"20.0\"/>";
        let chunk2 = "</event><event uid=\"2\"><point lat=\"30.0\" ";
        let chunk3 = "lon=\"40.0\"/></event>";

        let res1 = streamer.feed(chunk1);
        assert_eq!(res1.len(), 0);

        let res2 = streamer.feed(chunk2);
        assert_eq!(res2.len(), 1);
        assert!(res2[0].contains("uid=\"1\""));

        let res3 = streamer.feed(chunk3);
        assert_eq!(res3.len(), 1);
        assert!(res3[0].contains("uid=\"2\""));
        assert_eq!(streamer.pending_len(), 0);
    }
}
