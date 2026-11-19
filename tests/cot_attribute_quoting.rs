use sightline_cot::cot::CotEvent;

#[test]
fn single_quoted_attributes_are_accepted() {
    let xml = "<?xml version='1.0' encoding='UTF-8' standalone='yes'?>\
        <event version='2.0' uid='ANDROID-1' type='a-f-G-U-C' \
        time='2026-02-16T12:00:00.000Z' start='2026-02-16T12:00:00.000Z' \
        stale='2026-02-16T12:05:00.000Z' how='h-e'>\
        <point lat='38.5' lon='-77.25' hae='10.0' ce='5.0' le='5.0'/><detail/></event>";
    let ev = CotEvent::from_xml(xml).expect("XML 1.0 allows either quote style");
    assert_eq!(ev.uid, "ANDROID-1");
    assert_eq!(ev.lat_deg, 38.5);
}

#[test]
fn tab_separated_attributes_are_accepted() {
    let xml = "<event\tversion=\"2.0\"\tuid=\"U\"\ttype=\"a-f-G\"\thow=\"m-g\"\
        \ttime=\"2026-02-16T12:00:00.000Z\"\tstart=\"2026-02-16T12:00:00.000Z\"\
        \tstale=\"2026-02-16T12:05:00.000Z\"><point\tlat=\"1\"\tlon=\"2\"\thae=\"0\"\
        \tce=\"1\"\tle=\"1\"/></event>";
    assert_eq!(CotEvent::from_xml(xml).map(|e| e.uid), Ok("U".to_string()));
}
