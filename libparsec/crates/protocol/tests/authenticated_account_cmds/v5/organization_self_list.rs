// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

use super::authenticated_account_cmds;

// Request

pub fn req() {
    // Generated from Parsec 3.4.1-a.0+dev
    // Content:
    //   cmd: 'organization_self_list'
    let raw: &[u8] = hex!("81a3636d64b66f7267616e697a6174696f6e5f73656c665f6c697374").as_ref();

    let req = authenticated_account_cmds::organization_self_list::Req {};
    println!("***expected: {:?}", req.dump().unwrap());

    let expected = authenticated_account_cmds::AnyCmdReq::OrganizationSelfList(req.clone());
    let data = authenticated_account_cmds::AnyCmdReq::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = req.dump().unwrap();

    let data2 = authenticated_account_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    let raw_expected = [
        (
            // Generated from Parsec 3.4.1-a.0+dev
            // Content:
            //   status: 'ok'
            //   active: [ ]
            //   revoked: [ ]
            hex!("83a6737461747573a26f6ba661637469766590a77265766f6b656490").as_ref(),
            authenticated_account_cmds::organization_self_list::Rep::Ok {
                active: vec![],
                revoked: vec![],
            },
        ),
        (
            // Legacy API<5.3 format (with `allowed_client_agent/account_vault_strategy` fields`)
            // Generated from Parsec 3.4.1-a.0+dev
            // Content:
            //   status: 'ok'
            //   active: [
            //     {
            //       created_on: ext(1, 946684800000000) i.e. 2000-01-01T01:00:00Z,
            //       current_profile: 'ADMIN',
            //       is_frozen: False,
            //       organization_config: {
            //         account_vault_strategy: 'ALLOWED',
            //         active_users_limit: None,
            //         allowed_client_agent: 'NATIVE_OR_WEB',
            //         is_expired: False,
            //         user_profile_outsider_allowed: True,
            //       },
            //       organization_id: 'Org1',
            //       user_id: ext(2, 0xfb9973ce931a4a4a92ee15cead75dc9e),
            //     },
            //     {
            //       created_on: ext(1, 946684800000000) i.e. 2000-01-01T01:00:00Z,
            //       current_profile: 'STANDARD',
            //       is_frozen: True,
            //       organization_config: {
            //         account_vault_strategy: 'FORBIDDEN',
            //         active_users_limit: 8,
            //         allowed_client_agent: 'NATIVE_ONLY',
            //         is_expired: True,
            //         user_profile_outsider_allowed: False,
            //       },
            //       organization_id: 'Org2',
            //       user_id: ext(2, 0x956ff42e313c42f48cc5e54fc9ce64ba),
            //     },
            //   ]
            //   revoked: [
            //     {
            //       created_on: ext(1, 946684800000000) i.e. 2000-01-01T01:00:00Z,
            //       current_profile: 'OUTSIDER',
            //       organization_id: 'Org3',
            //       revoked_on: ext(1, 946771200000000) i.e. 2000-01-02T01:00:00Z,
            //       user_id: ext(2, 0x5c5bb5498d334006b6dfe15f9ed8922b),
            //     },
            //   ]
            hex!(
                "83a6737461747573a26f6ba66163746976659286aa637265617465645f6f6ed7010003"
                "5d013b37e000af63757272656e745f70726f66696c65a541444d494ea969735f66726f"
                "7a656ec2b36f7267616e697a6174696f6e5f636f6e66696785b66163636f756e745f76"
                "61756c745f7374726174656779a7414c4c4f574544b26163746976655f75736572735f"
                "6c696d6974c0b4616c6c6f7765645f636c69656e745f6167656e74ad4e41544956455f"
                "4f525f574542aa69735f65787069726564c2bd757365725f70726f66696c655f6f7574"
                "73696465725f616c6c6f776564c3af6f7267616e697a6174696f6e5f6964a44f726731"
                "a7757365725f6964d802fb9973ce931a4a4a92ee15cead75dc9e86aa63726561746564"
                "5f6f6ed70100035d013b37e000af63757272656e745f70726f66696c65a85354414e44"
                "415244a969735f66726f7a656ec3b36f7267616e697a6174696f6e5f636f6e66696785"
                "b66163636f756e745f7661756c745f7374726174656779a9464f5242494444454eb261"
                "63746976655f75736572735f6c696d697408b4616c6c6f7765645f636c69656e745f61"
                "67656e74ab4e41544956455f4f4e4c59aa69735f65787069726564c3bd757365725f70"
                "726f66696c655f6f757473696465725f616c6c6f776564c2af6f7267616e697a617469"
                "6f6e5f6964a44f726732a7757365725f6964d802956ff42e313c42f48cc5e54fc9ce64"
                "baa77265766f6b65649185aa637265617465645f6f6ed70100035d013b37e000af6375"
                "7272656e745f70726f66696c65a84f55545349444552af6f7267616e697a6174696f6e"
                "5f6964a44f726733aa7265766f6b65645f6f6ed70100035d15590f4000a7757365725f"
                "6964d8025c5bb5498d334006b6dfe15f9ed8922b"
            )
            .as_ref(),
            authenticated_account_cmds::organization_self_list::Rep::Ok {
                active: vec![
                    authenticated_account_cmds::organization_self_list::ActiveUser {
                        organization_id: "Org1".parse().unwrap(),
                        user_id: UserID::from_hex("fb9973ce931a4a4a92ee15cead75dc9e").unwrap(),
                        created_on: "2000-01-01T00:00:00Z".parse().unwrap(),
                        is_frozen: false,
                        current_profile: UserProfile::Admin,
                        organization_config:
                            authenticated_account_cmds::organization_self_list::OrganizationConfig {
                                is_expired: false,
                                user_profile_outsider_allowed: true,
                                active_users_limit: ActiveUsersLimit::NoLimit,
                            },
                    },
                    authenticated_account_cmds::organization_self_list::ActiveUser {
                        organization_id: "Org2".parse().unwrap(),
                        user_id: UserID::from_hex("956ff42e313c42f48cc5e54fc9ce64ba").unwrap(),
                        created_on: "2000-01-01T00:00:00Z".parse().unwrap(),
                        is_frozen: true,
                        current_profile: UserProfile::Standard,
                        organization_config:
                            authenticated_account_cmds::organization_self_list::OrganizationConfig {
                                is_expired: true,
                                user_profile_outsider_allowed: false,
                                active_users_limit: ActiveUsersLimit::LimitedTo(8),
                            },
                    },
                ],
                revoked: vec![
                    authenticated_account_cmds::organization_self_list::RevokedUser {
                        organization_id: "Org3".parse().unwrap(),
                        user_id: UserID::from_hex("5c5bb5498d334006b6dfe15f9ed8922b").unwrap(),
                        created_on: "2000-01-01T00:00:00Z".parse().unwrap(),
                        revoked_on: "2000-01-02T00:00:00Z".parse().unwrap(),
                        current_profile: UserProfile::Outsider,
                    },
                ],
            },
        ),
        (
            // Generated from Parsec 3.5.3-a.0+dev
            // Content:
            //   status: 'ok'
            //   active: [
            //     {
            //       created_on: ext(1, 946684800000000) i.e. 2000-01-01T01:00:00Z,
            //       current_profile: 'ADMIN',
            //       is_frozen: False,
            //       organization_config: {
            //         active_users_limit: None,
            //         is_expired: False,
            //         user_profile_outsider_allowed: True,
            //       },
            //       organization_id: 'Org1',
            //       user_id: ext(2, 0xfb9973ce931a4a4a92ee15cead75dc9e),
            //     },
            //     {
            //       created_on: ext(1, 946684800000000) i.e. 2000-01-01T01:00:00Z,
            //       current_profile: 'STANDARD',
            //       is_frozen: True,
            //       organization_config: {
            //         active_users_limit: 8,
            //         is_expired: True,
            //         user_profile_outsider_allowed: False,
            //       },
            //       organization_id: 'Org2',
            //       user_id: ext(2, 0x956ff42e313c42f48cc5e54fc9ce64ba),
            //     },
            //   ]
            //   revoked: [
            //     {
            //       created_on: ext(1, 946684800000000) i.e. 2000-01-01T01:00:00Z,
            //       current_profile: 'OUTSIDER',
            //       organization_id: 'Org3',
            //       revoked_on: ext(1, 946771200000000) i.e. 2000-01-02T01:00:00Z,
            //       user_id: ext(2, 0x5c5bb5498d334006b6dfe15f9ed8922b),
            //     },
            //   ]
            hex!(
                "83a6737461747573a26f6ba66163746976659286aa637265617465645f6f6ed7010003"
                "5d013b37e000af63757272656e745f70726f66696c65a541444d494ea969735f66726f"
                "7a656ec2b36f7267616e697a6174696f6e5f636f6e66696783b26163746976655f7573"
                "6572735f6c696d6974c0aa69735f65787069726564c2bd757365725f70726f66696c65"
                "5f6f757473696465725f616c6c6f776564c3af6f7267616e697a6174696f6e5f6964a4"
                "4f726731a7757365725f6964d802fb9973ce931a4a4a92ee15cead75dc9e86aa637265"
                "617465645f6f6ed70100035d013b37e000af63757272656e745f70726f66696c65a853"
                "54414e44415244a969735f66726f7a656ec3b36f7267616e697a6174696f6e5f636f6e"
                "66696783b26163746976655f75736572735f6c696d697408aa69735f65787069726564"
                "c3bd757365725f70726f66696c655f6f757473696465725f616c6c6f776564c2af6f72"
                "67616e697a6174696f6e5f6964a44f726732a7757365725f6964d802956ff42e313c42"
                "f48cc5e54fc9ce64baa77265766f6b65649185aa637265617465645f6f6ed70100035d"
                "013b37e000af63757272656e745f70726f66696c65a84f55545349444552af6f726761"
                "6e697a6174696f6e5f6964a44f726733aa7265766f6b65645f6f6ed70100035d15590f"
                "4000a7757365725f6964d8025c5bb5498d334006b6dfe15f9ed8922b"
            )
            .as_ref(),
            authenticated_account_cmds::organization_self_list::Rep::Ok {
                active: vec![
                    authenticated_account_cmds::organization_self_list::ActiveUser {
                        organization_id: "Org1".parse().unwrap(),
                        user_id: UserID::from_hex("fb9973ce931a4a4a92ee15cead75dc9e").unwrap(),
                        created_on: "2000-01-01T00:00:00Z".parse().unwrap(),
                        is_frozen: false,
                        current_profile: UserProfile::Admin,
                        organization_config:
                            authenticated_account_cmds::organization_self_list::OrganizationConfig {
                                is_expired: false,
                                user_profile_outsider_allowed: true,
                                active_users_limit: ActiveUsersLimit::NoLimit,
                            },
                    },
                    authenticated_account_cmds::organization_self_list::ActiveUser {
                        organization_id: "Org2".parse().unwrap(),
                        user_id: UserID::from_hex("956ff42e313c42f48cc5e54fc9ce64ba").unwrap(),
                        created_on: "2000-01-01T00:00:00Z".parse().unwrap(),
                        is_frozen: true,
                        current_profile: UserProfile::Standard,
                        organization_config:
                            authenticated_account_cmds::organization_self_list::OrganizationConfig {
                                is_expired: true,
                                user_profile_outsider_allowed: false,
                                active_users_limit: ActiveUsersLimit::LimitedTo(8),
                            },
                    },
                ],
                revoked: vec![
                    authenticated_account_cmds::organization_self_list::RevokedUser {
                        organization_id: "Org3".parse().unwrap(),
                        user_id: UserID::from_hex("5c5bb5498d334006b6dfe15f9ed8922b").unwrap(),
                        created_on: "2000-01-01T00:00:00Z".parse().unwrap(),
                        revoked_on: "2000-01-02T00:00:00Z".parse().unwrap(),
                        current_profile: UserProfile::Outsider,
                    },
                ],
            },
        ),
    ];

    for (raw, expected) in raw_expected {
        println!("***expected: {:?}", expected.dump().unwrap());
        let data = authenticated_account_cmds::organization_self_list::Rep::load(raw).unwrap();

        p_assert_eq!(data, expected);

        // Also test serialization round trip
        let raw2 = data.dump().unwrap();

        let data2 = authenticated_account_cmds::organization_self_list::Rep::load(&raw2).unwrap();

        p_assert_eq!(data2, expected);
    }
}
