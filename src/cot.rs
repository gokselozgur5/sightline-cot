//! Zero-dependency Cursor-on-Target 2.0 codec.

use std::fmt;
use std::io::Write;
use std::net::{SocketAddr, TcpStream, ToSocketAddrs, UdpSocket};

/// Non-blocking/reconnecting TCP streamer to a TAK Server endpoint.
#[derive(Debug)]
pub struct CotTcpStreamer {
    server_addr: SocketAddr,
    stream: Option<TcpStream>,
}

/// Errors produced during CoT parsing, validation, or dispatch.
#[derive(Debug, Clone, PartialEq)]
pub enum CotError {
    InvalidXml(String),
    MissingAttribute(&'static str),
    InvalidNumber(&'static str),
    CoordinateOutOfBounds(&'static str),
    NetworkError(String),
}
impl fmt::Display for CotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidXml(m) => write!(f, "invalid XML: {m}"),
            Self::MissingAttribute(a) => write!(f, "missing attribute: {a}"),
            Self::InvalidNumber(a) => write!(f, "invalid number: {a}"),
            Self::CoordinateOutOfBounds(a) => write!(f, "out of bounds: {a}"),
            Self::NetworkError(m) => write!(f, "network error: {m}"),
        }
    }
}

impl std::error::Error for CotError {}
/// Strongly-typed Cursor-on-Target 2.0 event payload.
#[derive(Debug, Clone, PartialEq)]
pub struct CotEvent {
    pub uid: String,
    pub cot_type: String,
    pub how: String,
    pub time_ms: i64,
    pub stale_ms: i64,
    pub lat_deg: f64,
    pub lon_deg: f64,
    pub hae_m: f64,
    pub ce_m: f64,
    pub le_m: f64,
    pub course_deg: f64,
    pub speed_mps: f64,
}
/// Escape XML attribute special characters to prevent XML injection.
pub fn escape_xml_attr(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 8);
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(c),
        }
    }
    out
}
/// Unescape XML attribute entities in a single pass.
pub fn unescape_xml_attr(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(idx) = rest.find('&') {
        out.push_str(&rest[..idx]);
        let rem = &rest[idx..];
        if let Some(semi) = rem.find(';') {
            match &rem[..=semi] {
                "&quot;" => out.push('"'),
                "&apos;" => out.push('\''),
                "&lt;" => out.push('<'),
                "&gt;" => out.push('>'),
                "&amp;" => out.push('&'),
                other => out.push_str(other),
            }
            rest = &rem[semi + 1..];
        } else {
            out.push_str(rem);
            return out;
        }
    }
    out.push_str(rest);
    out
}
fn days_to_ymd(days_since_epoch: i64) -> (i64, u8, u8) {
    let z = days_since_epoch + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u8;
    let m = (if mp < 10 { mp + 3 } else { mp - 9 }) as u8;
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}
/// Format epoch milliseconds into ISO-8601 UTC timestamp `YYYY-MM-DDTHH:MM:SS.mmmZ`.
pub fn format_iso8601_utc(ms: i64) -> String {
    let secs = ms.div_euclid(1000);
    let millis = ms.rem_euclid(1000) as u16;
    let days = secs.div_euclid(86400);
    let day_secs = secs.rem_euclid(86400) as u32;
    let hour = day_secs / 3600;
    let minute = (day_secs % 3600) / 60;
    let second = day_secs % 60;
    let (y, m, d) = days_to_ymd(days);
    format!("{y:04}-{m:02}-{d:02}T{hour:02}:{minute:02}:{second:02}.{millis:03}Z")
}
fn ymd_to_days(y: i64, m: u8, d: u8) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = (y - era * 400) as u64;
    let m = m as u64;
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + d as u64 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe as i64 - 719468
}
fn parse_date_part(s: &str) -> Result<(i64, u8, u8), CotError> {
    let p: Vec<&str> = s.split('-').collect();
    if p.len() != 3 {
        return Err(CotError::InvalidXml(format!("invalid date component: {s}")));
    }
    let y: i64 = p[0].parse().map_err(|_| CotError::InvalidNumber("year"))?;
    let m: u8 = p[1].parse().map_err(|_| CotError::InvalidNumber("month"))?;
    let d: u8 = p[2].parse().map_err(|_| CotError::InvalidNumber("day"))?;
    if !(1..=12).contains(&m) || !(1..=31).contains(&d) {
        return Err(CotError::CoordinateOutOfBounds(
            "month or day out of bounds",
        ));
    }
    let days = ymd_to_days(y, m, d);
    if days_to_ymd(days) != (y, m, d) {
        return Err(CotError::InvalidXml(format!("invalid calendar date: {s}")));
    }
    Ok((y, m, d))
}
fn parse_fraction_ms(s: &str) -> Result<u32, CotError> {
    if s.is_empty() || s.len() > 9 || !s.chars().all(|c| c.is_ascii_digit()) {
        return Err(CotError::InvalidNumber("sec_fraction"));
    }
    let mut padded = [b'0'; 3];
    for (i, b) in s.bytes().take(3).enumerate() {
        padded[i] = b;
    }
    let num_str =
        std::str::from_utf8(&padded).map_err(|_| CotError::InvalidNumber("sec_fraction"))?;
    num_str
        .parse()
        .map_err(|_| CotError::InvalidNumber("sec_fraction"))
}
fn parse_time_part(s: &str) -> Result<(u32, u32, u32, u32), CotError> {
    let p: Vec<&str> = s.split(':').collect();
    if p.len() != 3 {
        return Err(CotError::InvalidXml(format!("invalid time component: {s}")));
    }
    let h: u32 = p[0].parse().map_err(|_| CotError::InvalidNumber("hour"))?;
    let min: u32 = p[1].parse().map_err(|_| CotError::InvalidNumber("min"))?;
    let sp: Vec<&str> = p[2].split('.').collect();
    if sp.len() > 2 {
        return Err(CotError::InvalidNumber("seconds"));
    }
    let sec: u32 = sp[0].parse().map_err(|_| CotError::InvalidNumber("sec"))?;
    if h >= 24 || min >= 60 || sec >= 60 {
        return Err(CotError::CoordinateOutOfBounds("time value out of range"));
    }
    let ms: u32 = if sp.len() == 2 {
        parse_fraction_ms(sp[1])?
    } else {
        0
    };
    Ok((h, min, sec, ms))
}
/// Parse ISO-8601 UTC timestamp `YYYY-MM-DDTHH:MM:SS.mmmZ` to epoch milliseconds.
pub fn parse_iso8601_utc(s: &str) -> Result<i64, CotError> {
    let s = s.trim();
    if s.len() < 20 || !s.ends_with('Z') {
        return Err(CotError::InvalidXml(format!(
            "invalid timestamp format: {s}"
        )));
    }
    let p: Vec<&str> = s[..s.len() - 1].split('T').collect();
    if p.len() != 2 {
        return Err(CotError::InvalidXml(format!("missing T in timestamp: {s}")));
    }
    let clean = |t: &str, extra: &[u8]| t.bytes().all(|b| b.is_ascii_digit() || extra.contains(&b));
    if !clean(p[0], b"-") || !clean(p[1], b":.") {
        return Err(CotError::InvalidXml(format!("invalid timestamp: {s}")));
    }
    let (y, m, d) = parse_date_part(p[0])?;
    let (h, min, sec, ms) = parse_time_part(p[1])?;
    let days = ymd_to_days(y, m, d);
    let total_secs = days * 86400 + h as i64 * 3600 + min as i64 * 60 + sec as i64;
    Ok(total_secs * 1000 + ms as i64)
}
#[cfg(test)]
mod tests_ts_digits {
    use super::*;

