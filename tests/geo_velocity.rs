//! ECEF and ENU velocity rotation tests.

use sightline_cot::geo_velocity::{ecef_to_enu_velocity, enu_to_ecef_velocity};

#[test]
fn north_velocity_at_45n_maps_to_pure_north() {
    let p = 45.0_f64.to_radians();
    let v = [-10.0 * p.sin(), 0.0, 10.0 * p.cos()];
    let enu = ecef_to_enu_velocity(v, 45.0, 0.0);
    assert!(enu[0].abs() < 1e-9 && (enu[1] - 10.0).abs() < 1e-9 && enu[2].abs() < 1e-9);
}

#[test]
fn round_trip_preserves_speed_and_components() {
    let enu = [3.0, -4.0, 12.0];
    for (lat, lon) in [(0.0, 0.0), (45.0, 100.0), (-80.0, -170.0), (90.0, 30.0)] {
        let ecef = enu_to_ecef_velocity(enu, lat, lon);
        let norm = |v: [f64; 3]| (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
        assert!((norm(ecef) - 13.0).abs() < 1e-9);
        let back = ecef_to_enu_velocity(ecef, lat, lon);
        for i in 0..3 {
            assert!((back[i] - enu[i]).abs() < 1e-9);
        }
    }
}
