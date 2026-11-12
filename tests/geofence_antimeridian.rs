use sightline_cot::{Geofence, GeofenceState};

fn seam_box() -> Geofence {
    let poly = [[0.0, 179.0], [0.0, -179.0], [1.0, -179.0], [1.0, 179.0]];
    Geofence::new(&poly, 0.0, 1000.0).unwrap()
}

#[test]
fn boundary_distance_takes_the_short_way_across_antimeridian() {
    let near = seam_box().distance_to_boundary_m(0.5, 180.0);
    assert!((near - 55_660.0).abs() < 100.0, "got {near}");
    let far = seam_box().distance_to_boundary_m(0.5, 0.0);
    assert!(far > 19_800_000.0 && far < 20_000_000.0, "got {far}");
}
