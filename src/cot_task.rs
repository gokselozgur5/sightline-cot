//! CoT tactical task and route point XML builder and parser.

#[derive(Debug, Clone, PartialEq)]
pub enum CotTaskError {
    InvalidCoordinate(&'static str),
    InvalidParameter(&'static str),
    EmptyRoute,
    InsufficientPoints { expected: usize, found: usize },
    XmlParseError(String),
    MissingField(&'static str),
}

impl std::fmt::Display for CotTaskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCoordinate(m) => write!(f, "invalid coordinate: {m}"),
            Self::InvalidParameter(m) => write!(f, "invalid parameter: {m}"),
            Self::EmptyRoute => write!(f, "tactical task has no route points"),
            Self::InsufficientPoints { expected, found } => {
                write!(f, "insufficient points: expected {expected}, got {found}")
            }
            Self::XmlParseError(m) => write!(f, "XML parse error: {m}"),
            Self::MissingField(m) => write!(f, "missing field: {m}"),
        }
    }
}

impl std::error::Error for CotTaskError {}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TaskAction {
    #[default]
    FlyOver,
    FlyBy,
    Ingress,
    Egress,
    Orbit,
    Drop,
    Landing,
    Target,
    Checkpoint,
    Rendezvous,
    Custom(String),
}

impl TaskAction {
    pub fn as_str(&self) -> &str {
        match self {
            Self::FlyOver => "flyover",
            Self::FlyBy => "flyby",
            Self::Ingress => "ingress",
            Self::Egress => "egress",
            Self::Orbit => "orbit",
            Self::Drop => "drop",
            Self::Landing => "landing",
            Self::Target => "target",
            Self::Checkpoint => "checkpoint",
            Self::Rendezvous => "rendezvous",
            Self::Custom(s) => s.as_str(),
        }
    }

    pub fn parse_str(s: &str) -> Self {
        match s {
            "flyover" => Self::FlyOver,
            "flyby" => Self::FlyBy,
            "ingress" => Self::Ingress,
            "egress" => Self::Egress,
            "orbit" => Self::Orbit,
            "drop" => Self::Drop,
            "landing" => Self::Landing,
            "target" => Self::Target,
            "checkpoint" => Self::Checkpoint,
            "rendezvous" => Self::Rendezvous,
            other => Self::Custom(other.to_string()),
        }
    }
}

impl std::str::FromStr for TaskAction {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::parse_str(s))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TaskPriority {
    #[default]
    Routine,
    Priority,
    Immediate,
    Flash,
}

impl TaskPriority {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Routine => "routine",
            Self::Priority => "priority",
            Self::Immediate => "immediate",
            Self::Flash => "flash",
        }
    }

    pub fn parse_str(s: &str) -> Self {
        match s {
            "priority" => Self::Priority,
            "immediate" => Self::Immediate,
            "flash" => Self::Flash,
            _ => Self::Routine,
        }
    }
}

impl std::str::FromStr for TaskPriority {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::parse_str(s))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TaskStatus {
    #[default]
    Planned,
    InProgress,
    Completed,
    Cancelled,
}

impl TaskStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Planned => "planned",
            Self::InProgress => "in_progress",
            Self::Completed => "completed",
            Self::Cancelled => "cancelled",
        }
    }

    pub fn parse_str(s: &str) -> Self {
        match s {
            "in_progress" => Self::InProgress,
            "completed" => Self::Completed,
            "cancelled" => Self::Cancelled,
            _ => Self::Planned,
        }
    }
}

impl std::str::FromStr for TaskStatus {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::parse_str(s))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TaskPoint {
    pub order: u32,
    pub name: String,
    pub lat: f64,
    pub lon: f64,
    pub hae: Option<f64>,
    pub action: TaskAction,
    pub speed_mps: Option<f64>,
    pub eta_ms: Option<i64>,
    pub remarks: Option<String>,
}

