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
            //   administrators: [
            //     human_handle: [ 'bob@dev1', 'bob', ], online_status: 'UNKNOWN', user_id: ext(2, 0x109b68ba5cdf428ea0017fc6bcc04d4a), last_greeting_attempt_joined_on: None
            //     human_handle: [ 'carl@dev1', 'carl', ], online_status: 'ONLINE', user_id: ext(2, 0x109b68ba5cdf428ea0017fc6bcc04d4b), last_greeting_attempt_joined_on: ext(1, 946774800000000) i.e. 2000-01-02T02:00:00Z
            //   ]
            //   claimer_email: 'alice@dev1'
            //   created_by: { type: 'USER', human_handle: [ 'bob@dev1', 'bob', ], user_id: ext(2, 0x109b68ba5cdf428ea0017fc6bcc04d4a), }
            &hex!(
                "85a6737461747573a26f6ba474797065a455534552ae61646d696e6973747261746f72"
                "739284ac68756d616e5f68616e646c6592a8626f624064657631a3626f62bf6c617374"
                "5f6772656574696e675f617474656d70745f6a6f696e65645f6f6ec0ad6f6e6c696e65"
                "5f737461747573a7554e4b4e4f574ea7757365725f6964d802109b68ba5cdf428ea001"
                "7fc6bcc04d4a84ac68756d616e5f68616e646c6592a96361726c4064657631a4636172"
                "6cbf6c6173745f6772656574696e675f617474656d70745f6a6f696e65645f6f6ed701"
                "00035d162fa2e400ad6f6e6c696e655f737461747573a64f4e4c494e45a7757365725f"
                "6964d802109b68ba5cdf428ea0017fc6bcc04d4bad636c61696d65725f656d61696caa"
                "616c6963654064657631aa637265617465645f627983a474797065a455534552ac6875"
                "6d616e5f68616e646c6592a8626f624064657631a3626f62a7757365725f6964d80210"
                "9b68ba5cdf428ea0017fc6bcc04d4a"
            )[..],
            invited_cmds::invite_info::Rep::Ok(invited_cmds::invite_info::InvitationType::User {
                claimer_email: "alice@dev1".to_owned(),
                created_by: invited_cmds::invite_info::InvitationCreatedBy::User {
                    human_handle: HumanHandle::new("bob@dev1", "bob").unwrap(),
                    user_id: UserID::from_hex("109b68ba5cdf428ea0017fc6bcc04d4a").unwrap(),
                },
                administrators: vec![
                    invited_cmds::invite_info::UserGreetingAdministrator {
                        human_handle: HumanHandle::new("bob@dev1", "bob").unwrap(),
                        user_id: UserID::from_hex("109b68ba5cdf428ea0017fc6bcc04d4a").unwrap(),
                        online_status: invited_cmds::invite_info::UserOnlineStatus::Unknown,
                        last_greeting_attempt_joined_on: None,
                    },
                    invited_cmds::invite_info::UserGreetingAdministrator {
                        human_handle: HumanHandle::new("carl@dev1", "carl").unwrap(),
                        user_id: UserID::from_hex("109b68ba5cdf428ea0017fc6bcc04d4b").unwrap(),
                        online_status: invited_cmds::invite_info::UserOnlineStatus::Online,
                        last_greeting_attempt_joined_on: Some(
                            "2000-1-2T01:00:00Z".parse().unwrap(),
                        ),
                    },
                ],
            }),
        ),
        (
            // Generated from Parsec 3.2.4-a.0+dev
            // Content:
            //   status: 'ok'
            //   type: 'USER'
            //   administrators: [
            //     human_handle: [ 'bob@dev1', 'bob', ], online_status: 'UNKNOWN', user_id: ext(2, 0x109b68ba5cdf428ea0017fc6bcc04d4a), last_greeting_attempt_joined_on: None
            //     human_handle: [ 'carl@dev1', 'carl', ], online_status: 'ONLINE', user_id: ext(2, 0x109b68ba5cdf428ea0017fc6bcc04d4b), last_greeting_attempt_joined_on: ext(1, 946774800000000) i.e. 2000-01-02T02:00:00Z
            //   ]
            //   claimer_email: 'alice@dev1'
            //   created_by: { type: 'EXTERNAL_SERVICE', service_label: 'LDAP', }
            &hex!(
                "85a6737461747573a26f6ba474797065a455534552ae61646d696e6973747261746f72"
                "739284ac68756d616e5f68616e646c6592a8626f624064657631a3626f62bf6c617374"
                "5f6772656574696e675f617474656d70745f6a6f696e65645f6f6ec0ad6f6e6c696e65"
                "5f737461747573a7554e4b4e4f574ea7757365725f6964d802109b68ba5cdf428ea001"
                "7fc6bcc04d4a84ac68756d616e5f68616e646c6592a96361726c4064657631a4636172"
                "6cbf6c6173745f6772656574696e675f617474656d70745f6a6f696e65645f6f6ed701"
                "00035d162fa2e400ad6f6e6c696e655f737461747573a64f4e4c494e45a7757365725f"
                "6964d802109b68ba5cdf428ea0017fc6bcc04d4bad636c61696d65725f656d61696caa"
                "616c6963654064657631aa637265617465645f627982a474797065b045585445524e41"
                "4c5f53455256494345ad736572766963655f6c6162656ca44c444150"
            )[..],
            invited_cmds::invite_info::Rep::Ok(invited_cmds::invite_info::InvitationType::User {
                claimer_email: "alice@dev1".to_owned(),
                created_by: invited_cmds::invite_info::InvitationCreatedBy::ExternalService {
                    service_label: "LDAP".to_owned(),
                },
                administrators: vec![
                    invited_cmds::invite_info::UserGreetingAdministrator {
                        human_handle: HumanHandle::new("bob@dev1", "bob").unwrap(),
                        user_id: UserID::from_hex("109b68ba5cdf428ea0017fc6bcc04d4a").unwrap(),
                        online_status: invited_cmds::invite_info::UserOnlineStatus::Unknown,
                        last_greeting_attempt_joined_on: None,
                    },
                    invited_cmds::invite_info::UserGreetingAdministrator {
                        human_handle: HumanHandle::new("carl@dev1", "carl").unwrap(),
                        user_id: UserID::from_hex("109b68ba5cdf428ea0017fc6bcc04d4b").unwrap(),
                        online_status: invited_cmds::invite_info::UserOnlineStatus::Online,
                        last_greeting_attempt_joined_on: Some(
                            "2000-1-2T01:00:00Z".parse().unwrap(),
                        ),
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
            //   created_by: { type: 'USER', human_handle: [ 'bob@dev1', 'bob', ], user_id: ext(2, 0x109b68ba5cdf428ea0017fc6bcc04d4a), }
            &hex!(
                "85a6737461747573a26f6ba474797065a6444556494345b4636c61696d65725f68756d"
                "616e5f68616e646c6592a8626f624064657631a3626f62af636c61696d65725f757365"
                "725f6964d802109b68ba5cdf428ea0017fc6bcc04d4aaa637265617465645f627983a4"
                "74797065a455534552ac68756d616e5f68616e646c6592a8626f624064657631a3626f"
                "62a7757365725f6964d802109b68ba5cdf428ea0017fc6bcc04d4a"
            )[..],
            invited_cmds::invite_info::Rep::Ok(invited_cmds::invite_info::InvitationType::Device {
                claimer_user_id: UserID::from_hex("109b68ba5cdf428ea0017fc6bcc04d4a").unwrap(),
                claimer_human_handle: HumanHandle::new("bob@dev1", "bob").unwrap(),
                created_by: invited_cmds::invite_info::InvitationCreatedBy::User {
                    human_handle: HumanHandle::new("bob@dev1", "bob").unwrap(),
                    user_id: UserID::from_hex("109b68ba5cdf428ea0017fc6bcc04d4a").unwrap(),
                },
            }),
        ),
        (
            // Generated from Parsec 3.2.4-a.0+dev
            // Content:
            //   status: 'ok'
            //   type: 'SHAMIR_RECOVERY'
            //   claimer_human_handle: [ 'carl@example.com', 'carl', ]
            //   claimer_user_id: ext(2, 0x109b68ba5cdf428ea0017fc6bcc04d4c)
            //   created_by: { type: 'USER', human_handle: [ 'bob@dev1', 'bob', ], user_id: ext(2, 0x109b68ba5cdf428ea0017fc6bcc04d4a), }
            //   recipients: [ { human_handle: [ 'alice@example.com', 'alice', ], online_status: 'ONLINE', revoked_on: None, shares: 1, user_id: ext(2, 0x109b68ba5cdf428ea0017fc6bcc04d4a), }, { human_handle: [ 'bob@example.com', 'bob', ], online_status: 'UNKNOWN', revoked_on: ext(1, 946774800000000) i.e. 2000-01-02T02:00:00Z, shares: 1, user_id: ext(2, 0x109b68ba5cdf428ea0017fc6bcc04d4b), }, ]
            //   shamir_recovery_created_on: ext(1, 946688400000000) i.e. 2000-01-01T02:00:00Z
            //   threshold: 2
            &hex!(
                "88a6737461747573a26f6ba474797065af5348414d49525f5245434f56455259b4636c"
                "61696d65725f68756d616e5f68616e646c6592b06361726c406578616d706c652e636f"
                "6da46361726caf636c61696d65725f757365725f6964d802109b68ba5cdf428ea0017f"
                "c6bcc04d4caa637265617465645f627983a474797065a455534552ac68756d616e5f68"
                "616e646c6592a8626f624064657631a3626f62a7757365725f6964d802109b68ba5cdf"
                "428ea0017fc6bcc04d4aaa726563697069656e74739285ac68756d616e5f68616e646c"
                "6592b1616c696365406578616d706c652e636f6da5616c696365ad6f6e6c696e655f73"
                "7461747573a64f4e4c494e45aa7265766f6b65645f6f6ec0a673686172657301a77573"
                "65725f6964d802109b68ba5cdf428ea0017fc6bcc04d4a85ac68756d616e5f68616e64"
                "6c6592af626f62406578616d706c652e636f6da3626f62ad6f6e6c696e655f73746174"
                "7573a7554e4b4e4f574eaa7265766f6b65645f6f6ed70100035d162fa2e400a6736861"
                "72657301a7757365725f6964d802109b68ba5cdf428ea0017fc6bcc04d4bba7368616d"
                "69725f7265636f766572795f637265617465645f6f6ed70100035d0211cb8400a97468"
                "726573686f6c6402"
            )[..],
            invited_cmds::invite_info::Rep::Ok(
                invited_cmds::invite_info::InvitationType::ShamirRecovery {
                    claimer_user_id: UserID::from_hex("109b68ba5cdf428ea0017fc6bcc04d4c").unwrap(),
                    claimer_human_handle: HumanHandle::new("carl@example.com", "carl").unwrap(),
                    created_by: invited_cmds::invite_info::InvitationCreatedBy::User {
                        human_handle: HumanHandle::new("bob@dev1", "bob").unwrap(),
                        user_id: UserID::from_hex("109b68ba5cdf428ea0017fc6bcc04d4a").unwrap(),
                    },
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
