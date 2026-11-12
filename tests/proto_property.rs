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

#[test]
fn varint_roundtrip_over_generated_values() {
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    let mut seen = [false; 11];
    let mut buf = [0u8; 10];
    for i in 0..10_000u32 {
        let v = rng.next() >> (i % 64);
        let n = encode_varint(v, &mut buf).unwrap();
        seen[n] = true;
        assert_eq!(decode_varint(&buf[..n]), Ok((v, n)));
    }
    assert!(seen[1..].iter().all(|s| *s), "lengths covered: {seen:?}");
}

#[test]
fn varint_encode_into_undersized_buffer_fails() {
    let mut rng = Rng(0xDEAD_BEEF_CAFE_F00D);
    let mut full = [0u8; 10];
    for i in 0..1_000u32 {
        let v = rng.next() >> (i % 64);
        let n = encode_varint(v, &mut full).unwrap();
        let mut short = vec![0u8; n - 1];
        assert_eq!(
            encode_varint(v, &mut short),
            Err(ProtoError::BufferTooSmall)
        );
    }
}

fn gen_str(rng: &mut Rng) -> String {
    let len = (rng.next() % 24) as usize;
    (0..len)
        .map(|_| (b' ' + (rng.next() % 95) as u8) as char)
        .collect()
}

fn gen_coord(rng: &mut Rng, span: f64) -> f64 {
    (rng.next() >> 11) as f64 / (1u64 << 53) as f64 * 2.0 * span - span
}

fn gen_event(rng: &mut Rng) -> TakCotEvent {
    TakCotEvent {
        r#type: gen_str(rng),
        uid: gen_str(rng),
        send_time: rng.next(),
        start_time: rng.next() >> 20,
        stale_time: rng.next() % 1000,
        how: gen_str(rng),
        lat: gen_coord(rng, 90.0),
        lon: gen_coord(rng, 180.0),
        hae: gen_coord(rng, 10_000.0),
        ce: gen_coord(rng, 50.0),
        le: gen_coord(rng, 50.0),
    }
}

#[test]
fn packet_roundtrip_over_generated_events() {
    let mut rng = Rng(0x0123_4567_89AB_CDEF);
    for _ in 0..2_000 {
        let evt = gen_event(&mut rng);
        let mut packet = Vec::new();
        encode_tak_packet(&evt, &mut packet).unwrap();
        assert_eq!(decode_tak_packet(&packet), Ok(evt));
    }
}
