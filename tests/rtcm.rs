//! Tests for RTCM3 decoding via the safe `RtcmDecoder` wrapper.
//!
//! Feeds raw RTCM3 bytes from `tests/debug.rtcm` through `input_rtcm3`
//! one byte at a time. The file contains GPS and Galileo ephemeris messages
//! alongside GPS and Galileo MSM7 observation messages.
#![cfg(feature = "rtcm")]

use rtklib_ffi::rtcm::{DecodeResult, MsgType, RtcmDecoder};

#[test]
fn decode_rtcm3_ephemeris() {
    let data = std::fs::read("tests/debug.rtcm").expect("failed to read test file");
    let mut decoder = RtcmDecoder::new().expect("failed to init RTCM decoder");

    let mut ephemeris_count = 0u32;
    let mut msg_types: Vec<MsgType> = Vec::new();

    for &byte in &data {
        match decoder.decode(byte) {
            Ok(DecodeResult::Incomplete) => {}
            Ok(DecodeResult::Ephemeris) => {
                ephemeris_count += 1;
                if let Ok(mt) = decoder.message_type() {
                    msg_types.push(mt);
                }
            }
            Ok(_) => {
                if let Ok(mt) = decoder.message_type() {
                    msg_types.push(mt);
                }
            }
            Err(_) => {}
        }
    }

    // The file contains GPS and Galileo ephemeris messages alongside
    // GPS and Galileo MSM7 messages with sync=1 that don't produce
    // a complete observation epoch.
    assert!(
        ephemeris_count >= 2,
        "expected at least 2 ephemeris messages"
    );
    assert!(
        msg_types.contains(&MsgType::GpsEphemeris),
        "expected GPS ephemeris"
    );
    assert!(
        msg_types.contains(&MsgType::GalInavEphemeris),
        "expected Galileo ephemeris"
    );
}
