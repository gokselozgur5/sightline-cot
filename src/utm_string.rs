//! UTM coordinate string formatting and parsing.

/// Error returned when parsing a UTM coordinate string fails.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UtmStringError {
    InvalidFormat,
    InvalidZone,
    InvalidBand,
    InvalidEasting,
    InvalidNorthing,
}

/// A UTM coordinate with zone, latitude band letter, easting and northing.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UtmCoordinate {
    pub zone: u8,
    pub band: char,
    pub easting: f64,
    pub northing: f64,
}

impl UtmCoordinate {
    /// Create and validate a new UTM coordinate.
    pub fn new(zone: u8, band: char, easting: f64, northing: f64) -> Result<Self, UtmStringError> {
        if !(1..=60).contains(&zone) {
            return Err(UtmStringError::InvalidZone);
        }
        let band = band.to_ascii_uppercase();
        if !b"CDEFGHJKLMNPQRSTUVWX".contains(&(band as u8)) {
            return Err(UtmStringError::InvalidBand);
        }
        if !easting.is_finite() || easting < 0.0 {
            return Err(UtmStringError::InvalidEasting);
        }
        if !northing.is_finite() || northing < 0.0 {
            return Err(UtmStringError::InvalidNorthing);
        }
        Ok(Self {
            zone,
            band,
            easting,
            northing,
        })
    }
}

/// Parse a UTM coordinate string such as "33T 500000 4649776".
pub fn parse_utm_string(s: &str) -> Result<UtmCoordinate, UtmStringError> {
    let tok: Vec<&str> = s.split_whitespace().collect();
    let (zs, bs, es, ns) = match tok.as_slice() {
        [zb, e, n] => {
            let p = zb
                .find(|c: char| !c.is_ascii_digit())
                .ok_or(UtmStringError::InvalidFormat)?;
            (&zb[..p], &zb[p..], *e, *n)
        }
        [z, b, e, n] => (*z, *b, *e, *n),
        _ => return Err(UtmStringError::InvalidFormat),
    };
    let zone: u8 = zs.parse().map_err(|_| UtmStringError::InvalidZone)?;
    let mut b_chars = bs.chars();
    let band = b_chars.next().ok_or(UtmStringError::InvalidBand)?;
    if b_chars.next().is_some() {
        return Err(UtmStringError::InvalidBand);
    }
    let easting: f64 = es.parse().map_err(|_| UtmStringError::InvalidEasting)?;
    let northing: f64 = ns.parse().map_err(|_| UtmStringError::InvalidNorthing)?;
    UtmCoordinate::new(zone, band, easting, northing)
}

fn fmt_coord(v: f64) -> String {
    if v.fract() == 0.0 {
        format!("{:.0}", v)
    } else {
        format!("{}", v)
    }
}

/// Format a UTM coordinate into a standard string representation.
pub fn format_utm_string(coord: &UtmCoordinate) -> String {
    format!(
        "{}{} {} {}",
        coord.zone,
        coord.band,
        fmt_coord(coord.easting),
        fmt_coord(coord.northing)
    )
}

impl core::fmt::Display for UtmCoordinate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", format_utm_string(self))
    }
}

impl core::str::FromStr for UtmCoordinate {
    type Err = UtmStringError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_utm_string(s)
    }
}
