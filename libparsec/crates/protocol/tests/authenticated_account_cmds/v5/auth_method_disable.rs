// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

use super::authenticated_account_cmds;

// Request
pub fn req() {
    // Generated from Parsec 3.4.1-a.0+dev
    // Content:
    //   cmd: 'auth_method_disable'
    //   auth_method_id: ext(2, 0x9aae259f748045cc9fe7146eab0b132e)
    let raw: &[u8] = hex!(
        "82a3636d64b3617574685f6d6574686f645f64697361626c65ae617574685f6d657468"
        "6f645f6964d8029aae259f748045cc9fe7146eab0b132e"
    )
    .as_ref();

    let req = authenticated_account_cmds::auth_method_disable::Req {
        auth_method_id: AccountAuthMethodID::from_hex("9aae259f748045cc9fe7146eab0b132e").unwrap(),
    };
    let expected = authenticated_account_cmds::AnyCmdReq::AuthMethodDisable(req.clone());
    println!("***expected: {:?}", req.dump().unwrap());

    let data = authenticated_account_cmds::AnyCmdReq::load(raw).unwrap();
    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let authenticated_account_cmds::AnyCmdReq::AuthMethodDisable(req2) = data else {
        unreachable!()
    };

    let raw2 = req2.dump().unwrap();

    let data2 = authenticated_account_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    // Generated from Parsec 3.4.1-a.0+dev
    // Content:
    //   status: 'ok'
    let raw: &[u8] = hex!("81a6737461747573a26f6b").as_ref();

    let expected = authenticated_account_cmds::auth_method_disable::Rep::Ok {};
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = authenticated_account_cmds::auth_method_disable::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();
    let data2 = authenticated_account_cmds::auth_method_disable::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_auth_method_not_found() {
    // Generated from Parsec 3.4.1-a.0+dev
    // Content:
    //   status: 'auth_method_not_found'
    let raw: &[u8] = hex!("81a6737461747573b5617574685f6d6574686f645f6e6f745f666f756e64").as_ref();

    let expected = authenticated_account_cmds::auth_method_disable::Rep::AuthMethodNotFound {};
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = authenticated_account_cmds::auth_method_disable::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();
    let data2 = authenticated_account_cmds::auth_method_disable::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_auth_method_already_disabled() {
    // Generated from Parsec 3.4.1-a.0+dev
    // Content:
    //   status: 'auth_method_already_disabled'
    let raw: &[u8] = hex!(
        "81a6737461747573bc617574685f6d6574686f645f616c72656164795f64697361626c"
        "6564"
    )
    .as_ref();

    let expected =
        authenticated_account_cmds::auth_method_disable::Rep::AuthMethodAlreadyDisabled {};
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = authenticated_account_cmds::auth_method_disable::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();
    let data2 = authenticated_account_cmds::auth_method_disable::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_self_disable_not_allowed() {
    // Generated from Parsec 3.4.1-a.0+dev
    // Content:
    //   status: 'self_disable_not_allowed'
    let raw: &[u8] =
        hex!("81a6737461747573b873656c665f64697361626c655f6e6f745f616c6c6f776564").as_ref();

    let expected = authenticated_account_cmds::auth_method_disable::Rep::SelfDisableNotAllowed {};
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = authenticated_account_cmds::auth_method_disable::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();
    let data2 = authenticated_account_cmds::auth_method_disable::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
