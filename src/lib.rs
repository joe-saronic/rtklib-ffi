//! Safe Rust bindings for [RTKLIB](https://github.com/rtklibexplorer/RTKLIB).
//!
//! # Features
//!
//! Enable functionality via Cargo features:
//!
//! - **`ppk`** — Post-processed kinematic positioning via [`postpos()`].
//! - **`rtcm`** — RTCM3 message decoding via [`RtcmDecoder`].
//! - **`conv`** — File format conversion.
//! - **`raw`** — Raw receiver data decoding.
//! - **`net`** — Network streaming.
//! - **`gis`** — GIS data support.
//! - **`tle`** — TLE satellite tracking.

use rtklib_sys::rtklib as ffi;

#[cfg(feature = "ppk")]
pub mod ppk;
#[cfg(feature = "ppk")]
pub use ppk::*;

#[cfg(feature = "rtcm")]
pub mod rtcm;
#[cfg(feature = "rtcm")]
pub use rtcm::*;

bitflags::bitflags! {
    /// GNSS navigation system bitmask.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct NavSys: u32 {
        /// Global Positioning System. From SYS_GPS.
        const Gps = ffi::SYS_GPS;
        /// Satellite-Based Augmentation System. From SYS_SBS.
        const Sbs = ffi::SYS_SBS;
        /// GLONASS. From SYS_GLO.
        const Glo = ffi::SYS_GLO;
        /// Galileo. From SYS_GAL.
        const Gal = ffi::SYS_GAL;
        /// Quasi-Zenith Satellite System. From SYS_QZS.
        const Qzs = ffi::SYS_QZS;
        /// BeiDou Navigation Satellite System. From SYS_CMP.
        const Cmp = ffi::SYS_CMP;
        /// Indian Regional Navigation Satellite System / NavIC. From SYS_IRN.
        const Irn = ffi::SYS_IRN;
        /// Low Earth Orbit satellites. From SYS_LEO.
        const Leo = ffi::SYS_LEO;
        /// All navigation systems. From SYS_ALL.
        const All = ffi::SYS_ALL;
    }
}

/// Solution quality status.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum SolStatus {
    /// No solution available. From SOLQ_NONE.
    None = ffi::SOLQ_NONE as i32,
    /// Integer ambiguity resolved. From SOLQ_FIX.
    Fix = ffi::SOLQ_FIX as i32,
    /// Ambiguity not resolved, float solution. From SOLQ_FLOAT.
    Float = ffi::SOLQ_FLOAT as i32,
    /// SBAS corrected solution. From SOLQ_SBAS.
    Sbas = ffi::SOLQ_SBAS as i32,
    /// DGPS/DGNSS corrected solution. From SOLQ_DGPS.
    Dgps = ffi::SOLQ_DGPS as i32,
    /// Single point solution. From SOLQ_SINGLE.
    Single = ffi::SOLQ_SINGLE as i32,
    /// Precise Point Positioning converged solution. From SOLQ_PPP.
    Ppp = ffi::SOLQ_PPP as i32,
    /// Dead reckoning solution. From SOLQ_DR.
    DeadReckoning = ffi::SOLQ_DR as i32,
}
