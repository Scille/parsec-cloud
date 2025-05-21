// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;

use super::anonymous_account_cmds;

// Request

pub fn req() {
    // Generated from Parsec 3.4.0-a.7+dev
    // Content:
    //   cmd: 'account_create_send_validation_email'
    //   email: 'alice@invalid.com'
    let raw: &[u8] = hex!(
    "82a3636d64d9246163636f756e745f6372656174655f73656e645f76616c6964617469"
    "6f6e5f656d61696ca5656d61696cb1616c69636540696e76616c69642e636f6d"
    )
    .as_ref();

    let req = anonymous_account_cmds::account_create_send_validation_email::Req {
        email: "alice@invalid.com".parse().unwrap(),
    };

    let expected = anonymous_account_cmds::AnyCmdReq::AccountCreateSendValidationEmail(req.clone());
    println!("***expected: {:?}", req.dump().unwrap());

    let data = anonymous_account_cmds::AnyCmdReq::load(raw).unwrap();
    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let anonymous_account_cmds::AnyCmdReq::AccountCreateSendValidationEmail(req2) = data else {
        unreachable!()
    };

    let raw2 = req2.dump().unwrap();

    let data2 = anonymous_account_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    // Generated from Parsec 3.4.0-a.7+dev
    // Content:
    //   status: 'ok'
    let raw: &[u8] = hex!("81a6737461747573a26f6b").as_ref();
    let expected = anonymous_account_cmds::account_create_send_validation_email::Rep::Ok {};

    let data =
        anonymous_account_cmds::account_create_send_validation_email::Rep::load(raw).unwrap();
    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 =
        anonymous_account_cmds::account_create_send_validation_email::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_invalid_email() {
    // Generated from Parsec 3.4.0-a.7+dev
    // Content:
    //   status: 'invalid_email'
    let raw: &[u8] = hex!("81a6737461747573ad696e76616c69645f656d61696c").as_ref();

    let expected =
        anonymous_account_cmds::account_create_send_validation_email::Rep::InvalidEmail {};
    let data =
        anonymous_account_cmds::account_create_send_validation_email::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 =
        anonymous_account_cmds::account_create_send_validation_email::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_email_server_unavailable() {
    // Generated from Parsec 3.4.0-a.7+dev
    // Content:
    //   status: 'email_server_unavailable'
    let raw: &[u8] =
        hex!("81a6737461747573b8656d61696c5f7365727665725f756e617661696c61626c65").as_ref();
    let expected =
        anonymous_account_cmds::account_create_send_validation_email::Rep::EmailServerUnavailable {};
    let data =
        anonymous_account_cmds::account_create_send_validation_email::Rep::load(raw).unwrap();
    println!("***expected: {:?}", expected.dump().unwrap());
    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 =
        anonymous_account_cmds::account_create_send_validation_email::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_email_recipient_refused() {
    // Generated from Parsec 3.4.0-a.7+dev
    // Content:
    //   status: 'email_recipient_refused'
    let raw: &[u8] =
        hex!("81a6737461747573b7656d61696c5f726563697069656e745f72656675736564").as_ref();

    let expected =
        anonymous_account_cmds::account_create_send_validation_email::Rep::EmailRecipientRefused {};
    let data =
        anonymous_account_cmds::account_create_send_validation_email::Rep::load(raw).unwrap();
    println!("***expected: {:?}", expected.dump().unwrap());
    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 =
        anonymous_account_cmds::account_create_send_validation_email::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
