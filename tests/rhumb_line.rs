use sightline_cot::{interpolate_by_count, RhumbLineError, RhumbWaypoint};

#[test]
fn test_interpolate_by_count_rejects_count_exceeding_max_sample_count() {
    let p1 = RhumbWaypoint::new(10.0, 10.0);
    let p2 = RhumbWaypoint::new(10.0, 20.0);
    assert_eq!(
        interpolate_by_count(&p1, &p2, usize::MAX),
        Err(RhumbLineError::InvalidParameter)
    );
}
