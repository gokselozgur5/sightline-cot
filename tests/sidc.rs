use sightline_cot::mil_std::{parse_mil_std, MilStdAffiliation, MilStdDimension};
use sightline_cot::sidc::{SidcAffiliation, SidcDomain, TacticalSymbol};
use sightline_cot::sidc_parser::{parse_sidc2525d, StandardIdentity, SymbolSet};

#[test]
fn test_sidc_implementations_agree_on_neutral_land_unit() {
    let sidc = "10041000000000000000";

    let mil = parse_mil_std(sidc).expect("parse_mil_std");
    let parser = parse_sidc2525d(sidc).expect("parse_sidc2525d");
    let sym = TacticalSymbol::from_sidc(sidc).expect("TacticalSymbol::from_sidc");

    assert_eq!(mil.affiliation, MilStdAffiliation::Neutral);
    assert_eq!(mil.dimension, MilStdDimension::Land);
    assert_eq!(parser.identity, StandardIdentity::Neutral);
    assert_eq!(parser.symbol_set, SymbolSet::LandUnit);
    assert_eq!(sym.affiliation, SidcAffiliation::Neutral);
    assert_eq!(sym.domain, SidcDomain::Land);

    assert_eq!(mil.to_cot_type(), "a-n-G");
    assert_eq!(parser.to_cot_type(), "a-n-G");
    assert_eq!(sym.to_cot_type(), "a-n-G");
}

#[test]
fn test_sidc_implementations_agree_on_suspect_and_pending_cot_letters() {
    let suspect = "10051000000000000000";
    let mil_suspect = parse_mil_std(suspect).expect("parse suspect mil");
    let parser_suspect = parse_sidc2525d(suspect).expect("parse suspect 2525d");
    let sym_suspect = TacticalSymbol::from_sidc(suspect).expect("parse suspect tactical");

    assert_eq!(mil_suspect.to_cot_type(), "a-s-G");
    assert_eq!(parser_suspect.to_cot_type(), "a-s-G");
    assert_eq!(sym_suspect.to_cot_type(), "a-s-G");

    let pending = "10001000000000000000";
    let mil_pending = parse_mil_std(pending).expect("parse pending mil");
    let parser_pending = parse_sidc2525d(pending).expect("parse pending 2525d");
    let sym_pending = TacticalSymbol::from_sidc(pending).expect("parse pending tactical");

    assert_eq!(mil_pending.to_cot_type(), "a-p-G");
    assert_eq!(parser_pending.to_cot_type(), "a-p-G");
    assert_eq!(sym_pending.to_cot_type(), "a-p-G");
}
