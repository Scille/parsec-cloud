// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use hex_literal::hex;
use libparsec_protocol::anonymous_cmds::v2 as anonymous_cmds;
use libparsec_types::{DateTime, EnrollmentID};
use rstest::rstest;

#[rstest]
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

#[rstest]
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

#[rstest]
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
