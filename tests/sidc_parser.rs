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
    let mut sym = MilStd2525DSymbol::default();
    sym.identity = StandardIdentity::Hostile;
    sym.symbol_set = SymbolSet::Air;
    sym.status = SymbolStatus::Planned;
    sym.echelon = Echelon::Battalion;
    sym.entity_code = 123456;
    sym.modifier1 = 7;
    sym.modifier2 = 8;

    let sidc = sym.to_sidc20();
    assert_eq!(sidc, "10060110161234560708");

    let parsed = MilStd2525DSymbol::parse(&sidc).unwrap();
    assert_eq!(parsed, sym);

    let sidc30 = format!("{sidc}0000000000");
    let parsed30 = parse_sidc2525d(&sidc30).unwrap();
    assert_eq!(parsed30, sym);
}
