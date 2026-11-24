//! C-ABI surface for embedding the Sightline engine.

use crate::camera::{enu_to_geodetic, estimate_covariance_ce_le, intersect_ground};
use crate::camera::{CameraPose, DemGrid, Intrinsics};
use crate::classifier::{MilStdClassifier, TacticalAffiliation, TargetCategory};
use crate::cot::CotEvent;
use crate::proto::{encode_tak_packet, TakCotEvent};
use crate::tracker::{BoundingBox, Detection, MultiTargetTracker, Track};
use std::os::raw::c_char;

pub const SIGHTLINE_OK: i32 = 0;
pub const SIGHTLINE_ERR_NULL: i32 = -1;
pub const SIGHTLINE_ERR_INVALID: i32 = -2;
pub const SIGHTLINE_ERR_BUFFER: i32 = -3;
pub const SIGHTLINE_ERR_PANIC: i32 = -4;
pub const SIGHTLINE_ERR_CAMERA: i32 = -5;
pub const SIGHTLINE_ERR_TRACKER: i32 = -6;
/// CoT sentinel for an unknown error value.
pub const SIGHTLINE_UNKNOWN_ERROR_M: f64 = 9_999_999.0;
pub const SIGHTLINE_STATUS_TENTATIVE: u32 = 0;
pub const SIGHTLINE_STATUS_CONFIRMED: u32 = 1;
pub const SIGHTLINE_STATUS_COASTING: u32 = 2;
pub const SIGHTLINE_STATUS_DELETED: u32 = 3;

const _: () = assert!(std::mem::size_of::<SightlineCameraCfg>() == 152);
const _: () = assert!(std::mem::size_of::<SightlineDetectionC>() == 80);
const _: () = assert!(std::mem::size_of::<SightlineTrackC>() == 72);
const _: () = assert!(std::mem::size_of::<SightlinePoseC>() == 56);

fn guard<F: FnOnce() -> i32>(f: F) -> i32 {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)).unwrap_or(SIGHTLINE_ERR_PANIC)
}

/// Dynamic 6-DOF camera pose and geodetic reference for airborne platforms.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SightlinePoseC {
    pub roll_rad: f64,
    pub pitch_rad: f64,
    pub yaw_rad: f64,
    pub cam_up_m: f64,
    pub ref_lat_deg: f64,
    pub ref_lon_deg: f64,
    pub ref_hae_m: f64,
}

/// Camera, pose, geodetic origin and tracker parameters.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SightlineCameraCfg {
    pub fx: f64,
    pub fy: f64,
    pub cx: f64,
    pub cy: f64,
    pub k1: f64,
    pub k2: f64,
    pub is_fisheye: i32,
    pub roll_rad: f64,
    pub pitch_rad: f64,
    pub yaw_rad: f64,
    pub cam_up_m: f64,
    pub ground_up_m: f64,
    pub pixel_sigma_px: f64,
    pub attitude_sigma_rad: f64,
    pub ref_lat_deg: f64,
    pub ref_lon_deg: f64,
    pub ref_hae_m: f64,
    pub min_hits: u32,
    pub max_age: u32,
    pub q_acc: f64,
}

/// Pixel detection with bounding box and Re-ID embedding.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SightlineDetectionC {
    pub u: f64,
    pub v: f64,
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
    pub embedding: [f32; 8],
}

/// Geodetic track report.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct SightlineTrackC {
    pub id: u64,
    pub lat_deg: f64,
    pub lon_deg: f64,
    pub hae_m: f64,
    pub ce90_m: f64,
    pub le90_m: f64,
    pub course_deg: f64,
    pub speed_mps: f64,
    pub status: u32,
}

/// Opaque engine handle owned by the caller.
pub struct SightlineEngine {
    cam: Intrinsics,
    pose: CameraPose,
    cfg: SightlineCameraCfg,
    dem: Option<DemGrid>,
    tracker: MultiTargetTracker,
}

