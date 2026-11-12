//! MIL-STD-2525 and CoT 2.0 type classification engine.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TacticalAffiliation {
    Friendly,
    Hostile,
    Neutral,
    #[default]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TargetCategory {
    Vehicle,
    Person,
    Drone,
    Aircraft,
    Vessel,
    #[default]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MilStdClassifier {
    pub affiliation: TacticalAffiliation,
    pub category: TargetCategory,
}

impl MilStdClassifier {
    pub const fn new(affiliation: TacticalAffiliation, category: TargetCategory) -> Self {
        Self {
            affiliation,
            category,
        }
    }

    pub fn to_cot_type(&self) -> &'static str {
        match (self.affiliation, self.category) {
            (TacticalAffiliation::Friendly, TargetCategory::Vehicle) => "a-f-G-E-V-C",
            (TacticalAffiliation::Hostile, TargetCategory::Vehicle) => "a-h-G-E-V",
            (TacticalAffiliation::Friendly, TargetCategory::Person) => "a-f-G-E-S",
            (TacticalAffiliation::Hostile, TargetCategory::Person) => "a-h-G-E-S",
            (TacticalAffiliation::Friendly, TargetCategory::Drone) => "a-f-A-M-F",
            (TacticalAffiliation::Hostile, TargetCategory::Drone) => "a-h-A-M-F",
            (TacticalAffiliation::Friendly, TargetCategory::Aircraft) => "a-f-A",
            (TacticalAffiliation::Hostile, TargetCategory::Aircraft) => "a-h-A",
            (TacticalAffiliation::Friendly, TargetCategory::Vessel) => "a-f-S",
            (TacticalAffiliation::Hostile, TargetCategory::Vessel) => "a-h-S",
            (TacticalAffiliation::Neutral, TargetCategory::Vehicle) => "a-n-G-E-V",
            (TacticalAffiliation::Neutral, TargetCategory::Person) => "a-n-G-E-S",
            (TacticalAffiliation::Neutral, _) => "a-n-G",
            _ => "a-u-G",
        }
    }
}

fn match_category(s: &str) -> TargetCategory {
    if s.eq_ignore_ascii_case("car") || s.eq_ignore_ascii_case("truck") {
        TargetCategory::Vehicle
    } else if s.eq_ignore_ascii_case("person") {
        TargetCategory::Person
    } else if s.eq_ignore_ascii_case("drone") || s.eq_ignore_ascii_case("uav") {
        TargetCategory::Drone
    } else if s.eq_ignore_ascii_case("boat") || s.eq_ignore_ascii_case("vessel") {
        TargetCategory::Vessel
    } else {
        TargetCategory::Unknown
    }
}

impl MilStdClassifier {
    pub fn from_label(label: &str) -> Self {
        Self {
            affiliation: TacticalAffiliation::Unknown,
            category: match_category(label.trim()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_milstd_classification_mapping() {
        let friendly_veh =
            MilStdClassifier::new(TacticalAffiliation::Friendly, TargetCategory::Vehicle);
        assert_eq!(friendly_veh.to_cot_type(), "a-f-G-E-V-C");
        let hostile_veh =
            MilStdClassifier::new(TacticalAffiliation::Hostile, TargetCategory::Vehicle);
        assert_eq!(hostile_veh.to_cot_type(), "a-h-G-E-V");
        assert_eq!(MilStdClassifier::default().to_cot_type(), "a-u-G");
    }

    #[test]
    fn test_fuzzy_label_parsing_and_fallback() {
        assert_eq!(
            MilStdClassifier::from_label("car").category,
            TargetCategory::Vehicle
        );
        assert_eq!(
            MilStdClassifier::from_label("person").category,
            TargetCategory::Person
        );
        assert_eq!(
            MilStdClassifier::from_label("unknown").category,
            TargetCategory::Unknown
        );
    }
}
