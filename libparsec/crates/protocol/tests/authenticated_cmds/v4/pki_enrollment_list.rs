// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

use super::authenticated_cmds;

// Request

pub fn req() {
    // Generated from Python implementation (Parsec v2.14.0)
    // Content:
    //   cmd: "pki_enrollment_list"
    //
    let raw = hex!("81a3636d64b3706b695f656e726f6c6c6d656e745f6c697374");
    let expected = authenticated_cmds::AnyCmdReq::PkiEnrollmentList(
        authenticated_cmds::pki_enrollment_list::Req {},
    );

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let authenticated_cmds::AnyCmdReq::PkiEnrollmentList(req2) = data else {
        unreachable!()
    };

    let raw2 = req2.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    let raw_expected = [
        (
            // Generated from Parsec v3.0.0-b.11+dev
            // Content:
            //   enrollments: [
            //     {
            //       enrollment_id: ext(2, hex!("e1fe88bd0f054261887a6c8039710b40"))
            //       submit_payload: hex!("3c64756d6d793e")
            //       submit_payload_signature: hex!("3c7369676e61747572653e")
            //       submitted_on: ext(1, 1668594983.390001)
            //       submitter_der_x509_certificate: hex!("3c78353039206365727469663e")
            //     }
            //   ]
            //   status: "ok"
            &hex!(
                "82a6737461747573a26f6bab656e726f6c6c6d656e74739185ad656e726f6c6c6d656e"
                "745f6964d802e1fe88bd0f054261887a6c8039710b40ae7375626d69745f7061796c6f"
                "6164c4073c64756d6d793eb87375626d69745f7061796c6f61645f7369676e61747572"
                "65c40b3c7369676e61747572653eac7375626d69747465645f6f6ed7010005ed940b42"
                "4b31be7375626d69747465725f6465725f783530395f6365727469666963617465c40d"
                "3c78353039206365727469663e"
            )[..],
            authenticated_cmds::pki_enrollment_list::Rep::Ok {
                enrollments: vec![
                    authenticated_cmds::pki_enrollment_list::PkiEnrollmentListItem {
                        enrollment_id: EnrollmentID::from_hex("e1fe88bd0f054261887a6c8039710b40")
                            .unwrap(),
                        submit_payload: hex!("3c64756d6d793e").as_ref().into(),
                        submit_payload_signature: hex!("3c7369676e61747572653e").as_ref().into(),
                        submitted_on: DateTime::from_timestamp_micros(1668594983390001).unwrap(),
                        submitter_der_x509_certificate: hex!("3c78353039206365727469663e")
                            .as_ref()
                            .into(),
                    },
                ],
            },
        ),
        (
            // Generated from Python implementation (Parsec v2.14.0)
            // Content:
            //   enrollments: []
            //   status: "ok"
            &hex!("82ab656e726f6c6c6d656e747390a6737461747573a26f6b")[..],
            authenticated_cmds::pki_enrollment_list::Rep::Ok {
                enrollments: Vec::new(),
            },
        ),
    ];

    for (raw, expected) in raw_expected {
        let data = authenticated_cmds::pki_enrollment_list::Rep::load(raw).unwrap();

        p_assert_eq!(data, expected);

        let raw2 = data.dump().unwrap();

        let data2 = authenticated_cmds::pki_enrollment_list::Rep::load(&raw2).unwrap();

        p_assert_eq!(data2, expected);
    }
}

pub fn rep_author_not_allowed() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "author_not_allowed"
    let raw = hex!("81a6737461747573b2617574686f725f6e6f745f616c6c6f776564");
    let expected = authenticated_cmds::pki_enrollment_list::Rep::AuthorNotAllowed;

    let data = authenticated_cmds::pki_enrollment_list::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::pki_enrollment_list::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
