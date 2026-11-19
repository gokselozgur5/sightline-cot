use sightline_cot::mil_std::{parse_mil_std, MilStdAffiliation};
use sightline_cot::sidc::{SidcAffiliation, TacticalSymbol};

const NEUTRAL_LAND_UNIT: &str = "10041000000000000000";

#[test]
fn mil_std_reads_identity_4_as_neutral() {
    let sym = parse_mil_std(NEUTRAL_LAND_UNIT).unwrap();
    assert_eq!(sym.affiliation, MilStdAffiliation::Neutral);
    assert_eq!(sym.to_cot_type(), "a-n-G");
}

#[test]
fn tactical_symbol_accepts_20_digit_code_and_reads_identity_4_as_neutral() {
    let sym = TacticalSymbol::from_sidc(NEUTRAL_LAND_UNIT).expect("2525D codes have 20 digits");
    assert_eq!(sym.affiliation, SidcAffiliation::Neutral);
}

#[test]
fn tactical_symbol_writes_neutral_as_identity_4() {
    let sym = TacticalSymbol {
        affiliation: SidcAffiliation::Neutral,
        ..Default::default()
    };
    assert_eq!(&sym.to_2525d().as_str()[2..4], "04");
}
