use sightline_cot::is_counter_clockwise;

#[test]
fn ring_sw_se_ne_nw_is_counter_clockwise() {
    let ccw = [[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]];
    let cw = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
    assert_eq!(is_counter_clockwise(&ccw), Ok(true));
    assert_eq!(is_counter_clockwise(&cw), Ok(false));
}
