use sightline_cot::cot_shape::{CotShape, CotVertex};
use sightline_cot::cot_task::{TacticalTask, TaskPoint};

#[test]
fn shape_centroid_stays_near_the_antimeridian() {
    let a = CotVertex::new(10.0, 179.0).unwrap();
    let b = CotVertex::new(10.0, -179.0).unwrap();
    let [_, lon] = CotShape::new_polyline(vec![a, b]).unwrap().centroid();
    assert!(lon.abs() >= 179.0, "centroid longitude {lon}");
}

#[test]
fn task_centroid_stays_near_the_antimeridian() {
    let mut task = TacticalTask::new("T");
    task.add_point(TaskPoint::new(1, "A", 10.0, 179.0).unwrap());
    task.add_point(TaskPoint::new(2, "B", 10.0, -179.0).unwrap());
    let [_, lon] = task.centroid().unwrap();
    assert!(lon.abs() >= 179.0, "centroid longitude {lon}");
}
