use sightline_cot::sidc_parser::{
    cot_type_to_sidc20, generate_sidc2525d, parse_sidc2525d, sidc20_to_cot_type, Echelon,
    MilStd2525DSymbol, SidcParseError, StandardIdentity, SymbolSet, SymbolStatus,
};

#[test]
fn test_default_symbol_generation_and_parsing() {
    let sym = MilStd2525DSymbol::default();
    let sidc = generate_sidc2525d(&sym);
    assert_eq!(sidc, "10031000000000000000");
    let parsed = parse_sidc2525d(&sidc).expect("should parse default SIDC");
    assert_eq!(parsed.version, 10);
    assert_eq!(parsed.identity, StandardIdentity::Friend);
    assert_eq!(parsed.symbol_set, SymbolSet::LandUnit);
    assert_eq!(parsed.status, SymbolStatus::Present);
}

#[test]
fn test_custom_fields_and_30_digit_support() {
    let sym = MilStd2525DSymbol {
        identity: StandardIdentity::Hostile,
        symbol_set: SymbolSet::Air,
        status: SymbolStatus::Planned,
        echelon: Echelon::Battalion,
        entity_code: 123456,
        modifier1: 7,
        modifier2: 8,
        ..Default::default()
    };

    let sidc = sym.to_sidc20();
    assert_eq!(sidc, "10060110161234560708");

    let parsed = MilStd2525DSymbol::parse(&sidc).unwrap();
    assert_eq!(parsed, sym);

    let sidc30 = format!("{sidc}0000000000");
    let parsed30 = parse_sidc2525d(&sidc30).unwrap();
    assert_eq!(parsed30, sym);
}

#[test]
fn test_error_handling_on_invalid_sidc() {
    assert_eq!(parse_sidc2525d("123"), Err(SidcParseError::InvalidLength));
    assert_eq!(
        parse_sidc2525d("1003100000000000000A"),
        Err(SidcParseError::NonNumeric)
    );
    assert_eq!(
        parse_sidc2525d("99031000000000000000"),
        Err(SidcParseError::InvalidStandard)
    );
    assert_eq!(
        parse_sidc2525d("10091000000000000000"),
        Err(SidcParseError::InvalidAffiliation)
    );
    assert_eq!(
        parse_sidc2525d("10039900000000000000"),
        Err(SidcParseError::InvalidSymbolSet)
    );
}

#[test]
fn test_cot_bidirectional_mapping() {
    assert_eq!(sidc20_to_cot_type("10031000000000000000").unwrap(), "a-f-G");
    assert_eq!(sidc20_to_cot_type("10060100000000000000").unwrap(), "a-h-A");
    let sea_track = sidc20_to_cot_type("10043000000000000000").unwrap();
    assert_eq!(sea_track, "a-n-S");
    let sub_track = sidc20_to_cot_type("10013500000000000000").unwrap();
    assert_eq!(sub_track, "a-u-U");

    let sidc_air = cot_type_to_sidc20("a-h-A").unwrap();
    assert_eq!(&sidc_air[3..6], "601");

    let sidc_ground = cot_type_to_sidc20("a-f-G-E").unwrap();
    let ground_code = &sidc_ground[3..6];
    assert_eq!(ground_code, "315");

    assert_eq!(
        cot_type_to_sidc20("invalid"),
        Err(SidcParseError::InvalidCotType)
    );
}

#[test]
fn test_cot_affiliation_assumed_friend_mapping() {
    let sidc = cot_type_to_sidc20("a-a-G").unwrap();
    assert_eq!(&sidc[3..6], "210");
    assert_eq!(
        StandardIdentity::from_cot_char('a'),
        StandardIdentity::AssumedFriend
    );
    assert_eq!(
        StandardIdentity::from_cot_char('A'),
        StandardIdentity::AssumedFriend
    );
}
