// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_protocol::authenticated_cmds::v3 as authenticated_cmds;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

#[parsec_test]
fn serde_authenticated_pki_enrollment_accep_req() {
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
    let bytes = &hex!(
        "89ae6163636570745f7061796c6f6164c4073c64756d6d793eb86163636570745f7061796c"
        "6f61645f7369676e6174757265c40b3c7369676e61747572653ebd61636365707465725f64"
        "65725f783530395f6365727469666963617465c41f3c61636365707465725f6465725f7835"
        "30395f63657274696669636174653ea3636d64b5706b695f656e726f6c6c6d656e745f6163"
        "63657074b26465766963655f6365727469666963617465c4073c64756d6d793ead656e726f"
        "6c6c6d656e745f6964d80256f48ed307984f10830e197287399c22bb72656461637465645f"
        "6465766963655f6365727469666963617465c4073c64756d6d793eb972656461637465645f"
        "757365725f6365727469666963617465c4073c64756d6d793eb0757365725f636572746966"
        "6963617465c4073c64756d6d793e"
    )[..];
    let expected = authenticated_cmds::AnyCmdReq::PkiEnrollmentAccept(
        authenticated_cmds::pki_enrollment_accept::Req {
            accept_payload: hex!("3c64756d6d793e").as_ref().into(),
            accept_payload_signature: hex!("3c7369676e61747572653e").as_ref().into(),
            accepter_der_x509_certificate: hex!(
                "3c61636365707465725f6465725f783530395f63657274696669636174653e"
            )
            .as_ref()
            .into(),
            device_certificate: hex!("3c64756d6d793e").as_ref().into(),
            enrollment_id: EnrollmentID::from_hex("56f48ed307984f10830e197287399c22").unwrap(),
            redacted_device_certificate: hex!("3c64756d6d793e").as_ref().into(),
            redacted_user_certificate: hex!("3c64756d6d793e").as_ref().into(),
            user_certificate: hex!("3c64756d6d793e").as_ref().into(),
        },
    );

    let data = authenticated_cmds::AnyCmdReq::load(bytes).unwrap();

    assert_eq!(data, expected);

    // check roundtrip ...
    let raw2 = if let authenticated_cmds::AnyCmdReq::PkiEnrollmentAccept(data) = data {
        data.dump().unwrap()
    } else {
        unreachable!()
    };

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();
    assert_eq!(data2, expected);
}

#[parsec_test]
fn serde_authenticated_pki_enrollment_reject_req() {
    // Generated from Python implementation (Parsec v2.14.0)
    // Content:
    //   cmd: "pki_enrollment_reject"
    //   enrollment_id: ext(2, hex!("88a75cc10b8d43d9b1f91ad8a12be8ee"))
    let bytes = &hex!(
      "82a3636d64b5706b695f656e726f6c6c6d656e745f72656a656374ad656e726f6c6c6d656e"
      "745f6964d80288a75cc10b8d43d9b1f91ad8a12be8ee"
    )[..];
    let expected = authenticated_cmds::AnyCmdReq::PkiEnrollmentReject(
        authenticated_cmds::pki_enrollment_reject::Req {
            enrollment_id: EnrollmentID::from_hex("88a75cc10b8d43d9b1f91ad8a12be8ee").unwrap(),
        },
    );

    let data = authenticated_cmds::AnyCmdReq::load(bytes).unwrap();

    assert_eq!(data, expected);

    // check roundtrip ...
    let raw2 = if let authenticated_cmds::AnyCmdReq::PkiEnrollmentReject(data) = data {
        data.dump().unwrap()
    } else {
        unreachable!()
    };

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();
    assert_eq!(data2, expected);
}

// Generated from Python implementation (Parsec v2.14.0)
// Content:
//   status: "ok"
#[parsec_test]
fn serde_authenticated_accept_pki_rep() {
    let bytes = &hex!("81a6737461747573a26f6b")[..];
    let expected = authenticated_cmds::pki_enrollment_accept::Rep::Ok;

    let data = authenticated_cmds::pki_enrollment_accept::Rep::load(bytes).unwrap();

    assert_eq!(data, expected);
    let raw = data.dump().unwrap();
    assert_eq!(
        authenticated_cmds::pki_enrollment_accept::Rep::load(&raw).unwrap(),
        expected
    )
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
