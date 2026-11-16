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
