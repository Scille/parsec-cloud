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
    // Generated from Parsec v3.0.0-b.11+dev
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
        "82a6737461747573a26f6bab696e7669746174696f6e739285a474797065a455534552"
        "ad636c61696d65725f656d61696caa616c6963654064657631aa637265617465645f6f"
        "6ed70100035d162fa2e400a6737461747573a449444c45a5746f6b656ec410d864b93d"
        "ed264aae9ae583fd3d40c45a84a474797065a6444556494345aa637265617465645f6f"
        "6ed70100035d162fa2e400a6737461747573a449444c45a5746f6b656ec410d864b93d"
        "ed264aae9ae583fd3d40c45a"
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

    // Generated from Parsec 3.1.1-a.0+dev
    // Content:
    //   status: 'ok'
    //   invitations: [
    //     {
    //       type: 'USER',
    //       claimer_email: 'alice@example.com',
    //       created_on: ext(1, 946774800000000) i.e. 2000-01-02T02:00:00Z,
    //       status: 'IDLE',
    //       token: 0xd864b93ded264aae9ae583fd3d40c45a,
    //     },
    //     {
    //       type: 'DEVICE',
    //       created_on: ext(1, 946774800000000) i.e. 2000-01-02T02:00:00Z,
    //       status: 'IDLE',
    //       token: 0xd864b93ded264aae9ae583fd3d40c45a,
    //     },
    //     {
    //       type: 'SHAMIR_RECOVERY',
    //       claimer_user_id: ext(2, 0xa11cec00100000000000000000000000),
    //       created_on: ext(1, 946774800000000) i.e. 2000-01-02T02:00:00Z,
    //       created_on: ext(1, 946688400000000) i.e. 2000-01-02T01:00:00Z,
    //       status: 'IDLE',
    //       token: 0xd864b93ded264aae9ae583fd3d40c45a,
    //     },
    //   ]
    let raw: &[u8] = hex!(
    "82a6737461747573a26f6bab696e7669746174696f6e739385a474797065a455534552"
    "ad636c61696d65725f656d61696cb1616c696365406578616d706c652e636f6daa6372"
    "65617465645f6f6ed70100035d162fa2e400a6737461747573a449444c45a5746f6b65"
    "6ec410d864b93ded264aae9ae583fd3d40c45a84a474797065a6444556494345aa6372"
    "65617465645f6f6ed70100035d162fa2e400a6737461747573a449444c45a5746f6b65"
    "6ec410d864b93ded264aae9ae583fd3d40c45a86a474797065af5348414d49525f5245"
    "434f56455259af636c61696d65725f757365725f6964d802a11cec0010000000000000"
    "0000000000aa637265617465645f6f6ed70100035d162fa2e400ba7368616d69725f72"
    "65636f766572795f637265617465645f6f6ed70100035d0211cb8400a6737461747573"
    "a449444c45a5746f6b656ec410d864b93ded264aae9ae583fd3d40c45a"
    )
    .as_ref();

    let expected = authenticated_cmds::invite_list::Rep::Ok {
        invitations: vec![
            authenticated_cmds::invite_list::InviteListItem::User {
                token: InvitationToken::from_hex("d864b93ded264aae9ae583fd3d40c45a").unwrap(),
                created_on: "2000-1-2T01:00:00Z".parse().unwrap(),
                claimer_email: "alice@example.com".to_owned(),
                status: InvitationStatus::Idle,
            },
            authenticated_cmds::invite_list::InviteListItem::Device {
                token: InvitationToken::from_hex("d864b93ded264aae9ae583fd3d40c45a").unwrap(),
                created_on: "2000-1-2T01:00:00Z".parse().unwrap(),
                status: InvitationStatus::Idle,
            },
            authenticated_cmds::invite_list::InviteListItem::ShamirRecovery {
                token: InvitationToken::from_hex("d864b93ded264aae9ae583fd3d40c45a").unwrap(),
                created_on: "2000-1-2T01:00:00Z".parse().unwrap(),
                claimer_user_id: "alice".parse().unwrap(),
                shamir_recovery_created_on: "2000-1-1T01:00:00Z".parse().unwrap(),
                status: InvitationStatus::Idle,
            },
        ],
    };

    println!("***expected: {:?}", expected.dump().unwrap());
    let data = authenticated_cmds::invite_list::Rep::load(raw).unwrap();
    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::invite_list::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
