//! Deterministic End-to-End Multi-Target Pixel-to-CoT Integration Suite.

use sightline_cot::camera::{enu_to_geodetic, estimate_covariance_ce_le, intersect_ground};
use sightline_cot::camera::{CameraPose, Intrinsics};
use sightline_cot::cot::{CotEvent, CotUdpDispatcher};
use sightline_cot::tracker::{BoundingBox, Detection, MultiTargetTracker, Track, TrackStatus};
use std::collections::HashMap;
use std::net::UdpSocket;
use std::time::Duration;

fn setup_camera() -> (Intrinsics, CameraPose) {
    let intrinsics = Intrinsics::Pinhole {
        fx: 1000.0,
        fy: 1000.0,
        cx: 960.0,
        cy: 540.0,
    };

    let pose = CameraPose {
        roll_rad: 0.0,
        pitch_rad: 0.0,
        yaw_rad: 0.0,
        cam_up_m: 1000.0,
    };

    (intrinsics, pose)
}

fn make_bbox(px: f64, py: f64) -> BoundingBox {
    BoundingBox {
        x1: px - 10.0,
        y1: py - 10.0,
        x2: px + 10.0,
        y2: py + 10.0,
    }
}

fn make_detection(
    px: f64,
    py: f64,
    intr: &Intrinsics,
    pose: &CameraPose,
    emb_idx: usize,
) -> Detection {
    let ray = intr.pixel_to_ray(px, py).expect("ray");
    let enu = intersect_ground(ray, pose, 0.0).expect("ground intersection");
    let (ce90_m, le90_m) =
        estimate_covariance_ce_le(intr, pose, px, py, 0.0, 1.0, 0.001).expect("cov");
    let mut embedding = [0.0f32; 8];
    embedding[emb_idx] = 1.0;

    Detection {
        east_m: enu[0],
        north_m: enu[1],
        hae_m: 10.0,
        ce90_m,
        le90_m,
        bbox: make_bbox(px, py),
        embedding,
    }
}

fn convert_track_to_cot(
    trk: &Track,
    ref_lat: f64,
    ref_lon: f64,
    ref_alt: f64,
    time_ms: i64,
) -> CotEvent {
    let (lat_deg, lon_deg, hae_m) =
        enu_to_geodetic([trk.x[0], trk.x[1], trk.hae_m], ref_lat, ref_lon, ref_alt);
    let speed_mps = (trk.x[2].powi(2) + trk.x[3].powi(2)).sqrt();
    let course_deg = trk.x[2].atan2(trk.x[3]).to_degrees().rem_euclid(360.0);

    CotEvent {
        uid: format!("TRACK-{}", trk.id),
        cot_type: "a-f-G-E-V".into(),
        how: "m-g".into(),
        time_ms,
        stale_ms: time_ms + 10_000,
        lat_deg,
        lon_deg,
        hae_m,
        ce_m: (trk.p[0][0].sqrt() * 2.146).max(1.0),
        le_m: 5.0,
        course_deg,
        speed_mps,
    }
}

fn dispatch_tracks(dispatcher: &CotUdpDispatcher, tracks: &[Track], t_ms: i64) -> usize {
    let mut count = 0;
    for trk in tracks {
        let cot = convert_track_to_cot(trk, 37.7749, -122.4194, 0.0, t_ms);
        dispatcher.send_event(&cot).expect("dispatch");
        count += 1;
    }
    count
}

fn step_and_dispatch(
    tracker: &mut MultiTargetTracker,
    disp: &CotUdpDispatcher,
    dets: &[Detection],
    t_ms: i64,
) -> usize {
    tracker.step(0.1, dets).expect("tracker step");
    let confirmed: Vec<Track> = tracker
        .tracks
        .iter()
        .filter(|t| t.status == TrackStatus::Confirmed)
        .cloned()
        .collect();
    dispatch_tracks(disp, &confirmed, t_ms)
}

fn run_simulation(
    tracker: &mut MultiTargetTracker,
    disp: &CotUdpDispatcher,
    intr: &Intrinsics,
    pose: &CameraPose,
) -> usize {
    let mut dispatched = 0;
    for f in 0..10 {
        let t_ms = 1771234500000 + f as i64 * 100;
        let d1 = make_detection(
            960.0 + f as f64 * 2.0,
            540.0 + f as f64 * 1.5,
            intr,
            pose,
            0,
        );
        let d2 = make_detection(
            800.0 - f as f64 * 1.0,
            400.0 + f as f64 * 2.0,
            intr,
            pose,
            1,
        );
        dispatched += step_and_dispatch(tracker, disp, &[d1, d2], t_ms);
    }
    dispatched
}

fn collect_received_packets(
    receiver: &UdpSocket,
    expected_count: usize,
) -> HashMap<String, CotEvent> {
    let mut last_per_uid: HashMap<String, CotEvent> = HashMap::new();
    let mut buf = [0u8; 2048];
    for _ in 0..expected_count {
        let (len, _) = receiver.recv_from(&mut buf).expect("receive udp packet");
        let xml_str = std::str::from_utf8(&buf[..len]).expect("valid utf8");
        let parsed = CotEvent::from_xml(xml_str).expect("valid CoT XML");
        last_per_uid.insert(parsed.uid.clone(), parsed);
    }
    last_per_uid
}