fn make_intrinsics(c: &SightlineCameraCfg) -> Intrinsics {
    let (fx, fy, cx, cy, k1, k2) = (c.fx, c.fy, c.cx, c.cy, c.k1, c.k2);
    if c.is_fisheye != 0 {
        Intrinsics::EquidistantFisheye {
            fx,
            fy,
            cx,
            cy,
            k1,
            k2,
        }
    } else {
        Intrinsics::Pinhole { fx, fy, cx, cy }
    }
}

fn make_pose(c: &SightlineCameraCfg) -> CameraPose {
    CameraPose {
        roll_rad: c.roll_rad,
        pitch_rad: c.pitch_rad,
        yaw_rad: c.yaw_rad,
        cam_up_m: c.cam_up_m,
    }
}

fn all_finite(c: &SightlineCameraCfg) -> bool {
    let f = [
        c.fx,
        c.fy,
        c.cx,
        c.cy,
        c.k1,
        c.k2,
        c.roll_rad,
        c.pitch_rad,
        c.yaw_rad,
        c.cam_up_m,
        c.ground_up_m,
        c.pixel_sigma_px,
        c.attitude_sigma_rad,
        c.ref_lat_deg,
        c.ref_lon_deg,
        c.ref_hae_m,
        c.q_acc,
    ];
    f.iter().all(|x| x.is_finite())
}

fn cfg_valid(c: &SightlineCameraCfg) -> bool {
    let ranges = c.ref_lat_deg.abs() <= 90.0 && c.ref_lon_deg.abs() <= 180.0;
    let signs = c.pixel_sigma_px >= 0.0 && c.attitude_sigma_rad >= 0.0 && c.q_acc >= 0.0;
    all_finite(c)
        && ranges
        && signs
        && c.fx > 0.0
        && c.fy > 0.0
        && c.cam_up_m > c.ground_up_m
        && c.min_hits > 0
        && (c.is_fisheye == 0 || c.is_fisheye == 1)
}

fn build_engine(c: &SightlineCameraCfg) -> Option<SightlineEngine> {
    if !cfg_valid(c) {
        return None;
    }
    let tracker = MultiTargetTracker::new(c.min_hits as usize, c.max_age as usize, c.q_acc);
    Some(SightlineEngine {
        cam: make_intrinsics(c),
        pose: make_pose(c),
        cfg: *c,
        dem: None,
        tracker,
    })
}

/// # Safety
/// `cfg` must be null or point to a valid `SightlineCameraCfg`.
#[no_mangle]
pub unsafe extern "C" fn sightline_engine_new(
    cfg: *const SightlineCameraCfg,
) -> *mut SightlineEngine {
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if cfg.is_null() || !pointer_is_aligned(cfg) {
            return std::ptr::null_mut();
        }
        match build_engine(&*cfg) {
            Some(e) => Box::into_raw(Box::new(e)),
            None => std::ptr::null_mut(),
        }
    }));
    r.unwrap_or(std::ptr::null_mut())
}

/// # Safety
/// `e` must be null or a pointer returned by `sightline_engine_new`, freed once.
#[no_mangle]
pub unsafe extern "C" fn sightline_engine_free(e: *mut SightlineEngine) {
    if !e.is_null() {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| drop(Box::from_raw(e))));
    }
}

fn locate(eng: &SightlineEngine, d: &SightlineDetectionC) -> Result<[f64; 3], i32> {
    let ray = eng
        .cam
        .pixel_to_ray(d.u, d.v)
        .map_err(|_| SIGHTLINE_ERR_CAMERA)?;
    if let Some(ref dem) = eng.dem {
        dem.intersect_dem(ray, &eng.pose, 64)
            .map_err(|_| SIGHTLINE_ERR_CAMERA)
    } else {
        intersect_ground(ray, &eng.pose, eng.cfg.ground_up_m).map_err(|_| SIGHTLINE_ERR_CAMERA)
    }
}

