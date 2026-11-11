# sightline-cot

Real-time monocular 3D ranging, multi-target tracking and Cursor-on-Target (CoT) telemetry in pure Rust with a C-ABI.

Zero external dependencies: the crate uses only the Rust standard library (edition 2021) and builds as `rlib`, `cdylib` and `staticlib`.

License: MIT.

## Pipeline

```mermaid
flowchart LR
    A[Pixel + bbox + Re-ID] --> B[Camera model + SO3 ray]
    B --> C[Ground intersection]
    C --> D[WGS84 + CE90/LE90]
    D --> E[Gated Kalman tracker]
    E --> F[CoT 2.0 XML]
    F --> G[UDP dispatcher]
```

## Mathematical model

- Camera: pinhole or equidistant fisheye intrinsics back-project a pixel `(u, v)` into a unit ray. Fisheye undistortion inverts `r_d = theta * (1 + k1*theta^2 + k2*theta^4)` with bounded Newton-Raphson.
- Attitude: roll, pitch and yaw build an SO(3) rotation into local East-North-Up (ENU). Rays with `ray_enu.z >= -1e-6` are rejected as above the horizon.
- Ground range: `t = (ground_up_m - cam_up_m) / ray_enu.z` gives the ENU ground point.
- Uncertainty: `J * Sigma * J^T` propagates pixel and attitude variance into `ce90_m = 2.146 * sqrt(0.5 * (var_e + var_n))` and `le90_m = 1.6449 * sqrt(var_u)`.
- Geodesy: ENU offsets convert to WGS84 (`a = 6378137.0`, `f = 1/298.257223563`) through ECEF.
