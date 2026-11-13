use sightline_cot::geo_velocity::{ecef_to_enu_velocity, enu_to_ecef_velocity};

#[test]
fn north_velocity_at_45n_maps_to_enu_north() {
    let s = 45f64.to_radians();
    let ecef = [-10.0 * s.sin(), 0.0, 10.0 * s.cos()];
    let enu = ecef_to_enu_velocity(ecef, 45.0, 0.0);
    assert!(enu[0].abs() < 1e-9 && (enu[1] - 10.0).abs() < 1e-9 && enu[2].abs() < 1e-9);
}

#[test]
fn round_trip_preserves_speed() {
    let v = [3.0, -4.0, 12.0];
    let back = enu_to_ecef_velocity(ecef_to_enu_velocity(v, -33.5, 151.2), -33.5, 151.2);
    let n = |a: [f64; 3]| (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt();
    assert!((n(back) - n(v)).abs() < 1e-9);
    for i in 0..3 {
        assert!((back[i] - v[i]).abs() < 1e-9);
    }
}
