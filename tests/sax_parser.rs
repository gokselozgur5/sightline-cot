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

