// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use super::anonymous_server_cmds;
use bytes::Bytes;
use libparsec_tests_lite::prelude::*;

// Request

pub fn req() {
    // Generated from Parsec 3.8.2-a.0+dev
    // Content:
    //   cmd: 'scws_service_mutual_challenges'
    //   scws_service_challenge_payload: 0x736377735f736572766963655f6368616c6c656e67655f7061796c6f6164
    //   scws_service_challenge_signature: 0x736377735f736572766963655f6368616c6c656e67655f7369676e6174757265
    //   scws_service_challenge_key_id: 42
    //   web_application_challenge_payload: 0x7765625f6170706c69636174696f6e5f6368616c6c656e67655f7061796c6f6164
    let raw: &[u8] = hex!(
        "85a3636d64be736377735f736572766963655f6d757475616c5f6368616c6c656e6765"
        "73be736377735f736572766963655f6368616c6c656e67655f7061796c6f6164c41e73"
        "6377735f736572766963655f6368616c6c656e67655f7061796c6f6164d92073637773"
        "5f736572766963655f6368616c6c656e67655f7369676e6174757265c420736377735f"
        "736572766963655f6368616c6c656e67655f7369676e6174757265bd736377735f7365"
        "72766963655f6368616c6c656e67655f6b65795f69642ad9217765625f6170706c6963"
        "6174696f6e5f6368616c6c656e67655f7061796c6f6164c4217765625f6170706c6963"
        "6174696f6e5f6368616c6c656e67655f7061796c6f6164"
    )
    .as_ref();
    let req = anonymous_server_cmds::scws_service_mutual_challenges::Req {
        scws_service_challenge_payload: Bytes::from_static(b"scws_service_challenge_payload"),
        scws_service_challenge_signature: Bytes::from_static(b"scws_service_challenge_signature"),
        scws_service_challenge_key_id: 42,
        web_application_challenge_payload: Bytes::from_static(b"web_application_challenge_payload"),
    };

    println!("***expected: {:?}", req.dump().unwrap());
    let expected = anonymous_server_cmds::AnyCmdReq::ScwsServiceMutualChallenges(req);
    let data = anonymous_server_cmds::AnyCmdReq::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let anonymous_server_cmds::AnyCmdReq::ScwsServiceMutualChallenges(req2) = data else {
        unreachable!()
    };
    let raw2 = req2.dump().unwrap();

    let data2 = anonymous_server_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    // Generated from Parsec 3.8.2-a.0+dev
    // Content:
    //   status: 'ok'
    //   web_application_challenge_signature: 0x7765625f6170706c69636174696f6e5f6368616c6c656e67655f7369676e6174757265
    let raw: &[u8] = hex!(
        "82a6737461747573a26f6bd9237765625f6170706c69636174696f6e5f6368616c6c65"
        "6e67655f7369676e6174757265c4237765625f6170706c69636174696f6e5f6368616c"
        "6c656e67655f7369676e6174757265"
    )
    .as_ref();
    let rep = anonymous_server_cmds::scws_service_mutual_challenges::Rep::Ok {
        web_application_challenge_signature: Bytes::from_static(
            b"web_application_challenge_signature",
        ),
    };

    println!("***expected: {:?}", rep.dump().unwrap());

    let data = anonymous_server_cmds::scws_service_mutual_challenges::Rep::load(raw).unwrap();

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_server_cmds::scws_service_mutual_challenges::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, rep);
}

pub fn rep_not_available() {
    // Generated from Parsec 3.8.2-a.0+dev
    // Content:
    //   status: 'not_available'
    let raw: &[u8] = hex!("81a6737461747573ad6e6f745f617661696c61626c65").as_ref();
    let rep = anonymous_server_cmds::scws_service_mutual_challenges::Rep::NotAvailable;

    println!("***expected: {:?}", rep.dump().unwrap());

    let data = anonymous_server_cmds::scws_service_mutual_challenges::Rep::load(raw).unwrap();

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_server_cmds::scws_service_mutual_challenges::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, rep);
}

pub fn rep_unknown_scws_service_challenge_key_id() {
    // Generated from Parsec 3.8.2-a.0+dev
    // Content:
    //   status: 'unknown_scws_service_challenge_key_id'
    let raw: &[u8] = hex!(
        "81a6737461747573d925756e6b6e6f776e5f736377735f736572766963655f6368616c"
        "6c656e67655f6b65795f6964"
    )
    .as_ref();
    let rep = anonymous_server_cmds::scws_service_mutual_challenges::Rep::UnknownScwsServiceChallengeKeyId;

    println!("***expected: {:?}", rep.dump().unwrap());

    let data = anonymous_server_cmds::scws_service_mutual_challenges::Rep::load(raw).unwrap();

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_server_cmds::scws_service_mutual_challenges::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, rep);
}

pub fn rep_invalid_scws_service_challenge_signature() {
    // Generated from Parsec 3.8.2-a.0+dev
    // Content:
    //   status: 'invalid_scws_service_challenge_signature'
    let raw: &[u8] = hex!(
        "81a6737461747573d928696e76616c69645f736377735f736572766963655f6368616c"
        "6c656e67655f7369676e6174757265"
    )
    .as_ref();
    let rep = anonymous_server_cmds::scws_service_mutual_challenges::Rep::InvalidScwsServiceChallengeSignature;

    println!("***expected: {:?}", rep.dump().unwrap());

    let data = anonymous_server_cmds::scws_service_mutual_challenges::Rep::load(raw).unwrap();

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_server_cmds::scws_service_mutual_challenges::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, rep);
}

pub fn rep_invalid_web_application_challenge_payload() {
    // Generated from Parsec 3.8.2-a.0+dev
    // Content:
    //   status: 'invalid_web_application_challenge_payload'
    let raw: &[u8] = hex!(
        "81a6737461747573d929696e76616c69645f7765625f6170706c69636174696f6e5f63"
        "68616c6c656e67655f7061796c6f6164"
    )
    .as_ref();
    let rep = anonymous_server_cmds::scws_service_mutual_challenges::Rep::InvalidWebApplicationChallengePayload;

    println!("***expected: {:?}", rep.dump().unwrap());

    let data = anonymous_server_cmds::scws_service_mutual_challenges::Rep::load(raw).unwrap();

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_server_cmds::scws_service_mutual_challenges::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, rep);
}
