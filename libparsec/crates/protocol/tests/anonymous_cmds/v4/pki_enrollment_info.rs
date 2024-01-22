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
    //   cmd: "pki_enrollment_info"
    //   enrollment_id: ext(2, hex!("829d8cff327b4edbb39246a3c6767b07"))
    //
    let raw = hex!(
        "82a3636d64b3706b695f656e726f6c6c6d656e745f696e666fad656e726f6c6c6d656e745f"
        "6964d802829d8cff327b4edbb39246a3c6767b07"
    );

    let expected =
        anonymous_cmds::AnyCmdReq::PkiEnrollmentInfo(anonymous_cmds::pki_enrollment_info::Req {
            enrollment_id: EnrollmentID::from_hex("829d8cff327b4edbb39246a3c6767b07").unwrap(),
        });

    let data = anonymous_cmds::AnyCmdReq::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // roundtrip check ...
    let anonymous_cmds::AnyCmdReq::PkiEnrollmentInfo(req2) = data else {
        unreachable!()
    };

    let raw2 = req2.dump().unwrap();

    let data2 = anonymous_cmds::AnyCmdReq::load(&raw2).unwrap();
    p_assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    let raw_expected = [
        (
            // Generated from Python implementation (Parsec v2.14.0+dev)
            // Content:
            //   accept_payload: hex!("64756d6d79")
            //   accept_payload_signature: hex!("64756d6d79")
            //   accepted_on: ext(1, 1668768160.714565)
            //   accepter_der_x509_certificate: hex!("64756d6d79")
            //   enrollment_status: "ACCEPTED"
            //   status: "ok"
            //   submitted_on: ext(1, 1668768160.714573)
            &hex!(
                "87ae6163636570745f7061796c6f6164c40564756d6d79b86163636570745f7061796c6f61"
                "645f7369676e6174757265c40564756d6d79ab61636365707465645f6f6ed70141d8ddd868"
                "2dbb6fbd61636365707465725f6465725f783530395f6365727469666963617465c4056475"
                "6d6d79b1656e726f6c6c6d656e745f737461747573a84143434550544544a6737461747573"
                "a26f6bac7375626d69747465645f6f6ed70141d8ddd8682dbb90"
            )[..],
            anonymous_cmds::pki_enrollment_info::Rep::Ok(
                anonymous_cmds::pki_enrollment_info::PkiEnrollmentInfoStatus::Accepted {
                    accept_payload: hex!("64756d6d79").as_ref().into(),
                    accept_payload_signature: hex!("64756d6d79").as_ref().into(),
                    accepted_on: DateTime::from_f64_with_us_precision(1668768160.714565),
                    accepter_der_x509_certificate: hex!("64756d6d79").as_ref().into(),
                    submitted_on: DateTime::from_f64_with_us_precision(1668768160.714573),
                },
            ),
        ),
        (
            // Generated from Python implementation (Parsec v2.14.0+dev)
            // Content:
            //   cancelled_on: ext(1, 1668768160.716388)
            //   enrollment_status: "CANCELLED"
            //   status: "ok"
            //   submitted_on: ext(1, 1668768160.716395)
            &hex!(
                "84ac63616e63656c6c65645f6f6ed70141d8ddd8682dd94db1656e726f6c6c6d656e745f73"
                "7461747573a943414e43454c4c4544a6737461747573a26f6bac7375626d69747465645f6f"
                "6ed70141d8ddd8682dd96a"
            )[..],
            anonymous_cmds::pki_enrollment_info::Rep::Ok(
                anonymous_cmds::pki_enrollment_info::PkiEnrollmentInfoStatus::Cancelled {
                    cancelled_on: DateTime::from_f64_with_us_precision(1668768160.716388),
                    submitted_on: DateTime::from_f64_with_us_precision(1668768160.716395),
                },
            ),
        ),
        (
            // Generated from Python implementation (Parsec v2.14.0+dev)
            // Content:
            //   enrollment_status: "REJECTED"
            //   rejected_on: ext(1, 1668768160.716569)
            //   status: "ok"
            //   submitted_on: ext(1, 1668768160.716576)
            &hex!(
                "84b1656e726f6c6c6d656e745f737461747573a852454a4543544544ab72656a6563746564"
                "5f6f6ed70141d8ddd8682ddc44a6737461747573a26f6bac7375626d69747465645f6f6ed7"
                "0141d8ddd8682ddc62"
            )[..],
            anonymous_cmds::pki_enrollment_info::Rep::Ok(
                anonymous_cmds::pki_enrollment_info::PkiEnrollmentInfoStatus::Rejected {
                    rejected_on: DateTime::from_f64_with_us_precision(1668768160.716569),
                    submitted_on: DateTime::from_f64_with_us_precision(1668768160.716576),
                },
            ),
        ),
        (
            // Generated from Python implementation (Parsec v2.14.0+dev)
            // Content:
            //   enrollment_status: "SUBMITTED"
            //   status: "ok"
            //   submitted_on: ext(1, 1668768160.716755)
            &hex!(
                "83b1656e726f6c6c6d656e745f737461747573a95355424d4954544544a6737461747573a2"
                "6f6bac7375626d69747465645f6f6ed70141d8ddd8682ddf50"
            )[..],
            anonymous_cmds::pki_enrollment_info::Rep::Ok(
                anonymous_cmds::pki_enrollment_info::PkiEnrollmentInfoStatus::Submitted {
                    submitted_on: DateTime::from_f64_with_us_precision(1668768160.716755),
                },
            ),
        ),
    ];

    for (raw, expected) in raw_expected {
        let data = anonymous_cmds::pki_enrollment_info::Rep::load(raw).unwrap();

        p_assert_eq!(data, expected);

        // Also test serialization round trip
        let raw2 = data.dump().unwrap();

        let data2 = anonymous_cmds::pki_enrollment_info::Rep::load(&raw2).unwrap();

        p_assert_eq!(data2, expected);
    }
}

pub fn rep_enrollment_not_found() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "enrollment_not_found"
    let raw = hex!("81a6737461747573b4656e726f6c6c6d656e745f6e6f745f666f756e64");

    let expected = anonymous_cmds::pki_enrollment_info::Rep::EnrollmentNotFound;

    let data = anonymous_cmds::pki_enrollment_info::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_cmds::pki_enrollment_info::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