fn covariance(
    eng: &SightlineEngine,
    d: &SightlineDetectionC,
    ground_hae: f64,
) -> Result<(f64, f64), i32> {
    let c = &eng.cfg;
    estimate_covariance_ce_le(
        &eng.cam,
        &eng.pose,
        d.u,
        d.v,
        ground_hae,
        c.pixel_sigma_px,
        c.attitude_sigma_rad,
    )
    .map_err(|_| SIGHTLINE_ERR_CAMERA)
}

fn to_detection(eng: &SightlineEngine, d: &SightlineDetectionC) -> Result<Detection, i32> {
    let p = locate(eng, d)?;
    let ground_hae = if eng.dem.is_some() {
        p[2]
    } else {
        eng.cfg.ground_up_m
    };
    let (ce, le) = covariance(eng, d, ground_hae)?;
    let bbox = BoundingBox {
        x1: d.x1,
        y1: d.y1,
        x2: d.x2,
        y2: d.y2,
    };
    Ok(Detection {
        east_m: p[0],
        north_m: p[1],
        hae_m: ground_hae,
        ce90_m: ce,
        le90_m: le,
        bbox,
        embedding: d.embedding,
    })
}

fn to_track_c(eng: &SightlineEngine, t: &Track) -> SightlineTrackC {
    let c = &eng.cfg;
    let (lat, lon, hae) = enu_to_geodetic(
        [t.x[0], t.x[1], t.hae_m],
        c.ref_lat_deg,
        c.ref_lon_deg,
        c.ref_hae_m,
    );
    let speed = t.x[2].hypot(t.x[3]);
    let course = t.x[2].atan2(t.x[3]).to_degrees().rem_euclid(360.0);
    let ce = 2.146 * (0.5 * (t.p[0][0] + t.p[1][1])).max(0.0).sqrt();
    SightlineTrackC {
        id: t.id,
        lat_deg: lat,
        lon_deg: lon,
        hae_m: hae,
        ce90_m: ce,
        le90_m: t.le90_m,
        course_deg: course,
        speed_mps: speed,
        status: t.status as u32,
    }
}

fn engine_step(
    eng: &mut SightlineEngine,
    dt_s: f64,
    dets: &[SightlineDetectionC],
) -> Result<Vec<SightlineTrackC>, i32> {
    let mut v = Vec::with_capacity(dets.len());
    // Detections that cannot be located (NaN pixel, above horizon) are skipped individually.
    v.extend(dets.iter().filter_map(|d| to_detection(eng, d).ok()));
    eng.tracker
        .step(dt_s, &v)
        .map_err(|_| SIGHTLINE_ERR_TRACKER)?;
    Ok(eng
        .tracker
        .tracks
        .iter()
        .map(|t| to_track_c(eng, t))
        .collect())
}

type StepArgs = (*mut SightlineEngine, *const SightlineDetectionC, usize);

#[inline]
fn pointer_is_aligned<T>(p: *const T) -> bool {
    (p as usize) % std::mem::align_of::<T>() == 0
}

fn layout_ok<T>(p: *const T, n: usize) -> bool {
    let fits = n
        .checked_mul(std::mem::size_of::<T>())
        .is_some_and(|b| b <= isize::MAX as usize);
    fits && pointer_is_aligned(p)
}

unsafe fn step_impl(a: StepArgs, dt_s: f64, out: (*mut SightlineTrackC, usize, *mut usize)) -> i32 {
    let ((e, dets, n), (out, cap, out_n)) = (a, out);
    if e.is_null() || out_n.is_null() || (n > 0 && dets.is_null()) || (cap > 0 && out.is_null()) {
        return SIGHTLINE_ERR_NULL;
    }
    if !layout_ok(dets, n) || !layout_ok(out, cap) || !pointer_is_aligned(out_n) {
        return SIGHTLINE_ERR_INVALID;
    }
    let ds: &[SightlineDetectionC] = if n == 0 {
        &[]
    } else {
        std::slice::from_raw_parts(dets, n)
    };
    let tracks = match engine_step(&mut *e, dt_s, ds) {
        Ok(t) => t,
        Err(code) => return code,
    };
    *out_n = tracks.len();
    for (i, t) in tracks.iter().take(cap).enumerate() {
        *out.add(i) = *t;
    }
    if tracks.len() > cap {
        SIGHTLINE_ERR_BUFFER
    } else {
        SIGHTLINE_OK
    }
}

