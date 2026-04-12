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
    assert_eq!(SolStatus::None as i32, 0);
    assert_eq!(SolStatus::Fix as i32, 1);
    assert_eq!(SolStatus::Float as i32, 2);
    assert_eq!(SolStatus::Single as i32, 5);
    assert_ne!(SolStatus::Fix, SolStatus::Float);
}
