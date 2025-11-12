// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

use super::anonymous_server_cmds;

pub fn req() {
    let raw_req = [
        (
            // Generated from Parsec 3.4.1-a.0+dev
            // Content:
            //   cmd: 'account_recover_proceed'
            //   email: 'alice@invalid.com'
            //   validation_code: 'C88DEE'
            //   new_vault_key_access: 0x3c7661756c745f6b65795f6163636573733e
            //   new_auth_method_password_algorithm: { type: 'ARGON2ID', memlimit_kb: 131072, opslimit: 3, parallelism: 1, }
            //   new_auth_method_mac_key: 0x2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1
            //   new_auth_method_id: ext(2, 0x9aae259f748045cc9fe7146eab0b132e)
            hex!(
                "87a3636d64b76163636f756e745f7265636f7665725f70726f63656564a5656d61696c"
                "b1616c69636540696e76616c69642e636f6daf76616c69646174696f6e5f636f6465a6"
                "433838444545b46e65775f7661756c745f6b65795f616363657373c4123c7661756c74"
                "5f6b65795f6163636573733ed9226e65775f617574685f6d6574686f645f7061737377"
                "6f72645f616c676f726974686d84a474797065a84152474f4e324944ab6d656d6c696d"
                "69745f6b62ce00020000a86f70736c696d697403ab706172616c6c656c69736d01b76e"
                "65775f617574685f6d6574686f645f6d61635f6b6579c4202ff13803789977db4f8cca"
                "bfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1b26e65775f617574685f6d657468"
                "6f645f6964d8029aae259f748045cc9fe7146eab0b132e"
            )
            .as_ref(),
            anonymous_server_cmds::account_recover_proceed::Req {
                validation_code: "C88DEE".parse().unwrap(),
                email: "alice@invalid.com".parse().unwrap(),
                new_vault_key_access: Bytes::from_static(b"<vault_key_access>"),
                new_auth_method_id: AccountAuthMethodID::from_hex(
                    "9aae259f748045cc9fe7146eab0b132e",
                )
                .unwrap(),
                new_auth_method_mac_key: SecretKey::from(hex!(
                    "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
                )),
                new_auth_method_password_algorithm: Some(UntrustedPasswordAlgorithm::Argon2id {
                    memlimit_kb: 128 * 1024,
                    opslimit: 3,
                    parallelism: 1,
                }),
            },
        ),
        (
            // Generated from Parsec 3.4.1-a.0+dev
            // Content:
            //   cmd: 'account_recover_proceed'
            //   email: 'alice@invalid.com'
            //   validation_code: 'C88DEE'
            //   new_vault_key_access: 0x3c7661756c745f6b65795f6163636573733e
            //   new_auth_method_password_algorithm: None
            //   new_auth_method_mac_key: 0x2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1
            //   new_auth_method_id: ext(2, 0x9aae259f748045cc9fe7146eab0b132e)
            hex!(
                "87a3636d64b76163636f756e745f7265636f7665725f70726f63656564a5656d61696c"
                "b1616c69636540696e76616c69642e636f6daf76616c69646174696f6e5f636f6465a6"
                "433838444545b46e65775f7661756c745f6b65795f616363657373c4123c7661756c74"
                "5f6b65795f6163636573733ed9226e65775f617574685f6d6574686f645f7061737377"
                "6f72645f616c676f726974686dc0b76e65775f617574685f6d6574686f645f6d61635f"
                "6b6579c4202ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2"
                "e3f1b26e65775f617574685f6d6574686f645f6964d8029aae259f748045cc9fe7146e"
                "ab0b132e"
            )
            .as_ref(),
            anonymous_server_cmds::account_recover_proceed::Req {
                validation_code: "C88DEE".parse().unwrap(),
                email: "alice@invalid.com".parse().unwrap(),
                new_vault_key_access: Bytes::from_static(b"<vault_key_access>"),
                new_auth_method_id: AccountAuthMethodID::from_hex(
                    "9aae259f748045cc9fe7146eab0b132e",
                )
                .unwrap(),
                new_auth_method_mac_key: SecretKey::from(hex!(
                    "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
                )),
                new_auth_method_password_algorithm: None,
            },
        ),
    ];

    for (raw, req) in raw_req {
        let expected = anonymous_server_cmds::AnyCmdReq::AccountRecoverProceed(req.clone());
        println!("***expected: {:?}", req.dump().unwrap());

        let data = anonymous_server_cmds::AnyCmdReq::load(raw).unwrap();
        p_assert_eq!(data, expected);

        // Also test serialization round trip
        let anonymous_server_cmds::AnyCmdReq::AccountRecoverProceed(req2) = data else {
            unreachable!()
        };

        let raw2 = req2.dump().unwrap();

        let data2 = anonymous_server_cmds::AnyCmdReq::load(&raw2).unwrap();

        p_assert_eq!(data2, expected);
    }
}

// Responses

pub fn rep_ok() {
    // Generated from Parsec 3.4.0-a.7+dev
    // Content:
    //   status: 'ok'
    let raw: &[u8] = hex!("81a6737461747573a26f6b").as_ref();
    let expected = anonymous_server_cmds::account_recover_proceed::Rep::Ok {};
    println!("***expected: {:?}", expected.dump().unwrap());

    let data = anonymous_server_cmds::account_recover_proceed::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_server_cmds::account_recover_proceed::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_send_validation_email_required() {
    // Generated from Parsec 3.4.1-a.0+dev
    // Content:
    //   status: 'send_validation_email_required'
    let raw: &[u8] = hex!(
        "81a6737461747573be73656e645f76616c69646174696f6e5f656d61696c5f72657175"
        "69726564"
    )
    .as_ref();

    let expected =
        anonymous_server_cmds::account_recover_proceed::Rep::SendValidationEmailRequired {};
    println!("***expected: {:?}", expected.dump().unwrap());

    let data = anonymous_server_cmds::account_recover_proceed::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_server_cmds::account_recover_proceed::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_invalid_validation_code() {
    // Generated from Parsec 3.4.1-a.0+dev
    // Content:
    //   status: 'invalid_validation_code'
    let raw: &[u8] =
        hex!("81a6737461747573b7696e76616c69645f76616c69646174696f6e5f636f6465").as_ref();
    let expected = anonymous_server_cmds::account_recover_proceed::Rep::InvalidValidationCode {};
    println!("***expected: {:?}", expected.dump().unwrap());

    let data = anonymous_server_cmds::account_recover_proceed::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_server_cmds::account_recover_proceed::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_auth_method_id_already_exists() {
    // Generated from Parsec 3.4.0-a.7+dev
    // Content:
    //   status: 'auth_method_id_already_exists'
    let raw: &[u8] = hex!(
    "81a6737461747573bd617574685f6d6574686f645f69645f616c72656164795f657869"
    "737473"
    )
    .as_ref();

    let expected =
        anonymous_server_cmds::account_recover_proceed::Rep::AuthMethodIdAlreadyExists {};
    println!("***expected: {:?}", expected.dump().unwrap());

    let data = anonymous_server_cmds::account_recover_proceed::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_server_cmds::account_recover_proceed::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
