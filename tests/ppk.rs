//! Tests for the rtklib-ffi PPK module.
//!
//! Tests that call `postpos` are marked `#[serial]` because RTKLIB uses
//! global state internally, causing SIGSEGV when multiple postpos calls
//! run in parallel.
#![cfg(feature = "ppk")]

use rtklib_ffi::{
    ppk::{
        postpos, ArMode, FilOpt, IonoOpt, PosMode, PostposError, PrcOpt, SolFormat, SolOpt, TropOpt,
    },
    NavSys,
};
use serial_test::serial;
use std::{fs, path::Path};

#[test]
fn prcopt_kinematic_builder() {
    let mut opt = PrcOpt::kinematic();
    opt.set_navsys(NavSys::Gps | NavSys::Glo | NavSys::Gal)
        .set_frequencies(2)
        .set_elevation_mask(15.0)
        .set_ar_mode(ArMode::FixAndHold)
        .set_ionosphere(IonoOpt::IonFreeLc)
        .set_troposphere(TropOpt::Saastamoinen);

    assert_eq!(opt.mode(), PosMode::Kinematic);
    assert_eq!(opt.navsys(), NavSys::Gps | NavSys::Glo | NavSys::Gal);
    assert_eq!(opt.frequencies(), 2);
    assert!((opt.elevation_mask() - 15.0_f64.to_radians()).abs() < 1e-12);
    assert_eq!(opt.ar_mode(), ArMode::FixAndHold);
    assert_eq!(opt.ionosphere(), IonoOpt::IonFreeLc);
    assert_eq!(opt.troposphere(), TropOpt::Saastamoinen);
}

#[test]
fn prcopt_static_builder() {
    let mut opt = PrcOpt::static_mode();
    opt.set_navsys(NavSys::Gps)
        .set_frequencies(1)
        .set_elevation_mask(10.0)
        .set_ar_mode(ArMode::Continuous);

    assert_eq!(opt.mode(), PosMode::Static);
    assert_eq!(opt.navsys(), NavSys::Gps);
    assert_eq!(opt.frequencies(), 1);
    assert!((opt.elevation_mask() - 10.0_f64.to_radians()).abs() < 1e-12);
    assert_eq!(opt.ar_mode(), ArMode::Continuous);
}

#[test]
fn solopt_setters() {
    let mut sopt = SolOpt::default();
    sopt.set_format(SolFormat::Xyz)
        .set_time_format(1)
        .set_time_decimals(3)
        .set_output_header(true);

    assert_eq!(sopt.format(), SolFormat::Xyz);
    assert_eq!(sopt.time_format(), 1);
    assert_eq!(sopt.time_decimals(), 3);
    assert!(sopt.output_header());
}

#[test]
#[serial]
fn ppk_with_rinex2_test_data() {
    let data = "rtklib-sys/rtklib/test/data/rinex";
    let rover = format!("{}/07590920.05o", data);
    let base = format!("{}/30400920.05o", data);
    let nav = format!("{}/07590920.05n", data);
    let output = "/tmp/rtklib-ffi-test-output.pos";

    let mut popt = PrcOpt::kinematic();
    popt.set_navsys(NavSys::Gps)
        .set_frequencies(1)
        .set_elevation_mask(15.0)
        .set_ar_mode(ArMode::Continuous)
        .set_ionosphere(IonoOpt::Broadcast)
        .set_troposphere(TropOpt::Saastamoinen);

    let mut sopt = SolOpt::default();
    sopt.set_format(SolFormat::Llh)
        .set_time_format(1)
        .set_time_decimals(3);

    let fopt = FilOpt::default();

    postpos(&rover, &base, &[&nav], output, &popt, &sopt, &fopt).expect("postpos failed");

    let metadata = fs::metadata(output).expect("output file not created");
    assert!(metadata.len() > 0, "output file is empty");

    fs::remove_file(output).ok();
}

#[test]
#[serial]
fn postpos_missing_file_returns_error_or_zero() {
    let output = "/tmp/rtklib-ffi-test-missing.pos";

    let popt = PrcOpt::kinematic();
    let sopt = SolOpt::default();
    let fopt = FilOpt::default();

    let result = postpos(
        "nonexistent_rover.obs",
        "nonexistent_base.obs",
        &["nonexistent.nav"],
        output,
        &popt,
        &sopt,
        &fopt,
    );

    // RTKLIB returns success (0) even for missing files — it just processes
    // zero epochs and produces no output file.
    assert!(result.is_ok());
    assert!(!Path::new(output).exists());
}

#[test]
#[serial]
fn postpos_nul_byte_in_path() {
    let popt = PrcOpt::kinematic();
    let sopt = SolOpt::default();
    let fopt = FilOpt::default();

    let result = postpos(
        "rover\0.obs",
        "base.obs",
        &["nav.nav"],
        "out.pos",
        &popt,
        &sopt,
        &fopt,
    );

    assert!(matches!(result, Err(PostposError::NulByte(_))));
}
