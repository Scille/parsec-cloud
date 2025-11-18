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
        "82a3636d64b3706b695f656e726f6c6c6d656e745f696e666fad656e726f6c6c6d656e"
        "745f6964d802829d8cff327b4edbb39246a3c6767b07"
    );

    let expected =
        anonymous_cmds::AnyCmdReq::PkiEnrollmentInfo(anonymous_cmds::pki_enrollment_info::Req {
            enrollment_id: PKIEnrollmentID::from_hex("829d8cff327b4edbb39246a3c6767b07").unwrap(),
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
            // Generated from Parsec 3.5.3-a.0+dev
            // Content:
            //   status: 'ok'
            //   enrollment_status: 'ACCEPTED'
            //   accept_payload: 0x64756d6d79
            //   accept_payload_signature: 0x64756d6d79
            //   accept_payload_signature_algorithm: 'RSASSA-PSS-SHA256'
            //   accepted_on: ext(1, 1668768160714565) i.e. 2022-11-18T11:42:40.714565Z
            //   accepter_der_x509_certificate: 0x64756d6d79
            //   accepter_intermediate_der_x509_certificates: [ 0x6465616462656566, ]
            //   submitted_on: ext(1, 1668768160714573) i.e. 2022-11-18T11:42:40.714573Z
            hex!(
                "89a6737461747573a26f6bb1656e726f6c6c6d656e745f737461747573a84143434550"
                "544544ae6163636570745f7061796c6f6164c40564756d6d79b86163636570745f7061"
                "796c6f61645f7369676e6174757265c40564756d6d79d9226163636570745f7061796c"
                "6f61645f7369676e61747572655f616c676f726974686db15253415353412d5053532d"
                "534841323536ab61636365707465645f6f6ed7010005edbc5d6e8f45bd616363657074"
                "65725f6465725f783530395f6365727469666963617465c40564756d6d79d92b616363"
                "65707465725f696e7465726d6564696174655f6465725f783530395f63657274696669"
                "636174657391c4086465616462656566ac7375626d69747465645f6f6ed7010005edbc"
                "5d6e8f4d"
            )
            .as_ref(),
            anonymous_cmds::pki_enrollment_info::Rep::Ok(
                anonymous_cmds::pki_enrollment_info::PkiEnrollmentInfoStatus::Accepted {
                    accept_payload: hex!("64756d6d79").as_ref().into(),
                    accept_payload_signature: hex!("64756d6d79").as_ref().into(),
                    accept_payload_signature_algorithm: PkiSignatureAlgorithm::RsassaPssSha256,
                    accepted_on: DateTime::from_timestamp_micros(1668768160714565).unwrap(),
                    accepter_der_x509_certificate: hex!("64756d6d79").as_ref().into(),
                    accepter_intermediate_der_x509_certificates: [b"deadbeef".as_ref().into()]
                        .to_vec(),
                    submitted_on: DateTime::from_timestamp_micros(1668768160714573).unwrap(),
                },
            ),
        ),
        (
            // Generated from Parsec v3.0.0-b.11+dev
            // Content:
            //   cancelled_on: ext(1, 1668768160.716388)
            //   enrollment_status: "CANCELLED"
            //   status: "ok"
            //   submitted_on: ext(1, 1668768160.716395)
            &hex!(
                "84a6737461747573a26f6bb1656e726f6c6c6d656e745f737461747573a943414e4345"
                "4c4c4544ac63616e63656c6c65645f6f6ed7010005edbc5d6e9664ac7375626d697474"
                "65645f6f6ed7010005edbc5d6e966b"
            )[..],
            anonymous_cmds::pki_enrollment_info::Rep::Ok(
                anonymous_cmds::pki_enrollment_info::PkiEnrollmentInfoStatus::Cancelled {
                    cancelled_on: DateTime::from_timestamp_micros(1668768160716388).unwrap(),
                    submitted_on: DateTime::from_timestamp_micros(1668768160716395).unwrap(),
                },
            ),
        ),
        (
            // Generated from Parsec v3.0.0-b.11+dev
            // Content:
            //   enrollment_status: "REJECTED"
            //   rejected_on: ext(1, 1668768160.716569)
            //   status: "ok"
            //   submitted_on: ext(1, 1668768160.716576)
            &hex!(
                "84a6737461747573a26f6bb1656e726f6c6c6d656e745f737461747573a852454a4543"
                "544544ab72656a65637465645f6f6ed7010005edbc5d6e9719ac7375626d6974746564"
                "5f6f6ed7010005edbc5d6e9720"
            )[..],
            anonymous_cmds::pki_enrollment_info::Rep::Ok(
                anonymous_cmds::pki_enrollment_info::PkiEnrollmentInfoStatus::Rejected {
                    rejected_on: DateTime::from_timestamp_micros(1668768160716569).unwrap(),
                    submitted_on: DateTime::from_timestamp_micros(1668768160716576).unwrap(),
                },
            ),
        ),
        (
            // Generated from Parsec v3.0.0-b.11+dev
            // Content:
            //   enrollment_status: "SUBMITTED"
            //   status: "ok"
            //   submitted_on: ext(1, 1668768160.716755)
            &hex!(
                "83a6737461747573a26f6bb1656e726f6c6c6d656e745f737461747573a95355424d49"
                "54544544ac7375626d69747465645f6f6ed7010005edbc5d6e97d3"
            )[..],
            anonymous_cmds::pki_enrollment_info::Rep::Ok(
                anonymous_cmds::pki_enrollment_info::PkiEnrollmentInfoStatus::Submitted {
                    submitted_on: DateTime::from_timestamp_micros(1668768160716755).unwrap(),
                },
            ),
        ),
    ];

    for (raw, expected) in raw_expected {
        println!("***expected: {:?}", expected.dump().unwrap());
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
