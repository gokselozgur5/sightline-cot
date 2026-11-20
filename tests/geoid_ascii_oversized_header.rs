use sightline_cot::geoid_loader::parse_geoid_ascii;

#[test]
fn oversized_header_is_an_error_not_a_panic() {
    let text = "ncols 100000000000 nrows 10000000 xllcorner 0 yllcorner 0 cellsize 1\n1 2 3";
    assert!(parse_geoid_ascii(text).is_err());
}

#[test]
fn overflowing_header_product_is_an_error_not_a_panic() {
    let text = "ncols 4294967296 nrows 4294967296 xllcorner 0 yllcorner 0 cellsize 1\n1 2 3";
    assert!(parse_geoid_ascii(text).is_err());
}
