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

/// Group affiliation detail element in CoT 2.0 XML.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CotGroup {
    pub name: String,
    pub role: String,
}

impl CotGroup {
    /// Create a new CotGroup element.
    pub fn new(name: impl Into<String>, role: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            role: role.into(),
        }
    }

    /// Render group detail as XML snippet.
    pub fn to_xml(&self) -> String {
        format!(
            r#"<__group name="{}" role="{}"/>"#,
            escape_xml_attr(&self.name),
            escape_xml_attr(&self.role)
        )
    }
}

/// CoT detail payload combining contact and group elements.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CotDetailPayload {
    pub contact: Option<CotContact>,
    pub group: Option<CotGroup>,
}

impl CotDetailPayload {
    /// Create an empty detail container.
    pub fn new() -> Self {
        Self::default()
    }

    /// Attach contact.
    pub fn with_contact(mut self, contact: CotContact) -> Self {
        self.contact = Some(contact);
        self
    }

    /// Attach group.
    pub fn with_group(mut self, group: CotGroup) -> Self {
        self.group = Some(group);
        self
    }
}
