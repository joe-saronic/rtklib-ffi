//! Post-processed kinematic positioning.
//!
//! ```no_run
//! use rtklib_ffi::ppk::{PrcOpt, SolOpt, FilOpt, postpos};
//!
//! let popt = PrcOpt::kinematic();
//! let sopt = SolOpt::default();
//! let fopt = FilOpt::default();
//!
//! postpos(
//!     "rover.obs", "base.obs",
//!     &["nav.nav"], "output.pos",
//!     &popt, &sopt, &fopt,
//! ).unwrap();
//! ```

use crate::NavSys;
use rtklib_sys::rtklib as ffi;
use std::ffi::CString;

/// Positioning mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum PosMode {
    /// Single point positioning. From PMODE_SINGLE.
    Single = ffi::PMODE_SINGLE as i32,
    /// Differential GPS / DGNSS. From PMODE_DGPS.
    Dgps = ffi::PMODE_DGPS as i32,
    /// Kinematic positioning. From PMODE_KINEMA.
    Kinematic = ffi::PMODE_KINEMA as i32,
    /// Static positioning. From PMODE_STATIC.
    Static = ffi::PMODE_STATIC as i32,
    /// Static positioning starting from a known position. From PMODE_STATIC_START.
    StaticStart = ffi::PMODE_STATIC_START as i32,
    /// Moving base station. From PMODE_MOVEB.
    MovingBase = ffi::PMODE_MOVEB as i32,
    /// Fixed position. From PMODE_FIXED.
    Fixed = ffi::PMODE_FIXED as i32,
    /// Precise Point Positioning, kinematic. From PMODE_PPP_KINEMA.
    PppKinematic = ffi::PMODE_PPP_KINEMA as i32,
    /// Precise Point Positioning, static. From PMODE_PPP_STATIC.
    PppStatic = ffi::PMODE_PPP_STATIC as i32,
    /// Precise Point Positioning, fixed. From PMODE_PPP_FIXED.
    PppFixed = ffi::PMODE_PPP_FIXED as i32,
}

/// Solution output format.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum SolFormat {
    /// Latitude, longitude, and height. From SOLF_LLH.
    Llh = ffi::SOLF_LLH as i32,
    /// X, Y, Z in ECEF coordinates. From SOLF_XYZ.
    Xyz = ffi::SOLF_XYZ as i32,
    /// East, north, up baseline components. From SOLF_ENU.
    Enu = ffi::SOLF_ENU as i32,
    /// NMEA-0183 sentences. From SOLF_NMEA.
    Nmea = ffi::SOLF_NMEA as i32,
}

/// Ionosphere correction option.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum IonoOpt {
    /// Ionosphere correction disabled. From IONOOPT_OFF.
    Off = ffi::IONOOPT_OFF as i32,
    /// Klobuchar broadcast model. From IONOOPT_BRDC.
    Broadcast = ffi::IONOOPT_BRDC as i32,
    /// SBAS ionosphere model. From IONOOPT_SBAS.
    Sbas = ffi::IONOOPT_SBAS as i32,
    /// Iono-free linear combination of L1/L2 or L1/L5. From IONOOPT_IFLC.
    IonFreeLC = ffi::IONOOPT_IFLC as i32,
    /// Ionosphere delay estimation. From IONOOPT_EST.
    Estimation = ffi::IONOOPT_EST as i32,
    /// IONEX TEC grid model. From IONOOPT_TEC.
    Tec = ffi::IONOOPT_TEC as i32,
    /// QZSS broadcast ionosphere model. From IONOOPT_QZS.
    Qzs = ffi::IONOOPT_QZS as i32,
}

/// Troposphere correction option.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum TropOpt {
    /// Troposphere correction disabled. From TROPOPT_OFF.
    Off = ffi::TROPOPT_OFF as i32,
    /// Saastamoinen model. From TROPOPT_SAAS.
    Saastamoinen = ffi::TROPOPT_SAAS as i32,
    /// SBAS troposphere model. From TROPOPT_SBAS.
    Sbas = ffi::TROPOPT_SBAS as i32,
    /// Zenith total delay estimation. From TROPOPT_EST.
    Estimation = ffi::TROPOPT_EST as i32,
    /// Zenith total delay plus horizontal gradient estimation. From TROPOPT_ESTG.
    EstimationGrad = ffi::TROPOPT_ESTG as i32,
}