fn validate_lat_lon(lat: f64, lon: f64) -> Result<(), CotTaskError> {
    if !lat.is_finite() || !(-90.0..=90.0).contains(&lat) {
        return Err(CotTaskError::InvalidCoordinate("latitude in [-90, 90]"));
    }
    if !lon.is_finite() || !(-180.0..=180.0).contains(&lon) {
        return Err(CotTaskError::InvalidCoordinate("longitude in [-180, 180]"));
    }
    Ok(())
}

fn extract_attr(tag: &str, attr: &str) -> Option<String> {
    crate::cot::find_attr_in_tag(tag, attr).map(|s| s.to_string())
}

impl TaskPoint {
    pub fn new(
        order: u32,
        name: impl Into<String>,
        lat: f64,
        lon: f64,
    ) -> Result<Self, CotTaskError> {
        validate_lat_lon(lat, lon)?;
        Ok(Self {
            order,
            name: name.into(),
            lat,
            lon,
            hae: None,
            action: TaskAction::default(),
            speed_mps: None,
            eta_ms: None,
            remarks: None,
        })
    }

    pub fn with_hae(mut self, hae: f64) -> Result<Self, CotTaskError> {
        if !hae.is_finite() {
            return Err(CotTaskError::InvalidParameter("hae must be finite"));
        }
        self.hae = Some(hae);
        Ok(self)
    }

    pub fn with_action(mut self, action: TaskAction) -> Self {
        self.action = action;
        self
    }

    pub fn with_speed(mut self, speed_mps: f64) -> Result<Self, CotTaskError> {
        if !speed_mps.is_finite() || speed_mps < 0.0 {
            return Err(CotTaskError::InvalidParameter(
                "speed_mps must be non-negative",
            ));
        }
        self.speed_mps = Some(speed_mps);
        Ok(self)
    }

    pub fn with_eta(mut self, eta_ms: i64) -> Self {
        self.eta_ms = Some(eta_ms);
        self
    }

    pub fn with_remarks(mut self, remarks: impl Into<String>) -> Self {
        self.remarks = Some(remarks.into());
        self
    }