/// Advances the tracker by `dt_s` even when `ERR_BUFFER` is returned; `*out_n` holds the full count.
/// Not thread-safe: one caller per engine.
/// # Safety
/// Pointers must be null (with zero length) or valid for the stated lengths.
#[no_mangle]
pub unsafe extern "C" fn sightline_engine_step(
    e: *mut SightlineEngine,
    dt_s: f64,
    dets: *const SightlineDetectionC,
    n: usize,
    out: *mut SightlineTrackC,
    cap: usize,
    out_n: *mut usize,
) -> i32 {
    guard(|| step_impl((e, dets, n), dt_s, (out, cap, out_n)))
}

fn pose_valid(p: &SightlinePoseC, dem: bool, ground_up_m: f64) -> bool {
    let f = [
        p.roll_rad,
        p.pitch_rad,
        p.yaw_rad,
        p.cam_up_m,
        p.ref_lat_deg,
        p.ref_lon_deg,
        p.ref_hae_m,
    ];
    if !f.iter().all(|x| x.is_finite()) {
        return false;
    }
    if p.ref_lat_deg.abs() > 90.0 || p.ref_lon_deg.abs() > 180.0 {
        return false;
    }
    if !dem && p.cam_up_m <= ground_up_m {
        return false;
    }
    true
}

/// # Safety
/// Pointers must be null (with zero length) or valid for the stated lengths.
#[no_mangle]
pub unsafe extern "C" fn sightline_engine_step_pose(
    e: *mut SightlineEngine,
    dt_s: f64,
    pose: *const SightlinePoseC,
    dets: *const SightlineDetectionC,
    n: usize,
    out: *mut SightlineTrackC,
    cap: usize,
    out_n: *mut usize,
) -> i32 {
    guard(|| {
        if e.is_null() || pose.is_null() {
            return SIGHTLINE_ERR_NULL;
        }
        if !pointer_is_aligned(e) || !pointer_is_aligned(pose) {
            return SIGHTLINE_ERR_INVALID;
        }
        let p = &*pose;
        let eng = &mut *e;
        if !pose_valid(p, eng.dem.is_some(), eng.cfg.ground_up_m) {
            return SIGHTLINE_ERR_INVALID;
        }
        eng.pose = CameraPose {
            roll_rad: p.roll_rad,
            pitch_rad: p.pitch_rad,
            yaw_rad: p.yaw_rad,
            cam_up_m: p.cam_up_m,
        };
        eng.cfg.roll_rad = p.roll_rad;
        eng.cfg.pitch_rad = p.pitch_rad;
        eng.cfg.yaw_rad = p.yaw_rad;
        eng.cfg.cam_up_m = p.cam_up_m;
        eng.cfg.ref_lat_deg = p.ref_lat_deg;
        eng.cfg.ref_lon_deg = p.ref_lon_deg;
        eng.cfg.ref_hae_m = p.ref_hae_m;
        step_impl((e, dets, n), dt_s, (out, cap, out_n))
    })
}

