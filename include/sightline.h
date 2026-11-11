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

typedef struct SightlineEngine SightlineEngine;

typedef struct {
    double fx, fy, cx, cy, k1, k2;
    int32_t is_fisheye;
    double roll_rad, pitch_rad, yaw_rad, cam_up_m, ground_up_m;
    double pixel_sigma_px, attitude_sigma_rad;
    double ref_lat_deg, ref_lon_deg, ref_hae_m;
    uint32_t min_hits, max_age;
    double q_acc;
} SightlineCameraCfg;
