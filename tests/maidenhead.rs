use sightline_cot::maidenhead::{decode_maidenhead, encode_maidenhead};

#[test]
fn test_acceptance_fn31pr_roundtrip() {
    let (lat, lon) = decode_maidenhead("FN31pr").unwrap();
    assert!((lat - 41.7291667).abs() < 1e-5);
    assert!((lon - -72.7083333).abs() < 1e-5);
    let loc = encode_maidenhead(lat, lon, 3).unwrap();
    assert_eq!(loc, "FN31pr");
}

#[test]
fn test_decode_precision_levels() {
    let (lat, lon) = decode_maidenhead("FN").unwrap();
    assert_eq!((lat, lon), (45.0, -70.0));
    let (lat, lon) = decode_maidenhead("FN31").unwrap();
    assert_eq!((lat, lon), (41.5, -73.0));
    let (lat_ci, lon_ci) = decode_maidenhead("fn31PR").unwrap();
    let (lat_orig, lon_orig) = decode_maidenhead("FN31pr").unwrap();
    assert_eq!((lat_ci, lon_ci), (lat_orig, lon_orig));
}
