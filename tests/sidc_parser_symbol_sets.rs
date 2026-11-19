use sightline_cot::sidc_parser::{parse_sidc2525d, sidc20_to_cot_type, SymbolSet};

// MIL-STD-2525D symbol sets: 15 land equipment, 20 land installation, 25 control measure,
// 30 sea surface, 35 sea subsurface, 40 activities, 60 cyberspace.
#[test]
fn symbol_set_30_is_sea_surface() {
    assert_eq!(sidc20_to_cot_type("10033000000000000000").unwrap(), "a-f-S");
}

#[test]
fn symbol_set_35_is_sea_subsurface() {
    assert_eq!(sidc20_to_cot_type("10063500000000000000").unwrap(), "a-h-U");
}

#[test]
fn symbol_set_15_is_land_equipment() {
    let sym = parse_sidc2525d("10031500000000000000").unwrap();
    assert_eq!(sym.symbol_set, SymbolSet::LandEquipment);
}

#[test]
fn status_4_destroyed_is_accepted() {
    assert!(parse_sidc2525d("10031040000000000000").is_ok());
}