/// Ambiguity resolution mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum ArMode {
    /// Ambiguity resolution disabled.
    Off = 0,
    /// Continuous ambiguity resolution.
    Continuous = 1,
    /// Instantaneous ambiguity resolution.
    Instantaneous = 2,
    /// Fix-and-hold ambiguity resolution.
    FixAndHold = 3,
}

/// Processing options wrapper around `prcopt_t`.
pub struct PrcOpt(ffi::prcopt_t);

impl Default for PrcOpt {
    fn default() -> Self {
        Self(unsafe { ffi::prcopt_default })
    }
}

impl PrcOpt {
    /// Kinematic mode with combined forward+backward solution.
    pub fn kinematic() -> Self {
        let mut opt = Self::default();
        opt.0.mode = PosMode::Kinematic as i32;
        opt.0.soltype = 2;
        opt
    }

    /// Static mode with combined forward+backward solution.
    pub fn static_mode() -> Self {
        let mut opt = Self::default();
        opt.0.mode = PosMode::Static as i32;
        opt.0.soltype = 2;
        opt
    }

    /// Set positioning mode.
    pub fn set_mode(&mut self, mode: PosMode) -> &mut Self {
        self.0.mode = mode as i32;
        self
    }

    /// Get positioning mode as raw value.
    pub fn mode(&self) -> i32 {
        self.0.mode
    }

    /// Set enabled navigation systems.
    pub fn set_navsys(&mut self, sys: NavSys) -> &mut Self {
        self.0.navsys = sys.bits() as i32;
        self
    }

    /// Get enabled navigation systems.
    pub fn navsys(&self) -> NavSys {
        NavSys::from_bits_truncate(self.0.navsys as u32)
    }

    /// Set number of frequencies. 1=L1, 2=L1+L2, 3=L1+L2+L5.
    pub fn set_frequencies(&mut self, nf: i32) -> &mut Self {
        self.0.nf = nf;
        self
    }

    /// Get number of frequencies.
    pub fn frequencies(&self) -> i32 {
        self.0.nf
    }

    /// Set elevation mask angle in degrees.
    pub fn set_elevation_mask(&mut self, deg: f64) -> &mut Self {
        self.0.elmin = deg.to_radians();
        self
    }

    /// Get elevation mask angle in radians.
    pub fn elevation_mask(&self) -> f64 {
        self.0.elmin
    }

    /// Set ambiguity resolution mode.
    pub fn set_ar_mode(&mut self, mode: ArMode) -> &mut Self {
        self.0.modear = mode as i32;
        self
    }

    /// Get ambiguity resolution mode as raw value.
    pub fn ar_mode(&self) -> i32 {
        self.0.modear
    }

    /// Set ionosphere correction option.
    pub fn set_ionosphere(&mut self, opt: IonoOpt) -> &mut Self {
        self.0.ionoopt = opt as i32;
        self
    }

    /// Get ionosphere correction option as raw value.
    pub fn ionosphere(&self) -> i32 {
        self.0.ionoopt
    }

    /// Set base station position in ECEF coordinates (meters).
    ///
    /// Equivalent to the `-r` flag in `rnx2rtkp`.
    pub fn set_base_position_ecef(&mut self, x: f64, y: f64, z: f64) -> &mut Self {
        self.0.refpos = ffi::POSOPT_POS_XYZ as i32;
        self.0.rb = [x, y, z];
        self
    }

    /// Set base station position in geodetic coordinates
    /// (latitude and longitude in degrees, height in meters).
    ///
    /// Equivalent to the `-l` flag in `rnx2rtkp`.
    pub fn set_base_position_llh(&mut self, lat_deg: f64, lon_deg: f64, height: f64) -> &mut Self {
        self.0.refpos = ffi::POSOPT_POS_LLH as i32;
        let pos = [lat_deg.to_radians(), lon_deg.to_radians(), height];
        unsafe { ffi::pos2ecef(pos.as_ptr(), self.0.rb.as_mut_ptr()) };
        self
    }

    /// Get base station position in ECEF coordinates (meters).
    pub fn base_position_ecef(&self) -> [f64; 3] {
        self.0.rb
    }

    /// Set troposphere correction option.
    pub fn set_troposphere(&mut self, opt: TropOpt) -> &mut Self {
        self.0.tropopt = opt as i32;
        self
    }

    /// Get troposphere correction option as raw value.
    pub fn troposphere(&self) -> i32 {
        self.0.tropopt
    }

    pub(crate) fn as_ffi(&self) -> &ffi::prcopt_t {
        &self.0
    }
}

