// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

// wrap this in each test function
// macro that contains a closure
use super::authenticated_cmds;

use libparsec_tests_lite::{hex, p_assert_eq};
use libparsec_types::prelude::*;

// Request

pub fn req() {
    // Generated from Parsec 3.7.1-a.0+dev
    // Content:
    //   cmd: 'async_enrollment_reject'
    //   enrollment_id: ext(2, 0x1d3353157d7d4e95ad2fdea7b3bd19c5)
    let raw: &[u8] = hex!(
        "82a3636d64b76173796e635f656e726f6c6c6d656e745f72656a656374ad656e726f6c"
        "6c6d656e745f6964d8021d3353157d7d4e95ad2fdea7b3bd19c5"
    )
    .as_ref();

    let req = authenticated_cmds::async_enrollment_reject::Req {
        enrollment_id: AsyncEnrollmentID::from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5").unwrap(),
    };
    println!("***expected: {:?}", req.dump().unwrap());

    let expected = authenticated_cmds::AnyCmdReq::AsyncEnrollmentReject(req.clone());

    let data = authenticated_cmds::AnyCmdReq::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = req.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    // Generated from Parsec 3.7.1-a.0+dev
    // Content:
    //   status: 'ok'
    let raw: &[u8] = hex!("81a6737461747573a26f6b").as_ref();
    let expected = authenticated_cmds::async_enrollment_reject::Rep::Ok;
    println!("***expected: {:?}", expected.dump().unwrap());

    rep_helper(raw, expected);
}

pub fn rep_author_not_allowed() {
    // Generated from Parsec 3.7.1-a.0+dev
    // Content:
    //   status: 'author_not_allowed'
    let raw: &[u8] = hex!("81a6737461747573b2617574686f725f6e6f745f616c6c6f776564").as_ref();
    let expected = authenticated_cmds::async_enrollment_reject::Rep::AuthorNotAllowed;
    println!("***expected: {:?}", expected.dump().unwrap());

    rep_helper(raw, expected);
}

pub fn rep_enrollment_not_found() {
    // Generated from Parsec 3.7.1-a.0+dev
    // Content:
    //   status: 'enrollment_not_found'
    let raw: &[u8] = hex!("81a6737461747573b4656e726f6c6c6d656e745f6e6f745f666f756e64").as_ref();

    let expected = authenticated_cmds::async_enrollment_reject::Rep::EnrollmentNotFound;
    println!("***expected: {:?}", expected.dump().unwrap());

    rep_helper(raw, expected);
}

pub fn rep_enrollment_no_longer_available() {
    // Generated from Parsec 3.7.1-a.0+dev
    // Content:
    //   status: 'enrollment_no_longer_available'
    let raw: &[u8] = hex!(
        "81a6737461747573be656e726f6c6c6d656e745f6e6f5f6c6f6e6765725f617661696c"
        "61626c65"
    )
    .as_ref();
    let expected = authenticated_cmds::async_enrollment_reject::Rep::EnrollmentNoLongerAvailable;
    println!("***expected: {:?}", expected.dump().unwrap());

    rep_helper(raw, expected);
}

fn rep_helper(raw: &[u8], expected: authenticated_cmds::async_enrollment_reject::Rep) {
    let data = authenticated_cmds::async_enrollment_reject::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::async_enrollment_reject::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
