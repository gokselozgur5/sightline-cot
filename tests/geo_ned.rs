use sightline_cot::geo_ned::{enu_to_ned, ned_to_enu};

#[test]
fn test_enu_ned_axis_rotations() {
    let east = [1.0, 0.0, 0.0];
    let north = [0.0, 1.0, 0.0];
    let up = [0.0, 0.0, 1.0];

    assert_eq!(enu_to_ned(east), [0.0, 1.0, 0.0]);
    assert_eq!(enu_to_ned(north), [1.0, 0.0, 0.0]);
    assert_eq!(enu_to_ned(up), [0.0, 0.0, -1.0]);

    assert_eq!(ned_to_enu([0.0, 1.0, 0.0]), east);
    assert_eq!(ned_to_enu([1.0, 0.0, 0.0]), north);
    assert_eq!(ned_to_enu([0.0, 0.0, -1.0]), up);
}

#[test]
fn test_enu_ned_roundtrip_vectors() {
    let vectors = [
        [0.0, 0.0, 0.0],
        [123.4, -567.8, 90.1],
        [-1000.0, 2500.0, -350.0],
    ];
    for &v in &vectors {
        assert_eq!(ned_to_enu(enu_to_ned(v)), v);
        assert_eq!(enu_to_ned(ned_to_enu(v)), v);
    }
}

#[test]
fn test_geodetic_ned_roundtrip() {
    use sightline_cot::geo_ned::{geodetic_to_ned, ned_to_geodetic};

    let (r_lat, r_lon, r_hae) = (37.77, -122.41, 50.0);
    let ned_in = [250.0, -150.0, -45.0];
    let (lat, lon, hae) = ned_to_geodetic(ned_in, r_lat, r_lon, r_hae);
    let ned_out = geodetic_to_ned(lat, lon, hae, r_lat, r_lon, r_hae).unwrap();
    for i in 0..3 {
        assert!((ned_out[i] - ned_in[i]).abs() < 1e-4);
    }
}

#[test]
fn test_geodetic_to_ned_errors() {
    use sightline_cot::geo_enu::EnuError;
    use sightline_cot::geo_ned::geodetic_to_ned;

    assert_eq!(
        geodetic_to_ned(f64::NAN, 0.0, 0.0, 0.0, 0.0, 0.0),
        Err(EnuError::NonFinite)
    );
    assert_eq!(
        geodetic_to_ned(95.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        Err(EnuError::LatitudeOutOfRange)
    );
}
