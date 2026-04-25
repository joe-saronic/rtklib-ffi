//! Tests for common rtklib-ffi types.

use rtklib_ffi::{NavSys, SolStatus};

#[test]
fn navsys_bitflags() {
    let gps_glo = NavSys::Gps | NavSys::Glo;
    assert!(gps_glo.contains(NavSys::Gps));
    assert!(gps_glo.contains(NavSys::Glo));
    assert!(!gps_glo.contains(NavSys::Gal));

    assert!(NavSys::All.contains(NavSys::Gps));
    assert!(NavSys::All.contains(NavSys::Glo));
    assert!(NavSys::All.contains(NavSys::Gal));
}

#[test]
fn sol_status_values() {
    assert_eq!(SolStatus::try_from(0u32).unwrap(), SolStatus::None);
    assert_eq!(SolStatus::try_from(1u32).unwrap(), SolStatus::Fix);
    assert_eq!(SolStatus::try_from(2u32).unwrap(), SolStatus::Float);
    assert_eq!(SolStatus::try_from(5u32).unwrap(), SolStatus::Single);
    assert_ne!(SolStatus::Fix, SolStatus::Float);
}
