use sightline_cot::tracker::{BoundingBox, Detection, MultiTargetTracker, TrackStatus};

#[test]
fn stationary_detection_without_bbox_or_embedding_stays_one_track() {
    let det = Detection {
        east_m: 0.0,
        north_m: 0.0,
        hae_m: 0.0,
        ce90_m: 1.5,
        le90_m: 1.0,
        bbox: BoundingBox {
            x1: 0.0,
            y1: 0.0,
            x2: 0.0,
            y2: 0.0,
        },
        embedding: [0.0; 8],
    };
    let mut tracker = MultiTargetTracker::new(3, 5, 1.0);
    for _ in 0..4 {
        tracker.step(0.033, std::slice::from_ref(&det)).unwrap();
    }
    assert_eq!(tracker.tracks.len(), 1, "one target must be one track");
    assert_eq!(tracker.tracks[0].status, TrackStatus::Confirmed);
}
