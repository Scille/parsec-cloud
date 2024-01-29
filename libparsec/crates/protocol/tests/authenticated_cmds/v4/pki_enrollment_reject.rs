// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

use super::authenticated_cmds;

// Request

pub fn req() {
    // Generated from Python implementation (Parsec v2.14.0)
    // Content:
    //   cmd: "pki_enrollment_reject"
    //   enrollment_id: ext(2, hex!("88a75cc10b8d43d9b1f91ad8a12be8ee"))
    //
    let raw = hex!(
        "82a3636d64b5706b695f656e726f6c6c6d656e745f72656a656374ad656e726f6c6c6d656e"
        "745f6964d80288a75cc10b8d43d9b1f91ad8a12be8ee"
    );

    let expected = authenticated_cmds::AnyCmdReq::PkiEnrollmentReject(
        authenticated_cmds::pki_enrollment_reject::Req {
            enrollment_id: EnrollmentID::from_hex("88a75cc10b8d43d9b1f91ad8a12be8ee").unwrap(),
        },
    );

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let authenticated_cmds::AnyCmdReq::PkiEnrollmentReject(req2) = data else {
        unreachable!()
    };

    let raw2 = req2.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    // Generated from Python implementation (Parsec v2.14.0)
    // Content:
    //   status: "ok"
    //
    let raw = hex!("81a6737461747573a26f6b");

    let expected = authenticated_cmds::pki_enrollment_reject::Rep::Ok;

    let data = authenticated_cmds::pki_enrollment_reject::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::pki_enrollment_reject::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_author_not_allowed() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "author_not_allowed"
    let raw = hex!("81a6737461747573b2617574686f725f6e6f745f616c6c6f776564");

    let expected = authenticated_cmds::pki_enrollment_reject::Rep::AuthorNotAllowed;

    let data = authenticated_cmds::pki_enrollment_reject::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::pki_enrollment_reject::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_enrollment_not_found() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "enrollment_not_found"
    //
    let raw = hex!("81a6737461747573b4656e726f6c6c6d656e745f6e6f745f666f756e64");

    let expected = authenticated_cmds::pki_enrollment_reject::Rep::EnrollmentNotFound;

    let data = authenticated_cmds::pki_enrollment_reject::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::pki_enrollment_reject::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_enrollment_no_longer_available() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "enrollment_no_longer_available"
    //
    let raw = hex!(
        "81a6737461747573be656e726f6c6c6d656e745f6e6f5f6c6f6e6765725f617661696c6162"
        "6c65"
    );

    let expected = authenticated_cmds::pki_enrollment_reject::Rep::EnrollmentNoLongerAvailable;

    let data = authenticated_cmds::pki_enrollment_reject::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::pki_enrollment_reject::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