/// Solution output options wrapper around `solopt_t`.
pub struct SolOpt(ffi::solopt_t);

impl Default for SolOpt {
    fn default() -> Self {
        Self(unsafe { ffi::solopt_default })
    }
}

impl SolOpt {
    /// Set solution output format.
    pub fn set_format(&mut self, format: SolFormat) -> &mut Self {
        self.0.posf = format as i32;
        self
    }

    /// Get solution output format as raw value.
    pub fn format(&self) -> i32 {
        self.0.posf
    }

    /// Set time format. 0=sssss.s, 1=yyyy/mm/dd hh:mm:ss.s.
    pub fn set_time_format(&mut self, timef: i32) -> &mut Self {
        self.0.timef = timef;
        self
    }

    /// Get time format.
    pub fn time_format(&self) -> i32 {
        self.0.timef
    }

    /// Set number of decimal places for time output.
    pub fn set_time_decimals(&mut self, timeu: i32) -> &mut Self {
        self.0.timeu = timeu;
        self
    }

    /// Get number of decimal places for time output.
    pub fn time_decimals(&self) -> i32 {
        self.0.timeu
    }

    /// Enable or disable output header.
    pub fn set_output_header(&mut self, enable: bool) -> &mut Self {
        self.0.outhead = enable as i32;
        self
    }

    /// Get whether output header is enabled.
    pub fn output_header(&self) -> bool {
        self.0.outhead != 0
    }

    pub(crate) fn as_ffi(&self) -> &ffi::solopt_t {
        &self.0
    }
}

/// File options wrapper around `filopt_t`.
pub struct FilOpt(ffi::filopt_t);

impl Default for FilOpt {
    fn default() -> Self {
        Self(unsafe { std::mem::zeroed() })
    }
}

impl FilOpt {
    pub(crate) fn as_ffi(&self) -> &ffi::filopt_t {
        &self.0
    }
}

/// Error from PPK post-processing.
#[derive(Debug, thiserror::Error)]
pub enum PostposError {
    /// A file path contained an interior null byte.
    #[error("path contains null byte: {0}")]
    NulByte(String),
    /// Too many input files for RTKLIB's fixed-size array.
    #[error("{count} > {max} input files")]
    TooManyInputFiles { count: usize, max: usize },
    /// RTKLIB `postpos()` returned an error code.
    #[error("postpos processing failed with code {0}")]
    ProcessingFailed(i32),
}

/// Run PPK post-processing on RINEX observation and navigation files.
///
/// Returns `Ok(())` on success. Results are written to the output file.
pub fn postpos(
    rover_obs: &str,
    base_obs: &str,
    nav_files: &[&str],
    output: &str,
    popt: &PrcOpt,
    sopt: &SolOpt,
    fopt: &FilOpt,
) -> Result<(), PostposError> {
    const MAX_INFILE: usize = 1000;

    let total = 2 + nav_files.len();
    if total > MAX_INFILE {
        return Err(PostposError::TooManyInputFiles {
            count: total,
            max: MAX_INFILE,
        });
    }

    let mut cstrings = Vec::with_capacity(total);
    let mut paths = vec![rover_obs, base_obs];
    paths.extend_from_slice(nav_files);

    for p in &paths {
        cstrings.push(CString::new(*p).map_err(|_| PostposError::NulByte(p.to_string()))?);
    }

    // C signature is `const char **infile` — the strings are const but the
    // pointer to the array is not, so bindgen generates `*mut *const c_char`.
    // The function does not actually mutate the array.
    let mut ptrs: Vec<*const i8> = cstrings.iter().map(|s| s.as_ptr()).collect();
    let outfile = CString::new(output).map_err(|_| PostposError::NulByte(output.to_string()))?;

    let ts = ffi::gtime_t { time: 0, sec: 0.0 };
    let te = ffi::gtime_t { time: 0, sec: 0.0 };

    let rov = CString::new("").unwrap();
    let base = CString::new("").unwrap();

    let ret = unsafe {
        ffi::postpos(
            ts,
            te,
            0.0, // processing interval (0 = all)
            0.0, // processing unit time (0 = all)
            popt.as_ffi(),
            sopt.as_ffi(),
            fopt.as_ffi(),
            ptrs.as_mut_ptr(),
            ptrs.len() as i32,
            outfile.as_ptr(),
            rov.as_ptr(),
            base.as_ptr(),
        )
    };

    if ret == 0 {
        Ok(())
    } else {
        Err(PostposError::ProcessingFailed(ret))
    }
}
