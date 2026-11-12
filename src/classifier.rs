//! MIL-STD-2525 / Hierarchical CoT type classification.
//!
//! Provides zero-alloc tactical affiliation, category, and standard
//! MIL-STD-2525 / CoT 2.0 type string mapping with fuzzy label parsing.

/// Standard military tactical affiliation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TacticalAffiliation {
    Friendly,
    Hostile,
    Neutral,
    #[default]
    Unknown,
}

/// Operational target category according to MIL-STD-2525 / CoT hierarchy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TargetCategory {
    Vehicle,
    Person,
    Drone,
    Aircraft,
    Vessel,
    #[default]
    Unknown,
}

impl TacticalAffiliation {
    /// Returns the single-letter CoT affiliation code character.
    #[inline]
    pub const fn cot_char(&self) -> char {
        match self {
            Self::Friendly => 'f',
            Self::Hostile => 'h',
            Self::Neutral => 'n',
            Self::Unknown => 'u',
        }
    }
}

/// Tactical classification pairing affiliation and category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TacticalClassification {
    pub affiliation: TacticalAffiliation,
    pub category: TargetCategory,
}

impl TacticalClassification {
    /// Creates a new tactical classification pair.
    #[inline]
    pub const fn new(affiliation: TacticalAffiliation, category: TargetCategory) -> Self {
        Self { affiliation, category }
    }
}

impl TargetCategory {
    /// Returns the ground/air/sea CoT domain component string.
    #[inline]
    pub const fn cot_domain(&self) -> &'static str {
        match self {
            Self::Vehicle => "G-E-V-C",
            Self::Person => "G-U-C",
            Self::Drone => "A-M-F-Q",
            Self::Aircraft => "A",
            Self::Vessel => "S",
            Self::Unknown => "G",
        }
    }
}

impl TacticalClassification {
    const fn cot_type_friendly_hostile(&self) -> Option<&'static str> {
        match (self.affiliation, self.category) {
            (TacticalAffiliation::Friendly, TargetCategory::Vehicle) => Some("a-f-G-E-V-C"),
            (TacticalAffiliation::Friendly, TargetCategory::Person) => Some("a-f-G-U-C"),
            (TacticalAffiliation::Friendly, TargetCategory::Drone) => Some("a-f-A-M-F-Q"),
            (TacticalAffiliation::Friendly, TargetCategory::Aircraft) => Some("a-f-A"),
            (TacticalAffiliation::Friendly, TargetCategory::Vessel) => Some("a-f-S"),
            (TacticalAffiliation::Friendly, TargetCategory::Unknown) => Some("a-f-G"),
            (TacticalAffiliation::Hostile, TargetCategory::Vehicle) => Some("a-h-G-E-V-C"),
            (TacticalAffiliation::Hostile, TargetCategory::Person) => Some("a-h-G-U-C"),
            (TacticalAffiliation::Hostile, TargetCategory::Drone) => Some("a-h-A-M-F-Q"),
            (TacticalAffiliation::Hostile, TargetCategory::Aircraft) => Some("a-h-A"),
            (TacticalAffiliation::Hostile, TargetCategory::Vessel) => Some("a-h-S"),
            (TacticalAffiliation::Hostile, TargetCategory::Unknown) => Some("a-h-G"),
            _ => None,
        }
    }
}

impl TacticalClassification {
    const fn cot_type_neutral_unknown(&self) -> &'static str {
        match (self.affiliation, self.category) {
            (TacticalAffiliation::Neutral, TargetCategory::Vehicle) => "a-n-G-E-V-C",
            (TacticalAffiliation::Neutral, TargetCategory::Person) => "a-n-G-U-C",
            (TacticalAffiliation::Neutral, TargetCategory::Drone) => "a-n-A-M-F-Q",
            (TacticalAffiliation::Neutral, TargetCategory::Aircraft) => "a-n-A",
            (TacticalAffiliation::Neutral, TargetCategory::Vessel) => "a-n-S",
            (TacticalAffiliation::Neutral, TargetCategory::Unknown) => "a-n-G",
            (TacticalAffiliation::Unknown, TargetCategory::Vehicle) => "a-u-G-E-V-C",
            (TacticalAffiliation::Unknown, TargetCategory::Person) => "a-u-G-U-C",
            (TacticalAffiliation::Unknown, TargetCategory::Drone) => "a-u-A-M-F-Q",
            (TacticalAffiliation::Unknown, TargetCategory::Aircraft) => "a-u-A",
            (TacticalAffiliation::Unknown, TargetCategory::Vessel) => "a-u-S",
            _ => "a-u-G",
        }
    }
}

impl TacticalClassification {
    /// Generates canonical MIL-STD-2525 / CoT 2.0 type string.
    pub const fn to_cot_type(&self) -> &'static str {
        if let Some(t) = self.cot_type_friendly_hostile() {
            t
        } else {
            self.cot_type_neutral_unknown()
        }
    }
}

impl TargetCategory {
    const fn match_ground_label(s: &str) -> Option<Self> {
        let b = s.as_bytes();
        match b {
            b"car" | b"truck" | b"bus" | b"vehicle" | b"tank" | b"apc" => Some(Self::Vehicle),
            b"person" | b"pedestrian" | b"soldier" | b"infantry" => Some(Self::Person),
            _ => None,
        }
    }
}
