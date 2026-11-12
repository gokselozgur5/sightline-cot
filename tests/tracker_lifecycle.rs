use sightline_cot::tracker::{BoundingBox, Detection, MultiTargetTracker, TrackStatus};

fn emb(k: usize) -> [f32; 8] {
    let mut e = [0.0; 8];
    e[k] = 1.0;
    e
}

fn det(x: f64, y: f64, k: usize) -> Detection {
    let bb = BoundingBox {
        x1: 10.0,
        y1: 10.0,
        x2: 50.0,
        y2: 50.0,
    };
    Detection {
        east_m: x,
        north_m: y,
        hae_m: 10.0,
        ce90_m: 1.5,
        le90_m: 1.0,
        bbox: bb,
        embedding: emb(k),
    }
}

#[test]
fn birth_is_tentative_until_min_hits() {
    let mut t = MultiTargetTracker::new(3, 4, 0.5);
    t.step(1.0, &[det(0.0, 0.0, 0)]).unwrap();
    assert_eq!(t.tracks[0].status, TrackStatus::Tentative);
    t.step(1.0, &[det(1.0, 0.0, 0)]).unwrap();
    assert_eq!(t.tracks[0].status, TrackStatus::Tentative);
    t.step(1.0, &[det(2.0, 0.0, 0)]).unwrap();
    assert_eq!(t.tracks[0].status, TrackStatus::Confirmed);
}

#[test]
fn min_hits_one_confirms_at_birth() {
    let mut t = MultiTargetTracker::new(1, 4, 0.5);
    t.step(1.0, &[det(0.0, 0.0, 0)]).unwrap();
    assert_eq!(t.tracks[0].status, TrackStatus::Confirmed);
}

#[test]
fn missed_detection_coasts_then_dies_after_max_age() {
    let mut t = MultiTargetTracker::new(1, 2, 0.5);
    t.step(1.0, &[det(0.0, 0.0, 0)]).unwrap();
    t.step(1.0, &[]).unwrap();
    assert_eq!(t.tracks[0].status, TrackStatus::Coasting);
    t.step(1.0, &[]).unwrap();
    assert_eq!(t.tracks[0].status, TrackStatus::Coasting);
    t.step(1.0, &[]).unwrap();
    assert!(t.tracks.is_empty());
}

#[test]
fn update_after_coast_resets_staleness() {
    let mut t = MultiTargetTracker::new(1, 3, 0.5);
    t.step(1.0, &[det(0.0, 0.0, 0)]).unwrap();
    t.step(1.0, &[]).unwrap();
    assert_eq!(t.tracks[0].time_since_update, 1);
    t.step(1.0, &[det(0.5, 0.0, 0)]).unwrap();
    assert_eq!(t.tracks[0].time_since_update, 0);
    assert_eq!(t.tracks[0].status, TrackStatus::Confirmed);
}

#[test]
fn ids_are_not_reused_after_death() {
    let mut t = MultiTargetTracker::new(1, 0, 0.5);
    t.step(1.0, &[det(0.0, 0.0, 0)]).unwrap();
    let first = t.tracks[0].id;
    t.step(1.0, &[]).unwrap();
    t.step(1.0, &[det(500.0, 500.0, 1)]).unwrap();
    assert!(t.tracks.iter().all(|k| k.id != first));
}
