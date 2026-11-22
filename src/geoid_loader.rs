//! ASCII grid format loader for geoid undulation tables.

/// Error conditions during ASCII geoid grid parsing and sampling.
#[derive(Debug, Clone, PartialEq)]
pub enum GeoidError {
    EmptyInput,
    InvalidHeader(String),
    MissingHeader(&'static str),
    InvalidDimensions { cols: usize, rows: usize },
    InvalidCellSize(f64),
    InvalidDataValue(String),
    DataCountMismatch { expected: usize, found: usize },
}

impl std::fmt::Display for GeoidError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyInput => write!(f, "empty ascii grid input"),
            Self::InvalidHeader(m) => write!(f, "invalid ascii header: {}", m),
            Self::MissingHeader(h) => write!(f, "missing required header: {}", h),
            Self::InvalidDimensions { cols, rows } => {
                write!(f, "dimensions must be >= 1, got {}x{}", cols, rows)
            }
            Self::InvalidCellSize(s) => write!(f, "invalid cell size: {}", s),
            Self::InvalidDataValue(v) => write!(f, "invalid float token: {}", v),
            Self::DataCountMismatch { expected, found } => {
                write!(f, "expected {} values, found {}", expected, found)
            }
        }
    }
}

impl std::error::Error for GeoidError {}

/// Regular geoid undulation grid loaded from an ASCII raster.
#[derive(Debug, Clone, PartialEq)]
pub struct GeoidGrid {
    pub cols: usize,
    pub rows: usize,
    pub xllcorner: f64,
    pub yllcorner: f64,
    pub cellsize: f64,
    pub nodata_value: Option<f64>,
    pub data: Vec<Option<f64>>,
}

impl GeoidGrid {
    /// Construct and validate a new GeoidGrid raster instance.
    pub fn new(
        cols: usize,
        rows: usize,
        x0: f64,
        y0: f64,
        step: f64,
        nodata: Option<f64>,
        data: Vec<Option<f64>>,
    ) -> Result<Self, GeoidError> {
        if cols == 0 || rows == 0 {
            return Err(GeoidError::InvalidDimensions { cols, rows });
        }
        if !step.is_finite() || step <= 0.0 {
            return Err(GeoidError::InvalidCellSize(step));
        }
        if !x0.is_finite() || !y0.is_finite() {
            return Err(GeoidError::InvalidHeader("non-finite origin".into()));
        }
        let total = cols
            .checked_mul(rows)
            .ok_or(GeoidError::InvalidDimensions { cols, rows })?;
        if data.len() != total {
            let (expected, found) = (total, data.len());
            return Err(GeoidError::DataCountMismatch { expected, found });
        }
        Ok(Self {
            cols,
            rows,
            xllcorner: x0,
            yllcorner: y0,
            cellsize: step,
            nodata_value: nodata,
            data,
        })
    }

    /// Retrieve undulation value at integer grid coordinate (col, row).
    pub fn get(&self, col: usize, row: usize) -> Option<f64> {
        (col < self.cols && row < self.rows).then(|| self.data[row * self.cols + col])?
    }

    /// Geographic coverage bounds in degrees.
    pub fn min_lon(&self) -> f64 {
        self.xllcorner
    }
    pub fn max_lon(&self) -> f64 {
        self.xllcorner + self.cols as f64 * self.cellsize
    }
    pub fn min_lat(&self) -> f64 {
        self.yllcorner
    }
    pub fn max_lat(&self) -> f64 {
        self.yllcorner + self.rows as f64 * self.cellsize
    }

    /// Test if longitude and latitude fall within the grid coverage.
    pub fn contains(&self, lon: f64, lat: f64) -> bool {
        lon >= self.min_lon()
            && lon <= self.max_lon()
            && lat >= self.min_lat()
            && lat <= self.max_lat()
    }

    /// Bilinear interpolation of geoid undulation at continuous geographic coordinates.
    pub fn undulation_at(&self, lon: f64, lat: f64) -> Option<f64> {
        if !self.contains(lon, lat) || !lon.is_finite() || !lat.is_finite() {
            return None;
        }
        if self.cols == 1 && self.rows == 1 {
            return self.get(0, 0);
        }
        let u = (lon - self.xllcorner) / self.cellsize - 0.5;
        let v_norm = (lat - self.yllcorner) / self.cellsize - 0.5;
        let v_top = self.rows.saturating_sub(1) as f64 - v_norm;
        crate::dem_bilinear::sample_bilinear_cells(u, v_top, self.cols, self.rows, false, |c, r| {
            self.get(c, r)
        })
    }
}

