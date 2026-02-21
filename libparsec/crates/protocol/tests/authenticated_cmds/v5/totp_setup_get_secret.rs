// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

use super::authenticated_cmds;

// Request

pub fn req() {
    // Generated from Parsec 3.7.2-a.0+dev
    // Content:
    //   cmd: 'totp_setup_get_secret'
    let raw: &[u8] = hex!("81a3636d64b5746f74705f73657475705f6765745f736563726574").as_ref();

    let req = authenticated_cmds::totp_setup_get_secret::Req;

    println!("***expected: {:?}", req.dump().unwrap());

    let expected = authenticated_cmds::AnyCmdReq::TotpSetupGetSecret(req);

    let data = authenticated_cmds::AnyCmdReq::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let authenticated_cmds::AnyCmdReq::TotpSetupGetSecret(req2) = data else {
        unreachable!()
    };

    let raw2 = req2.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    // Generated from Parsec 3.7.2-a.0+dev
    // Content:
    //   status: 'ok'
    //   totp_secret: 'JBSWY3DPEHPK3PXP'  // cspell:disable-line
    let raw: &[u8] = hex!(
        "82a6737461747573a26f6bab746f74705f736563726574b04a42535759334450454850"
        "4b33505850"
    )
    .as_ref();

    let expected = authenticated_cmds::totp_setup_get_secret::Rep::Ok {
        totp_secret: Bytes::from_static(b"JBSWY3DPEHPK3PXP"), // cspell:disable-line
    };

    println!("***expected: {:?}", expected.dump().unwrap());

    let data = authenticated_cmds::totp_setup_get_secret::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::totp_setup_get_secret::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_already_setup() {
    // Generated from Parsec 3.7.2-a.0+dev
    // Content:
    //   status: 'already_setup'
    let raw: &[u8] = hex!("81a6737461747573ad616c72656164795f7365747570").as_ref();

    let expected = authenticated_cmds::totp_setup_get_secret::Rep::AlreadySetup;

    println!("***expected: {:?}", expected.dump().unwrap());

    let data = authenticated_cmds::totp_setup_get_secret::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::totp_setup_get_secret::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
