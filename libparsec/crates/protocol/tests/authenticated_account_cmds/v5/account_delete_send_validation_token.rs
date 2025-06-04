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
    //   cmd: 'account_delete_send_validation_token'
    let raw: &[u8] = hex!(
        "81a3636d64d9246163636f756e745f64656c6574655f73656e645f76616c6964617469"
        "6f6e5f746f6b656e"
    )
    .as_ref();

    let req = authenticated_account_cmds::account_delete_send_validation_token::Req {};
    let expected =
        authenticated_account_cmds::AnyCmdReq::AccountDeleteSendValidationToken(req.clone());
    println!("***expected: {:?}", req.dump().unwrap());
    let data = authenticated_account_cmds::AnyCmdReq::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = req.dump().unwrap();
    let data2 = authenticated_account_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected)
}

// Responses

pub fn rep_ok() {
    // Generated from Parsec 3.4.0-a.7+dev
    // Content:
    //   status: 'ok'
    let raw: &[u8] = hex!("81a6737461747573a26f6b").as_ref();

    let expected = authenticated_account_cmds::account_delete_send_validation_token::Rep::Ok;
    println!("***expected: {:?}", expected.dump().unwrap());
    let data =
        authenticated_account_cmds::account_delete_send_validation_token::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();
    let data2 =
        authenticated_account_cmds::account_delete_send_validation_token::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_email_server_unavailable() {
    // Generated from Parsec 3.4.0-a.7+dev
    // Content:
    //   status: 'email_server_unavailable'
    let raw: &[u8] =
        hex!("81a6737461747573b8656d61696c5f7365727665725f756e617661696c61626c65").as_ref();

    let expected = authenticated_account_cmds::account_delete_send_validation_token::Rep::EmailServerUnavailable;
    println!("***expected: {:?}", expected.dump().unwrap());
    let data =
        authenticated_account_cmds::account_delete_send_validation_token::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();
    let data2 =
        authenticated_account_cmds::account_delete_send_validation_token::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_email_recipient_refused() {
    // Generated from Parsec 3.4.0-a.7+dev
    // Content:
    //   status: 'email_recipient_refused'
    let raw: &[u8] =
        hex!("81a6737461747573b7656d61696c5f726563697069656e745f72656675736564").as_ref();

    let expected = authenticated_account_cmds::account_delete_send_validation_token::Rep::EmailRecipientRefused;
    println!("***expected: {:?}", expected.dump().unwrap());
    let data =
        authenticated_account_cmds::account_delete_send_validation_token::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();
    let data2 =
        authenticated_account_cmds::account_delete_send_validation_token::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
