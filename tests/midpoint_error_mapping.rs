    // An out-of-range fraction is not an antipodal configuration.
    use sightline_cot::geo_midpoint::{geodesic_midpoint_ellipsoid, MidpointError};

    #[test]
    fn invalid_fraction_is_not_reported_as_antipodal() {
        let r = geodesic_midpoint_ellipsoid(0.0, 0.0, 1.0, 1.0, 1.5);
        assert!(r.is_err());
        assert_ne!(r, Err(MidpointError::AntipodalPoints));
        assert_eq!(r, Err(MidpointError::InvalidFraction));
    }
