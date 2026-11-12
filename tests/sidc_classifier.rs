use sightline_cot::sidc::{SidcAffiliation, SidcDomain, TacticalSymbol};

const AFFILIATIONS: [(SidcAffiliation, &str, char); 9] = [
    (SidcAffiliation::Pending, "01", 'P'),
    (SidcAffiliation::Unknown, "02", 'U'),
    (SidcAffiliation::Friend, "03", 'F'),
    (SidcAffiliation::AssumedFriend, "04", 'A'),
    (SidcAffiliation::Neutral, "05", 'N'),
    (SidcAffiliation::Hostile, "06", 'H'),
    (SidcAffiliation::Suspect, "07", 'S'),
    (SidcAffiliation::Joker, "08", 'J'),
    (SidcAffiliation::Faker, "09", 'K'),
];

#[test]
fn affiliation_2525d_round_trip() {
    for (aff, code, _) in AFFILIATIONS {
        assert_eq!(aff.to_2525d_code(), code);
        assert_eq!(SidcAffiliation::from_2525d_code(code), Ok(aff));
    }
}

#[test]
fn affiliation_2525c_round_trip() {
    for (aff, _, ch) in AFFILIATIONS {
        assert_eq!(aff.to_2525c_char(), ch);
        assert_eq!(SidcAffiliation::from_2525c_char(ch), Ok(aff));
    }
}
