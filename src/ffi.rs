//! C-ABI surface for embedding the Sightline engine.

use crate::camera::{enu_to_geodetic, estimate_covariance_ce_le, intersect_ground};
use crate::camera::{CameraPose, Intrinsics};
use crate::cot::CotEvent;
use crate::tracker::{BoundingBox, Detection, MultiTargetTracker, Track};
use std::os::raw::c_char;

pub const SIGHTLINE_OK: i32 = 0;
pub const SIGHTLINE_ERR_NULL: i32 = -1;
pub const SIGHTLINE_ERR_INVALID: i32 = -2;
pub const SIGHTLINE_ERR_BUFFER: i32 = -3;
pub const SIGHTLINE_ERR_PANIC: i32 = -4;
pub const SIGHTLINE_ERR_CAMERA: i32 = -5;
pub const SIGHTLINE_ERR_TRACKER: i32 = -6;

fn guard<F: FnOnce() -> i32>(f: F) -> i32 {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)).unwrap_or(SIGHTLINE_ERR_PANIC)
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

fn build_engine(c: &SightlineCameraCfg) -> Option<SightlineEngine> {
    let ok = c.fx.is_finite() && c.fy.is_finite() && c.fx > 0.0 && c.fy > 0.0;
    if !ok || c.min_hits == 0 || !c.q_acc.is_finite() || c.q_acc < 0.0 {
        return None;
    }
    let tracker = MultiTargetTracker::new(c.min_hits as usize, c.max_age as usize, c.q_acc);
    Some(SightlineEngine {
        cam: make_intrinsics(c),
        pose: make_pose(c),
        cfg: *c,
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
        if cfg.is_null() {
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
    intersect_ground(ray, &eng.pose, eng.cfg.ground_up_m).map_err(|_| SIGHTLINE_ERR_CAMERA)
}

fn covariance(eng: &SightlineEngine, d: &SightlineDetectionC) -> Result<(f64, f64), i32> {
    let c = &eng.cfg;
    estimate_covariance_ce_le(
        &eng.cam,
        &eng.pose,
        d.u,
        d.v,
        c.ground_up_m,
        c.pixel_sigma_px,
        c.attitude_sigma_rad,
    )
    .map_err(|_| SIGHTLINE_ERR_CAMERA)
}

fn to_detection(eng: &SightlineEngine, d: &SightlineDetectionC) -> Result<Detection, i32> {
    let p = locate(eng, d)?;
    let (ce, le) = covariance(eng, d)?;
    let bbox = BoundingBox {
        x1: d.x1,
        y1: d.y1,
        x2: d.x2,
        y2: d.y2,
    };
    Ok(Detection {
        east_m: p[0],
        north_m: p[1],
        hae_m: eng.cfg.ground_up_m,
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
    let ce = (t.p[0][0] + t.p[1][1]).max(0.0).sqrt() * 1.073;
    SightlineTrackC {
        id: t.id,
        lat_deg: lat,
        lon_deg: lon,
        hae_m: hae,
        ce90_m: ce * 2.0,
        le90_m: 0.0,
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
    for d in dets {
        v.push(to_detection(eng, d)?);
    }
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

unsafe fn step_impl(a: StepArgs, dt_s: f64, out: (*mut SightlineTrackC, usize, *mut usize)) -> i32 {
    let ((e, dets, n), (out, cap, out_n)) = (a, out);
    if e.is_null() || out_n.is_null() || (n > 0 && dets.is_null()) || (cap > 0 && out.is_null()) {
        return SIGHTLINE_ERR_NULL;
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
    if tracks.len() > cap {
        return SIGHTLINE_ERR_BUFFER;
    }
    for (i, t) in tracks.iter().enumerate() {
        *out.add(i) = *t;
    }
    SIGHTLINE_OK
}

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
