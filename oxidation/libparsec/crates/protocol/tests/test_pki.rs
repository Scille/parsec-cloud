// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use hex_literal::hex;

use libparsec_protocol::{
    anonymous_cmds::v2 as anonymous_cmds, authenticated_cmds::v2 as authenticated_cmds,
};
use libparsec_tests_fixtures::parsec_test;
use libparsec_types::{DateTime, EnrollmentID};

#[parsec_test]
#[case::submit(
    // Generated from Python implementation (Parsec v2.14.0+dev)
    // Content:
    //   cmd: "pki_enrollment_submit"
    //   enrollment_id: ext(2, hex!("34556b1fcabe496dafb64a69ca932666"))
    //   force: false
    //   submit_payload: hex!("64756d6d79")
    //   submit_payload_signature: hex!("64756d6d79")
    //   submitter_der_x509_certificate: hex!("64756d6d79")
    //   submitter_der_x509_certificate_email: "mail@mail.com"
    &hex!(
        "87a3636d64b5706b695f656e726f6c6c6d656e745f7375626d6974ad656e726f6c6c6d656e"
        "745f6964d80234556b1fcabe496dafb64a69ca932666a5666f726365c2ae7375626d69745f"
        "7061796c6f6164c40564756d6d79b87375626d69745f7061796c6f61645f7369676e617475"
        "7265c40564756d6d79be7375626d69747465725f6465725f783530395f6365727469666963"
        "617465c40564756d6d79d9247375626d69747465725f6465725f783530395f636572746966"
        "69636174655f656d61696cad6d61696c406d61696c2e636f6d"
    )[..],
        anonymous_cmds::AnyCmdReq::PkiEnrollmentSubmit(anonymous_cmds::pki_enrollment_submit::Req {
            enrollment_id: EnrollmentID::from_hex("34556b1fcabe496dafb64a69ca932666").unwrap(),
            force: false,
            submit_payload: hex!("64756d6d79").to_vec(),
            submit_payload_signature: hex!("64756d6d79").to_vec(),
            submitter_der_x509_certificate: hex!("64756d6d79").to_vec(),
            submitter_der_x509_certificate_email: Some("mail@mail.com".to_string()),
    })
)]
#[case::info(
    // Generated from Python implementation (Parsec v2.14.0+dev)
    // Content:
    //   cmd: "pki_enrollment_info"
    //   enrollment_id: ext(2, hex!("829d8cff327b4edbb39246a3c6767b07"))
    &hex!(
        "82a3636d64b3706b695f656e726f6c6c6d656e745f696e666fad656e726f6c6c6d656e745f"
        "6964d802829d8cff327b4edbb39246a3c6767b07"
    )[..],
    anonymous_cmds::AnyCmdReq::PkiEnrollmentInfo(anonymous_cmds::pki_enrollment_info::Req {
        enrollment_id: EnrollmentID::from_hex("829d8cff327b4edbb39246a3c6767b07").unwrap(),
    })
)]
fn serde_anonymous_pki_req(#[case] bytes: &[u8], #[case] expected: anonymous_cmds::AnyCmdReq) {
    let data = anonymous_cmds::AnyCmdReq::load(bytes).expect("Failed to deserialize bytes");
    assert_eq!(data, expected);

    // roundtrip check ...
    let raw_again = data.dump().expect("Failed to dump data again");
    let data_again =
        anonymous_cmds::AnyCmdReq::load(&raw_again).expect("Failed to deserialize raw again");
    assert_eq!(data_again, expected);
}

