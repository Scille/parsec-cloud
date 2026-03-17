// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use super::anonymous_server_cmds;
use bytes::Bytes;
use libparsec_tests_lite::prelude::*;

// Request

pub fn req() {
    // Generated from Parsec 3.8.1-a.0+dev
    // Content:
    //   cmd: 'scws_service_challenge'
    //   middleware_challenge: 0x6d6964646c6577617265206368616c6c656e6765
    //   middleware_signature: 0x6d6964646c6577617265207369676e6174757265
    //   server_challenge: 0x736572766572206368616c6c656e6765
    //   pubkey_id: 42
    let raw: &[u8] = hex!(
        "85a3636d64b6736377735f736572766963655f6368616c6c656e6765b46d6964646c65"
        "776172655f6368616c6c656e6765c4146d6964646c6577617265206368616c6c656e67"
        "65b46d6964646c65776172655f7369676e6174757265c4146d6964646c657761726520"
        "7369676e6174757265b07365727665725f6368616c6c656e6765c41073657276657220"
        "6368616c6c656e6765a97075626b65795f69642a"
    )
    .as_ref();
    let req = anonymous_server_cmds::scws_service_challenge::Req {
        middleware_challenge: Bytes::from_static(b"middleware challenge"),
        middleware_signature: Bytes::from_static(b"middleware signature"),
        pubkey_id: 42,
        server_challenge: Bytes::from_static(b"server challenge"),
    };

    println!("***expected: {:?}", req.dump().unwrap());
    let expected = anonymous_server_cmds::AnyCmdReq::ScwsServiceChallenge(req);
    let data = anonymous_server_cmds::AnyCmdReq::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let anonymous_server_cmds::AnyCmdReq::ScwsServiceChallenge(req2) = data else {
        unreachable!()
    };
    let raw2 = req2.dump().unwrap();

    let data2 = anonymous_server_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    // Generated from Parsec 3.8.1-a.0+dev
    // Content:
    //   status: 'ok'
    //   server_signature: 0x736572766572207369676e6174757265
    let raw: &[u8] = hex!(
        "82a6737461747573a26f6bb07365727665725f7369676e6174757265c4107365727665"
        "72207369676e6174757265"
    )
    .as_ref();
    let rep = anonymous_server_cmds::scws_service_challenge::Rep::Ok {
        server_signature: Bytes::from_static(b"server signature"),
    };

    println!("***expected: {:?}", rep.dump().unwrap());

    let data = anonymous_server_cmds::scws_service_challenge::Rep::load(raw).unwrap();

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_server_cmds::scws_service_challenge::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, rep);
}

pub fn rep_not_available() {
    // Generated from Parsec 3.8.1-a.0+dev
    // Content:
    //   status: 'not_available'
    let raw: &[u8] = hex!("81a6737461747573ad6e6f745f617661696c61626c65").as_ref();
    let rep = anonymous_server_cmds::scws_service_challenge::Rep::NotAvailable;

    println!("***expected: {:?}", rep.dump().unwrap());

    let data = anonymous_server_cmds::scws_service_challenge::Rep::load(raw).unwrap();

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_server_cmds::scws_service_challenge::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, rep);
}

pub fn rep_unknown_service_public_key() {
    // Generated from Parsec 3.8.1-a.0+dev
    // Content:
    //   status: 'unknown_service_public_key'
    let raw: &[u8] =
        hex!("81a6737461747573ba756e6b6e6f776e5f736572766963655f7075626c69635f6b6579").as_ref();
    let rep = anonymous_server_cmds::scws_service_challenge::Rep::UnknownServicePublicKey;

    println!("***expected: {:?}", rep.dump().unwrap());

    let data = anonymous_server_cmds::scws_service_challenge::Rep::load(raw).unwrap();

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_server_cmds::scws_service_challenge::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, rep);
}

pub fn rep_invalid_service_signature() {
    // Generated from Parsec 3.8.1-a.0+dev
    // Content:
    //   status: 'invalid_service_signature'
    let raw: &[u8] =
        hex!("81a6737461747573b9696e76616c69645f736572766963655f7369676e6174757265").as_ref();
    let rep = anonymous_server_cmds::scws_service_challenge::Rep::InvalidServiceSignature;

    println!("***expected: {:?}", rep.dump().unwrap());

    let data = anonymous_server_cmds::scws_service_challenge::Rep::load(raw).unwrap();

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_server_cmds::scws_service_challenge::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, rep);
}
