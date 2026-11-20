use sightline_cot::{ElevationMaskError, MaskSector};

#[test]
fn full_circle_sector_covers_every_azimuth_or_is_rejected() {
    match MaskSector::new(0.0, 360.0, 10.0) {
        Ok(sector) => {
            assert!(sector.contains_azimuth(180.0));
            assert!(sector.contains_azimuth(359.0));
        }
        Err(e) => assert_eq!(e, ElevationMaskError::InvalidSectorSpan),
    }
}
