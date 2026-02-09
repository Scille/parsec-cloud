// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use super::authenticated_cmds;
use libparsec_types::prelude::*;

use libparsec_tests_lite::{hex, p_assert_eq};

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
    // Generated from Parsec 3.2.4-a.0+dev
    // Content:
    //   invitations: [
    //     {
    //       type: "USER"
    //       claimer_email: "alice@dev1"
    //       created_on: ext(1, 946774800.0)
    //       created_by: { type: 'USER', human_handle: [ 'bob@dev1', 'bob', ], user_id: ext(2, 0x109b68ba5cdf428ea0017fc6bcc04d4a), }
    //       status: "PENDING"
    //       token: ext(2, hex!("d864b93ded264aae9ae583fd3d40c45a"))
    //     }
    //     {
    //       type: "DEVICE"
    //       created_on: ext(1, 946774800.0)
    //       created_by: { type: 'USER', human_handle: [ 'carl@dev1', 'carl', ], user_id: ext(2, 0x109b68ba5cdf428ea0017fc6bcc04d4b), }
    //       status: "PENDING"
    //       token: ext(2, hex!("d864b93ded264aae9ae583fd3d40c45a"))
    //     }
    //   ]
    //   status: "ok"
    let raw: &[u8] = hex!(
        "82a6737461747573a26f6bab696e7669746174696f6e739286a474797065a455534552"
        "ad636c61696d65725f656d61696caa616c6963654064657631aa637265617465645f62"
        "7983a474797065a455534552ac68756d616e5f68616e646c6592a8626f624064657631"
        "a3626f62a7757365725f6964d802109b68ba5cdf428ea0017fc6bcc04d4aaa63726561"
        "7465645f6f6ed70100035d162fa2e400a6737461747573a750454e44494e47a5746f6b"
        "656ec410d864b93ded264aae9ae583fd3d40c45a85a474797065a6444556494345aa63"
        "7265617465645f627983a474797065a455534552ac68756d616e5f68616e646c6592a9"
        "6361726c4064657631a46361726ca7757365725f6964d802109b68ba5cdf428ea0017f"
        "c6bcc04d4baa637265617465645f6f6ed70100035d162fa2e400a6737461747573a750"
        "454e44494e47a5746f6b656ec410d864b93ded264aae9ae583fd3d40c45a"
    )
    .as_ref();

    let expected = authenticated_cmds::invite_list::Rep::Ok {
        invitations: vec![
            authenticated_cmds::invite_list::InviteListItem::User {
                token: AccessToken::from_hex("d864b93ded264aae9ae583fd3d40c45a").unwrap(),
                created_on: "2000-1-2T01:00:00Z".parse().unwrap(),
                created_by: authenticated_cmds::invite_list::InvitationCreatedBy::User {
                    human_handle: HumanHandle::from_raw("bob@dev1", "bob").unwrap(),
                    user_id: UserID::from_hex("109b68ba5cdf428ea0017fc6bcc04d4a").unwrap(),
                },
                claimer_email: "alice@dev1".parse().unwrap(),
                status: InvitationStatus::Pending,
            },
            authenticated_cmds::invite_list::InviteListItem::Device {
                token: AccessToken::from_hex("d864b93ded264aae9ae583fd3d40c45a").unwrap(),
                created_on: "2000-1-2T01:00:00Z".parse().unwrap(),
                created_by: authenticated_cmds::invite_list::InvitationCreatedBy::User {
                    human_handle: HumanHandle::from_raw("carl@dev1", "carl").unwrap(),
                    user_id: UserID::from_hex("109b68ba5cdf428ea0017fc6bcc04d4b").unwrap(),
                },
                status: InvitationStatus::Pending,
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

    // Generated from Parsec 3.2.4-a.0+dev
    // Content:
    //   status: 'ok'
    //   invitations: [
    //     {
    //       type: 'USER',
    //       claimer_email: 'alice@example.com',
    //       created_on: ext(1, 946774800000000) i.e. 2000-01-02T02:00:00Z,
    //       created_by: { type: 'USER', human_handle: [ 'bob@dev1', 'bob', ], user_id: ext(2, 0x109b68ba5cdf428ea0017fc6bcc04d4a), },
    //       status: 'PENDING',
    //       token: 0xd864b93ded264aae9ae583fd3d40c45a,
    //     },
    //     {
    //       type: 'DEVICE',
    //       created_on: ext(1, 946774800000000) i.e. 2000-01-02T02:00:00Z,
    //       created_by: { type: 'USER', human_handle: [ 'carl@dev1', 'carl', ], user_id: ext(2, 0x109b68ba5cdf428ea0017fc6bcc04d4b), },
    //       status: 'PENDING',
    //       token: 0xd864b93ded264aae9ae583fd3d40c45a,
    //     },
    //     {
    //       type: 'SHAMIR_RECOVERY',
    //       claimer_user_id: ext(2, 0xa11cec00100000000000000000000000),
    //       created_on: ext(1, 946774800000000) i.e. 2000-01-02T02:00:00Z,
    //       created_by: { type: 'EXTERNAL_SERVICE', service_label: 'LDAP', },
    //       shamir_recovery_created_on: ext(1, 946688400000000) i.e. 2000-01-02T01:00:00Z,
    //       status: 'PENDING',
    //       token: 0xd864b93ded264aae9ae583fd3d40c45a,
    //     },
    //   ]
    let raw: &[u8] = hex!(
        "82a6737461747573a26f6bab696e7669746174696f6e739386a474797065a455534552"
        "ad636c61696d65725f656d61696cb1616c696365406578616d706c652e636f6daa6372"
        "65617465645f627983a474797065a455534552ac68756d616e5f68616e646c6592a862"
        "6f624064657631a3626f62a7757365725f6964d802109b68ba5cdf428ea0017fc6bcc0"
        "4d4aaa637265617465645f6f6ed70100035d162fa2e400a6737461747573a750454e44"
        "494e47a5746f6b656ec410d864b93ded264aae9ae583fd3d40c45a85a474797065a644"
        "4556494345aa637265617465645f627983a474797065a455534552ac68756d616e5f68"
        "616e646c6592a96361726c4064657631a46361726ca7757365725f6964d802109b68ba"
        "5cdf428ea0017fc6bcc04d4baa637265617465645f6f6ed70100035d162fa2e400a673"
        "7461747573a750454e44494e47a5746f6b656ec410d864b93ded264aae9ae583fd3d40"
        "c45a87a474797065af5348414d49525f5245434f56455259af636c61696d65725f7573"
        "65725f6964d802a11cec00100000000000000000000000aa637265617465645f627982"
        "a474797065b045585445524e414c5f53455256494345ad736572766963655f6c616265"
        "6ca44c444150aa637265617465645f6f6ed70100035d162fa2e400ba7368616d69725f"
        "7265636f766572795f637265617465645f6f6ed70100035d0211cb8400a67374617475"
        "73a750454e44494e47a5746f6b656ec410d864b93ded264aae9ae583fd3d40c45a"
    )
    .as_ref();

    let expected = authenticated_cmds::invite_list::Rep::Ok {
        invitations: vec![
            authenticated_cmds::invite_list::InviteListItem::User {
                token: AccessToken::from_hex("d864b93ded264aae9ae583fd3d40c45a").unwrap(),
                created_on: "2000-1-2T01:00:00Z".parse().unwrap(),
                created_by: authenticated_cmds::invite_list::InvitationCreatedBy::User {
                    human_handle: HumanHandle::from_raw("bob@dev1", "bob").unwrap(),
                    user_id: UserID::from_hex("109b68ba5cdf428ea0017fc6bcc04d4a").unwrap(),
                },
                claimer_email: "alice@example.com".parse().unwrap(),
                status: InvitationStatus::Pending,
            },
            authenticated_cmds::invite_list::InviteListItem::Device {
                token: AccessToken::from_hex("d864b93ded264aae9ae583fd3d40c45a").unwrap(),
                created_on: "2000-1-2T01:00:00Z".parse().unwrap(),
                created_by: authenticated_cmds::invite_list::InvitationCreatedBy::User {
                    human_handle: HumanHandle::from_raw("carl@dev1", "carl").unwrap(),
                    user_id: UserID::from_hex("109b68ba5cdf428ea0017fc6bcc04d4b").unwrap(),
                },
                status: InvitationStatus::Pending,
            },
            authenticated_cmds::invite_list::InviteListItem::ShamirRecovery {
                token: AccessToken::from_hex("d864b93ded264aae9ae583fd3d40c45a").unwrap(),
                created_on: "2000-1-2T01:00:00Z".parse().unwrap(),
                created_by: authenticated_cmds::invite_list::InvitationCreatedBy::ExternalService {
                    service_label: "LDAP".to_owned(),
                },
                claimer_user_id: "alice".parse().unwrap(),
                shamir_recovery_created_on: "2000-1-1T01:00:00Z".parse().unwrap(),
                status: InvitationStatus::Pending,
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
