use sightline_cot::tracker::{BoundingBox, Detection, MultiTargetTracker};

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

/// Runs two crossing targets and returns the track id seen for each truth target per frame.
fn run_crossing(frames: usize) -> Vec<[Option<u64>; 2]> {
    let mut t = MultiTargetTracker::new(2, 4, 0.5);
    let mut out = Vec::new();
    for i in 0..frames {
        let x = i as f64 * 2.0;
        t.step(1.0, &[det(x, 0.0, 0), det(100.0 - x, 0.5, 1)])
            .unwrap();
        let find = |k: usize| {
            t.tracks
                .iter()
                .find(|tr| tr.embedding[k] > 0.9)
                .map(|tr| tr.id)
        };
        out.push([find(0), find(1)]);
    }
    out
}

#[test]
fn crossing_targets_have_zero_id_switches() {
    let hist = run_crossing(50);
    for k in 0..2 {
        let ids: Vec<u64> = hist.iter().filter_map(|f| f[k]).collect();
        let switches = ids.windows(2).filter(|w| w[0] != w[1]).count();
        assert_eq!(switches, 0, "target {k} switched ids");
    }
}

#[test]
fn track_purity_is_one_and_no_fragmentation() {
    let hist = run_crossing(50);
    for k in 0..2 {
        let present = hist.iter().filter(|f| f[k].is_some()).count();
        assert_eq!(present, hist.len(), "target {k} fragmented");
    }
    assert_ne!(hist[0][0], hist[0][1]);
}
