// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use hex_literal::hex;
use libparsec_types::{Maybe, UserProfile};
use rstest::rstest;

use libparsec_protocol::*;

#[rstest]
fn serde_organization_stats_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "organization_stats"
    let raw = hex!("81a3636d64b26f7267616e697a6174696f6e5f7374617473");

    let req = authenticated_cmds::organization_stats::Req;

    let expected = authenticated_cmds::AnyCmdReq::OrganizationStats(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[rstest]
#[case::ok(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   active_users: 1
        //   raw_size: 8
        //   metaraw_size: 8
        //   realms: 1
        //   status: "ok"
        //   users: 1
        //   users_per_profile_detail: [{active:1, profile:"ADMIN", revoked:0}]
        &hex!(
            "87ac6163746976655f757365727301a9646174615f73697a6508ad6d657461646174615f73"
            "697a6508a67265616c6d7301a6737461747573a26f6ba5757365727301b875736572735f70"
            "65725f70726f66696c655f64657461696c9183a770726f66696c65a541444d494ea6616374"
            "69766501a77265766f6b656400"
        )[..],
        authenticated_cmds::organization_stats::Rep::Ok {
            data_size: 8,
            metadata_size: 8,
            realms: 1,
            users: 1,
            active_users: 1,
            users_per_profile_detail: vec![authenticated_cmds::organization_stats::UsersPerProfileDetailItem {
                profile: UserProfile::Admin,
                active: 1,
                revoked: 0,
            }],
        }
    )
)]
#[case::not_allowed(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   reason: "foobar"
        //   status: "not_allowed"
        &hex!(
            "82a6726561736f6ea6666f6f626172a6737461747573ab6e6f745f616c6c6f776564"
        )[..],
        authenticated_cmds::organization_stats::Rep::NotAllowed {
            reason: Some("foobar".to_owned())
        }
    )
)]
#[case::not_found(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "not_found"
        &hex!(
            "81a6737461747573a96e6f745f666f756e64"
        )[..],
        authenticated_cmds::organization_stats::Rep::NotFound
    )
)]
fn serde_organization_stats_rep(
    #[case] raw_expected: (&[u8], authenticated_cmds::organization_stats::Rep),
) {
    let (raw, expected) = raw_expected;

    let data = authenticated_cmds::organization_stats::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::organization_stats::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[rstest]
fn serde_organization_config_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "organization_config"
    let raw = hex!("81a3636d64b36f7267616e697a6174696f6e5f636f6e666967");

    let req = authenticated_cmds::organization_config::Req;

    let expected = authenticated_cmds::AnyCmdReq::OrganizationConfig(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[rstest]
#[case::ok_legacy(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   active_users_limit: 1
        //   status: "ok"
        //   user_profile_outsider_allowed: false
        &hex!(
            "83b26163746976655f75736572735f6c696d697401a6737461747573a26f6bbd757365725f"
            "70726f66696c655f6f757473696465725f616c6c6f776564c2"
        )[..],
        authenticated_cmds::organization_config::Rep::Ok {
            user_profile_outsider_allowed: false,
            active_users_limit: Some(1),
            sequester_authority_certificate: Maybe::Absent,
            sequester_services_certificates: Maybe::Absent,
        }
    )
)]
#[case::ok_without(
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
            active_users_limit: None,
            sequester_authority_certificate: Maybe::Present(None),
            sequester_services_certificates: Maybe::Present(None),
        }
    )
)]
#[case::ok_full(
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
            active_users_limit: Some(1),
            sequester_authority_certificate: Maybe::Present(Some(b"foobar".to_vec())),
            sequester_services_certificates: Maybe::Present(Some(vec![b"foo".to_vec(), b"bar".to_vec()])),
        }
    )
)]
#[case::not_found(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "not_found"
        &hex!(
            "81a6737461747573a96e6f745f666f756e64"
        )[..],
        authenticated_cmds::organization_config::Rep::NotFound
    )
)]
fn serde_organization_config_rep(
    #[case] raw_expected: (&[u8], authenticated_cmds::organization_config::Rep),
) {
    let (raw, expected) = raw_expected;

    let data = authenticated_cmds::organization_config::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::organization_config::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}
