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

const DOMAINS: [(SidcDomain, &str, char); 6] = [
    (SidcDomain::Air, "01", 'A'),
    (SidcDomain::Space, "05", 'P'),
    (SidcDomain::Land, "10", 'G'),
    (SidcDomain::Surface, "30", 'S'),
    (SidcDomain::Subsurface, "35", 'U'),
    (SidcDomain::Activity, "40", 'X'),
];

#[test]
fn domain_2525d_round_trip() {
    for (dom, code, _) in DOMAINS {
        assert_eq!(dom.to_2525d_code(), code);
        assert_eq!(SidcDomain::from_2525d_code(code), Ok(dom));
    }
}