    #[test]
    fn signed_components_are_rejected() {
        assert!(parse_iso8601_utc("2026-02-16T+1:00:00.000Z").is_err());
        assert!(parse_iso8601_utc("2026-+2-16T01:00:00.000Z").is_err());
        assert!(parse_iso8601_utc("+026-02-16T01:00:00.000Z").is_err());
        assert!(parse_iso8601_utc("2026-02-16T01:00:+0.000Z").is_err());
        assert!(parse_iso8601_utc("2026-02-16T01:00:00.000Z").is_ok());
    }
}
fn extract_tag_content<'a>(xml: &'a str, tag: &str) -> Option<&'a str> {
    let open_pat = format!("<{tag}");
    let start = xml.find(&open_pat)?;
    let end = xml[start..].find('>')? + start;
    Some(&xml[start..=end])
}

fn find_attr_in_tag<'a>(tag_str: &'a str, attr: &str) -> Option<&'a str> {
    let needle = format!(" {attr}=\"");
    let start = tag_str.find(&needle)? + needle.len();
    let end = tag_str[start..].find('"')? + start;
    Some(&tag_str[start..end])
}
impl CotEvent {
    /// Serialize CotEvent into canonical CoT 2.0 XML.
    pub fn to_xml(&self) -> String {
        let time_str = format_iso8601_utc(self.time_ms);
        let stale_str = format_iso8601_utc(self.stale_ms);
        format!(
            "<event version=\"2.0\" uid=\"{}\" type=\"{}\" how=\"{}\" time=\"{}\" start=\"{}\" stale=\"{}\">\
<point lat=\"{:.7}\" lon=\"{:.7}\" hae=\"{:.2}\" ce=\"{:.2}\" le=\"{:.2}\"/>\
<detail><track course=\"{:.2}\" speed=\"{:.2}\"/></detail></event>",
            escape_xml_attr(&self.uid),
            escape_xml_attr(&self.cot_type),
            escape_xml_attr(&self.how),
            time_str,
            time_str,
            stale_str,
            self.lat_deg,
            self.lon_deg,
            self.hae_m,
            self.ce_m,
            self.le_m,
            self.course_deg,
            self.speed_mps,
        )
    }
}
fn parse_f64_attr(tag_str: &str, attr: &'static str) -> Result<f64, CotError> {
    let val = find_attr_in_tag(tag_str, attr).ok_or(CotError::MissingAttribute(attr))?;
    let num: f64 = val.parse().map_err(|_| CotError::InvalidNumber(attr))?;
    if !num.is_finite() {
        return Err(CotError::InvalidNumber(attr));
    }
    Ok(num)
}

