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
    //   accept_payload: hex!("3c64756d6d793e")
    //   accept_payload_signature: hex!("3c7369676e61747572653e")
    //   accepter_der_x509_certificate: hex!("3c61636365707465725f6465725f783530395f63657274696669636174653e")
    //   cmd: "pki_enrollment_accept"
    //   device_certificate: hex!("3c64756d6d793e")
    //   enrollment_id: ext(2, hex!("e89621b91f8e4a7c8d3182ee513e380f"))
    //   redacted_device_certificate: hex!("3c64756d6d793e")
    //   redacted_user_certificate: hex!("3c64756d6d793e")
    //   user_certificate: hex!("3c64756d6d793e")
    //
    let raw = hex!(
        "89ae6163636570745f7061796c6f6164c4073c64756d6d793eb86163636570745f7061796c"
        "6f61645f7369676e6174757265c40b3c7369676e61747572653ebd61636365707465725f64"
        "65725f783530395f6365727469666963617465c41f3c61636365707465725f6465725f7835"
        "30395f63657274696669636174653ea3636d64b5706b695f656e726f6c6c6d656e745f6163"
        "63657074b26465766963655f6365727469666963617465c4073c64756d6d793ead656e726f"
        "6c6c6d656e745f6964d80256f48ed307984f10830e197287399c22bb72656461637465645f"
        "6465766963655f6365727469666963617465c4073c64756d6d793eb972656461637465645f"
        "757365725f6365727469666963617465c4073c64756d6d793eb0757365725f636572746966"
        "6963617465c4073c64756d6d793e"
    );

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

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let authenticated_cmds::AnyCmdReq::PkiEnrollmentAccept(req2) = data else {
        unreachable!()
    };

    let raw2 = req2.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    // Generated from Python implementation (Parsec v2.14.0)
    // Content:
    //   status: "ok"
    //
    let raw = hex!("81a6737461747573a26f6b");

    let expected = authenticated_cmds::pki_enrollment_accept::Rep::Ok;

    let data = authenticated_cmds::pki_enrollment_accept::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::pki_enrollment_accept::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected)
}

pub fn rep_not_allowed() {
    // Generated from Rust implementation (Parsec v2.15.0+dev)
    // Content:
    //   status: "not_allowed"
    //
    let raw = hex!("82a6737461747573ab6e6f745f616c6c6f776564a6726561736f6ec0");

    let expected = authenticated_cmds::pki_enrollment_accept::Rep::NotAllowed { reason: None };

    let data = authenticated_cmds::pki_enrollment_accept::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::pki_enrollment_accept::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected)
}

pub fn rep_invalid_payload_data() {
    // Generated from Rust implementation (Parsec v2.15.0+dev)
    // Content:
    //   status: "invalid_payload_data"
    //
    let raw = hex!("82a6737461747573b4696e76616c69645f7061796c6f61645f64617461a6726561736f6ec0");

    let expected =
        authenticated_cmds::pki_enrollment_accept::Rep::InvalidPayloadData { reason: None };

    let data = authenticated_cmds::pki_enrollment_accept::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::pki_enrollment_accept::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected)
}

pub fn rep_invalid_certification() {
    // Generated from Rust implementation (Parsec v2.15.0+dev)
    // Content:
    //   status: "invalid_certification"
    //
    let raw = hex!("82a6737461747573b5696e76616c69645f63657274696669636174696f6ea6726561736f6ec0");

    let expected =
        authenticated_cmds::pki_enrollment_accept::Rep::InvalidCertification { reason: None };

    let data = authenticated_cmds::pki_enrollment_accept::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::pki_enrollment_accept::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected)
}

pub fn rep_invalid_data() {
    // Generated from Rust implementation (Parsec v2.15.0+dev)
    // Content:
    //   status: "invalid_data"
    //
    let raw = hex!("82a6737461747573ac696e76616c69645f64617461a6726561736f6ec0");

    let expected = authenticated_cmds::pki_enrollment_accept::Rep::InvalidData { reason: None };

    let data = authenticated_cmds::pki_enrollment_accept::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::pki_enrollment_accept::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected)
}