#[derive(Default)]
struct AsciiHeader {
    cols: Option<usize>,
    rows: Option<usize>,
    xcorner: Option<f64>,
    ycorner: Option<f64>,
    xcenter: Option<f64>,
    ycenter: Option<f64>,
    cellsize: Option<f64>,
    nodata: Option<f64>,
}

impl AsciiHeader {
    #[allow(clippy::type_complexity)]
    fn resolve(self) -> Result<(usize, usize, f64, f64, f64, Option<f64>), GeoidError> {
        let cols = self.cols.ok_or(GeoidError::MissingHeader("ncols"))?;
        let rows = self.rows.ok_or(GeoidError::MissingHeader("nrows"))?;
        let step = self.cellsize.ok_or(GeoidError::MissingHeader("cellsize"))?;
        let x0 = self
            .xcorner
            .or_else(|| self.xcenter.map(|c| c - 0.5 * step))
            .ok_or(GeoidError::MissingHeader("xllcorner"))?;
        let y0 = self
            .ycorner
            .or_else(|| self.ycenter.map(|c| c - 0.5 * step))
            .ok_or(GeoidError::MissingHeader("yllcorner"))?;
        Ok((cols, rows, x0, y0, step, self.nodata))
    }

    fn set_field(&mut self, key: &str, val: &str) -> Result<bool, GeoidError> {
        let err = || GeoidError::InvalidHeader(val.into());
        match key.to_ascii_lowercase().as_str() {
            "ncols" => self.cols = Some(val.parse().map_err(|_| err())?),
            "nrows" => self.rows = Some(val.parse().map_err(|_| err())?),
            "xllcorner" => self.xcorner = Some(val.parse().map_err(|_| err())?),
            "yllcorner" => self.ycorner = Some(val.parse().map_err(|_| err())?),
            "xllcenter" => self.xcenter = Some(val.parse().map_err(|_| err())?),
            "yllcenter" => self.ycenter = Some(val.parse().map_err(|_| err())?),
            "cellsize" | "dx" | "dy" => self.cellsize = Some(val.parse().map_err(|_| err())?),
            "nodata_value" | "nodata" => self.nodata = Some(val.parse().map_err(|_| err())?),
            _ => return Ok(false),
        }
        Ok(true)
    }
}

fn is_header_key(k: &str) -> bool {
    matches!(
        k.to_ascii_lowercase().as_str(),
        "ncols"
            | "nrows"
            | "xllcorner"
            | "yllcorner"
            | "xllcenter"
            | "yllcenter"
            | "cellsize"
            | "dx"
            | "dy"
            | "nodata_value"
            | "nodata"
    )
}

/// Parse an ESRI ASCII grid formatted string into a GeoidGrid.
pub fn parse_geoid_ascii(ascii: &str) -> Result<GeoidGrid, GeoidError> {
    let mut tokens = ascii.split_whitespace().peekable();
    if tokens.peek().is_none() {
        return Err(GeoidError::EmptyInput);
    }
    let mut header = AsciiHeader::default();
    while let Some(&tok) = tokens.peek() {
        if !is_header_key(tok) {
            break;
        }
        let key = tokens.next().unwrap();
        let val = tokens
            .next()
            .ok_or_else(|| GeoidError::InvalidHeader(key.into()))?;
        header.set_field(key, val)?;
    }
    let (cols, rows, x0, y0, step, nodata) = header.resolve()?;
    let mut data = Vec::new();
    for tok in tokens {
        let v: f64 = tok
            .parse()
            .map_err(|_| GeoidError::InvalidDataValue(tok.into()))?;
        let is_nd = !v.is_finite() || nodata.is_some_and(|nd| (v - nd).abs() < 1e-5);
        data.push((!is_nd).then_some(v));
    }
    GeoidGrid::new(cols, rows, x0, y0, step, nodata, data)
}

impl std::str::FromStr for GeoidGrid {
    type Err = GeoidError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_geoid_ascii(s)
    }
}

impl GeoidGrid {
    /// Parse an ASCII raster string into a GeoidGrid.
    pub fn from_ascii(ascii: &str) -> Result<Self, GeoidError> {
        parse_geoid_ascii(ascii)
    }
}
