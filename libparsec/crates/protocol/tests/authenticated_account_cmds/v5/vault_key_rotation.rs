// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use std::collections::HashMap;

use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

use super::authenticated_account_cmds;

// Request

pub fn req() {
    // Generated from Parsec 3.4.0-a.7+dev
    // Content:
    //   cmd: 'vault_key_rotation'
    //   key_access: 0x3c6b6579206163636573733e
    //   items: {
    //     0x076a27c79e5ace2a3d47f9dd2e83e4ff6ea8872b3c2218f66c92b89b55f36560: 0x646174612031,
    //     0xe37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6: 0x646174612032
    //   }
    let raw: &[u8] = hex!(
        "83a3636d64b27661756c745f6b65795f726f746174696f6eaa6b65795f616363657373"
        "c40c3c6b6579206163636573733ea56974656d7382c420076a27c79e5ace2a3d47f9dd"
        "2e83e4ff6ea8872b3c2218f66c92b89b55f36560c406646174612031c420e37ce3b00a"
        "1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6c406646174612032"
    )
    .as_ref();

    let req = authenticated_account_cmds::vault_key_rotation::Req {
        items: HashMap::from_iter([
            (
                HashDigest::from(hex!(
                    "076a27c79e5ace2a3d47f9dd2e83e4ff6ea8872b3c2218f66c92b89b55f36560"
                )),
                b"data 1".as_ref().into(),
            ),
            (
                HashDigest::from(hex!(
                    "e37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6"
                )),
                b"data 2".as_ref().into(),
            ),
        ]),
        key_access: b"<key access>".as_ref().into(),
    };

    let expected = authenticated_account_cmds::AnyCmdReq::VaultKeyRotation(req.clone());
    println!("***expected: {:?}", req.dump().unwrap());
    let data = authenticated_account_cmds::AnyCmdReq::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = req.dump().unwrap();

    let data2 = authenticated_account_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    // Generated from Parsec 3.3.3-a.0+dev
    // Content:
    //   status: 'ok'
    //   pong: 'pong'
    let raw: &[u8] = hex!("82a6737461747573a26f6ba4706f6e67a4706f6e67").as_ref();

    let expected = authenticated_account_cmds::vault_key_rotation::Rep::Ok {};
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = authenticated_account_cmds::vault_key_rotation::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_account_cmds::vault_key_rotation::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