/// # Safety
/// Pointers must be null (with zero length) or valid for the stated lengths.
#[no_mangle]
pub unsafe extern "C" fn sightline_engine_set_dem(
    e: *mut SightlineEngine,
    origin_east_m: f64,
    origin_north_m: f64,
    cell_size_m: f64,
    rows: usize,
    cols: usize,
    elevations: *const f64,
) -> i32 {
    guard(|| {
        if e.is_null() {
            return SIGHTLINE_ERR_NULL;
        }
        if !pointer_is_aligned(e) {
            return SIGHTLINE_ERR_INVALID;
        }
        let total = match rows.checked_mul(cols) {
            Some(n) => n,
            None => return SIGHTLINE_ERR_INVALID,
        };
        if total == 0 {
            return SIGHTLINE_ERR_INVALID;
        }
        if elevations.is_null() {
            return SIGHTLINE_ERR_NULL;
        }
        if !layout_ok(elevations, total) {
            return SIGHTLINE_ERR_INVALID;
        }
        let elevs = std::slice::from_raw_parts(elevations, total).to_vec();
        match DemGrid::new(
            origin_east_m,
            origin_north_m,
            cell_size_m,
            cols,
            rows,
            elevs,
        ) {
            Ok(grid) => {
                (*e).dem = Some(grid);
                SIGHTLINE_OK
            }
            Err(_) => SIGHTLINE_ERR_INVALID,
        }
    })
}

/// # Safety
/// `e` must be null or valid.
#[no_mangle]
pub unsafe extern "C" fn sightline_engine_clear_dem(e: *mut SightlineEngine) -> i32 {
    guard(|| {
        if e.is_null() {
            return SIGHTLINE_ERR_NULL;
        }
        if !pointer_is_aligned(e) {
            return SIGHTLINE_ERR_INVALID;
        }
        (*e).dem = None;
        SIGHTLINE_OK
    })
}

const STALE_OFFSET_MS: i64 = 30_000;

fn to_cot(t: &SightlineTrackC, cot_type: &str) -> CotEvent {
    CotEvent {
        uid: format!("sightline-{}", t.id),
        cot_type: cot_type.to_string(),
        how: "m-g".to_string(),
        time_ms: 0,
        start_ms: 0,
        stale_ms: STALE_OFFSET_MS,
        lat_deg: t.lat_deg,
        lon_deg: t.lon_deg,
        hae_m: t.hae_m,
        ce_m: t.ce90_m,
        le_m: t.le90_m,
        course_deg: t.course_deg,
        speed_mps: t.speed_mps,
    }
}

fn track_valid(t: &SightlineTrackC) -> bool {
    let f = [
        t.lat_deg,
        t.lon_deg,
        t.hae_m,
        t.ce90_m,
        t.le90_m,
        t.course_deg,
        t.speed_mps,
    ];
    f.iter().all(|x| x.is_finite())
}

fn cot_type_valid(cot_type: &str) -> bool {
    let trimmed = cot_type.trim();
    if trimmed.is_empty() {
        return false;
    }
    for c in trimmed.chars() {
        if c == '<' || c == '>' || c == '&' || c == '"' || c == '\'' || c.is_control() {
            return false;
        }
        if !c.is_ascii_alphanumeric() && c != '-' && c != '.' && c != '_' {
            return false;
        }
    }
    true
}

