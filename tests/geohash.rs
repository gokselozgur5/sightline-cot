use sightline_cot::geohash::{decode, encode};

#[test]
fn test_geohash_encode_decode_basic() {
    let hash = encode(0.0, 0.0, 5).expect("encode 0,0");
    assert_eq!(hash, "s0000");
    let (lat, lon) = decode(&hash).expect("decode s0000");
    assert!((lat - 0.0).abs() < 0.05);
    assert!((lon - 0.0).abs() < 0.05);
}
