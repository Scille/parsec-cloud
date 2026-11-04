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
    //   cmd: 'totp_create_opaque_key'
    let raw: &[u8] = hex!("81a3636d64b6746f74705f6372656174655f6f70617175655f6b6579").as_ref();

    let req = authenticated_cmds::totp_create_opaque_key::Req;

    println!("***expected: {:?}", req.dump().unwrap());

    let expected = authenticated_cmds::AnyCmdReq::TotpCreateOpaqueKey(req);

    let data = authenticated_cmds::AnyCmdReq::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let authenticated_cmds::AnyCmdReq::TotpCreateOpaqueKey(req2) = data else {
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
    //   opaque_key: 0xb1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1a2
    //   opaque_key_id: ext(2, 0xa1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6)
    let raw: &[u8] = hex!(
        "83a6737461747573a26f6baa6f70617175655f6b6579c420b1b2c3d4e5f6a7b8c9d0e1"
        "f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1a2ad6f70617175655f6b65795f6964"
        "d802a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6"
    )
    .as_ref();

    let expected = authenticated_cmds::totp_create_opaque_key::Rep::Ok {
        opaque_key_id: TOTPOpaqueKeyID::from_hex("a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6").unwrap(),
        opaque_key: SecretKey::from(hex!(
            "b1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1a2"
        )),
    };

    println!("***expected: {:?}", expected.dump().unwrap());

    let data = authenticated_cmds::totp_create_opaque_key::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::totp_create_opaque_key::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
