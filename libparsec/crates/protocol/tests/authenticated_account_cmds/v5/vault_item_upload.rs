// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;

use super::authenticated_account_cmds;

// Request

pub fn req() {
    // Generated from Parsec 3.4.0-a.7+dev
    // Content:
    //   cmd: 'vault_item_upload'
    //   item_fingerprint: 0x3c64756d6d792066696e6765727072696e743e
    //   item: 0x3c64756d6d79206974656d3e
    let raw: &[u8] = hex!(
        "83a3636d64b17661756c745f6974656d5f75706c6f6164b06974656d5f66696e676572"
        "7072696e74c4133c64756d6d792066696e6765727072696e743ea46974656dc40c3c64"
        "756d6d79206974656d3e"
    )
    .as_ref();

    let req = authenticated_account_cmds::vault_item_upload::Req {
        item: b"<dummy item>".as_ref().into(),
        item_fingerprint: b"<dummy fingerprint>".as_ref().into(),
    };

    let expected = authenticated_account_cmds::AnyCmdReq::VaultItemUpload(req.clone());
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

    let expected = authenticated_account_cmds::vault_item_upload::Rep::Ok {};
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = authenticated_account_cmds::vault_item_upload::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_account_cmds::vault_item_upload::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_fingerprint_already_exists() {
    // Generated from Parsec 3.4.0-a.7+dev
    // Content:
    //   status: 'fingerprint_already_exists'
    let raw: &[u8] =
        hex!("81a6737461747573ba66696e6765727072696e745f616c72656164795f657869737473").as_ref();

    let expected = authenticated_account_cmds::vault_item_upload::Rep::FingerprintAlreadyExists {};
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = authenticated_account_cmds::vault_item_upload::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_account_cmds::vault_item_upload::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