fn parse_opt_f64_attr(
    tag_str: Option<&str>,
    attr: &'static str,
    def: f64,
) -> Result<f64, CotError> {
    match tag_str.and_then(|t| find_attr_in_tag(t, attr)) {
        None => Ok(def),
        Some(val) => {
            let num: f64 = val.parse().map_err(|_| CotError::InvalidNumber(attr))?;
            if !num.is_finite() {
                return Err(CotError::InvalidNumber(attr));
            }
            Ok(num)
        }
    }
}
fn parse_event_header(ev_tag: &str) -> Result<(String, String, String, i64, i64), CotError> {
    let uid = unescape_xml_attr(
        find_attr_in_tag(ev_tag, "uid").ok_or(CotError::MissingAttribute("uid"))?,
    );
    let cot_type = unescape_xml_attr(
        find_attr_in_tag(ev_tag, "type").ok_or(CotError::MissingAttribute("type"))?,
    );
    let how = unescape_xml_attr(
        find_attr_in_tag(ev_tag, "how").ok_or(CotError::MissingAttribute("how"))?,
    );
    let time_str = find_attr_in_tag(ev_tag, "time").ok_or(CotError::MissingAttribute("time"))?;
    let stale_str = find_attr_in_tag(ev_tag, "stale").ok_or(CotError::MissingAttribute("stale"))?;
    let time_ms = parse_iso8601_utc(time_str)?;
    let stale_ms = parse_iso8601_utc(stale_str)?;
    Ok((uid, cot_type, how, time_ms, stale_ms))
}
fn validate_cot_bounds(lat: f64, lon: f64, ce: f64, le: f64, spd: f64) -> Result<(), CotError> {
    if !(-90.0..=90.0).contains(&lat) {
        return Err(CotError::CoordinateOutOfBounds(
            "lat must be in [-90.0, 90.0]",
        ));
    }
    if !(-180.0..=180.0).contains(&lon) {
        return Err(CotError::CoordinateOutOfBounds(
            "lon must be in [-180.0, 180.0]",
        ));
    }
    if ce < 0.0 || le < 0.0 {
        return Err(CotError::CoordinateOutOfBounds("ce and le must be >= 0.0"));
    }
    if spd < 0.0 {
        return Err(CotError::CoordinateOutOfBounds("speed must be >= 0.0"));
    }
    Ok(())
}
struct PointTrack(f64, f64, f64, f64, f64, f64, f64);