#[parsec_test]
#[case::ok(
    // Generated from Python implementation (Parsec v2.14.0+dev)
    // Content:
    //   status: "ok"
    //   submitted_on: ext(1, 1668767275.338466)
    &hex!("82a6737461747573a26f6bac7375626d69747465645f6f6ed70141d8ddd78ad5a96d")[..],
    anonymous_cmds::pki_enrollment_submit::Rep::Ok {
        submitted_on: DateTime::from_f64_with_us_precision(1668767275.338466),
    }
)]
fn serde_anonymous_pki_rep_submit(
    #[case] bytes: &[u8],
    #[case] expected: anonymous_cmds::pki_enrollment_submit::Rep,
) {
    let data =
        anonymous_cmds::pki_enrollment_submit::Rep::load(bytes).expect("failed to load bytes");
    assert_eq!(data, expected);

    let raw_again = data.dump().expect("failed to dump data");
    let data_again = anonymous_cmds::pki_enrollment_submit::Rep::load(&raw_again)
        .expect("failed to load raw again");
    assert_eq!(data_again, expected);
}

#[parsec_test]
#[case::accepted(
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
            accept_payload: hex!("64756d6d79").to_vec(),
            accept_payload_signature: hex!("64756d6d79").to_vec(),
            accepted_on: DateTime::from_f64_with_us_precision(1668768160.714565),
            accepter_der_x509_certificate: hex!("64756d6d79").to_vec(),
            submitted_on: DateTime::from_f64_with_us_precision(1668768160.714573)
        }
    )
)]
#[case::cancelled(
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
        }
    )
)]
#[case::rejected(
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
        }
    )
)]
#[case::submitted(
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
            submitted_on: DateTime::from_f64_with_us_precision(1668768160.716755)
        }
    )
)]
fn serde_anonymous_pki_rep_info(
    #[case] bytes: &[u8],
    #[case] expected: anonymous_cmds::pki_enrollment_info::Rep,
) {
    let data =
        anonymous_cmds::pki_enrollment_info::Rep::load(bytes).expect("failed to load info rep");
    assert_eq!(data, expected);

    // roundtrip check ...
    let raw_again = data.dump().expect("failed to dump data");
    let data_again = anonymous_cmds::pki_enrollment_info::Rep::load(&raw_again)
        .expect("failed to load raw again");
    assert_eq!(data_again, expected);
}

