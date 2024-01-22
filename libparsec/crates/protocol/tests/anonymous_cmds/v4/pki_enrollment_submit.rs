// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

use super::anonymous_cmds;

// Request

pub fn req() {
    // Generated from Python implementation (Parsec v2.14.0+dev)
    // Content:
    //   cmd: "pki_enrollment_submit"
    //   enrollment_id: ext(2, hex!("34556b1fcabe496dafb64a69ca932666"))
    //   force: false
    //   submit_payload: hex!("64756d6d79")
    //   submit_payload_signature: hex!("64756d6d79")
    //   submitter_der_x509_certificate: hex!("64756d6d79")
    //   submitter_der_x509_certificate_email: "mail@mail.com"
    //
    let raw = hex!(
        "87a3636d64b5706b695f656e726f6c6c6d656e745f7375626d6974ad656e726f6c6c6d656e"
        "745f6964d80234556b1fcabe496dafb64a69ca932666a5666f726365c2ae7375626d69745f"
        "7061796c6f6164c40564756d6d79b87375626d69745f7061796c6f61645f7369676e617475"
        "7265c40564756d6d79be7375626d69747465725f6465725f783530395f6365727469666963"
        "617465c40564756d6d79d9247375626d69747465725f6465725f783530395f636572746966"
        "69636174655f656d61696cad6d61696c406d61696c2e636f6d"
    );

    let expected = anonymous_cmds::AnyCmdReq::PkiEnrollmentSubmit(
        anonymous_cmds::pki_enrollment_submit::Req {
            enrollment_id: EnrollmentID::from_hex("34556b1fcabe496dafb64a69ca932666").unwrap(),
            force: false,
            submit_payload: hex!("64756d6d79").as_ref().into(),
            submit_payload_signature: hex!("64756d6d79").as_ref().into(),
            submitter_der_x509_certificate: hex!("64756d6d79").as_ref().into(),
            submitter_der_x509_certificate_email: "mail@mail.com".to_string(),
        },
    );

    let data = anonymous_cmds::AnyCmdReq::load(&raw).unwrap();

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
    // Generated from Python implementation (Parsec v2.14.0+dev)
    // Content:
    //   status: "ok"
    //   submitted_on: ext(1, 1668767275.338466)
    //
    let raw = hex!("82a6737461747573a26f6bac7375626d69747465645f6f6ed70141d8ddd78ad5a96d");

    let expected = anonymous_cmds::pki_enrollment_submit::Rep::Ok {
        submitted_on: DateTime::from_f64_with_us_precision(1668767275.338466),
    };

    let data = anonymous_cmds::pki_enrollment_submit::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_cmds::pki_enrollment_submit::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_x509_certificate_already_submitted() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "x509_certificate_already_submitted"
    //   submitted_on: ext(1, 1668767275.338466)
    //
    let raw = hex!(
        "82a6737461747573d922783530395f63657274696669636174655f616c72656164795f7375"
        "626d6974746564ac7375626d69747465645f6f6ed70141d8ddd78ad5a96d"
    );

    let expected = anonymous_cmds::pki_enrollment_submit::Rep::X509CertificateAlreadySubmitted {
        submitted_on: DateTime::from_f64_with_us_precision(1668767275.338466),
    };

    let data = anonymous_cmds::pki_enrollment_submit::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_cmds::pki_enrollment_submit::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_enrollment_id_already_used() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "enrollment_id_already_used"
    //
    let raw = hex!("81a6737461747573ba656e726f6c6c6d656e745f69645f616c72656164795f75736564");

    let expected = anonymous_cmds::pki_enrollment_submit::Rep::EnrollmentIdAlreadyUsed;

    let data = anonymous_cmds::pki_enrollment_submit::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_cmds::pki_enrollment_submit::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_email_already_used() {
    // Generated from Rust implementation (Parsec v2.15.0+dev)
    // Content:
    //   status: "email_already_used"
    //
    let raw = hex!("81a6737461747573b2656d61696c5f616c72656164795f75736564");

    let expected = anonymous_cmds::pki_enrollment_submit::Rep::EmailAlreadyUsed;

    let data = anonymous_cmds::pki_enrollment_submit::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_cmds::pki_enrollment_submit::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_already_enrolled() {
    // Generated from Rust implementation (Parsec v2.15.0+dev)
    // Content:
    //   status: "already_enrolled"
    //
    let raw = hex!("81a6737461747573b0616c72656164795f656e726f6c6c6564");

    let expected = anonymous_cmds::pki_enrollment_submit::Rep::AlreadyEnrolled;

    let data = anonymous_cmds::pki_enrollment_submit::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_cmds::pki_enrollment_submit::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_invalid_payload_data() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "invalid_payload_data"
    //
    let raw = hex!("81a6737461747573b4696e76616c69645f7061796c6f61645f64617461");

    let expected = anonymous_cmds::pki_enrollment_submit::Rep::InvalidPayloadData;

    let data = anonymous_cmds::pki_enrollment_submit::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_cmds::pki_enrollment_submit::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
