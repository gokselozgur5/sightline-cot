use sightline_cot::camera::{CameraError, DemGrid};

#[test]
fn dem_grid_new_rejects_dimensions_whose_product_overflows() {
    let res = DemGrid::new(0.0, 0.0, 1.0, 1usize << 63, 2, Vec::new());
    assert_eq!(res.err(), Some(CameraError::InvalidParameters));
}
