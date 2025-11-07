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
            // Generated from Parsec 3.5.1-a.0+dev
            // Content:
            //   status: 'ok'
            //   enrollments: [
            //     {
            //       der_x509_certificate: 0x3c78353039206365727469663e,
            //       enrollment_id: ext(2, 0xe1fe88bd0f054261887a6c8039710b40),
            //       payload: 0x3c64756d6d793e,
            //       payload_signature: 0x3c7369676e61747572653e,
            //       submitted_on: ext(1, 1668594983390001) i.e. 2022-11-16T11:36:23.390001Z,
            //     },
            //   ]
            hex!(
                "82a6737461747573a26f6bab656e726f6c6c6d656e74739185b46465725f783530395f"
                "6365727469666963617465c40d3c78353039206365727469663ead656e726f6c6c6d65"
                "6e745f6964d802e1fe88bd0f054261887a6c8039710b40a77061796c6f6164c4073c64"
                "756d6d793eb17061796c6f61645f7369676e6174757265c40b3c7369676e6174757265"
                "3eac7375626d69747465645f6f6ed7010005ed940b424b31"
            )
            .as_ref(),
            authenticated_cmds::pki_enrollment_list::Rep::Ok {
                enrollments: vec![
                    authenticated_cmds::pki_enrollment_list::PkiEnrollmentListItem {
                        enrollment_id: PKIEnrollmentID::from_hex(
                            "e1fe88bd0f054261887a6c8039710b40",
                        )
                        .unwrap(),
                        payload: hex!("3c64756d6d793e").as_ref().into(),
                        payload_signature: hex!("3c7369676e61747572653e").as_ref().into(),
                        submitted_on: DateTime::from_timestamp_micros(1668594983390001).unwrap(),
                        der_x509_certificate: hex!("3c78353039206365727469663e").as_ref().into(),
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
    // Generated from Parsec 3.5.1-a.0+dev
    // Content:
    //   status: 'author_not_allowed'
    let raw: &[u8] = hex!("81a6737461747573b2617574686f725f6e6f745f616c6c6f776564").as_ref();
    let expected = authenticated_cmds::pki_enrollment_list::Rep::AuthorNotAllowed;
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = authenticated_cmds::pki_enrollment_list::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::pki_enrollment_list::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
