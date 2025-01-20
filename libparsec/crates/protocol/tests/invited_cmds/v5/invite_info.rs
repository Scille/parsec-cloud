// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

#![allow(clippy::duplicate_mod)]
// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

// The compat module allows to re-use tests from previous major API

use super::invited_cmds;

// Request

pub fn req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "invite_info"
    let raw = hex!("81a3636d64ab696e766974655f696e666f");

    let req = invited_cmds::invite_info::Req;

    let expected = invited_cmds::AnyCmdReq::InviteInfo(req);

    let data = invited_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let invited_cmds::AnyCmdReq::InviteInfo(req2) = data else {
        unreachable!()
    };

    let raw2 = req2.dump().unwrap();

    let data2 = invited_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    let raw_expected = [
        (
            // Generated from Parsec 3.2.4-a.0+dev
            // Content:
            //   status: 'ok'
            //   type: 'USER'
            //   administrators: [ { human_handle: [ 'bob@dev1', 'bob', ], online_status: { type: 'UNKNOWN', }, user_id: ext(2, 0x109b68ba5cdf428ea0017fc6bcc04d4a), }, { human_handle: [ 'carl@dev1', 'carl', ], online_status: { type: 'ONLINE', }, user_id: ext(2, 0x109b68ba5cdf428ea0017fc6bcc04d4b), }, ]
            //   claimer_email: 'alice@dev1'
            //   created_by: { type: 'ORGANIZATION_ADMINISTRATOR', human_handle: [ 'bob@dev1', 'bob', ], user_id: ext(2, 0x109b68ba5cdf428ea0017fc6bcc04d4a), }
            &hex!(
            "85a6737461747573a26f6ba474797065a455534552ae61646d696e6973747261746f72"
            "739283ac68756d616e5f68616e646c6592a8626f624064657631a3626f62ad6f6e6c69"
            "6e655f73746174757381a474797065a7554e4b4e4f574ea7757365725f6964d802109b"
            "68ba5cdf428ea0017fc6bcc04d4a83ac68756d616e5f68616e646c6592a96361726c40"
            "64657631a46361726cad6f6e6c696e655f73746174757381a474797065a64f4e4c494e"
            "45a7757365725f6964d802109b68ba5cdf428ea0017fc6bcc04d4bad636c61696d6572"
            "5f656d61696caa616c6963654064657631aa637265617465645f627983a474797065ba"
            "4f5247414e495a4154494f4e5f41444d494e4953545241544f52ac68756d616e5f6861"
            "6e646c6592a8626f624064657631a3626f62a7757365725f6964d802109b68ba5cdf42"
            "8ea0017fc6bcc04d4a"
            )[..],
            invited_cmds::invite_info::Rep::Ok(invited_cmds::invite_info::InvitationType::User {
                claimer_email: "alice@dev1".to_owned(),
                created_by:
                    invited_cmds::invite_info::UserInvitationCreatedBy::OrganizationAdministrator {
                        human_handle: HumanHandle::new("bob@dev1", "bob").unwrap(),
                        user_id: UserID::from_hex("109b68ba5cdf428ea0017fc6bcc04d4a").unwrap(),
                    },
                administrators: vec![
                    invited_cmds::invite_info::InviteInfoAdministrator {
                        human_handle: HumanHandle::new("bob@dev1", "bob").unwrap(),
                        user_id: UserID::from_hex("109b68ba5cdf428ea0017fc6bcc04d4a").unwrap(),
                        online_status: invited_cmds::invite_info::UserOnlineStatus::Unknown,
                    },
                    invited_cmds::invite_info::InviteInfoAdministrator {
                        human_handle: HumanHandle::new("carl@dev1", "carl").unwrap(),
                        user_id: UserID::from_hex("109b68ba5cdf428ea0017fc6bcc04d4b").unwrap(),
                        online_status: invited_cmds::invite_info::UserOnlineStatus::Online,
                    },
                ],
            }),
        ),
        (
            // Generated from Parsec 3.2.4-a.0+dev
            // Content:
            //   status: 'ok'
            //   type: 'USER'
            //   administrators: [ { human_handle: [ 'bob@dev1', 'bob', ], online_status: { type: 'UNKNOWN', }, user_id: ext(2, 0x109b68ba5cdf428ea0017fc6bcc04d4a), }, { human_handle: [ 'carl@dev1', 'carl', ], online_status: { type: 'ONLINE', }, user_id: ext(2, 0x109b68ba5cdf428ea0017fc6bcc04d4b), }, ]
            //   claimer_email: 'alice@dev1'
            //   created_by: { type: 'EXTERNAL_SERVICE', service_name: 'LDAP', }
            &hex!(
            "85a6737461747573a26f6ba474797065a455534552ae61646d696e6973747261746f72"
            "739283ac68756d616e5f68616e646c6592a8626f624064657631a3626f62ad6f6e6c69"
            "6e655f73746174757381a474797065a7554e4b4e4f574ea7757365725f6964d802109b"
            "68ba5cdf428ea0017fc6bcc04d4a83ac68756d616e5f68616e646c6592a96361726c40"
            "64657631a46361726cad6f6e6c696e655f73746174757381a474797065a64f4e4c494e"
            "45a7757365725f6964d802109b68ba5cdf428ea0017fc6bcc04d4bad636c61696d6572"
            "5f656d61696caa616c6963654064657631aa637265617465645f627982a474797065b0"
            "45585445524e414c5f53455256494345ac736572766963655f6e616d65a44c444150"
            )[..],
            invited_cmds::invite_info::Rep::Ok(invited_cmds::invite_info::InvitationType::User {
                claimer_email: "alice@dev1".to_owned(),
                created_by: invited_cmds::invite_info::UserInvitationCreatedBy::ExternalService {
                    service_name: "LDAP".to_owned(),
                },
                administrators: vec![
                    invited_cmds::invite_info::InviteInfoAdministrator {
                        human_handle: HumanHandle::new("bob@dev1", "bob").unwrap(),
                        user_id: UserID::from_hex("109b68ba5cdf428ea0017fc6bcc04d4a").unwrap(),
                        online_status: invited_cmds::invite_info::UserOnlineStatus::Unknown,
                    },
                    invited_cmds::invite_info::InviteInfoAdministrator {
                        human_handle: HumanHandle::new("carl@dev1", "carl").unwrap(),
                        user_id: UserID::from_hex("109b68ba5cdf428ea0017fc6bcc04d4b").unwrap(),
                        online_status: invited_cmds::invite_info::UserOnlineStatus::Online,
                    },
                ],
            }),
        ),
        (
            // Generated from Parsec 3.2.4-a.0+dev
            // Content:
            //   status: 'ok'
            //   type: 'DEVICE'
            //   claimer_human_handle: [ 'bob@dev1', 'bob', ]
            //   claimer_user_id: ext(2, 0x109b68ba5cdf428ea0017fc6bcc04d4a)
            &hex!(
                "84a6737461747573a26f6ba474797065a6444556494345b4636c61696d65725f68756d"
                "616e5f68616e646c6592a8626f624064657631a3626f62af636c61696d65725f757365"
                "725f6964d802109b68ba5cdf428ea0017fc6bcc04d4a"
            )[..],
            invited_cmds::invite_info::Rep::Ok(invited_cmds::invite_info::InvitationType::Device {
                claimer_user_id: UserID::from_hex("109b68ba5cdf428ea0017fc6bcc04d4a").unwrap(),
                claimer_human_handle: HumanHandle::new("bob@dev1", "bob").unwrap(),
            }),
        ),
        (
            // Generated from Parsec 3.2.4-a.0+dev
            // Content:
            //   status: 'ok'
            //   type: 'SHAMIR_RECOVERY'
            //   claimer_human_handle: [ 'carl@example.com', 'carl', ]
            //   claimer_user_id: ext(2, 0x109b68ba5cdf428ea0017fc6bcc04d4c)
            //   recipients: [ { human_handle: [ 'alice@example.com', 'alice', ], online_status: { type: 'ONLINE', }, revoked_on: None, shares: 1, user_id: ext(2, 0x109b68ba5cdf428ea0017fc6bcc04d4a), }, { human_handle: [ 'bob@example.com', 'bob', ], online_status: { type: 'UNKNOWN', }, revoked_on: ext(1, 946774800000000) i.e. 2000-01-02T02:00:00Z, shares: 1, user_id: ext(2, 0x109b68ba5cdf428ea0017fc6bcc04d4b), }, ]
            //   shamir_recovery_created_on: ext(1, 946688400000000) i.e. 2000-01-01T02:00:00Z
            //   threshold: 2
            &hex!(
                "87a6737461747573a26f6ba474797065af5348414d49525f5245434f56455259b4636c"
                "61696d65725f68756d616e5f68616e646c6592b06361726c406578616d706c652e636f"
                "6da46361726caf636c61696d65725f757365725f6964d802109b68ba5cdf428ea0017f"
                "c6bcc04d4caa726563697069656e74739285ac68756d616e5f68616e646c6592b1616c"
                "696365406578616d706c652e636f6da5616c696365ad6f6e6c696e655f737461747573"
                "81a474797065a64f4e4c494e45aa7265766f6b65645f6f6ec0a673686172657301a775"
                "7365725f6964d802109b68ba5cdf428ea0017fc6bcc04d4a85ac68756d616e5f68616e"
                "646c6592af626f62406578616d706c652e636f6da3626f62ad6f6e6c696e655f737461"
                "74757381a474797065a7554e4b4e4f574eaa7265766f6b65645f6f6ed70100035d162f"
                "a2e400a673686172657301a7757365725f6964d802109b68ba5cdf428ea0017fc6bcc0"
                "4d4bba7368616d69725f7265636f766572795f637265617465645f6f6ed70100035d02"
                "11cb8400a97468726573686f6c6402"
            )[..],
            invited_cmds::invite_info::Rep::Ok(
                invited_cmds::invite_info::InvitationType::ShamirRecovery {
                    claimer_user_id: UserID::from_hex("109b68ba5cdf428ea0017fc6bcc04d4c").unwrap(),
                    claimer_human_handle: HumanHandle::new("carl@example.com", "carl").unwrap(),
                    shamir_recovery_created_on: "2000-1-1T01:00:00Z".parse().unwrap(),
                    recipients: vec![
                        invited_cmds::invite_info::ShamirRecoveryRecipient {
                            user_id: UserID::from_hex("109b68ba5cdf428ea0017fc6bcc04d4a").unwrap(),
                            human_handle: HumanHandle::new("alice@example.com", "alice").unwrap(),
                            shares: 1.try_into().unwrap(),
                            revoked_on: None,
                            online_status: invited_cmds::invite_info::UserOnlineStatus::Online,
                        },
                        invited_cmds::invite_info::ShamirRecoveryRecipient {
                            user_id: UserID::from_hex("109b68ba5cdf428ea0017fc6bcc04d4b").unwrap(),
                            human_handle: HumanHandle::new("bob@example.com", "bob").unwrap(),
                            shares: 1.try_into().unwrap(),
                            revoked_on: Some("2000-1-2T01:00:00Z".parse().unwrap()),
                            online_status: invited_cmds::invite_info::UserOnlineStatus::Unknown,
                        },
                    ],
                    threshold: 2.try_into().unwrap(),
                },
            ),
        ),
    ];

    for (raw, expected) in raw_expected {
        println!("***expected: {:?}", expected.dump().unwrap());
        let data = invited_cmds::invite_info::Rep::load(raw).unwrap();
        assert_eq!(data, expected);

        // Also test serialization round trip
        let raw2 = data.dump().unwrap();

        let data2 = invited_cmds::invite_info::Rep::load(&raw2).unwrap();

        assert_eq!(data2, expected);
    }
}
