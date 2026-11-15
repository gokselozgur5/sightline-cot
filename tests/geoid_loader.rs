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

#[test]
fn test_xllcenter_and_from_str() {
    let ascii = "\
ncols 2
nrows 2
xllcenter 10.5
yllcenter 20.5
cellsize 1.0
10.0 20.0
30.0 40.0
";
    let grid: GeoidGrid = ascii.parse().expect("valid center grid");
    assert_eq!(grid.xllcorner, 10.0);
    assert_eq!(grid.yllcorner, 20.0);
    assert_eq!(grid.nodata_value, None);
    assert_eq!(grid.get(0, 0), Some(10.0));
    assert_eq!(grid.get(1, 1), Some(40.0));
}

#[test]
fn test_bilinear_interpolation_accuracy() {
    let ascii = "\
ncols 2
nrows 2
xllcorner 0.0
yllcorner 0.0
cellsize 1.0
10.0 20.0
30.0 40.0
";
    let grid = parse_geoid_ascii(ascii).unwrap();
    assert_eq!(grid.undulation_at(0.0, 1.0), Some(10.0));
    assert_eq!(grid.undulation_at(1.0, 1.0), Some(20.0));
    assert_eq!(grid.undulation_at(0.0, 0.0), Some(30.0));
    assert_eq!(grid.undulation_at(1.0, 0.0), Some(40.0));
    assert_eq!(grid.undulation_at(0.5, 0.5), Some(25.0));
}

#[test]
fn test_nodata_and_out_of_bounds() {
    let ascii = "\
ncols 2
nrows 2
xllcorner 0.0
yllcorner 0.0
cellsize 1.0
nodata_value -9999
10.0 -9999
30.0 40.0
";
    let grid = parse_geoid_ascii(ascii).unwrap();
    assert_eq!(grid.undulation_at(1.0, 1.0), None);
    assert_eq!(grid.undulation_at(0.5, 0.5), None);
    assert_eq!(grid.undulation_at(0.0, 0.0), Some(30.0));
    assert_eq!(grid.undulation_at(-0.1, 0.5), None);
    assert_eq!(grid.undulation_at(0.5, 2.5), None);
}
