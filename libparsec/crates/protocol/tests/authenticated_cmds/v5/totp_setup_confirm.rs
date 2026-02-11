// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;

use super::authenticated_cmds;

// Request

pub fn req() {
    // Generated from Parsec 3.7.2-a.0+dev
    // Content:
    //   cmd: 'totp_setup_confirm'
    //   one_time_password: '123456'
    let raw: &[u8] = hex!(
        "82a3636d64b2746f74705f73657475705f636f6e6669726db16f6e655f74696d655f70"
        "617373776f7264a6313233343536"
    )
    .as_ref();

    let req = authenticated_cmds::totp_setup_confirm::Req {
        one_time_password: "123456".to_owned(),
    };

    println!("***expected: {:?}", req.dump().unwrap());

    let expected = authenticated_cmds::AnyCmdReq::TotpSetupConfirm(req);

    let data = authenticated_cmds::AnyCmdReq::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let authenticated_cmds::AnyCmdReq::TotpSetupConfirm(req2) = data else {
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
    let raw: &[u8] = hex!("81a6737461747573a26f6b").as_ref();

    let expected = authenticated_cmds::totp_setup_confirm::Rep::Ok;

    println!("***expected: {:?}", expected.dump().unwrap());

    let data = authenticated_cmds::totp_setup_confirm::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::totp_setup_confirm::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_invalid_one_time_password() {
    // Generated from Parsec 3.7.2-a.0+dev
    // Content:
    //   status: 'invalid_one_time_password'
    let raw: &[u8] =
        hex!("81a6737461747573b9696e76616c69645f6f6e655f74696d655f70617373776f7264").as_ref();

    let expected = authenticated_cmds::totp_setup_confirm::Rep::InvalidOneTimePassword;

    println!("***expected: {:?}", expected.dump().unwrap());

    let data = authenticated_cmds::totp_setup_confirm::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::totp_setup_confirm::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_already_setup() {
    // Generated from Parsec 3.7.2-a.0+dev
    // Content:
    //   status: 'already_setup'
    let raw: &[u8] = hex!("81a6737461747573ad616c72656164795f7365747570").as_ref();

    let expected = authenticated_cmds::totp_setup_confirm::Rep::AlreadySetup;

    println!("***expected: {:?}", expected.dump().unwrap());

    let data = authenticated_cmds::totp_setup_confirm::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::totp_setup_confirm::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
