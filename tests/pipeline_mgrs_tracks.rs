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
fn tracked_latlon() -> Vec<(f64, f64)> {
    let mut tracker = MultiTargetTracker::new(2, 5, 1.0);
    for _ in 0..3 {
        tracker
            .step(0.1, &[det(0.0, 0), det(100.0, 1)])
            .expect("step");
    }
    let mut pts: Vec<_> = tracker
        .tracks
        .iter()
        .map(|t| enu_to_geodetic([t.x[0], t.x[1], t.hae_m], REF.0, REF.1, 0.0))
        .map(|(lat, lon, _)| (lat, lon))
        .collect();
    pts.sort_by(|a, b| a.1.total_cmp(&b.1));
    pts
}
#[test]
fn tracked_targets_have_distinct_ten_digit_mgrs() {
    let pts = tracked_latlon();
    assert_eq!(pts.len(), 2);
    let a = format_mgrs(pts[0].0, pts[0].1, 5).expect("a");
    let b = format_mgrs(pts[1].0, pts[1].1, 5).expect("b");
    assert_eq!(a.as_str().len(), 15);
    assert!(a.as_str().starts_with("10S"));
    assert_ne!(a.as_str(), b.as_str());
    let east = |s: &str| s[5..10].parse::<i64>().unwrap();
    assert!((east(b.as_str()) - east(a.as_str()) - 100).abs() <= 2);
}
