use sightline_cot::cot::CotTcpStreamer;
use std::sync::mpsc;
use std::time::Duration;

#[test]
fn send_raw_returns_when_the_peer_stops_reading() {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let mut streamer = CotTcpStreamer::new(addr).unwrap();
        streamer.set_write_timeout(Duration::from_millis(100));
        let payload = vec![b'x'; 1024 * 1024];
        let all_ok = (0..256).all(|_| streamer.send_raw(&payload).is_ok());
        let _ = tx.send(all_ok);
    });
    let (_peer, _) = listener.accept().unwrap();
    let outcome = rx.recv_timeout(Duration::from_secs(5));
    assert_eq!(
        outcome,
        Ok(false),
        "send_raw must fail with a timeout instead of blocking"
    );
}
