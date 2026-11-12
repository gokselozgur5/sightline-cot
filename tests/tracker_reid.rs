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
