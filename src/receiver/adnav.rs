//! Advanced Navigation Packet Protocol (ANPP) receiver decoder.
//!
//! ```no_run
//! use rtklib_ffi::receiver::{AnppDecoder, DecodeStatus};
//!
//! let mut decoder = AnppDecoder::try_new(0).unwrap();
//! # let anpp_bytes: Vec<u8> = vec![];
//!
//! for &byte in &anpp_bytes {
//!     let Some(status) = decoder.decode(byte) else { continue; };
//!     match status {
//!         DecodeStatus::Observation => {
//!             let obs = decoder.observations();
//!             // process observations...
//!         }
//!         DecodeStatus::Ephemeris => {
//!             let sat = decoder.ephemeris_sat();
//!             // handle ephemeris update for satellite sat...
//!         }
//!         _ => {}
//!     }
//! }
//! ```

use super::{DecodeStatus, RawReceiver};
use crate::{util::copy_osstr, DecoderInitError};
use rtklib_sys::rtklib as ffi;
use std::{convert::TryFrom, ops::Deref};

/// Advanced Navigation ANPP receiver data decoder.
pub struct AnppDecoder(RawReceiver);

impl AnppDecoder {
    /// Create a new ANPP decoder that emits observations from the antenna
    /// identified by `receiver_num`.
    ///
    /// Advanced Navigation systems can support multiple antennae in packet 60,
    /// identified by a zero-based receiver number. Packets whose
    /// `receiver_number` does not match `receiver_num` are dropped silently.
    ///
    /// Internally this writes `-RCVR<n>` into the underlying `raw_t.opt`
    /// field, the same string-option mechanism the C decoder uses for tools
    /// like convbin (`-ro "-RCVR1"`).
    ///
    /// Returns `Err` if RTKLIB cannot allocate internal buffers.
    pub fn try_new(receiver_num: u8) -> Result<Self, DecoderInitError> {
        let mut decoder = Self(RawReceiver::init(ffi::STRFMT_ANPP as i32)?);
        copy_osstr(&mut decoder.0.0.opt, format!("-RCVR{receiver_num}"));
        Ok(decoder)
    }

    /// Feed one byte into the ANPP decoder.
    ///
    /// Returns `None` if the byte did not complete a recognized message.
    pub fn decode(&mut self, byte: u8) -> Option<DecodeStatus> {
        let ret = unsafe { ffi::input_anpp(self.0.0.as_mut(), byte) };
        DecodeStatus::try_from(ret).ok()
    }
}

impl Deref for AnppDecoder {
    type Target = RawReceiver;

    fn deref(&self) -> &RawReceiver { &self.0 }
}
