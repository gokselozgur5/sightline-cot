//! CoT XML detail formatter.

use crate::cot::escape_xml_attr;

/// Contact detail schema element in CoT 2.0 XML.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CotContact {
    pub callsign: String,
    pub endpoint: Option<String>,
}

impl CotContact {
    /// Create a new CotContact element.
    pub fn new(callsign: impl Into<String>) -> Self {
        Self {
            callsign: callsign.into(),
            endpoint: None,
        }
    }

    /// Set contact endpoint.
    pub fn with_endpoint(mut self, ep: impl Into<String>) -> Self {
        self.endpoint = Some(ep.into());
        self
    }
}

impl CotContact {
    /// Render contact detail as XML snippet.
    pub fn to_xml(&self) -> String {
        let cs = escape_xml_attr(&self.callsign);
        match &self.endpoint {
            Some(ep) => format!(
                r#"<contact callsign="{cs}" endpoint="{}"/>"#,
                escape_xml_attr(ep)
            ),
            None => format!(r#"<contact callsign="{cs}"/>"#),
        }
    }
}