fn parse_point_and_track(pt: &str, tr: Option<&str>) -> Result<PointTrack, CotError> {
    let (lat, lon) = (parse_f64_attr(pt, "lat")?, parse_f64_attr(pt, "lon")?);
    let (hae, ce, le) = (
        parse_f64_attr(pt, "hae")?,
        parse_f64_attr(pt, "ce")?,
        parse_f64_attr(pt, "le")?,
    );
    let (crs, spd) = (
        parse_opt_f64_attr(tr, "course", 0.0)?,
        parse_opt_f64_attr(tr, "speed", 0.0)?,
    );
    validate_cot_bounds(lat, lon, ce, le, spd)?;
    Ok(PointTrack(lat, lon, hae, ce, le, crs, spd))
}
fn extract_core_elements(xml: &str) -> Result<(&str, &str, Option<&str>), CotError> {
    if !xml.contains("</event>") {
        return Err(CotError::InvalidXml("missing </event> closing tag".into()));
    }
    let ev = extract_tag_content(xml, "event")
        .ok_or_else(|| CotError::InvalidXml("missing event tag".into()))?;
    let pt = extract_tag_content(xml, "point")
        .ok_or_else(|| CotError::InvalidXml("missing point tag".into()))?;
    let tr = extract_tag_content(xml, "track");
    Ok((ev, pt, tr))
}
impl CotEvent {
    /// Parse and validate a CoT 2.0 XML string.
    pub fn from_xml(xml: &str) -> Result<Self, CotError> {
        let (ev, pt, tr) = extract_core_elements(xml)?;
        let (uid, cot_type, how, time_ms, stale_ms) = parse_event_header(ev)?;
        let PointTrack(lat_deg, lon_deg, hae_m, ce_m, le_m, course_deg, speed_mps) =
            parse_point_and_track(pt, tr)?;
        Ok(Self {
            uid,
            cot_type,
            how,
            time_ms,
            stale_ms,
            lat_deg,
            lon_deg,
            hae_m,
            ce_m,
            le_m,
            course_deg,
            speed_mps,
        })
    }
}
/// Non-blocking UDP dispatcher for CoT XML messages.
pub struct CotUdpDispatcher {
    socket: UdpSocket,
    target: SocketAddr,
}
impl CotUdpDispatcher {
    /// Create a new non-blocking UDP dispatcher bound to an ephemeral port.
    pub fn new(bind_addr: &str, target: SocketAddr) -> Result<Self, CotError> {
        let socket =
            UdpSocket::bind(bind_addr).map_err(|e| CotError::NetworkError(e.to_string()))?;
        socket
            .set_nonblocking(true)
            .map_err(|e| CotError::NetworkError(e.to_string()))?;
        Ok(Self { socket, target })
    }

    /// Access underlying socket reference.
    pub fn socket(&self) -> &UdpSocket {
        &self.socket
    }
}
impl CotUdpDispatcher {
    /// Send raw XML string to target address.
    pub fn send_xml(&self, xml: &str) -> Result<usize, CotError> {
        self.socket
            .send_to(xml.as_bytes(), self.target)
            .map_err(|e| CotError::NetworkError(e.to_string()))
    }

    /// Serialize and dispatch a CotEvent over UDP.
    pub fn send_event(&self, event: &CotEvent) -> Result<usize, CotError> {
        let xml = event.to_xml();
        self.send_xml(&xml)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iso8601_roundtrip() {
        let ms = 1731312345678_i64;
        let ts = format_iso8601_utc(ms);
        assert!(ts.ends_with('Z'));
        let parsed = parse_iso8601_utc(&ts).expect("valid timestamp");
        assert_eq!(ms, parsed);
    }
}
#[cfg(test)]
mod tests_calendar {
    use super::*;

    #[test]
    fn test_leap_years_and_negative_epoch() {
        assert!(parse_iso8601_utc("2000-02-29T12:00:00.000Z").is_ok());
        assert!(parse_iso8601_utc("2024-02-29T12:00:00.000Z").is_ok());
        assert!(parse_iso8601_utc("2023-02-29T12:00:00.000Z").is_err());
        let neg_ms = -1_i64;
        let neg_ts = format_iso8601_utc(neg_ms);
        assert_eq!(neg_ts, "1969-12-31T23:59:59.999Z");
        assert_eq!(parse_iso8601_utc(&neg_ts).unwrap(), neg_ms);
    }
}
#[cfg(test)]
mod tests_escape {
    use super::*;

