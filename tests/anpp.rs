//! Integration tests for the Advanced Navigation ANPP decoder.
#![cfg(feature = "receivers")]

use rtklib_ffi::receiver::{AnppDecoder, DecodeStatus};

const SNIPPET: &str = "tests/snippet.anpp";

/// Count decode-status events of a given variant when streaming the snippet
/// through an `AnppDecoder` configured for the given receiver number.
fn count_status(receiver_num: u8, target: DecodeStatus) -> usize {
    let data = std::fs::read(SNIPPET).expect("failed to read snippet");
    let mut decoder = AnppDecoder::try_new(receiver_num).expect("failed to init decoder");
    data.iter()
        .filter(|&&b| decoder.decode(b) == Some(target))
        .count()
}

// The snippet starts and ends mid-packet (LRC of first/last 5 bytes is
// nonzero), exercising the sliding-window resync. Both antennae produce
// exactly one fully reassembled epoch each: antenna 0 from a 17-fragment
// run, antenna 1 from an 18-fragment run, with IMU packets (SystemState,
// UnixTime, RawSensors, etc.) interleaved between fragments and a single
// GPS PRN 20 ephemeris arriving between the two epochs.

#[test]
fn decode_anpp_snippet_antenna_0_observations() {
    assert_eq!(count_status(0, DecodeStatus::Observation), 1);
}

#[test]
fn decode_anpp_snippet_antenna_1_observations() {
    assert_eq!(count_status(1, DecodeStatus::Observation), 1);
}

#[test]
fn decode_anpp_snippet_one_ephemeris() {
    // Ephemeris is not gated on receiver_num (it's a constellation-level
    // packet), so both decoders see the same single GPS ephemeris.
    assert_eq!(count_status(0, DecodeStatus::Ephemeris), 1);
    assert_eq!(count_status(1, DecodeStatus::Ephemeris), 1);
}
