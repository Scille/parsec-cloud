// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use hex_literal::hex;
use parsec_api_types::UserProfile;
use rstest::rstest;

use parsec_api_protocol::*;

#[rstest]
fn serde_organization_stats_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "organization_stats"
    let raw = hex!("81a3636d64b26f7267616e697a6174696f6e5f7374617473");

    let req = authenticated_cmds::organization_stats::Req;

    let expected = authenticated_cmds::AnyCmdReq::OrganizationStats(req.clone());

    let data = authenticated_cmds::AnyCmdReq::loads(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dumps().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::loads(&raw2).unwrap();

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
            users_per_profile_detail: vec![UsersPerProfileDetailItem {
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

    let data = authenticated_cmds::organization_stats::Rep::loads(&raw);

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dumps().unwrap();

    let data2 = authenticated_cmds::organization_stats::Rep::loads(&raw2);

    assert_eq!(data2, expected);
}

#[rstest]
fn serde_organization_config_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "organization_config"
    let raw = hex!("81a3636d64b36f7267616e697a6174696f6e5f636f6e666967");

    let req = authenticated_cmds::organization_config::Req;

    let expected = authenticated_cmds::AnyCmdReq::OrganizationConfig(req.clone());

    let data = authenticated_cmds::AnyCmdReq::loads(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dumps().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::loads(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[rstest]
#[case::ok(
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

    let data = authenticated_cmds::organization_config::Rep::loads(&raw);

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dumps().unwrap();

    let data2 = authenticated_cmds::organization_config::Rep::loads(&raw2);

    assert_eq!(data2, expected);
}
