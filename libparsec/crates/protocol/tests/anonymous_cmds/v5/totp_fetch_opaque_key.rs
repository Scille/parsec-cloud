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
    //   cmd: 'totp_fetch_opaque_key'
    //   user_id: ext(2, 0x85a277d37f664edcb8bab56f225c9c3d)
    //   opaque_key_id: ext(2, 0xa1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6)
    //   one_time_password: '123456'
    let raw: &[u8] = hex!(
        "84a3636d64b5746f74705f66657463685f6f70617175655f6b6579a7757365725f6964"
        "d80285a277d37f664edcb8bab56f225c9c3dad6f70617175655f6b65795f6964d802a1"
        "b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6b16f6e655f74696d655f70617373776f7264a631"
        "3233343536"
    )
    .as_ref();

    let req = anonymous_cmds::totp_fetch_opaque_key::Req {
        user_id: UserID::from_hex("85a277d37f664edcb8bab56f225c9c3d").unwrap(),
        opaque_key_id: TOTPOpaqueKeyID::from_hex("a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6").unwrap(),
        one_time_password: "123456".to_owned(),
    };

    println!("***expected: {:?}", req.dump().unwrap());

    let expected = anonymous_cmds::AnyCmdReq::TotpFetchOpaqueKey(req);

    let data = anonymous_cmds::AnyCmdReq::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let anonymous_cmds::AnyCmdReq::TotpFetchOpaqueKey(req2) = data else {
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
    //   opaque_key: 0xb1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1a2
    let raw: &[u8] = hex!(
        "82a6737461747573a26f6baa6f70617175655f6b6579c420b1b2c3d4e5f6a7b8c9d0e1"
        "f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1a2"
    )
    .as_ref();

    let expected = anonymous_cmds::totp_fetch_opaque_key::Rep::Ok {
        opaque_key: SecretKey::from(hex!(
            "b1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1a2"
        )),
    };

    println!("***expected: {:?}", expected.dump().unwrap());

    let data = anonymous_cmds::totp_fetch_opaque_key::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_cmds::totp_fetch_opaque_key::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_invalid_one_time_password() {
    // Generated from Parsec 3.7.2-a.0+dev
    // Content:
    //   status: 'invalid_one_time_password'
    let raw: &[u8] =
        hex!("81a6737461747573b9696e76616c69645f6f6e655f74696d655f70617373776f7264").as_ref();

    let expected = anonymous_cmds::totp_fetch_opaque_key::Rep::InvalidOneTimePassword;

    println!("***expected: {:?}", expected.dump().unwrap());

    let data = anonymous_cmds::totp_fetch_opaque_key::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_cmds::totp_fetch_opaque_key::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_throttled() {
    // Generated from Parsec 3.7.2-a.0+dev
    // Content:
    //   status: 'throttled'
    //   wait_until: ext(1, 946774800000000) i.e. 2000-01-02T02:00:00Z
    let raw: &[u8] = hex!(
        "82a6737461747573a97468726f74746c6564aa776169745f756e74696cd70100035d16"
        "2fa2e400"
    )
    .as_ref();

    let expected = anonymous_cmds::totp_fetch_opaque_key::Rep::Throttled {
        wait_until: "2000-01-02T01:00:00Z".parse().unwrap(),
    };

    println!("***expected: {:?}", expected.dump().unwrap());

    let data = anonymous_cmds::totp_fetch_opaque_key::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_cmds::totp_fetch_opaque_key::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
