//! MIL-STD-2525 and CoT 2.0 type classification engine.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TacticalAffiliation {
    Friendly,
    Hostile,
    Neutral,
    #[default]
    Unknown,
}
