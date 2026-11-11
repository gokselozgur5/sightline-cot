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
