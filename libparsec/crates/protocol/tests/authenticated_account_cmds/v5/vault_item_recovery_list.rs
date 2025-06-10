// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use std::collections::HashMap;

use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

use super::authenticated_account_cmds;

// Request

pub fn req() {
    // Generated from Parsec 3.4.0-a.7+dev
    // Content:
    //   cmd: 'vault_item_recovery_list'
    let raw: &[u8] = hex!("81a3636d64b87661756c745f6974656d5f7265636f766572795f6c697374").as_ref();

    let req = authenticated_account_cmds::vault_item_recovery_list::Req {};

    let expected = authenticated_account_cmds::AnyCmdReq::VaultItemRecoveryList(req.clone());
    println!("***expected: {:?}", req.dump().unwrap());
    let data = authenticated_account_cmds::AnyCmdReq::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = req.dump().unwrap();

    let data2 = authenticated_account_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    // Generated from Parsec 3.4.0-a.7+dev
    // Content:
    //   status: 'ok'
    //   current_vault: {
    //     auth_methods: [
    //       {
    //         created_by_ip: '127.0.0.4',
    //         created_by_user_agent: 'Parsec-Client/3.4.4 Windows',
    //         created_on: ext(1, 946944000000000) i.e. 2000-01-04T01:00:00Z,
    //         disabled_on: None,
    //         password_algorithm: { type: 'ARGON2ID', memlimit_kb: 3, opslimit: 65536, parallelism: 1, salt: 0x3c73616c7420343e, },
    //         vault_key_access: 0x3c7661756c745f6b65795f61636365737320343e,
    //       },
    //     ],
    //     vault_items: {
    //       0x546e2c936f85bc9994e31361a58910b379b6292ff6dd465bb803629b9c75c5ca: 0x646174612033,
    //       0xe37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6: 0x646174612032,
    //     },
    //   }
    //   previous_vaults: [
    //     {
    //       auth_methods: [
    //         {
    //           created_by_ip: '127.0.0.1',
    //           created_by_user_agent: 'Parsec-Client/3.4.1 Windows',
    //           created_on: ext(1, 946684800000000) i.e. 2000-01-01T01:00:00Z,
    //           disabled_on: ext(1, 946771200000000) i.e. 2000-01-02T01:00:00Z,
    //           password_algorithm: { type: 'ARGON2ID', memlimit_kb: 3, opslimit: 65536, parallelism: 1, salt: 0x3c73616c7420313e, },
    //           vault_key_access: 0x3c7661756c745f6b65795f61636365737320313e,
    //         },
    //       ],
    //       vault_items: { },
    //     },
    //     {
    //       auth_methods: [
    //         {
    //           created_by_ip: '127.0.0.2',
    //           created_by_user_agent: 'Parsec-Client/3.4.2 Windows',
    //           created_on: ext(1, 946771200000000) i.e. 2000-01-02T01:00:00Z,
    //           disabled_on: ext(1, 946944000000000) i.e. 2000-01-04T01:00:00Z,
    //           password_algorithm: { type: 'ARGON2ID', memlimit_kb: 3, opslimit: 65536, parallelism: 1, salt: 0x3c73616c7420323e, },
    //           vault_key_access: 0x3c7661756c745f6b65795f61636365737320323e,
    //         },
    //         {
    //           created_by_ip: '',
    //           created_by_user_agent: 'Parsec-Client/3.4.1 Windows',
    //           created_on: ext(1, 946857600000000) i.e. 2000-01-03T01:00:00Z,
    //           disabled_on: ext(1, 946944000000000) i.e. 2000-01-04T01:00:00Z,
    //           password_algorithm: None,
    //           vault_key_access: 0x3c7661756c745f6b65795f61636365737320313e,
    //         },
    //       ],
    //       vault_items: {
    //         0x076a27c79e5ace2a3d47f9dd2e83e4ff6ea8872b3c2218f66c92b89b55f36560: 0x646174612031,
    //         0xe37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6: 0x646174612032,
    //       },
    //     },
    //   ]
    let raw: &[u8] = hex!(
        "83a6737461747573a26f6bad63757272656e745f7661756c7482ac617574685f6d6574"
        "686f64739186ad637265617465645f62795f6970a93132372e302e302e34b563726561"
        "7465645f62795f757365725f6167656e74bb5061727365632d436c69656e742f332e34"
        "2e342057696e646f7773aa637265617465645f6f6ed70100035d3d94be0000ab646973"
        "61626c65645f6f6ec0b270617373776f72645f616c676f726974686d85a474797065a8"
        "4152474f4e324944ab6d656d6c696d69745f6b6203a86f70736c696d6974ce00010000"
        "ab706172616c6c656c69736d01a473616c74c4083c73616c7420343eb07661756c745f"
        "6b65795f616363657373c4143c7661756c745f6b65795f61636365737320343eab7661"
        "756c745f6974656d7382c420e37ce3b00a1f15b3de62029972345420b76313a885c6cc"
        "c6e3b5547857b3ecc6c406646174612032c420546e2c936f85bc9994e31361a58910b3"
        "79b6292ff6dd465bb803629b9c75c5cac406646174612033af70726576696f75735f76"
        "61756c74739282ac617574685f6d6574686f64739186ad637265617465645f62795f69"
        "70a93132372e302e302e31b5637265617465645f62795f757365725f6167656e74bb50"
        "61727365632d436c69656e742f332e342e312057696e646f7773aa637265617465645f"
        "6f6ed70100035d013b37e000ab64697361626c65645f6f6ed70100035d15590f4000b2"
        "70617373776f72645f616c676f726974686d85a474797065a84152474f4e324944ab6d"
        "656d6c696d69745f6b6203a86f70736c696d6974ce00010000ab706172616c6c656c69"
        "736d01a473616c74c4083c73616c7420313eb07661756c745f6b65795f616363657373"
        "c4143c7661756c745f6b65795f61636365737320313eab7661756c745f6974656d7380"
        "82ac617574685f6d6574686f64739286ad637265617465645f62795f6970a93132372e"
        "302e302e32b5637265617465645f62795f757365725f6167656e74bb5061727365632d"
        "436c69656e742f332e342e322057696e646f7773aa637265617465645f6f6ed7010003"
        "5d15590f4000ab64697361626c65645f6f6ed70100035d3d94be0000b270617373776f"
        "72645f616c676f726974686d85a474797065a84152474f4e324944ab6d656d6c696d69"
        "745f6b6203a86f70736c696d6974ce00010000ab706172616c6c656c69736d01a47361"
        "6c74c4083c73616c7420323eb07661756c745f6b65795f616363657373c4143c766175"
        "6c745f6b65795f61636365737320323e86ad637265617465645f62795f6970a0b56372"
        "65617465645f62795f757365725f6167656e74bb5061727365632d436c69656e742f33"
        "2e342e312057696e646f7773aa637265617465645f6f6ed70100035d2976e6a000ab64"
        "697361626c65645f6f6ed70100035d3d94be0000b270617373776f72645f616c676f72"
        "6974686dc0b07661756c745f6b65795f616363657373c4143c7661756c745f6b65795f"
        "61636365737320313eab7661756c745f6974656d7382c420076a27c79e5ace2a3d47f9"
        "dd2e83e4ff6ea8872b3c2218f66c92b89b55f36560c406646174612031c420e37ce3b0"
        "0a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6c4066461746120"
        "32"
    )
    .as_ref();

    let expected = authenticated_account_cmds::vault_item_recovery_list::Rep::Ok {
        current_vault: authenticated_account_cmds::vault_item_recovery_list::VaultItemRecoveryVault {
            auth_methods: vec![
                authenticated_account_cmds::vault_item_recovery_list::VaultItemRecoveryAuthMethod {
                    created_on: "2000-01-04T00:00:00Z".parse().unwrap(),
                    disabled_on: None,
                    created_by_ip: "127.0.0.4".to_string(),
                    created_by_user_agent: "Parsec-Client/3.4.4 Windows".to_string(),
                    vault_key_access: b"<vault_key_access 4>".as_ref().into(),
                    password_algorithm: Some(PasswordAlgorithm::Argon2id {
                        salt: b"<salt 4>".to_vec(),
                        opslimit: 65536,
                        memlimit_kb: 3,
                        parallelism: 1,
                    }),
                }
            ],
            vault_items: HashMap::from_iter([
                    (
                        HashDigest::from(hex!(
                            "546e2c936f85bc9994e31361a58910b379b6292ff6dd465bb803629b9c75c5ca"
                        )),
                        b"data 3".as_ref().into()
                    ),
                    (
                        HashDigest::from(hex!(
                            "e37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6"
                        )),
                        b"data 2".as_ref().into()
                    ),
            ]),
        },
        previous_vaults: vec![
            authenticated_account_cmds::vault_item_recovery_list::VaultItemRecoveryVault {
                auth_methods: vec![
                    authenticated_account_cmds::vault_item_recovery_list::VaultItemRecoveryAuthMethod {
                        created_on: "2000-01-01T00:00:00Z".parse().unwrap(),
                        disabled_on: Some("2000-01-02T00:00:00Z".parse().unwrap()),
                        created_by_ip: "127.0.0.1".to_string(),
                        created_by_user_agent: "Parsec-Client/3.4.1 Windows".to_string(),
                        vault_key_access: b"<vault_key_access 1>".as_ref().into(),
                        password_algorithm: Some(PasswordAlgorithm::Argon2id {
                            salt: b"<salt 1>".to_vec(),
                            opslimit: 65536,
                            memlimit_kb: 3,
                            parallelism: 1,
                        })
                    },
                ],
                vault_items: HashMap::from_iter([]),
            },
            authenticated_account_cmds::vault_item_recovery_list::VaultItemRecoveryVault {
                auth_methods: vec![
                    authenticated_account_cmds::vault_item_recovery_list::VaultItemRecoveryAuthMethod {
                        created_on: "2000-01-02T00:00:00Z".parse().unwrap(),
                        disabled_on: Some("2000-01-04T00:00:00Z".parse().unwrap()),
                        created_by_ip: "127.0.0.2".to_string(),
                        created_by_user_agent: "Parsec-Client/3.4.2 Windows".to_string(),
                        vault_key_access: b"<vault_key_access 2>".as_ref().into(),
                        password_algorithm: Some(PasswordAlgorithm::Argon2id {
                            salt: b"<salt 2>".to_vec(),
                            opslimit: 65536,
                            memlimit_kb: 3,
                            parallelism: 1,
                        })
                    },
                    authenticated_account_cmds::vault_item_recovery_list::VaultItemRecoveryAuthMethod {
                        created_on: "2000-01-03T00:00:00Z".parse().unwrap(),
                        disabled_on: Some("2000-01-04T00:00:00Z".parse().unwrap()),
                        created_by_ip: "".to_string(),
                        created_by_user_agent: "Parsec-Client/3.4.1 Windows".to_string(),
                        vault_key_access: b"<vault_key_access 1>".as_ref().into(),
                        password_algorithm: None,
                    }
                ],
                vault_items: HashMap::from_iter([
                    (
                        HashDigest::from(hex!(
                            "076a27c79e5ace2a3d47f9dd2e83e4ff6ea8872b3c2218f66c92b89b55f36560"
                        )),
                        b"data 1".as_ref().into()
                    ),
                    (
                        HashDigest::from(hex!(
                            "e37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6"
                        )),
                        b"data 2".as_ref().into()
                    ),
                ]),
            },
        ]
    };
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = authenticated_account_cmds::vault_item_recovery_list::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_account_cmds::vault_item_recovery_list::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
