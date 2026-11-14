use sightline_cot::camera::enu_to_geodetic;
use sightline_cot::geo_enu::{geodetic_to_enu, EnuError};

#[test]
fn test_geodetic_to_enu_offsets() {
    let origins = [(37.77, -122.41, 50.0), (0.0, 0.0, 0.0), (-45.0, 170.0, 200.0)];
    let offsets: &[[f64; 3]] = &[
        [0.0, 0.0, 0.0],
        [100.0, -50.0, 10.0],
        [-25000.0, 30000.0, 500.0],
        [0.0, 50000.0, 1000.0],
    ];
    for &(r_lat, r_lon, r_hae) in &origins {
        for &enu_in in offsets {
            let (lat, lon, hae) = enu_to_geodetic(enu_in, r_lat, r_lon, r_hae);
            let enu_out = geodetic_to_enu(lat, lon, hae, r_lat, r_lon, r_hae).unwrap();
            let dx = enu_out[0] - enu_in[0];
            let dy = enu_out[1] - enu_in[1];
            let dz = enu_out[2] - enu_in[2];
            assert!((dx * dx + dy * dy + dz * dz).sqrt() < 1e-4);
        }
    }
}

#[test]
fn test_geodetic_to_enu_error_handling() {
    assert_eq!(
        geodetic_to_enu(f64::NAN, 0.0, 0.0, 0.0, 0.0, 0.0),
        Err(EnuError::NonFinite)
    );
    assert_eq!(
        geodetic_to_enu(95.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        Err(EnuError::LatitudeOutOfRange)
    );
    assert_eq!(
        geodetic_to_enu(0.0, 0.0, 0.0, -91.0, 0.0, 0.0),
        Err(EnuError::LatitudeOutOfRange)
    );
}