fn compute_expected_geo(px: f64, py: f64, intr: &Intrinsics, pose: &CameraPose) -> (f64, f64, f64) {
    let ray = intr.pixel_to_ray(px, py).unwrap();
    let enu = intersect_ground(ray, pose, 0.0).unwrap();
    enu_to_geodetic([enu[0], enu[1], 10.0], 37.7749, -122.4194, 0.0)
}

fn verify_single_target(
    last_per_uid: &HashMap<String, CotEvent>,
    expected_geo: (f64, f64, f64),
    exp_speed: f64,
    exp_course: f64,
) {
    let (exp_lat, exp_lon, exp_hae) = expected_geo;
    let t = last_per_uid
        .values()
        .find(|e| (e.lat_deg - exp_lat).abs() < 1e-5)
        .expect("target match");

    assert!((t.lon_deg - exp_lon).abs() < 1e-5);
    assert!((t.hae_m - exp_hae).abs() < 1e-2);
    assert!((t.speed_mps - exp_speed).abs() < 5.0);
    assert!((t.course_deg - exp_course).abs() < 15.0);
}

fn verify_target_geometry(
    last_per_uid: &HashMap<String, CotEvent>,
    intr: &Intrinsics,
    pose: &CameraPose,
) {
    assert_eq!(
        last_per_uid.len(),
        2,
        "exactly two confirmed tracks received"
    );

    let geo1 = compute_expected_geo(960.0 + 9.0 * 2.0, 540.0 + 9.0 * 1.5, intr, pose);
    verify_single_target(last_per_uid, geo1, 25.0, 126.9);

    let geo2 = compute_expected_geo(800.0 - 9.0 * 1.0, 400.0 + 9.0 * 2.0, intr, pose);
    verify_single_target(last_per_uid, geo2, 22.4, 206.6);
}

#[test]
fn test_e2e_pixel_to_cot_pipeline() {
    let (intrinsics, pose) = setup_camera();
    let mut tracker = MultiTargetTracker::new(2, 5, 1.0);
    let receiver = UdpSocket::bind("127.0.0.1:0").expect("bind receiver");
    let recv_addr = receiver.local_addr().expect("receiver addr");
    receiver
        .set_read_timeout(Some(Duration::from_secs(2)))
        .expect("timeout");
    let dispatcher = CotUdpDispatcher::new("127.0.0.1:0", recv_addr).expect("dispatcher");

    let dispatched = run_simulation(&mut tracker, &dispatcher, &intrinsics, &pose);
    assert_eq!(
        dispatched, 18,
        "expected 2 tracks * 9 confirmed frames = 18 packets"
    );

    let last_per_uid = collect_received_packets(&receiver, dispatched);
    verify_target_geometry(&last_per_uid, &intrinsics, &pose);

    let mut buf = [0u8; 128];
    assert!(
        receiver.recv_from(&mut buf).is_err(),
        "no residual packets in socket"
    );
}

fn verify_coasting_and_outlier(
    tracker: &mut MultiTargetTracker,
    intr: &Intrinsics,
    pose: &CameraPose,
    init_id: u64,
) {
    tracker.step(0.1, &[]).expect("missed frame");
    assert_eq!(tracker.tracks[0].id, init_id);
    assert_eq!(tracker.tracks[0].status, TrackStatus::Coasting);
    assert_eq!(tracker.tracks[0].time_since_update, 1);
    let coasting_snapshot = tracker.tracks[0].clone();

    let outlier = make_detection(100.0, 100.0, intr, pose, 3);
    tracker
        .step(0.1, std::slice::from_ref(&outlier))
        .expect("outlier frame");
    assert_eq!(
        tracker.tracks.len(),
        2,
        "outlier creates distinct tentative track"
    );
    assert_eq!(
        tracker.tracks[0].hits, coasting_snapshot.hits,
        "coasting track hits unchanged"
    );
    assert_eq!(tracker.tracks[0].time_since_update, 2);
}

#[test]
fn test_e2e_coasting_and_outlier_rejection() {
    let (intrinsics, pose) = setup_camera();
    let mut tracker = MultiTargetTracker::new(2, 5, 1.0);
    let d1 = make_detection(960.0, 540.0, &intrinsics, &pose, 0);
    tracker.step(0.1, std::slice::from_ref(&d1)).expect("f1");
    tracker.step(0.1, std::slice::from_ref(&d1)).expect("f2");
    let init_id = tracker.tracks[0].id;
    assert_eq!(tracker.tracks[0].status, TrackStatus::Confirmed);

    verify_coasting_and_outlier(&mut tracker, &intrinsics, &pose, init_id);

    let d_recover = make_detection(960.0, 540.0, &intrinsics, &pose, 0);
    tracker
        .step(0.1, std::slice::from_ref(&d_recover))
        .expect("recovery frame");
    assert_eq!(
        tracker.tracks[0].id, init_id,
        "track recovers with same uid"
    );
    assert_eq!(tracker.tracks[0].status, TrackStatus::Confirmed);
    assert_eq!(tracker.tracks[0].time_since_update, 0);
}
