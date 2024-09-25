// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;

use super::tos_cmds;

// Request

pub fn req() {
    // Generated from Parsec 3.0.1-a.0
    // Content:
    //   cmd: 'tos_accept'
    //   tos_updated_on: ext(1, 946684800000000) i.e. 2000-01-01T01:00:00Z
    let raw: &[u8] = hex!(
        "82a3636d64aa746f735f616363657074ae746f735f757064617465645f6f6ed7010003"
        "5d013b37e000"
    )
    .as_ref();

    let expected = tos_cmds::AnyCmdReq::TosAccept(tos_cmds::tos_accept::Req {
        tos_updated_on: "2000-01-01T00:00:00Z".parse().unwrap(),
    });

    let data = tos_cmds::AnyCmdReq::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // roundtrip check ...
    let tos_cmds::AnyCmdReq::TosAccept(req2) = data else {
        unreachable!()
    };

    let raw2 = req2.dump().unwrap();

    let data2 = tos_cmds::AnyCmdReq::load(&raw2).unwrap();
    p_assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    // Generated from Parsec 3.0.1-a.0
    // Content:
    //   status: 'ok'
    let raw: &[u8] = hex!("81a6737461747573a26f6b").as_ref();

    let expected = tos_cmds::tos_accept::Rep::Ok;
    let data = tos_cmds::tos_accept::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = tos_cmds::tos_accept::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_no_tos() {
    // Generated from Parsec 3.0.1-a.0
    // Content:
    //   status: 'no_tos'
    let raw: &[u8] = hex!("81a6737461747573a66e6f5f746f73").as_ref();

    let expected = tos_cmds::tos_accept::Rep::NoTos;

    let data = tos_cmds::tos_accept::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = tos_cmds::tos_accept::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_tos_mismatch() {
    // Generated from Parsec 3.0.1-a.0
    // Content:
    //   status: 'tos_mismatch'
    let raw: &[u8] = hex!("81a6737461747573ac746f735f6d69736d61746368").as_ref();

    let expected = tos_cmds::tos_accept::Rep::TosMismatch;

    let data = tos_cmds::tos_accept::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = tos_cmds::tos_accept::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
