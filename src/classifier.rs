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
