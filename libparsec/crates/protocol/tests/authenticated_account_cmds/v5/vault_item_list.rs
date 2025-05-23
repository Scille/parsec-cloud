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
    //   cmd: 'vault_item_list'
    let raw: &[u8] = hex!("81a3636d64af7661756c745f6974656d5f6c697374").as_ref();

    let req = authenticated_account_cmds::vault_item_list::Req {};

    let expected = authenticated_account_cmds::AnyCmdReq::VaultItemList(req.clone());
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
    // Generated from Parsec 3.4.0-a.7+dev
    // Content:
    //   status: 'ok'
    //   items: {
    //     0x076a27c79e5ace2a3d47f9dd2e83e4ff6ea8872b3c2218f66c92b89b55f36560: 0x646174612031,
    //     0xe37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6: 0x646174612032,
    //   }
    //   key_access: 0x3c6b6579206163636573733e
    let raw: &[u8] = hex!(
        "83a6737461747573a26f6ba56974656d7382c420076a27c79e5ace2a3d47f9dd2e83e4"
        "ff6ea8872b3c2218f66c92b89b55f36560c406646174612031c420e37ce3b00a1f15b3"
        "de62029972345420b76313a885c6ccc6e3b5547857b3ecc6c406646174612032aa6b65"
        "795f616363657373c40c3c6b6579206163636573733e"
    )
    .as_ref();

    let expected = authenticated_account_cmds::vault_item_list::Rep::Ok {
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
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = authenticated_account_cmds::vault_item_list::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_account_cmds::vault_item_list::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
