use sightline_cot::mil_std::{parse_mil_std, MilStdAffiliation, MilStdDimension};

#[test]
fn test_mil_std_2525c_basic_parse() {
    let sym = parse_mil_std("SFGPUCI----K---").expect("parse 2525c");
    assert_eq!(sym.affiliation, MilStdAffiliation::Friend);
    assert_eq!(sym.dimension, MilStdDimension::Land);
}

#[test]
fn test_mil_std_2525d_basic_parse() {
    let sym = parse_mil_std("10031000001201000000").expect("parse 2525d");
    assert_eq!(sym.affiliation, MilStdAffiliation::Friend);
    assert_eq!(sym.dimension, MilStdDimension::Land);
}

#[test]
fn test_mil_std_2525c_roundtrip() {
    let original = "SFGPUCI----K---";
    let sym = parse_mil_std(original).expect("parse");
    let serialized = sym.to_2525c_string();
    assert_eq!(serialized, original);
}

#[test]
fn test_mil_std_2525d_echelon_and_cot() {
    let sym = parse_mil_std("10031000150000000000").expect("parse company");
    assert_eq!(sym.echelon, MilStdEchelon::Company);
    assert_eq!(sym.to_cot_type(), "a-f-G");
}

#[test]
fn test_mil_std_errors() {
    assert_eq!(parse_mil_std("SHORT"), Err(MilStdError::InvalidLength));
    assert_eq!(parse_mil_std("SFGPUCI----K--\u{00FF}"), Err(MilStdError::InvalidCharacter));
    assert_eq!(parse_mil_std("SZGPUCI----K---"), Err(MilStdError::InvalidAffiliation));
    assert_eq!(parse_mil_std("SFZPUCI----K---"), Err(MilStdError::InvalidDimension));
}
