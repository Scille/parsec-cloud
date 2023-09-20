// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

use super::authenticated_cmds;

// Request

pub fn req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "organization_config"
    let raw = hex!("81a3636d64b36f7267616e697a6174696f6e5f636f6e666967");

    let req = authenticated_cmds::organization_config::Req;

    let expected = authenticated_cmds::AnyCmdReq::OrganizationConfig(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let authenticated_cmds::AnyCmdReq::OrganizationConfig(req2) = data else {
        unreachable!()
    };

    let raw2 = req2.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   active_users_limit: 1
    //   status: "ok"
    //   user_profile_outsider_allowed: false
    //
    // Note that raw data does not contain:
    //  - sequester_authority_certificate
    //  - sequester_services_certificates
    // This was valid behavior in api v2 but is no longer valid from v3 onwards.
    // The corresponding expected values used here are therefore not important
    // since loading raw data should fail.
    //
    let raw = hex!(
        "83b26163746976655f75736572735f6c696d697401a6737461747573a26f6bbd757365725f"
        "70726f66696c655f6f757473696465725f616c6c6f776564c2"
    );

    let err = authenticated_cmds::organization_config::Rep::load(&raw).unwrap_err();
    let expected_err =
        rmp_serde::decode::Error::Syntax("missing field `sequester_authority_certificate`".into());

    assert!(matches!(err, expected_err));

    let raw_expected = [
        (
            // Generated from Rust implementation (Parsec v2.12.1+dev)
            // Content:
            //   active_users_limit: None
            //   sequester_authority_certificate: None
            //   sequester_services_certificates: None
            //   status: "ok"
            //   user_profile_outsider_allowed: false
            //
            &hex!(
                "85a6737461747573a26f6bbd757365725f70726f66696c655f6f757473696465725f616c6c"
                "6f776564c2b26163746976655f75736572735f6c696d6974c0bf7365717565737465725f61"
                "7574686f726974795f6365727469666963617465c0bf7365717565737465725f7365727669"
                "6365735f636572746966696361746573c0"
            )[..],
            authenticated_cmds::organization_config::Rep::Ok {
                user_profile_outsider_allowed: false,
                active_users_limit: ActiveUsersLimit::NoLimit,
                sequester_authority_certificate: None,
                sequester_services_certificates: None,
            },
        ),
        (
            // Generated from Python implementation (Parsec v2.11.1+dev)
            // Content:
            //   active_users_limit: 1
            //   sequester_authority_certificate: hex!("666f6f626172")
            //   sequester_services_certificates: [hex!("666f6f"), hex!("626172")]
            //   status: "ok"
            //   user_profile_outsider_allowed: false
            //
            &hex!(
            "85b26163746976655f75736572735f6c696d697401bf7365717565737465725f617574686f"
            "726974795f6365727469666963617465c406666f6f626172bf7365717565737465725f7365"
            "7276696365735f63657274696669636174657392c403666f6fc403626172a6737461747573"
            "a26f6bbd757365725f70726f66696c655f6f757473696465725f616c6c6f776564c2"
            )[..],
            authenticated_cmds::organization_config::Rep::Ok {
                user_profile_outsider_allowed: false,
                active_users_limit: ActiveUsersLimit::LimitedTo(1),
                sequester_authority_certificate: Some(b"foobar".as_ref().into()),
                sequester_services_certificates: Some(vec![
                    b"foo".as_ref().into(),
                    b"bar".as_ref().into(),
                ]),
            },
        ),
    ];

    for (raw, expected) in raw_expected {
        let data = authenticated_cmds::organization_config::Rep::load(raw).unwrap();

        p_assert_eq!(data, expected);

        // Also test serialization round trip
        let raw2 = data.dump().unwrap();

        let data2 = authenticated_cmds::organization_config::Rep::load(&raw2).unwrap();

        p_assert_eq!(data2, expected);
    }
}

pub fn rep_not_found() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_found"
    let raw = hex!("81a6737461747573a96e6f745f666f756e64");

    let expected = authenticated_cmds::organization_config::Rep::NotFound;

    let data = authenticated_cmds::organization_config::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::organization_config::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
