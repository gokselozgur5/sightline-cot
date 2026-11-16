use sightline_cot::cot_task::{
    CotTaskError, TacticalTask, TaskAction, TaskPoint, TaskPriority, TaskStatus,
};

#[test]
fn test_cot_task_full_workflow() {
    let mut task = TacticalTask::new("PATROL-GRID")
        .with_type("patrol")
        .with_priority(TaskPriority::Priority)
        .with_status(TaskStatus::InProgress);

    let p1 = TaskPoint::new(1, "ENTRY", 36.0, -115.0)
        .unwrap()
        .with_hae(500.0)
        .unwrap()
        .with_action(TaskAction::Ingress);
    let p2 = TaskPoint::new(2, "STATION", 36.1, -115.1)
        .unwrap()
        .with_action(TaskAction::Orbit)
        .with_speed(80.0)
        .unwrap();
    let p3 = TaskPoint::new(3, "EXIT", 36.2, -115.0)
        .unwrap()
        .with_action(TaskAction::Egress);
