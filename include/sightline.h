/* Sightline C-ABI. Matches src/ffi.rs exactly. */
#ifndef SIGHTLINE_H
#define SIGHTLINE_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define SIGHTLINE_OK 0
#define SIGHTLINE_ERR_NULL (-1)
#define SIGHTLINE_ERR_INVALID (-2)
#define SIGHTLINE_ERR_BUFFER (-3)
#define SIGHTLINE_ERR_PANIC (-4)
#define SIGHTLINE_ERR_CAMERA (-5)
#define SIGHTLINE_ERR_TRACKER (-6)

#define SIGHTLINE_STATUS_TENTATIVE 0
#define SIGHTLINE_STATUS_CONFIRMED 1
#define SIGHTLINE_STATUS_COASTING 2
#define SIGHTLINE_STATUS_DELETED 3

/* CoT sentinel for an unknown error value (reported in le90_m). */
#define SIGHTLINE_UNKNOWN_ERROR_M 9999999.0

typedef struct SightlineEngine SightlineEngine;

typedef struct {
    /* fx, fy, cx, cy: focal lengths and principal point.
     * k1, k2: distortion coefficients (used only when is_fisheye == 1).
     * is_fisheye: 0 for pinhole (k1/k2 ignored), 1 for equidistant fisheye. */
    double fx, fy, cx, cy, k1, k2;
    int32_t is_fisheye;
    /* roll_rad: roll about optical axis (rad).
     * pitch_rad: pitch elevation angle (rad).
     * yaw_rad: counter-clockwise rotation about ENU Up axis (rad; 0 = East, +pi/2 = North).
     * Note: unlike platform heading/azimuth (clockwise from North), yaw_rad is CCW from East.
     * cam_up_m must be strictly greater than ground_up_m to permit ray-ground intersection. */
    double roll_rad, pitch_rad, yaw_rad, cam_up_m, ground_up_m;
    double pixel_sigma_px, attitude_sigma_rad;
    double ref_lat_deg, ref_lon_deg, ref_hae_m;
    uint32_t min_hits, max_age;
    double q_acc;
} SightlineCameraCfg;

typedef struct {
    double u, v, x1, y1, x2, y2;
    float embedding[8];
} SightlineDetectionC;

typedef struct {
    uint64_t id;
    double lat_deg, lon_deg, hae_m, ce90_m, le90_m, course_deg, speed_mps;
    uint32_t status;
} SightlineTrackC;

typedef struct {
    double roll_rad, pitch_rad, yaw_rad, cam_up_m;
    double ref_lat_deg, ref_lon_deg, ref_hae_m;
} SightlinePoseC;

/* Layout guards (C99): size mismatch yields a negative array size. */
typedef char sightline_assert_cfg[(sizeof(SightlineCameraCfg) == 152) ? 1 : -1];
typedef char sightline_assert_det[(sizeof(SightlineDetectionC) == 80) ? 1 : -1];
typedef char sightline_assert_trk[(sizeof(SightlineTrackC) == 72) ? 1 : -1];
typedef char sightline_assert_pos[(sizeof(SightlinePoseC) == 56) ? 1 : -1];

SightlineEngine *sightline_engine_new(const SightlineCameraCfg *cfg);
void sightline_engine_free(SightlineEngine *e);
/* Not thread-safe: use one caller per engine; free exactly once.
 * step advances the tracker even when SIGHTLINE_ERR_BUFFER is returned;
 * *out_n then holds the full track count and min(cap, count) reports are
 * written. Detections that cannot be located are skipped individually. */
int32_t sightline_engine_step(SightlineEngine *e, double dt_s,
                              const SightlineDetectionC *dets, size_t n,
                              SightlineTrackC *out, size_t cap, size_t *out_n);
int32_t sightline_engine_step_pose(SightlineEngine *e, double dt_s,
                                   const SightlinePoseC *pose,
                                   const SightlineDetectionC *dets, size_t n,
                                   SightlineTrackC *out, size_t cap, size_t *out_n);
int32_t sightline_engine_set_dem(SightlineEngine *e, double origin_east_m,
                                 double origin_north_m, double cell_size_m,
                                 size_t rows, size_t cols, const double *elevations);
int32_t sightline_engine_clear_dem(SightlineEngine *e);
/* The event stale time is time_ms plus 30000 ms (saturating). */
int32_t sightline_track_to_cot_xml(const SightlineTrackC *t, int64_t time_ms,
                                   char *buf, size_t cap);
int32_t sightline_track_to_cot_xml_typed(const SightlineTrackC *t, int64_t time_ms,
                                         const char *cot_type, char *buf, size_t cap);
const char *sightline_cot_type_from_class(uint32_t affiliation, uint32_t category);
int32_t sightline_track_to_tak_proto(const SightlineTrackC *t, int64_t time_ms,
                                     const char *cot_type, uint8_t *buf,
                                     size_t cap, size_t *out_len);

#ifdef __cplusplus
}
#endif

#endif /* SIGHTLINE_H */