#[parsec_test]
#[case::accept(
    // Generated from Python implementation (Parsec v2.14.0)
    // Content:
    //   accept_payload: hex!("3c64756d6d793e")
    //   accept_payload_signature: hex!("3c7369676e61747572653e")
    //   accepter_der_x509_certificate: hex!("3c61636365707465725f6465725f783530395f63657274696669636174653e")
    //   cmd: "pki_enrollment_accept"
    //   device_certificate: hex!("3c64756d6d793e")
    //   enrollment_id: ext(2, hex!("e89621b91f8e4a7c8d3182ee513e380f"))
    //   redacted_device_certificate: hex!("3c64756d6d793e")
    //   redacted_user_certificate: hex!("3c64756d6d793e")
    //   user_certificate: hex!("3c64756d6d793e")
    &hex!(
        "89ae6163636570745f7061796c6f6164c4073c64756d6d793eb86163636570745f7061796c"
        "6f61645f7369676e6174757265c40b3c7369676e61747572653ebd61636365707465725f64"
        "65725f783530395f6365727469666963617465c41f3c61636365707465725f6465725f7835"
        "30395f63657274696669636174653ea3636d64b5706b695f656e726f6c6c6d656e745f6163"
        "63657074b26465766963655f6365727469666963617465c4073c64756d6d793ead656e726f"
        "6c6c6d656e745f6964d80256f48ed307984f10830e197287399c22bb72656461637465645f"
        "6465766963655f6365727469666963617465c4073c64756d6d793eb972656461637465645f"
        "757365725f6365727469666963617465c4073c64756d6d793eb0757365725f636572746966"
        "6963617465c4073c64756d6d793e"
    )[..],
    authenticated_cmds::AnyCmdReq::PkiEnrollmentAccept(authenticated_cmds::pki_enrollment_accept::Req {
        accept_payload: hex!("3c64756d6d793e").to_vec(),
        accept_payload_signature: hex!("3c7369676e61747572653e").to_vec(),
        accepter_der_x509_certificate: hex!("3c61636365707465725f6465725f783530395f63657274696669636174653e").to_vec(),
        device_certificate: hex!("3c64756d6d793e").to_vec(),
        enrollment_id: EnrollmentID::from_hex("56f48ed307984f10830e197287399c22").unwrap(),
        redacted_device_certificate: hex!("3c64756d6d793e").to_vec(),
        redacted_user_certificate: hex!("3c64756d6d793e").to_vec(),
        user_certificate: hex!("3c64756d6d793e").to_vec()
    })
)]
#[case::list(
    // Generated from Python implementation (Parsec v2.14.0)
    // Content:
    //   cmd: "pki_enrollment_list"
    &hex!("81a3636d64b3706b695f656e726f6c6c6d656e745f6c697374")[..],
    authenticated_cmds::AnyCmdReq::PkiEnrollmentList(authenticated_cmds::pki_enrollment_list::Req { })
)]
#[case::reject(
    // Generated from Python implementation (Parsec v2.14.0)
    // Content:
    //   cmd: "pki_enrollment_reject"
    //   enrollment_id: ext(2, hex!("88a75cc10b8d43d9b1f91ad8a12be8ee"))
    &hex!(
      "82a3636d64b5706b695f656e726f6c6c6d656e745f72656a656374ad656e726f6c6c6d656e"
      "745f6964d80288a75cc10b8d43d9b1f91ad8a12be8ee"
    )[..],
    authenticated_cmds::AnyCmdReq::PkiEnrollmentReject(authenticated_cmds::pki_enrollment_reject::Req {
        enrollment_id: EnrollmentID::from_hex("88a75cc10b8d43d9b1f91ad8a12be8ee").unwrap()
    })
)]
fn serde_authenticated_pki_req(
    #[case] bytes: &[u8],
    #[case] expected: authenticated_cmds::AnyCmdReq,
) {
    let data = authenticated_cmds::AnyCmdReq::load(bytes).expect("Failed to load raw req");

    assert_eq!(data, expected);

    // check roundtrip ...
    let raw_again = data.dump().expect("Failed to dump data req");
    let data_again =
        authenticated_cmds::AnyCmdReq::load(&raw_again).expect("Failed to load data_again req");
    assert_eq!(data_again, expected);
}

// Generated from Python implementation (Parsec v2.14.0)
// Content:
//   status: "ok"
#[parsec_test]
#[case::accept(
    &hex!("81a6737461747573a26f6b")[..],
    authenticated_cmds::pki_enrollment_accept::Rep::Ok
)]
fn serde_authenticated_accept_pki_rep(
    #[case] bytes: &[u8],
    #[case] expected_result: authenticated_cmds::pki_enrollment_accept::Rep,
) {
    let data = authenticated_cmds::pki_enrollment_accept::Rep::load(bytes)
        .expect("Failed to load accept pki rep");

    assert_eq!(data, expected_result);
    let raw = data.dump().expect("Failed to dump accept pki rep");
    assert_eq!(
        authenticated_cmds::pki_enrollment_accept::Rep::load(&raw).unwrap(),
        expected_result
    )
}

