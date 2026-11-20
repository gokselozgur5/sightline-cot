use sightline_cot::geoid_loader::parse_geoid_ascii;

#[test]
fn declared_cell_centre_returns_the_cell_value() {
    let text = "ncols 2 nrows 2 xllcenter 10 yllcenter 20 cellsize 1\n1 2\n3 4";
    let grid = parse_geoid_ascii(text).unwrap();
    assert_eq!(grid.undulation_at(10.0, 20.0), Some(3.0));
    assert_eq!(grid.undulation_at(11.0, 21.0), Some(2.0));
    assert_eq!(grid.undulation_at(10.5, 20.5), Some(2.5));
}
