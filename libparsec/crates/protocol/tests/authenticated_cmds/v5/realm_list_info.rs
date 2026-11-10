// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use super::authenticated_cmds;
use libparsec_types::prelude::*;

use libparsec_tests_lite::{hex, p_assert_eq};

// Request

pub fn req() {
    // Generated from Parsec v3.11.1-a.0+dev
    // Content:
    //   cmd: 'realm_list_info'
    let raw = hex!("81a3636d64af7265616c6d5f6c6973745f696e666f");

    let req = authenticated_cmds::realm_list_info::Req;

    let expected = authenticated_cmds::AnyCmdReq::RealmListInfo(req.clone());

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = req.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    // Generated from Parsec 3.11.1-a.0+dev
    // Content:
    //   status: 'ok'
    //   realms: [
    //     {
    //       data_size: 2048,
    //       metadata_size: 1024,
    //       realm_id: ext(2, 0x2b5f314728134a12863da1ce49c112f6),
    //       users_per_role: { OWNER: 1, CONTRIBUTOR: 2, MANAGER: 0, READER: 0, },
    //     },
    //   ]
    let raw: &[u8] = hex!(
        "82a6737461747573a26f6ba67265616c6d739184a9646174615f73697a65cd0800ad6d"
        "657461646174615f73697a65cd0400a87265616c6d5f6964d8022b5f314728134a1286"
        "3da1ce49c112f6ae75736572735f7065725f726f6c6584a54f574e455201ab434f4e54"
        "52494255544f5202a74d414e4147455200a652454144455200"
    )
    .as_ref();

    let expected = authenticated_cmds::realm_list_info::Rep::Ok {
        realms: vec![authenticated_cmds::realm_list_info::RealmInfoItem {
            realm_id: VlobID::from_hex("2b5f314728134a12863da1ce49c112f6").unwrap(),
            metadata_size: 1024,
            data_size: 2048,
            users_per_role: [
                (RealmRole::Owner, 1),
                (RealmRole::Manager, 0),
                (RealmRole::Contributor, 2),
                (RealmRole::Reader, 0),
            ]
            .into_iter()
            .collect(),
        }],
    };

    let data = authenticated_cmds::realm_list_info::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_list_info::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_author_not_allowed() {
    // Generated from Parsec v3.11.1-a.0+dev
    // Content:
    //   status: 'author_not_allowed'
    let raw: &[u8] = hex!("81a6737461747573b2617574686f725f6e6f745f616c6c6f776564").as_ref();
    let expected = authenticated_cmds::realm_list_info::Rep::AuthorNotAllowed;
    let data = authenticated_cmds::realm_list_info::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_list_info::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
