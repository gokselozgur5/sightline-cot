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
    task.add_point(p1);
    task.add_point(p2);
    task.add_point(p3);

    assert_eq!(task.point_count(), 3);
    assert!(task.total_distance_m() > 0.0);

    let cot_event = task
        .to_cot_event_xml("route-123", "EAGLE-LEAD", 1700000000000, 1700000600000)
        .expect("to_cot_event_xml should succeed");
    assert!(cot_event.contains(r#"uid="route-123""#));
    assert!(cot_event.contains(r#"contact callsign="EAGLE-LEAD""#));

    let parsed = TacticalTask::from_cot_event_xml(&cot_event).expect("parsing cot event should succeed");
    assert_eq!(parsed.name, "PATROL-GRID");
    assert_eq!(parsed.points.len(), 3);
    assert_eq!(parsed.points[1].action, TaskAction::Orbit);
}

#[test]
fn test_cot_task_empty_and_error_handling() {
    let empty_task = TacticalTask::new("EMPTY");
    assert_eq!(empty_task.centroid(), Err(CotTaskError::EmptyRoute));
    assert_eq!(empty_task.total_distance_m(), 0.0);

    let bad_pt = TaskPoint::new(1, "ERR", 91.0, 0.0);
    assert!(matches!(bad_pt, Err(CotTaskError::InvalidCoordinate(_))));
}

#[test]
fn eta_attribute_is_quoted_and_round_trips() {
    let pt = TaskPoint::new(1, "WP1", 10.0, 20.0)
        .unwrap()
        .with_eta(1_700_000_000_000)
        .with_remarks("hold");
    let xml = pt.to_xml();
    assert!(xml.contains("eta=\"1700000000000\""), "{xml}");
    assert_eq!(TaskPoint::parse(&xml).unwrap().eta_ms, Some(1_700_000_000_000));
}