    pub fn to_xml(&self) -> String {
        let name_esc = crate::cot::escape_xml_attr(&self.name);
        let mut xml = format!(
            r#"<point order="{}" name="{}" lat="{:.7}" lon="{:.7}""#,
            self.order, name_esc, self.lat, self.lon
        );
        if let Some(h) = self.hae {
            xml.push_str(&format!(r#" hae="{:.2}""#, h));
        }
        let action_esc = crate::cot::escape_xml_attr(self.action.as_str());
        xml.push_str(&format!(r#" action="{}""#, action_esc));
        if let Some(spd) = self.speed_mps {
            xml.push_str(&format!(r#" speed="{:.2}""#, spd));
        }
        if let Some(eta) = self.eta_ms {
            xml.push_str(&format!(r#" eta="{}""#, eta));
        }
        if let Some(rem) = &self.remarks {
            xml.push_str(&format!(
                r#" remarks="{}""#,
                crate::cot::escape_xml_attr(rem)
            ));
        }
        xml.push_str("/>");
        xml
    }

    pub fn parse(xml: &str) -> Result<Self, CotTaskError> {
        let order_s = extract_attr(xml, "order").ok_or(CotTaskError::MissingField("order"))?;
        let order = order_s
            .parse::<u32>()
            .map_err(|_| CotTaskError::InvalidParameter("order"))?;
        let name = extract_attr(xml, "name")
            .map(|s| crate::cot::unescape_xml_attr(&s))
            .unwrap_or_else(|| "WP".to_string());
        let lat_s = extract_attr(xml, "lat").ok_or(CotTaskError::MissingField("lat"))?;
        let lon_s = extract_attr(xml, "lon").ok_or(CotTaskError::MissingField("lon"))?;
        let lat = lat_s
            .parse::<f64>()
            .map_err(|_| CotTaskError::InvalidCoordinate("lat"))?;
        let lon = lon_s
            .parse::<f64>()
            .map_err(|_| CotTaskError::InvalidCoordinate("lon"))?;
        let mut pt = Self::new(order, name, lat, lon)?;
        if let Some(h_s) = extract_attr(xml, "hae") {
            let h = h_s
                .parse::<f64>()
                .map_err(|_| CotTaskError::InvalidParameter("hae"))?;
            pt = pt.with_hae(h)?;
        }
        if let Some(act_s) = extract_attr(xml, "action") {
            pt = pt.with_action(TaskAction::parse_str(&act_s));
        }
        if let Some(s_s) = extract_attr(xml, "speed") {
            let s = s_s
                .parse::<f64>()
                .map_err(|_| CotTaskError::InvalidParameter("speed"))?;
            pt = pt.with_speed(s)?;
        }
        if let Some(e_s) = extract_attr(xml, "eta") {
            if let Ok(eta) = e_s.parse::<i64>() {
                pt = pt.with_eta(eta);
            }
        }
        if let Some(rem) = extract_attr(xml, "remarks") {
            pt = pt.with_remarks(crate::cot::unescape_xml_attr(&rem));
        }
        Ok(pt)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TacticalTask {
    pub name: String,
    pub task_type: String,
    pub priority: TaskPriority,
    pub status: TaskStatus,
    pub points: Vec<TaskPoint>,
}

impl TacticalTask {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            task_type: "route".to_string(),
            priority: TaskPriority::default(),
            status: TaskStatus::default(),
            points: Vec::new(),
        }
    }

    pub fn with_type(mut self, task_type: impl Into<String>) -> Self {
        self.task_type = task_type.into();
        self
    }

    pub fn with_priority(mut self, priority: TaskPriority) -> Self {
        self.priority = priority;
        self
    }

    pub fn with_status(mut self, status: TaskStatus) -> Self {
        self.status = status;
        self
    }

    pub fn add_point(&mut self, point: TaskPoint) {
        self.points.push(point);
    }

    pub fn with_points(mut self, points: Vec<TaskPoint>) -> Self {
        self.points = points;
        self
    }

    pub fn point_count(&self) -> usize {
        self.points.len()
    }

    pub fn centroid(&self) -> Result<[f64; 2], CotTaskError> {
        let first = self.points.first().ok_or(CotTaskError::EmptyRoute)?;
        let ref_lon = first.lon;
        let n = self.points.len() as f64;
        let sum_lat: f64 = self.points.iter().map(|p| p.lat).sum();
        let sum_unwrapped: f64 = self
            .points
            .iter()
            .map(|p| ref_lon + (p.lon - ref_lon + 180.0).rem_euclid(360.0) - 180.0)
            .sum();
        let lon = (sum_unwrapped / n + 180.0).rem_euclid(360.0) - 180.0;
        Ok([sum_lat / n, lon])
    }

    pub fn total_distance_m(&self) -> f64 {
        if self.points.len() < 2 {
            return 0.0;
        }
        let mut total = 0.0;
        for w in self.points.windows(2) {
            total += crate::geo_haversine::haversine_distance_m(
                [w[0].lat, w[0].lon],
                [w[1].lat, w[1].lon],
            );
        }
        total
    }

    pub fn estimated_duration_secs(&self, default_speed_mps: f64) -> Result<f64, CotTaskError> {
        if !default_speed_mps.is_finite() || default_speed_mps <= 0.0 {
            return Err(CotTaskError::InvalidParameter(
                "default_speed_mps must be positive",
            ));
        }
        if self.points.len() < 2 {
            return Ok(0.0);
        }
        let mut total_duration = 0.0;
        for w in self.points.windows(2) {
            let dist = crate::geo_haversine::haversine_distance_m(
                [w[0].lat, w[0].lon],
                [w[1].lat, w[1].lon],
            );
            let spd = w[1].speed_mps.unwrap_or(default_speed_mps);
            if spd > 0.0 {
                total_duration += dist / spd;
            }
        }
        Ok(total_duration)
    }

    pub fn to_task_xml(&self) -> String {
        let name_esc = crate::cot::escape_xml_attr(&self.name);
        let type_esc = crate::cot::escape_xml_attr(&self.task_type);
        let mut xml = format!(
            r#"<task name="{}" type="{}" priority="{}" status="{}">"#,
            name_esc,
            type_esc,
            self.priority.as_str(),
            self.status.as_str()
        );
        for pt in &self.points {
            xml.push_str(&pt.to_xml());
        }
        xml.push_str("</task>");
        xml
    }

    pub fn to_cot_event_xml(
        &self,
        uid: &str,
        callsign: &str,
        time_ms: i64,
        stale_ms: i64,
    ) -> Result<String, CotTaskError> {
        let [c_lat, c_lon] = self.centroid()?;
        let time_str = crate::cot::format_iso8601_utc(time_ms);
        let stale_str = crate::cot::format_iso8601_utc(stale_ms);
        let uid_esc = crate::cot::escape_xml_attr(uid);
        let cs_esc = crate::cot::escape_xml_attr(callsign);
        let task_xml = self.to_task_xml();
        Ok(format!(
            "<event version=\"2.0\" uid=\"{uid_esc}\" type=\"b-m-r\" how=\"h-e\" time=\"{time_str}\" start=\"{time_str}\" stale=\"{stale_str}\"><point lat=\"{c_lat:.7}\" lon=\"{c_lon:.7}\" hae=\"0.00\" ce=\"10.00\" le=\"10.00\"/><detail><contact callsign=\"{cs_esc}\"/>{task_xml}</detail></event>"
        ))
    }

    pub fn parse_task_xml(xml: &str) -> Result<Self, CotTaskError> {
        let t_start = xml
            .find("<task")
            .ok_or_else(|| CotTaskError::XmlParseError("missing <task tag".into()))?;
        let header_end = xml[t_start..]
            .find('>')
            .ok_or_else(|| CotTaskError::XmlParseError("unclosed <task tag".into()))?;
        let header = &xml[t_start..t_start + header_end];
        let name = extract_attr(header, "name")
            .map(|s| crate::cot::unescape_xml_attr(&s))
            .unwrap_or_else(|| "task".into());
        let task_type = extract_attr(header, "type")
            .map(|s| crate::cot::unescape_xml_attr(&s))
            .unwrap_or_else(|| "route".into());
        let priority = extract_attr(header, "priority")
            .map(|p| TaskPriority::parse_str(&p))
            .unwrap_or_default();
        let status = extract_attr(header, "status")
            .map(|s| TaskStatus::parse_str(&s))
            .unwrap_or_default();

        let mut points = Vec::new();
        let mut rest = &xml[t_start + header_end..];
        while let Some(p_start) = rest.find("<point") {
            let p_end = rest[p_start..]
                .find("/>")
                .ok_or_else(|| CotTaskError::XmlParseError("unclosed <point tag".into()))?;
            let p_xml = &rest[p_start..p_start + p_end + 2];
            points.push(TaskPoint::parse(p_xml)?);
            rest = &rest[p_start + p_end + 2..];
        }

        Ok(Self {
            name,
            task_type,
            priority,
            status,
            points,
        })
    }

    pub fn from_cot_event_xml(xml: &str) -> Result<Self, CotTaskError> {
        Self::parse_task_xml(xml)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_point_creation_and_bounds() {
        let pt = TaskPoint::new(1, "WP-ALPHA", 37.7749, -122.4194).unwrap();
        assert_eq!(pt.order, 1);
        assert_eq!(pt.name, "WP-ALPHA");
        assert_eq!(pt.action, TaskAction::FlyOver);

        assert!(TaskPoint::new(1, "ERR", 95.0, 0.0).is_err());
        assert!(TaskPoint::new(1, "ERR", 0.0, 190.0).is_err());
        assert!(TaskPoint::new(1, "ERR", f64::NAN, 0.0).is_err());
    }

    #[test]
    fn test_task_point_builders_and_xml() {
        let pt = TaskPoint::new(2, "IP", 36.12, -115.15)
            .unwrap()
            .with_hae(1500.0)
            .unwrap()
            .with_action(TaskAction::Ingress)
            .with_speed(120.5)
            .unwrap()
            .with_eta(1700000000)
            .with_remarks("Initial Point for strike");
        let xml = pt.to_xml();
        assert!(xml.contains("order=\"2\""));
        assert!(xml.contains("name=\"IP\""));
        assert!(xml.contains("hae=\"1500.00\""));
        assert!(xml.contains("action=\"ingress\""));
        assert!(xml.contains("speed=\"120.50\""));
        assert!(xml.contains("remarks=\"Initial Point for strike\""));

        let parsed = TaskPoint::parse(&xml).unwrap();
        assert_eq!(parsed.order, 2);
        assert_eq!(parsed.name, "IP");
        assert_eq!(parsed.action, TaskAction::Ingress);
        assert_eq!(parsed.hae, Some(1500.0));
        assert_eq!(parsed.speed_mps, Some(120.5));
        assert_eq!(parsed.eta_ms, Some(1700000000));
    }

    #[test]
    fn test_tactical_task_xml_roundtrip() {
        let mut task = TacticalTask::new("ALPHA-ROUTE")
            .with_type("strike")
            .with_priority(TaskPriority::Immediate)
            .with_status(TaskStatus::InProgress);
        let p1 = TaskPoint::new(1, "START", 35.0, -115.0).unwrap();
        let p2 = TaskPoint::new(2, "END", 35.5, -114.5)
            .unwrap()
            .with_action(TaskAction::Target);
        task.add_point(p1);
        task.add_point(p2);

        let xml = task.to_task_xml();
        let parsed = TacticalTask::parse_task_xml(&xml).unwrap();
        assert_eq!(parsed.name, "ALPHA-ROUTE");
        assert_eq!(parsed.task_type, "strike");
        assert_eq!(parsed.priority, TaskPriority::Immediate);
        assert_eq!(parsed.status, TaskStatus::InProgress);
        assert_eq!(parsed.points.len(), 2);
        assert_eq!(parsed.points[1].action, TaskAction::Target);
    }

    #[test]
    fn test_tactical_task_cot_event_xml() {
        let mut task = TacticalTask::new("RECON-1");
        task.add_point(TaskPoint::new(1, "WP1", 10.0, 20.0).unwrap());
        task.add_point(TaskPoint::new(2, "WP2", 12.0, 22.0).unwrap());

        let event_xml = task
            .to_cot_event_xml("task-uid-01", "GHOST-1", 1700000000000, 1700000300000)
            .unwrap();
        assert!(event_xml.contains("type=\"b-m-r\""));
        assert!(event_xml.contains("uid=\"task-uid-01\""));
        assert!(event_xml.contains("contact callsign=\"GHOST-1\""));

        let parsed = TacticalTask::from_cot_event_xml(&event_xml).unwrap();
        assert_eq!(parsed.name, "RECON-1");
        assert_eq!(parsed.point_count(), 2);
    }

    #[test]
    fn test_tactical_task_geometry() {
        let mut task = TacticalTask::new("GEO-TASK");
        assert!(task.centroid().is_err());
        assert_eq!(task.total_distance_m(), 0.0);

        task.add_point(TaskPoint::new(1, "P1", 0.0, 0.0).unwrap());
        task.add_point(TaskPoint::new(2, "P2", 1.0, 0.0).unwrap());

        let c = task.centroid().unwrap();
        assert!((c[0] - 0.5).abs() < 1e-6);
        assert!((c[1] - 0.0).abs() < 1e-6);

        let dist = task.total_distance_m();
        assert!((dist - 111195.0).abs() < 500.0);

        let dur = task.estimated_duration_secs(50.0).unwrap();
        assert!((dur - (dist / 50.0)).abs() < 1.0);
    }
}
