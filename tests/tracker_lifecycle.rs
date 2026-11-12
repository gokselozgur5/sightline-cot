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
