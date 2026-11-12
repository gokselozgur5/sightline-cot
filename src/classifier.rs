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
