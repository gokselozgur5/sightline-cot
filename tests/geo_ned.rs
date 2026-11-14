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
