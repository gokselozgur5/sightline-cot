//! ECEF and ENU velocity rotation tests.

use sightline_cot::geo_velocity::{ecef_to_enu_velocity, enu_to_ecef_velocity};

#[test]
fn north_velocity_at_45n_maps_to_pure_north() {
    let p = 45.0_f64.to_radians();
    let v = [-10.0 * p.sin(), 0.0, 10.0 * p.cos()];
    let enu = ecef_to_enu_velocity(v, 45.0, 0.0);
    assert!(enu[0].abs() < 1e-9 && (enu[1] - 10.0).abs() < 1e-9 && enu[2].abs() < 1e-9);
}
