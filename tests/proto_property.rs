//! Deterministic property-style tests for the TAK protobuf codec.

use sightline_cot::proto::{
    decode_tak_packet, decode_varint, encode_tak_packet, encode_varint, ProtoError, TakCotEvent,
};

/// Seeded xorshift64 generator so every run sees the same inputs.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
}
