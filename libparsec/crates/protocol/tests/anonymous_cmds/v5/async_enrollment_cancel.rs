// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::{hex, p_assert_eq};
use libparsec_types::prelude::*;

use super::anonymous_cmds;

// Request

pub fn req() {
    // Generated from Parsec 3.7.2-a.0+dev
    // Content:
    //   cmd: 'async_enrollment_cancel'
    //   enrollment_id: ext(2, 0x7c021dd1e2cd4c328f00dd6e64b89279)
    let raw: &[u8] = hex!(
        "82a3636d64b76173796e635f656e726f6c6c6d656e745f63616e63656cad656e726f6c"
        "6c6d656e745f6964d8027c021dd1e2cd4c328f00dd6e64b89279"
    )
    .as_ref();

    let req = anonymous_cmds::async_enrollment_cancel::Req {
        enrollment_id: AsyncEnrollmentID::from_hex("7c021dd1e2cd4c328f00dd6e64b89279").unwrap(),
    };
    println!("***expected: {:?}", req.dump().unwrap());

    let expected = anonymous_cmds::AnyCmdReq::AsyncEnrollmentCancel(req.clone());

    let data = anonymous_cmds::AnyCmdReq::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = req.dump().unwrap();

    let data2 = anonymous_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    // Generated from Parsec 3.7.2-a.0+dev
    // Content:
    //   status: 'ok'
    let raw: &[u8] = hex!("81a6737461747573a26f6b").as_ref();

    let expected = anonymous_cmds::async_enrollment_cancel::Rep::Ok;
    println!("***expected: {:?}", expected.dump().unwrap());

    let data = anonymous_cmds::async_enrollment_cancel::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_cmds::async_enrollment_cancel::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_enrollment_not_found() {
    // Generated from Parsec 3.7.2-a.0+dev
    // Content:
    //   status: 'enrollment_not_found'
    let raw: &[u8] = hex!("81a6737461747573b4656e726f6c6c6d656e745f6e6f745f666f756e64").as_ref();

    let expected = anonymous_cmds::async_enrollment_cancel::Rep::EnrollmentNotFound;
    println!("***expected: {:?}", expected.dump().unwrap());

    let data = anonymous_cmds::async_enrollment_cancel::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_cmds::async_enrollment_cancel::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_enrollment_no_longer_available() {
    // Generated from Parsec 3.7.2-a.0+dev
    // Content:
    //   status: 'enrollment_no_longer_available'
    let raw: &[u8] = hex!(
        "81a6737461747573be656e726f6c6c6d656e745f6e6f5f6c6f6e6765725f617661696c"
        "61626c65"
    )
    .as_ref();

    let expected = anonymous_cmds::async_enrollment_cancel::Rep::EnrollmentNoLongerAvailable;
    println!("***expected: {:?}", expected.dump().unwrap());

    let data = anonymous_cmds::async_enrollment_cancel::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_cmds::async_enrollment_cancel::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
