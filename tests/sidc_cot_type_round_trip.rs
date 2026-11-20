use sightline_cot::sidc::TacticalSymbol;

#[test]
fn tactical_symbol_cot_type_round_trips_for_every_listed_domain() {
    for cot in ["a-f-U", "a-h-U", "a-n-S", "a-u-A", "a-u-S"] {
        assert_eq!(TacticalSymbol::from_cot_type(cot).to_cot_type(), cot);
    }
}
