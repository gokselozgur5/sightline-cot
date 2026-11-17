//! Integration tests for terrain elevation masking validator.

use sightline_cot::elevation_mask::*;

#[test]
fn test_elevation_mask_error_display() {
    let err = ElevationMaskError::InvalidAzimuth;
    assert!(!format!("{}", err).is_empty());
}
