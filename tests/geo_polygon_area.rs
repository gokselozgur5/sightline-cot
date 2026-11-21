use sightline_cot::geo_polygon_area::*;

#[test]
fn perimeter_and_winding_reject_invalid_coords() {
    let nan_poly = [[f64::NAN, 0.0], [0.0, 1.0], [1.0, 0.0]];
    let lat_oob = [[95.0, 0.0], [0.0, 1.0], [1.0, 0.0]];
    let lon_nan = [[0.0, f64::NAN], [0.0, 1.0], [1.0, 0.0]];
    let lon_oob = [[0.0, 195.0], [0.0, 1.0], [1.0, 0.0]];
    let exp_errs = [
        GeodesicAreaError::NonFiniteCoordinate,
        GeodesicAreaError::LatitudeOutOfRange,
        GeodesicAreaError::NonFiniteCoordinate,
        GeodesicAreaError::LongitudeOutOfRange,
    ];
    for (poly, exp) in [nan_poly, lat_oob, lon_nan, lon_oob].iter().zip(exp_errs) {
        assert_eq!(polygon_geodesic_perimeter(poly), Err(exp));
        assert_eq!(is_counter_clockwise(poly), Err(exp));
    }
}
