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
    let raw_req = [
        (
            // Generated from Parsec 3.4.0-a.7+dev
            // Content:
            //   cmd: 'vault_key_rotation'
            //   new_auth_method_id: ext(2, 0x96691586ce5b4789ae8a0a4c052e8986)
            //   new_auth_method_mac_key: 0xa2754dba7066a49f487737790388548c2af0ddfbed609805184ca5023afe1983
            //   new_auth_method_password_algorithm: { Argon2id: { salt: 0x3c73616c7420323e, opslimit: 65536, memlimit_kb: 3, parallelism: 1, }, }
            //   new_vault_key_access: 0x3c6b6579206163636573733e
            //   items: {
            //     0x076a27c79e5ace2a3d47f9dd2e83e4ff6ea8872b3c2218f66c92b89b55f36560: 0x646174612031,
            //     0xe37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6: 0x646174612032,
            //   }
            hex!(
                "86a3636d64b27661756c745f6b65795f726f746174696f6eb26e65775f617574685f6d"
                "6574686f645f6964d80296691586ce5b4789ae8a0a4c052e8986b76e65775f61757468"
                "5f6d6574686f645f6d61635f6b6579c420a2754dba7066a49f487737790388548c2af0"
                "ddfbed609805184ca5023afe1983d9226e65775f617574685f6d6574686f645f706173"
                "73776f72645f616c676f726974686d81a84172676f6e32696484a473616c74c4083c73"
                "616c7420323ea86f70736c696d6974ce00010000ab6d656d6c696d69745f6b6203ab70"
                "6172616c6c656c69736d01b46e65775f7661756c745f6b65795f616363657373c40c3c"
                "6b6579206163636573733ea56974656d7382c420076a27c79e5ace2a3d47f9dd2e83e4"
                "ff6ea8872b3c2218f66c92b89b55f36560c406646174612031c420e37ce3b00a1f15b3"
                "de62029972345420b76313a885c6ccc6e3b5547857b3ecc6c406646174612032"
            )
            .as_ref(),
            authenticated_account_cmds::vault_key_rotation::Req {
                new_auth_method_id: AccountAuthMethodID::from_hex(
                    "96691586ce5b4789ae8a0a4c052e8986",
                )
                .unwrap(),
                new_auth_method_mac_key: SecretKey::from(hex!(
                    "a2754dba7066a49f487737790388548c2af0ddfbed609805184ca5023afe1983"
                )),
                new_auth_method_password_algorithm: Some(PasswordAlgorithm::Argon2id {
                    salt: Bytes::from_static(b"<salt 2>"),
                    opslimit: 65536,
                    memlimit_kb: 3,
                    parallelism: 1,
                }),
                items: HashMap::from_iter([
                    (
                        HashDigest::from(hex!(
                            "076a27c79e5ace2a3d47f9dd2e83e4ff6ea8872b3c2218f66c92b89b55f36560"
                        )),
                        b"data 1".as_ref().into(),
                    ),
                    (
                        HashDigest::from(hex!(
                            "e37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6"
                        )),
                        b"data 2".as_ref().into(),
                    ),
                ]),
                new_vault_key_access: b"<key access>".as_ref().into(),
            },
        ),
        (
            // Generated from Parsec 3.4.0-a.7+dev
            // Content:
            //   cmd: 'vault_key_rotation'
            //   new_auth_method_id: ext(2, 0x96691586ce5b4789ae8a0a4c052e8986)
            //   new_auth_method_mac_key: 0xa2754dba7066a49f487737790388548c2af0ddfbed609805184ca5023afe1983
            //   new_auth_method_password_algorithm: None
            //   new_vault_key_access: 0x3c6b6579206163636573733e
            //   items: { }
            hex!(
                "86a3636d64b27661756c745f6b65795f726f746174696f6eb26e65775f617574685f6d"
                "6574686f645f6964d80296691586ce5b4789ae8a0a4c052e8986b76e65775f61757468"
                "5f6d6574686f645f6d61635f6b6579c420a2754dba7066a49f487737790388548c2af0"
                "ddfbed609805184ca5023afe1983d9226e65775f617574685f6d6574686f645f706173"
                "73776f72645f616c676f726974686dc0b46e65775f7661756c745f6b65795f61636365"
                "7373c40c3c6b6579206163636573733ea56974656d7380"
            )
            .as_ref(),
            authenticated_account_cmds::vault_key_rotation::Req {
                new_auth_method_id: AccountAuthMethodID::from_hex(
                    "96691586ce5b4789ae8a0a4c052e8986",
                )
                .unwrap(),
                new_auth_method_mac_key: SecretKey::from(hex!(
                    "a2754dba7066a49f487737790388548c2af0ddfbed609805184ca5023afe1983"
                )),
                new_auth_method_password_algorithm: None,
                items: HashMap::from_iter([]),
                new_vault_key_access: b"<key access>".as_ref().into(),
            },
        ),
    ];

    for (raw, req) in raw_req {
        let expected = authenticated_account_cmds::AnyCmdReq::VaultKeyRotation(req.clone());
        println!("***expected: {:?}", req.dump().unwrap());
        let data = authenticated_account_cmds::AnyCmdReq::load(raw).unwrap();

        p_assert_eq!(data, expected);

        // Also test serialization round trip
        let raw2 = req.dump().unwrap();

        let data2 = authenticated_account_cmds::AnyCmdReq::load(&raw2).unwrap();

        p_assert_eq!(data2, expected);
    }
}

// Responses

pub fn rep_ok() {
    // Generated from Parsec 3.4.0-a.7+dev
    // Content:
    //   status: 'ok'
    let raw: &[u8] = hex!("81a6737461747573a26f6b").as_ref();

    let expected = authenticated_account_cmds::vault_key_rotation::Rep::Ok {};
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = authenticated_account_cmds::vault_key_rotation::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_account_cmds::vault_key_rotation::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_items_mismatch() {
    // Generated from Parsec 3.4.0-a.7+dev
    // Content:
    //   status: 'items_mismatch'
    let raw: &[u8] = hex!("81a6737461747573ae6974656d735f6d69736d61746368").as_ref();

    let expected = authenticated_account_cmds::vault_key_rotation::Rep::ItemsMismatch {};
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = authenticated_account_cmds::vault_key_rotation::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_account_cmds::vault_key_rotation::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_new_auth_method_id_already_exists() {
    // Generated from Parsec 3.4.0-a.7+dev
    // Content:
    //   status: 'new_auth_method_id_already_exists'
    let raw: &[u8] = hex!(
        "81a6737461747573d9216e65775f617574685f6d6574686f645f69645f616c72656164"
        "795f657869737473"
    )
    .as_ref();

    let expected =
        authenticated_account_cmds::vault_key_rotation::Rep::NewAuthMethodIdAlreadyExists {};
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = authenticated_account_cmds::vault_key_rotation::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_account_cmds::vault_key_rotation::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