    #[test]
    fn test_xml_injection_escaping() {
        let event = CotEvent {
            uid: "TARGET-1&<>'\"".into(),
            cot_type: "a-f-G-U-C\"".into(),
            how: "m-g&".into(),
            time_ms: 1000,
            stale_ms: 2000,
            lat_deg: 41.0123456,
            lon_deg: 28.9765432,
            hae_m: 100.5,
            ce_m: 10.0,
            le_m: 15.0,
            course_deg: 180.0,
            speed_mps: 25.0,
        };
        let xml = event.to_xml();
        assert!(xml.contains("&amp;&lt;&gt;&apos;&quot;"));
        let parsed = CotEvent::from_xml(&xml).expect("roundtrip parse");
        assert_eq!(parsed.uid, "TARGET-1&<>'\"");
        assert_eq!(parsed.cot_type, "a-f-G-U-C\"");
        assert_eq!(parsed.how, "m-g&");
    }
}
#[cfg(test)]
mod tests_bounds {
    use super::*;

    #[test]
    fn test_validation_out_of_bounds() {
        let base_xml = |lat: &str, lon: &str, ce: &str, le: &str, crs: &str, spd: &str| {
            format!(
                "<event version=\"2.0\" uid=\"t1\" type=\"a-f-G\" how=\"m-g\" \
                time=\"2026-02-16T12:00:00.000Z\" start=\"2026-02-16T12:00:00.000Z\" \
                stale=\"2026-02-16T12:01:00.000Z\">\
                <point lat=\"{lat}\" lon=\"{lon}\" hae=\"10.0\" ce=\"{ce}\" le=\"{le}\"/>\
                <detail><track course=\"{crs}\" speed=\"{spd}\"/></detail></event>"
            )
        };
        assert!(CotEvent::from_xml(&base_xml("91.0", "0.0", "1.0", "1.0", "0.0", "0.0")).is_err());
        assert!(CotEvent::from_xml(&base_xml("-91.0", "0.0", "1.0", "1.0", "0.0", "0.0")).is_err());
        assert!(CotEvent::from_xml(&base_xml("0.0", "181.0", "1.0", "1.0", "0.0", "0.0")).is_err());
        assert!(CotEvent::from_xml(&base_xml("0.0", "0.0", "-0.1", "1.0", "0.0", "0.0")).is_err());
        assert!(CotEvent::from_xml(&base_xml("0.0", "0.0", "1.0", "-0.1", "0.0", "0.0")).is_err());
        assert!(CotEvent::from_xml(&base_xml("0.0", "0.0", "1.0", "1.0", "NaN", "0.0")).is_err());
        assert!(CotEvent::from_xml(&base_xml("0.0", "0.0", "1.0", "1.0", "0.0", "-1.0")).is_err());
        assert!(CotEvent::from_xml("not xml").is_err());
    }
}
impl CotTcpStreamer {
    pub fn new<A: ToSocketAddrs>(addr: A) -> Result<Self, CotError> {
        let server_addr = addr
            .to_socket_addrs()
            .map_err(|e| CotError::NetworkError(e.to_string()))?
            .next()
            .ok_or_else(|| CotError::NetworkError("address resolution failed".into()))?;
        Ok(Self {
            server_addr,
            stream: None,
        })
    }

    pub fn server_addr(&self) -> SocketAddr {
        self.server_addr
    }
}

impl CotTcpStreamer {
    pub fn is_connected(&self) -> bool {
        self.stream.is_some()
    }

    pub fn connect(&mut self) -> Result<(), CotError> {
        match TcpStream::connect_timeout(&self.server_addr, std::time::Duration::from_millis(500)) {
            Ok(s) => {
                let _ = s.set_nonblocking(false);
                let _ = s.set_nodelay(true);
                self.stream = Some(s);
                Ok(())
            }
            Err(e) => {
                self.stream = None;
                Err(CotError::NetworkError(e.to_string()))
            }
        }
    }
}

impl CotTcpStreamer {
    pub fn send_event(&mut self, event: &CotEvent) -> Result<(), CotError> {
        let xml = event.to_xml();
        self.send_raw(xml.as_bytes())
    }

