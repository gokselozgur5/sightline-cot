//! A tracked target crossing a geofence drives the emitted CoT event.

use sightline_cot::camera::enu_to_geodetic;
use sightline_cot::cot::CotEvent;
use sightline_cot::tracker::{BoundingBox, Detection, MultiTargetTracker, TrackStatus};
use sightline_cot::{Geofence, GeofenceState};

const REF: (f64, f64) = (37.005, -122.0102);

fn det(east_m: f64) -> Detection {
    let bbox = BoundingBox {
        x1: east_m - 5.0,
        y1: -5.0,
        x2: east_m + 5.0,
        y2: 5.0,
    };
    Detection {
        east_m,
        north_m: 0.0,
        hae_m: 10.0,
        ce90_m: 1.0,
        le90_m: 1.0,
        bbox,
        embedding: [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    }
}
fn fence() -> Geofence {
    let poly = [
        [37.0, -122.0100],
        [37.0, -122.0095],
        [37.01, -122.0095],
        [37.01, -122.0100],
    ];
    Geofence::new(&poly, -100.0, 100.0).expect("fence")
}
fn run_crossing() -> Vec<(GeofenceState, CotEvent)> {
    let (gf, mut tracker) = (fence(), MultiTargetTracker::new(2, 5, 1.0));
    let (mut prev, mut out) = (None, Vec::new());
    for f in 0..40 {
        tracker.step(1.0, &[det(2.0 * f as f64)]).expect("step");
        let Some(t) = tracker
            .tracks
            .iter()
            .find(|t| t.status == TrackStatus::Confirmed)
        else {
            continue;
        };
        let (lat, lon, hae) = enu_to_geodetic([t.x[0], t.x[1], t.hae_m], REF.0, REF.1, 0.0);
        let b = gf.evaluate_breach(lat, lon, hae, prev);
        prev = Some(matches!(
            b.state,
            GeofenceState::Inside | GeofenceState::Entered
        ));
        let ty = if prev == Some(true) { "a-h-G" } else { "a-f-G" };
        out.push((
            b.state,
            t.to_cot_event("TRK", ty, lat, lon, f * 1000, f * 1000 + 9000),
        ));
    }
    out
}
