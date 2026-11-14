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