/// Returns a static NUL-terminated CoT type string for the given affiliation and category.
#[no_mangle]
pub extern "C" fn sightline_cot_type_from_class(affiliation: u32, category: u32) -> *const c_char {
    let aff = match affiliation {
        0 => TacticalAffiliation::Friendly,
        1 => TacticalAffiliation::Hostile,
        2 => TacticalAffiliation::Neutral,
        _ => TacticalAffiliation::Unknown,
    };
    let cat = match category {
        0 => TargetCategory::Vehicle,
        1 => TargetCategory::Person,
        2 => TargetCategory::Drone,
        3 => TargetCategory::Aircraft,
        4 => TargetCategory::Vessel,
        _ => TargetCategory::Unknown,
    };
    let cls = MilStdClassifier::new(aff, cat);
    let s = cls.to_cot_type();
    // Static null-terminated strings
    match s {
        "a-f-G-E-V-C" => b"a-f-G-E-V-C\0".as_ptr().cast::<c_char>(),
        "a-h-G-E-V" => b"a-h-G-E-V\0".as_ptr().cast::<c_char>(),
        "a-f-G-E-S" => b"a-f-G-E-S\0".as_ptr().cast::<c_char>(),
        "a-h-G-E-S" => b"a-h-G-E-S\0".as_ptr().cast::<c_char>(),
        "a-f-A-M-F" => b"a-f-A-M-F\0".as_ptr().cast::<c_char>(),
        "a-h-A-M-F" => b"a-h-A-M-F\0".as_ptr().cast::<c_char>(),
        "a-f-A" => b"a-f-A\0".as_ptr().cast::<c_char>(),
        "a-h-A" => b"a-h-A\0".as_ptr().cast::<c_char>(),
        "a-f-S" => b"a-f-S\0".as_ptr().cast::<c_char>(),
        "a-h-S" => b"a-h-S\0".as_ptr().cast::<c_char>(),
        "a-n-G-E-V" => b"a-n-G-E-V\0".as_ptr().cast::<c_char>(),
        "a-n-G-E-S" => b"a-n-G-E-S\0".as_ptr().cast::<c_char>(),
        "a-n-G" => b"a-n-G\0".as_ptr().cast::<c_char>(),
        _ => b"a-u-G\0".as_ptr().cast::<c_char>(),
    }
}

/// # Safety
/// `t` must be valid; `buf` must be writable for `cap` bytes.
#[no_mangle]
pub unsafe extern "C" fn sightline_track_to_cot_xml(
    t: *const SightlineTrackC,
    time_ms: i64,
    buf: *mut c_char,
    cap: usize,
) -> i32 {
    sightline_track_to_cot_xml_typed(t, time_ms, b"a-u-G\0".as_ptr().cast::<c_char>(), buf, cap)
}

/// # Safety
/// `t` and `cot_type` must be valid; `buf` must be writable for `cap` bytes.
#[no_mangle]
pub unsafe extern "C" fn sightline_track_to_cot_xml_typed(
    t: *const SightlineTrackC,
    time_ms: i64,
    cot_type: *const c_char,
    buf: *mut c_char,
    cap: usize,
) -> i32 {
    guard(|| {
        if t.is_null() || buf.is_null() || cot_type.is_null() {
            return SIGHTLINE_ERR_NULL;
        }
        if !pointer_is_aligned(t) || !track_valid(&*t) {
            return SIGHTLINE_ERR_INVALID;
        }
        let ty_str = match std::ffi::CStr::from_ptr(cot_type).to_str() {
            Ok(s) => s,
            Err(_) => return SIGHTLINE_ERR_INVALID,
        };
        if !cot_type_valid(ty_str) {
            return SIGHTLINE_ERR_INVALID;
        }
        let mut ev = to_cot(&*t, ty_str);
        ev.time_ms = time_ms;
        ev.start_ms = time_ms;
        ev.stale_ms = time_ms.saturating_add(STALE_OFFSET_MS);
        let xml = ev.to_xml();
        if xml.len() + 1 > cap {
            return SIGHTLINE_ERR_BUFFER;
        }
        std::ptr::copy_nonoverlapping(xml.as_ptr(), buf.cast::<u8>(), xml.len());
        *buf.add(xml.len()) = 0;
        SIGHTLINE_OK
    })
}