    pub fn send_raw(&mut self, payload: &[u8]) -> Result<(), CotError> {
        if self.stream.is_none() {
            self.connect()?;
        }
        let stream = self.stream.as_mut().unwrap();
        if let Err(e) = stream.write_all(payload).and_then(|_| stream.flush()) {
            self.stream = None;
            return Err(CotError::NetworkError(e.to_string()));
        }
        Ok(())
    }
}

impl CotTcpStreamer {
    pub fn generate_sa_heartbeat(
        uid: &str,
        time_ms: i64,
        lat_deg: f64,
        lon_deg: f64,
        hae_m: f64,
        speed_mps: f64,
        course_deg: f64,
    ) -> CotEvent {
        CotEvent {
            uid: uid.to_string(),
            cot_type: "a-f-G-U-C".to_string(),
            how: "m-g".to_string(),
            time_ms,
            stale_ms: time_ms + 30_000,
            lat_deg,
            lon_deg,
            hae_m,
            ce_m: 5.0,
            le_m: 10.0,
            course_deg,
            speed_mps,
        }
    }
}

#[cfg(test)]
mod tests_udp_rt {
    use super::*;

    pub(crate) fn sample_drone() -> CotEvent {
        CotEvent {
            uid: "DRONE-99".into(),
            cot_type: "a-f-A-M-F".into(),
            how: "m-g".into(),
            time_ms: 1771234567890,
            stale_ms: 1771234597890,
            lat_deg: 37.7749,
            lon_deg: -122.4194,
            hae_m: 125.5,
            ce_m: 4.2,
            le_m: 6.8,
            course_deg: 135.0,
            speed_mps: 18.5,
        }
    }
}
#[test]
fn test_udp_dispatcher_full_roundtrip() {
    let receiver = UdpSocket::bind("127.0.0.1:0").expect("bind receiver");
    let recv_addr = receiver.local_addr().expect("receiver local addr");
    receiver
        .set_read_timeout(Some(std::time::Duration::from_secs(2)))
        .expect("timeout");
    let dispatcher = CotUdpDispatcher::new("127.0.0.1:0", recv_addr).expect("create dispatcher");
    let evt = tests_udp_rt::sample_drone();
    dispatcher.send_event(&evt).expect("send event");
    let mut buf = [0u8; 2048];
    let (len, _) = receiver.recv_from(&mut buf).expect("recv udp");
    let decoded = CotEvent::from_xml(std::str::from_utf8(&buf[..len]).unwrap()).unwrap();
    assert_eq!(decoded.uid, "DRONE-99");
    assert_eq!(decoded.cot_type, "a-f-A-M-F");
    assert_eq!(decoded.time_ms, 1771234567890);
    assert!((decoded.lat_deg - 37.7749).abs() < 1e-6);
    assert!((decoded.lon_deg - (-122.4194)).abs() < 1e-6);
    assert!((decoded.speed_mps - 18.5).abs() < 1e-2);
}

#[test]
fn test_sa_heartbeat_generation() {
    let hb = CotTcpStreamer::generate_sa_heartbeat(
        "ALPHA-1",
        1700000000000,
        37.5,
        -122.2,
        150.0,
        12.0,
        90.0,
    );
    assert_eq!(hb.uid, "ALPHA-1");
    assert_eq!(hb.cot_type, "a-f-G-U-C");
    assert_eq!(hb.stale_ms, 1700000030000);
    assert_eq!(hb.lat_deg, 37.5);
    assert_eq!(hb.lon_deg, -122.2);
}

#[cfg(test)]
mod test_tcp_helper {
    use std::io::Read;
    use std::net::TcpListener;

    pub fn spawn_mock_server(listener: TcpListener) -> std::thread::JoinHandle<Vec<u8>> {
        std::thread::spawn(move || {
            let (mut sock, _) = listener.accept().expect("accept");
            let mut buf = vec![0u8; 1024];
            let n = sock.read(&mut buf).expect("read");
            buf.truncate(n);
            buf
        })
    }
}

#[test]
fn test_tcp_streamer_loopback_roundtrip() {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind tcp");
    let addr = listener.local_addr().expect("local addr");
    let mut streamer = CotTcpStreamer::new(addr).expect("new streamer");
    let handle = test_tcp_helper::spawn_mock_server(listener);
    let hb = CotTcpStreamer::generate_sa_heartbeat(
        "TEST-SA",
        1700000000000,
        35.0,
        -120.0,
        200.0,
        5.0,
        45.0,
    );
    streamer.send_event(&hb).expect("send event");
    let received = handle.join().expect("join");
    let received_str = std::str::from_utf8(&received).expect("utf8");
    assert!(received_str.contains("a-f-G-U-C") && received_str.contains("TEST-SA"));
}
