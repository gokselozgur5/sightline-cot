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
