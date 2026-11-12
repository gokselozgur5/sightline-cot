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
