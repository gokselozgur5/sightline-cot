//! MGRS formatting of tracked target positions.

use sightline_cot::camera::enu_to_geodetic;
use sightline_cot::tracker::{BoundingBox, Detection, MultiTargetTracker};
use sightline_cot::{format_mgrs, wgs84_to_utm};

const REF: (f64, f64) = (37.7749, -122.4194);

fn det(east_m: f64, idx: usize) -> Detection {
    let mut embedding = [0.0f32; 8];
    embedding[idx] = 1.0;
    let bbox = BoundingBox {
        x1: east_m - 5.0,
        y1: -5.0,
        x2: east_m + 5.0,
        y2: 5.0,
    };
    Detection {
        east_m,
        north_m: 0.0,
        hae_m: 5.0,
        ce90_m: 1.0,
        le90_m: 1.0,
        bbox,
        embedding,
    }
}
