// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use super::authenticated_cmds;

use libparsec_tests_lite::{hex, p_assert_eq};
use libparsec_types::{InvitationStatus, InvitationToken};

// Request

pub fn req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "invite_list"
    let raw = hex!("81a3636d64ab696e766974655f6c697374");

    let req = authenticated_cmds::invite_list::Req;

    let expected = authenticated_cmds::AnyCmdReq::InviteList(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let authenticated_cmds::AnyCmdReq::InviteList(req2) = data else {
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
    //   invitations: [
    //     {
    //       type: "USER"
    //       claimer_email: "alice@dev1"
    //       created_on: ext(1, 946774800.0)
    //       status: "IDLE"
    //       token: ext(2, hex!("d864b93ded264aae9ae583fd3d40c45a"))
    //     }
    //     {
    //       type: "DEVICE"
    //       created_on: ext(1, 946774800.0)
    //       status: "IDLE"
    //       token: ext(2, hex!("d864b93ded264aae9ae583fd3d40c45a"))
    //     }
    //   ]
    //   status: "ok"
    let raw = hex!(
        "82ab696e7669746174696f6e739285ad636c61696d65725f656d61696caa616c6963654064"
        "657631aa637265617465645f6f6ed70141cc375188000000a6737461747573a449444c45a5"
        "746f6b656ed802d864b93ded264aae9ae583fd3d40c45aa474797065a45553455284aa6372"
        "65617465645f6f6ed70141cc375188000000a6737461747573a449444c45a5746f6b656ed8"
        "02d864b93ded264aae9ae583fd3d40c45aa474797065a6444556494345a6737461747573a2"
        "6f6b"
    );

    let expected = authenticated_cmds::invite_list::Rep::Ok {
        invitations: vec![
            authenticated_cmds::invite_list::InviteListItem::User {
                token: InvitationToken::from_hex("d864b93ded264aae9ae583fd3d40c45a").unwrap(),
                created_on: "2000-1-2T01:00:00Z".parse().unwrap(),
                claimer_email: "alice@dev1".to_owned(),
                status: InvitationStatus::Idle,
            },
            authenticated_cmds::invite_list::InviteListItem::Device {
                token: InvitationToken::from_hex("d864b93ded264aae9ae583fd3d40c45a").unwrap(),
                created_on: "2000-1-2T01:00:00Z".parse().unwrap(),
                status: InvitationStatus::Idle,
            },
        ],
    };

    let data = authenticated_cmds::invite_list::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::invite_list::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