pub fn rep_not_found() {
    // Generated from Rust implementation (Parsec v2.15.0+dev)
    // Content:
    //   status: "not_found"
    //
    let raw = hex!("82a6737461747573a96e6f745f666f756e64a6726561736f6ec0");

    let expected = authenticated_cmds::pki_enrollment_accept::Rep::NotFound { reason: None };

    let data = authenticated_cmds::pki_enrollment_accept::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::pki_enrollment_accept::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected)
}

pub fn rep_no_longer_available() {
    // Generated from Rust implementation (Parsec v2.15.0+dev)
    // Content:
    //   status: "no_longer_available"
    //
    let raw = hex!("82a6737461747573b36e6f5f6c6f6e6765725f617661696c61626c65a6726561736f6ec0");

    let expected =
        authenticated_cmds::pki_enrollment_accept::Rep::NoLongerAvailable { reason: None };

    let data = authenticated_cmds::pki_enrollment_accept::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::pki_enrollment_accept::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected)
}

pub fn rep_already_exists() {
    // Generated from Rust implementation (Parsec v2.15.0+dev)
    // Content:
    //   status: "already_exists"
    //
    let raw = hex!("82a6737461747573ae616c72656164795f657869737473a6726561736f6ec0");

    let expected = authenticated_cmds::pki_enrollment_accept::Rep::AlreadyExists { reason: None };

    let data = authenticated_cmds::pki_enrollment_accept::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::pki_enrollment_accept::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected)
}

pub fn rep_active_users_limit_reached() {
    // Generated from Rust implementation (Parsec v2.15.0+dev)
    // Content:
    //   status: "active_users_limit_reached"
    //
    let raw = hex!("81a6737461747573ba6163746976655f75736572735f6c696d69745f72656163686564");

    let expected = authenticated_cmds::pki_enrollment_accept::Rep::ActiveUsersLimitReached;

    let data = authenticated_cmds::pki_enrollment_accept::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::pki_enrollment_accept::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected)
}

pub fn rep_bad_timestamp() {
    // Generated from Rust implementation (Parsec v2.15.0+dev)
    // Content:
    //   backend_timestamp: ext(1, 946774800.0)
    //   ballpark_client_early_offset: 50.0
    //   ballpark_client_late_offset: 70.0
    //   client_timestamp: ext(1, 946774800.0)
    //   status: "bad_timestamp"
    //
    let raw = hex!(
        "86a6737461747573ad6261645f74696d657374616d70b16261636b656e645f74696d657374"
        "616d70d70141cc375188000000bc62616c6c7061726b5f636c69656e745f6561726c795f6f"
        "6666736574cb4049000000000000bb62616c6c7061726b5f636c69656e745f6c6174655f6f"
        "6666736574cb4051800000000000b0636c69656e745f74696d657374616d70d70141cc3751"
        "88000000a6726561736f6ec0"
    );

    let expected = authenticated_cmds::pki_enrollment_accept::Rep::BadTimestamp {
        reason: None,
        ballpark_client_early_offset: 50.,
        ballpark_client_late_offset: 70.,
        backend_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
        client_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
    };

    let data = authenticated_cmds::pki_enrollment_accept::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::pki_enrollment_accept::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected)
}

pub fn rep_require_greater_timestamp() {
    // Generated from Python implementation (Parsec v2.11.1+dev)
    // Content:
    //   status: "require_greater_timestamp"
    //   strictly_greater_than: ext(1, 946774800.0)
    //
    let raw = hex!(
        "82a6737461747573b9726571756972655f677265617465725f74696d657374616d70b57374"
        "726963746c795f677265617465725f7468616ed70141cc375188000000"
    );

    let expected = authenticated_cmds::pki_enrollment_accept::Rep::RequireGreaterTimestamp {
        strictly_greater_than: "2000-1-2T01:00:00Z".parse().unwrap(),
    };

    let data = authenticated_cmds::pki_enrollment_accept::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::pki_enrollment_accept::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected)
}
