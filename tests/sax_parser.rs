use sightline_cot::sax_parser::{CotChunkStreamer, CotSaxParser};

#[test]
fn test_integration_full_cot_sax_stream() {
    let raw_stream = r#"
        <?xml version="1.0" standalone="yes"?>
        <event version="2.0" uid="DRONE-99" type="a-f-A-M-F" how="m-g" time="2026-02-16T12:00:00.000Z" start="2026-02-16T12:00:00.000Z" stale="2026-02-16T12:05:00.000Z">
            <!-- Sensor tracking data -->
            <point lat="32.7767" lon="-96.7970" hae="250.0" ce="5.0" le="10.0"/>
            <detail>
                <contact callsign="REAPER-01"/>
            </detail>
        </event>
        <event version="2.0" uid="DRONE-100" type="a-f-A-M-F" how="m-g" time="2026-02-16T12:00:01.000Z" start="2026-02-16T12:00:01.000Z" stale="2026-02-16T12:05:01.000Z">
            <point lat="32.7770" lon="-96.7965" hae="252.0" ce="5.0" le="10.0"/>
            <detail>
                <contact callsign="REAPER-02"/>
            </detail>
        </event>
    "#;

    let events = CotSaxParser::parse_events(raw_stream).expect("failed parsing events");
    assert_eq!(events.len(), 2);

    assert_eq!(events[0].uid, "DRONE-99");
    assert_eq!(events[0].callsign, Some("REAPER-01"));
    let pt0 = events[0].point.unwrap();
    assert!((pt0.lat - 32.7767).abs() < 1e-4);

    assert_eq!(events[1].uid, "DRONE-100");
    assert_eq!(events[1].callsign, Some("REAPER-02"));
    let pt1 = events[1].point.unwrap();
    assert!((pt1.lat - 32.7770).abs() < 1e-4);
}

#[test]
fn test_integration_chunk_streamer_reassembly() {
    let mut streamer = CotChunkStreamer::new();
    let parts = [
        "<ev",
        "ent version=\"2.0\" uid=\"T1\" type=\"a-u-G\" how=\"h-e\" time=\"2026-02-16T12:00:00.000Z\" start=\"2026-02-16T12:00:00.000Z\" stale=\"2026-02-16T12:01:00.000Z\">",
        "<point lat=\"1.0\" lon=\"2.0\" hae=\"3.0\" ce=\"4.0\" le=\"5.0\"/>",
        "</event>",
    ];

    let mut all_events = Vec::new();
    for part in parts {
        let batch = streamer.feed(part);
        for item in batch {
            all_events.push(item);
        }
    }

    assert_eq!(all_events.len(), 1);
    let parsed = CotSaxParser::parse_events(&all_events[0]).unwrap();
    assert_eq!(parsed.len(), 1);
    assert_eq!(parsed[0].uid, "T1");
}
