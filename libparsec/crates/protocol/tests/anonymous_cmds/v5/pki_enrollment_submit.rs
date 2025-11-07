// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

use super::anonymous_cmds;

// Request

pub fn req() {
    // Generated from Parsec 3.5.3-a.0+dev
    // Content:
    //   cmd: 'pki_enrollment_submit'
    //   enrollment_id: ext(2, 0xe1fe88bd0f054261887a6c8039710b40)
    //   force: True
    //   der_x509_certificate: 0x3c78353039206365727469663e
    //   payload_signature: 0x3c7369676e61747572653e
    //   payload_signature_algorithm: 'RSASSA-PSS-SHA256'
    //   payload: 0x3c64756d6d793e
    let raw: &[u8] = hex!(
        "87a3636d64b5706b695f656e726f6c6c6d656e745f7375626d6974ad656e726f6c6c6d"
        "656e745f6964d802e1fe88bd0f054261887a6c8039710b40a5666f726365c3b4646572"
        "5f783530395f6365727469666963617465c40d3c78353039206365727469663eb17061"
        "796c6f61645f7369676e6174757265c40b3c7369676e61747572653ebb7061796c6f61"
        "645f7369676e61747572655f616c676f726974686db15253415353412d5053532d5348"
        "41323536a77061796c6f6164c4073c64756d6d793e"
    )
    .as_ref();
    let req = anonymous_cmds::pki_enrollment_submit::Req {
        der_x509_certificate: hex!("3c78353039206365727469663e").as_ref().into(),
        enrollment_id: EnrollmentID::from_hex("e1fe88bd0f054261887a6c8039710b40").unwrap(),
        force: true,
        payload: hex!("3c64756d6d793e").as_ref().into(),
        payload_signature: hex!("3c7369676e61747572653e").as_ref().into(),
        payload_signature_algorithm: PkiSignatureAlgorithm::RsassaPssSha256,
    };
    println!("***expected: {:?}", req.dump().unwrap());

    let expected = anonymous_cmds::AnyCmdReq::PkiEnrollmentSubmit(req);

    let data = anonymous_cmds::AnyCmdReq::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let anonymous_cmds::AnyCmdReq::PkiEnrollmentSubmit(req2) = data else {
        unreachable!()
    };

    let raw2 = req2.dump().unwrap();

    let data2 = anonymous_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    // Generated from Parsec 3.5.1-a.0+dev
    // Content:
    //   status: 'ok'
    //   submitted_on: ext(1, 1668594983390001) i.e. 2022-11-16T11:36:23.390001Z
    let raw: &[u8] =
        hex!("82a6737461747573a26f6bac7375626d69747465645f6f6ed7010005ed940b424b31").as_ref();
    let expected = anonymous_cmds::pki_enrollment_submit::Rep::Ok {
        submitted_on: DateTime::from_timestamp_micros(1668594983390001).unwrap(),
    };
    println!("***expected: {:?}", expected.dump().unwrap());

    let data = anonymous_cmds::pki_enrollment_submit::Rep::load(raw).unwrap();
    p_assert_eq!(data, expected);
    let raw2 = data.dump().unwrap();
    let data2 = anonymous_cmds::pki_enrollment_submit::Rep::load(&raw2).unwrap();
    p_assert_eq!(data2, expected);
}

pub fn rep_already_submitted() {
    // Generated from Parsec 3.5.1-a.0+dev
    // Content:
    //   status: 'already_submitted'
    //   submitted_on: ext(1, 1668594983390001) i.e. 2022-11-16T11:36:23.390001Z
    let raw: &[u8] = hex!(
        "82a6737461747573b1616c72656164795f7375626d6974746564ac7375626d69747465"
        "645f6f6ed7010005ed940b424b31"
    )
    .as_ref();
    let expected = anonymous_cmds::pki_enrollment_submit::Rep::AlreadySubmitted {
        submitted_on: DateTime::from_timestamp_micros(1668594983390001).unwrap(),
    };
    println!("***expected: {:?}", expected.dump().unwrap());

    let data = anonymous_cmds::pki_enrollment_submit::Rep::load(raw).unwrap();
    p_assert_eq!(data, expected);
    let raw2 = data.dump().unwrap();
    let data2 = anonymous_cmds::pki_enrollment_submit::Rep::load(&raw2).unwrap();
    p_assert_eq!(data2, expected);
}

pub fn rep_id_already_used() {
    // Generated from Parsec 3.5.1-a.0+dev
    // Content:
    //   status: 'id_already_used'
    let raw: &[u8] = hex!("81a6737461747573af69645f616c72656164795f75736564").as_ref();
    let expected = anonymous_cmds::pki_enrollment_submit::Rep::IdAlreadyUsed {};
    println!("***expected: {:?}", expected.dump().unwrap());

    let data = anonymous_cmds::pki_enrollment_submit::Rep::load(raw).unwrap();
    p_assert_eq!(data, expected);
    let raw2 = data.dump().unwrap();
    let data2 = anonymous_cmds::pki_enrollment_submit::Rep::load(&raw2).unwrap();
    p_assert_eq!(data2, expected);
}

pub fn rep_email_already_used() {
    // Generated from Parsec 3.5.1-a.0+dev
    // Content:
    //   status: 'email_already_used'
    let raw: &[u8] = hex!("81a6737461747573b2656d61696c5f616c72656164795f75736564").as_ref();
    let expected = anonymous_cmds::pki_enrollment_submit::Rep::EmailAlreadyUsed {};
    println!("***expected: {:?}", expected.dump().unwrap());

    let data = anonymous_cmds::pki_enrollment_submit::Rep::load(raw).unwrap();
    p_assert_eq!(data, expected);
    let raw2 = data.dump().unwrap();
    let data2 = anonymous_cmds::pki_enrollment_submit::Rep::load(&raw2).unwrap();
    p_assert_eq!(data2, expected);
}
pub fn rep_already_enrolled() {
    // Generated from Parsec 3.5.1-a.0+dev
    // Content:
    //   status: 'already_enrolled'
    let raw: &[u8] = hex!("81a6737461747573b0616c72656164795f656e726f6c6c6564").as_ref();
    let expected = anonymous_cmds::pki_enrollment_submit::Rep::AlreadyEnrolled {};
    println!("***expected: {:?}", expected.dump().unwrap());

    let data = anonymous_cmds::pki_enrollment_submit::Rep::load(raw).unwrap();
    p_assert_eq!(data, expected);
    let raw2 = data.dump().unwrap();
    let data2 = anonymous_cmds::pki_enrollment_submit::Rep::load(&raw2).unwrap();
    p_assert_eq!(data2, expected);
}

pub fn rep_invalid_payload() {
    // Generated from Parsec 3.5.1-a.0+dev
    // Content:
    //   status: 'invalid_payload'
    let raw: &[u8] = hex!("81a6737461747573af696e76616c69645f7061796c6f6164").as_ref();
    let expected = anonymous_cmds::pki_enrollment_submit::Rep::InvalidPayload {};
    println!("***expected: {:?}", expected.dump().unwrap());

    let data = anonymous_cmds::pki_enrollment_submit::Rep::load(raw).unwrap();
    p_assert_eq!(data, expected);
    let raw2 = data.dump().unwrap();
    let data2 = anonymous_cmds::pki_enrollment_submit::Rep::load(&raw2).unwrap();
    p_assert_eq!(data2, expected);
}
