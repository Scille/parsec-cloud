// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

use super::anonymous_cmds;

// Request

pub fn req() {
    // Generated from Parsec 3.7.2-a.0+dev
    // Content:
    //   cmd: 'totp_setup_get_secret'
    //   user_id: ext(2, 0x85a277d37f664edcb8bab56f225c9c3d)
    //   token: 0xa1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6
    let raw: &[u8] = hex!(
        "83a3636d64b5746f74705f73657475705f6765745f736563726574a7757365725f6964"
        "d80285a277d37f664edcb8bab56f225c9c3da5746f6b656ec410a1b2c3d4e5f6a7b8c9"
        "d0e1f2a3b4c5d6"
    )
    .as_ref();

    let req = anonymous_cmds::totp_setup_get_secret::Req {
        user_id: UserID::from_hex("85a277d37f664edcb8bab56f225c9c3d").unwrap(),
        token: AccessToken::from_hex("a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6").unwrap(),
    };

    println!("***expected: {:?}", req.dump().unwrap());

    let expected = anonymous_cmds::AnyCmdReq::TotpSetupGetSecret(req);

    let data = anonymous_cmds::AnyCmdReq::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let anonymous_cmds::AnyCmdReq::TotpSetupGetSecret(req2) = data else {
        unreachable!()
    };

    let raw2 = req2.dump().unwrap();

    let data2 = anonymous_cmds::AnyCmdReq::load(&raw2).unwrap();

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

    let expected = anonymous_cmds::totp_setup_get_secret::Rep::Ok {
        totp_secret: Bytes::from_static(b"JBSWY3DPEHPK3PXP"), // cspell:disable-line
    };

    println!("***expected: {:?}", expected.dump().unwrap());

    let data = anonymous_cmds::totp_setup_get_secret::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_cmds::totp_setup_get_secret::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_bad_token() {
    // Generated from Parsec 3.7.2-a.0+dev
    // Content:
    //   status: 'bad_token'
    let raw: &[u8] = hex!("81a6737461747573a96261645f746f6b656e").as_ref();

    let expected = anonymous_cmds::totp_setup_get_secret::Rep::BadToken;

    println!("***expected: {:?}", expected.dump().unwrap());

    let data = anonymous_cmds::totp_setup_get_secret::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_cmds::totp_setup_get_secret::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
