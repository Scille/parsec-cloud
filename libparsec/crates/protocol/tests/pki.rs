// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_protocol::authenticated_cmds::v3 as authenticated_cmds;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

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
