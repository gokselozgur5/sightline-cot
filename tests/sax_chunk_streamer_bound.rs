use sightline_cot::sax_parser::CotChunkStreamer;

#[test]
fn unterminated_event_does_not_grow_the_buffer_without_bound() {
    let mut streamer = CotChunkStreamer::new();
    assert!(streamer.feed("<event uid=\"U\">").is_empty());
    let chunk = "a".repeat(4 * 1024 * 1024);
    let mut fed = 0usize;
    for _ in 0..16 {
        assert!(streamer.feed(&chunk).is_empty());
        fed += chunk.len();
    }
    // 64 MiB arrived without a closing tag; a bounded receiver must have dropped it.
    assert!(streamer.pending_len() < fed, "pending {} of {fed}", streamer.pending_len());
}