/// # Safety
/// `t` and `cot_type` must be valid; `buf` must be writable for `cap` bytes; `out_len` must be writable.
#[no_mangle]
pub unsafe extern "C" fn sightline_track_to_tak_proto(
    t: *const SightlineTrackC,
    time_ms: i64,
    cot_type: *const c_char,
    buf: *mut u8,
    cap: usize,
    out_len: *mut usize,
) -> i32 {
    guard(|| {
        if t.is_null() || cot_type.is_null() || out_len.is_null() || (cap > 0 && buf.is_null()) {
            return SIGHTLINE_ERR_NULL;
        }
        if !pointer_is_aligned(t) || !pointer_is_aligned(out_len) || !track_valid(&*t) {
            return SIGHTLINE_ERR_INVALID;
        }
        let ty_str = match std::ffi::CStr::from_ptr(cot_type).to_str() {
            Ok(s) => s,
            Err(_) => return SIGHTLINE_ERR_INVALID,
        };
        if !cot_type_valid(ty_str) {
            return SIGHTLINE_ERR_INVALID;
        }
        let mut ev = to_cot(&*t, ty_str);
        ev.time_ms = time_ms;
        ev.start_ms = time_ms;
        ev.stale_ms = time_ms.saturating_add(STALE_OFFSET_MS);
        let tak_ev = TakCotEvent::from(&ev);
        let mut packet = Vec::new();
        if encode_tak_packet(&tak_ev, &mut packet).is_err() {
            return SIGHTLINE_ERR_INVALID;
        }
        *out_len = packet.len();
        if packet.len() > cap {
            return SIGHTLINE_ERR_BUFFER;
        }
        std::ptr::copy_nonoverlapping(packet.as_ptr(), buf, packet.len());
        SIGHTLINE_OK
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg() -> SightlineCameraCfg {
        SightlineCameraCfg {
            fx: 1000.0,
            fy: 1000.0,
            cx: 500.0,
            cy: 500.0,
            k1: 0.0,
            k2: 0.0,
            is_fisheye: 0,
            roll_rad: 0.0,
            pitch_rad: 0.0,
            yaw_rad: 0.0,
            cam_up_m: 100.0,
            ground_up_m: 0.0,
            pixel_sigma_px: 1.0,
            attitude_sigma_rad: 0.001,
            ref_lat_deg: 37.0,
            ref_lon_deg: -122.0,
            ref_hae_m: 10.0,
            min_hits: 1,
            max_age: 3,
            q_acc: 0.5,
        }
    }

    fn det() -> SightlineDetectionC {
        SightlineDetectionC {
            u: 520.0,
            v: 480.0,
            x1: 500.0,
            y1: 460.0,
            x2: 540.0,
            y2: 500.0,
            embedding: [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        }
    }

    #[test]
    fn test_full_lifecycle_and_cot_output() {
        let c = cfg();
        unsafe {
            let e = sightline_engine_new(&c);
            assert!(!e.is_null());
            let mut out = [SightlineTrackC::default(); 4];
            let mut n = 0usize;
            let d = det();
            let rc = sightline_engine_step(e, 0.1, &d, 1, out.as_mut_ptr(), 4, &mut n);
            assert_eq!(rc, SIGHTLINE_OK);
            assert_eq!(n, 1);
            let mut buf = vec![0 as c_char; 1024];
            let rc = sightline_track_to_cot_xml(&out[0], 1_700_000_000_000, buf.as_mut_ptr(), 1024);
            assert_eq!(rc, SIGHTLINE_OK);
            let s = std::ffi::CStr::from_ptr(buf.as_ptr()).to_str().unwrap();
            assert!(s.starts_with("<event") && s.contains("version=\"2.0\""));
            sightline_engine_free(e);
        }
    }

    #[test]
    fn test_null_and_invalid_rejection() {
        unsafe {
            assert!(sightline_engine_new(std::ptr::null()).is_null());
            let mut bad = cfg();
            bad.fx = f64::NAN;
            assert!(sightline_engine_new(&bad).is_null());
            let mut n = 0usize;
            let rc = sightline_engine_step(
                std::ptr::null_mut(),
                0.1,
                std::ptr::null(),
                0,
                std::ptr::null_mut(),
                0,
                &mut n,
            );
            assert_eq!(rc, SIGHTLINE_ERR_NULL);
            sightline_engine_free(std::ptr::null_mut());
        }
    }

    #[test]
    fn test_small_buffers_are_rejected() {
        let c = cfg();
        unsafe {
            let e = sightline_engine_new(&c);
            let mut n = 0usize;
            let d = det();
            let rc = sightline_engine_step(e, 0.1, &d, 1, std::ptr::null_mut(), 0, &mut n);
            assert_eq!(rc, SIGHTLINE_ERR_BUFFER);
            assert_eq!(n, 1);
            let t = SightlineTrackC::default();
            let mut tiny = [0 as c_char; 8];
            let rc = sightline_track_to_cot_xml(&t, 0, tiny.as_mut_ptr(), 8);
            assert_eq!(rc, SIGHTLINE_ERR_BUFFER);
            sightline_engine_free(e);
        }
    }
    #[test]
    fn test_panic_is_caught() {
        assert_eq!(guard(|| -> i32 { panic!("boom") }), SIGHTLINE_ERR_PANIC);
    }
    #[test]
    fn test_invalid_config_fields_rejected() {
        let mut c = cfg();
        c.ref_lat_deg = 95.0;
        assert!(unsafe { sightline_engine_new(&c) }.is_null());
        let mut c = cfg();
        c.is_fisheye = 2;
        assert!(unsafe { sightline_engine_new(&c) }.is_null());
    }
    #[test]
    fn test_nan_pixel_is_skipped_not_fatal() {
        let c = cfg();
        let mut d = det();
        d.u = f64::NAN;
        let (mut out, mut n) = ([SightlineTrackC::default(); 2], 9usize);
        unsafe {
            let e = sightline_engine_new(&c);
            let rc = sightline_engine_step(e, 0.1, &d, 1, out.as_mut_ptr(), 2, &mut n);
            assert_eq!((rc, n), (SIGHTLINE_OK, 0));
            sightline_engine_free(e);
        }
    }
    #[test]
    fn test_misaligned_pointer_rejected() {
        let c = cfg();
        let d = [det(), det()];
        let (mut out, mut n) = ([SightlineTrackC::default(); 2], 0usize);
        unsafe {
            let e = sightline_engine_new(&c);
            let bad = (d.as_ptr() as *const u8).add(1) as *const SightlineDetectionC;
            let rc = sightline_engine_step(e, 0.1, bad, 1, out.as_mut_ptr(), 2, &mut n);
            assert_eq!(rc, SIGHTLINE_ERR_INVALID);
            sightline_engine_free(e);
        }
    }
    #[test]
    fn test_consecutive_steps_keep_id_and_numbers() {
        let c = cfg();
        let (d, mut out, mut n) = (det(), [SightlineTrackC::default(); 2], 0usize);
        unsafe {
            let e = sightline_engine_new(&c);
            let rc = sightline_engine_step(e, 0.1, &d, 1, std::ptr::null_mut(), 0, &mut n);
            assert_eq!((rc, n), (SIGHTLINE_ERR_BUFFER, 1));
            let rc = sightline_engine_step(e, 0.1, &d, 1, out.as_mut_ptr(), 2, &mut n);
            assert_eq!((rc, n, out[0].id), (SIGHTLINE_OK, 1, 1));
            assert!((out[0].lat_deg - 37.0).abs() < 0.01 && (out[0].lon_deg + 122.0).abs() < 0.01);
            assert!(out[0].ce90_m > 0.0 && out[0].ce90_m.is_finite());
            assert!(
                out[0].le90_m > 0.0
                    && out[0].le90_m.is_finite()
                    && out[0].le90_m != SIGHTLINE_UNKNOWN_ERROR_M
            );
            sightline_engine_free(e);
        }
    }
    #[test]
    fn test_misaligned_cfg_and_track_rejected() {
        let words = [0u64; 40];
        let bad = (words.as_ptr() as *const u8).wrapping_add(1);
        let mut buf = [0 as c_char; 8];
        unsafe {
            assert!(sightline_engine_new(bad as *const SightlineCameraCfg).is_null());
            let rc =
                sightline_track_to_cot_xml(bad as *const SightlineTrackC, 0, buf.as_mut_ptr(), 8);
            assert_eq!(rc, SIGHTLINE_ERR_INVALID);
        }
    }
}