#[parsec_test]
#[case::ok_one_element(
    // Generated from Python implementation (Parsec v2.14.0)
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
        "82ab656e726f6c6c6d656e74739185be7375626d69747465725f6465725f783530395f6365"
        "727469666963617465c40d3c78353039206365727469663eac7375626d69747465645f6f6e"
        "d70141d8dd2f49d8f5c7ad656e726f6c6c6d656e745f6964d802e1fe88bd0f054261887a6c"
        "8039710b40b87375626d69745f7061796c6f61645f7369676e6174757265c40b3c7369676e"
        "61747572653eae7375626d69745f7061796c6f6164c4073c64756d6d793ea6737461747573"
        "a26f6b"
    ),
    authenticated_cmds::pki_enrollment_list::Rep::Ok {
            enrollments: vec![authenticated_cmds::pki_enrollment_list::PkiEnrollmentListItem {
                enrollment_id: EnrollmentID::from_hex("e1fe88bd0f054261887a6c8039710b40").unwrap(),
                submit_payload: hex!("3c64756d6d793e").to_vec(),
                submit_payload_signature: hex!("3c7369676e61747572653e").to_vec(),
                submitted_on: DateTime::from_f64_with_us_precision(1668594983.390001f64),
                submitter_der_x509_certificate: hex!("3c78353039206365727469663e").to_vec(),
            }],
        }
)]
#[case::ok_empty_list(
    // Generated from Python implementation (Parsec v2.14.0)
    // Content:
    //   enrollments: []
    //   status: "ok"
    &hex!("82ab656e726f6c6c6d656e747390a6737461747573a26f6b")[..],
    authenticated_cmds::pki_enrollment_list::Rep::Ok { enrollments: Vec::new() }
)]
#[case::not_allowed(
    // Generated from Python implementation (Parsec v2.14.0)
    // Content:
    //   reason: "oof"
    //   status: "not_allowed"
    &hex!("82a6726561736f6ea36f6f66a6737461747573ab6e6f745f616c6c6f776564")[..],
    authenticated_cmds::pki_enrollment_list::Rep::NotAllowed { reason: Some("oof".to_string()) }
)]
fn serde_authenticated_list_pki_rep(
    #[case] bytes: &[u8],
    #[case] expected: authenticated_cmds::pki_enrollment_list::Rep,
) {
    let data = authenticated_cmds::pki_enrollment_list::Rep::load(bytes).unwrap();
    assert_eq!(data, expected);

    let raw_data = data.dump().unwrap();
    assert_eq!(
        authenticated_cmds::pki_enrollment_list::Rep::load(&raw_data).unwrap(),
        expected
    );
}

#[parsec_test]
#[case::ok(
    // Generated from Python implementation (Parsec v2.14.0)
    // Content:
    //   status: "ok"
    &hex!("81a6737461747573a26f6b")[..],
    authenticated_cmds::pki_enrollment_reject::Rep::Ok
)]
#[case::not_allowed(
    // Generated from Python implementation (Parsec v2.14.0)
    // Content:
    //   reason: "oof"
    //   status: "not_allowed"
    &hex!("82a6726561736f6ea36f6f66a6737461747573ab6e6f745f616c6c6f776564"),
    authenticated_cmds::pki_enrollment_reject::Rep::NotAllowed{ reason: Some("oof".to_string()) }
)]
#[case::not_found(
    // Generated from Python implementation (Parsec v2.14.0)
    // Content:
    //   reason: "oof"
    //   status: "not_found"
    &hex!("82a6726561736f6ea36f6f66a6737461747573a96e6f745f666f756e64"),
    authenticated_cmds::pki_enrollment_reject::Rep::NotFound{ reason: Some("oof".to_string()) }
)]
#[case::no_longer_available(
    // Generated from Python implementation (Parsec v2.14.0)
    // Content:
    //   reason: "oof"
    //   status: "no_longer_available"
    &hex!(
        "82a6726561736f6ea36f6f66a6737461747573b36e6f5f6c6f6e6765725f617661696c6162"
        "6c65"
    ),
    authenticated_cmds::pki_enrollment_reject::Rep::NoLongerAvailable { reason: Some("oof".to_string()) }
)]
fn serde_authenticated_reject_pki_rep(
    #[case] bytes: &[u8],
    #[case] expected: authenticated_cmds::pki_enrollment_reject::Rep,
) {
    let data = authenticated_cmds::pki_enrollment_reject::Rep::load(bytes).unwrap();
    assert_eq!(data, expected);

    let raw = data.dump().unwrap();
    assert_eq!(
        authenticated_cmds::pki_enrollment_reject::Rep::load(&raw).unwrap(),
        expected
    );
}
