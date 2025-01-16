#![allow(clippy::unwrap_used)]
// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_lite::prelude::*;
use libparsec_types::UserID;

use super::authenticated_cmds;

pub fn rep_author_not_allowed() {
    // Generated from Parsec 3.1.1-a.0+dev
    // Content:
    //   status: 'author_not_allowed'
    let raw: &[u8] = hex!("81a6737461747573b2617574686f725f6e6f745f616c6c6f776564").as_ref();
    let expected = authenticated_cmds::list_frozen_users::Rep::AuthorNotAllowed;
    let data = authenticated_cmds::list_frozen_users::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::list_frozen_users::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_ok() {
    // Generated from Parsec 3.1.1-a.0+dev
    // Content:
    //   status: 'ok'
    //   frozen_users: [ ext(2, 0x109b68ba5cdf428ea0017fc6bcc04d4a), ]
    let raw: &[u8] = hex!(
    "82a6737461747573a26f6bac66726f7a656e5f757365727391d802109b68ba5cdf428e"
    "a0017fc6bcc04d4a"
    )
    .as_ref();

    let expected = authenticated_cmds::list_frozen_users::Rep::Ok {
        frozen_users: vec![UserID::from_hex("109b68ba5cdf428ea0017fc6bcc04d4a").unwrap()],
    };
    let data = authenticated_cmds::list_frozen_users::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::list_frozen_users::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn req() {
    // Generated from Parsec 3.1.1-a.0+dev
    // Content:
    //   cmd: 'list_frozen_users'
    let raw: &[u8] = hex!("81a3636d64b16c6973745f66726f7a656e5f7573657273").as_ref();

    let req = authenticated_cmds::list_frozen_users::Req;

    let expected = authenticated_cmds::AnyCmdReq::ListFrozenUsers(req.clone());
    let data = authenticated_cmds::AnyCmdReq::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = req.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
