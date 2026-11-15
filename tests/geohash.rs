use sightline_cot::geohash::{decode, encode};

#[test]
fn test_geohash_encode_decode_basic() {
    let hash = encode(0.0, 0.0, 5).expect("encode 0,0");
    assert_eq!(hash, "s0000");
    let (lat, lon) = decode(&hash).expect("decode s0000");
    assert!((lat - 0.0).abs() < 0.05);
    assert!((lon - 0.0).abs() < 0.05);
}

#[test]
fn test_geohash_bbox_contains_point() {
    use sightline_cot::geohash::decode_bbox;
    let bbox = decode_bbox("s0000").expect("valid bbox");
    assert!(bbox.contains(0.0, 0.0));
    assert!(bbox.min_lat <= 0.0 && bbox.max_lat >= 0.0);
    assert!(bbox.min_lon <= 0.0 && bbox.max_lon >= 0.0);
}

#[test]
fn test_geohash_known_locations() {
    let cases = [
        (57.64911, 10.40744, 11, "u4pruydqqvj"),
        (51.5007, -0.1246, 7, "gcpuvpm"),
        (-33.8568, 151.2153, 6, "r3gx2u"),
    ];
    for (lat, lon, prec, expected) in cases {
        let h = encode(lat, lon, prec).expect("valid encode");
        assert_eq!(h, expected);
        let (dlat, dlon) = decode(&h).expect("valid decode");
        assert!((dlat - lat).abs() < 0.01);
        assert!((dlon - lon).abs() < 0.01);
    }
}

#[test]
fn test_geohash_error_handling() {
    use sightline_cot::geohash::GeohashError;
    assert_eq!(encode(91.0, 0.0, 5), Err(GeohashError::LatitudeOutOfRange));
    assert_eq!(encode(0.0, 181.0, 5), Err(GeohashError::LongitudeOutOfRange));
    assert_eq!(encode(0.0, 0.0, 0), Err(GeohashError::InvalidPrecision));
    assert_eq!(encode(0.0, 0.0, 13), Err(GeohashError::InvalidPrecision));
    assert_eq!(decode(""), Err(GeohashError::EmptyHash));
    assert_eq!(decode("s000a"), Err(GeohashError::InvalidCharacter('a')));
}
