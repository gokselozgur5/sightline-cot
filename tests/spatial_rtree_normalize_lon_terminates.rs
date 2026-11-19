use sightline_cot::spatial_rtree::normalize_lon;
use std::sync::mpsc::channel;
use std::time::Duration;

fn returns_in_time(lon: f64) -> bool {
    let (tx, rx) = channel();
    std::thread::spawn(move || {
        let _ = tx.send(normalize_lon(lon));
    });
    rx.recv_timeout(Duration::from_secs(2)).is_ok()
}

#[test]
fn normalize_lon_terminates_for_infinite_and_huge_input() {
    assert!(returns_in_time(540.0));
    assert!(
        returns_in_time(f64::INFINITY),
        "normalize_lon(inf) never returns"
    );
    assert!(returns_in_time(1.0e19), "normalize_lon(1e19) never returns");
}
