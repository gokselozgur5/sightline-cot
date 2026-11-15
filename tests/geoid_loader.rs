use sightline_cot::geoid_loader::{parse_geoid_ascii, GeoidGrid};

#[test]
fn test_parse_ascii_grid_basic() {
    let ascii = "\
ncols 3
nrows 2
xllcorner 10.0
yllcorner 20.0
cellsize 0.5
NODATA_value -9999
1.0 2.0 3.0
4.0 -9999 6.0
";
    let grid = parse_geoid_ascii(ascii).expect("valid ascii grid");
    assert_eq!(grid.cols, 3);
    assert_eq!(grid.rows, 2);
    assert_eq!(grid.xllcorner, 10.0);
    assert_eq!(grid.yllcorner, 20.0);
    assert_eq!(grid.cellsize, 0.5);
    assert_eq!(grid.nodata_value, Some(-9999.0));
    assert_eq!(grid.get(0, 0), Some(1.0));
    assert_eq!(grid.get(1, 1), None);
}
