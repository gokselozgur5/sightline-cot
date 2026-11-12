use sightline_cot::tracker::{BoundingBox, Detection, MultiTargetTracker, TrackerError};

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
fn far_detection_spawns_new_track() {
    let mut t = MultiTargetTracker::new(1, 4, 0.5);
    t.step(1.0, &[det(0.0, 0.0, 0)]).unwrap();
    t.step(1.0, &[det(0.0, 0.0, 0), det(1000.0, 0.0, 0)])
        .unwrap();
    assert_eq!(t.tracks.len(), 2);
}

#[test]
fn nonfinite_detection_is_rejected_without_state_change() {
    let mut t = MultiTargetTracker::new(1, 4, 0.5);
    t.step(1.0, &[det(0.0, 0.0, 0)]).unwrap();
    let before = t.tracks[0].age;
    let r = t.step(1.0, &[det(f64::NAN, 0.0, 0)]);
    assert_eq!(r, Err(TrackerError::NonFiniteInput));
    assert_eq!(t.tracks[0].age, before);
}
