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
    //   cmd: "organization_stats"
    let raw = hex!("81a3636d64b26f7267616e697a6174696f6e5f7374617473");

    let req = authenticated_cmds::organization_stats::Req;

    let expected = authenticated_cmds::AnyCmdReq::OrganizationStats(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let authenticated_cmds::AnyCmdReq::OrganizationStats(req2) = data else {
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
    //   active_users: 1
    //   raw_size: 8
    //   metaraw_size: 8
    //   realms: 1
    //   status: "ok"
    //   users: 1
    //   users_per_profile_detail: [{active:1, profile:"ADMIN", revoked:0}]
    let raw = hex!(
        "87ac6163746976655f757365727301a9646174615f73697a6508ad6d657461646174615f73"
        "697a6508a67265616c6d7301a6737461747573a26f6ba5757365727301b875736572735f70"
        "65725f70726f66696c655f64657461696c9183a770726f66696c65a541444d494ea6616374"
        "69766501a77265766f6b656400"
    );

    let expected = authenticated_cmds::organization_stats::Rep::Ok {
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
    };

    let data = authenticated_cmds::organization_stats::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::organization_stats::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_not_allowed() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   reason: "foobar"
    //   status: "not_allowed"
    let raw = hex!("82a6726561736f6ea6666f6f626172a6737461747573ab6e6f745f616c6c6f776564");

    let expected = authenticated_cmds::organization_stats::Rep::NotAllowed {
        reason: Some("foobar".to_owned()),
    };

    let data = authenticated_cmds::organization_stats::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::organization_stats::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_not_found() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_found"
    let raw = hex!("81a6737461747573a96e6f745f666f756e64");

    let expected = authenticated_cmds::organization_stats::Rep::NotFound;

    let data = authenticated_cmds::organization_stats::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::organization_stats::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
