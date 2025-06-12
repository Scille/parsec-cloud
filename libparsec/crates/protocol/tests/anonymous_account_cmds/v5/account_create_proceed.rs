// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use bytes::Bytes;

use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

use super::anonymous_account_cmds;

// Request

pub fn req() {
    let raw_req = [
        (
            // Generated from Parsec 3.4.0-a.7+dev
            // Content:
            //   cmd: 'account_create_proceed'
            //   validation_token: 0x672bc6ba9c43455da28344e975dc72b7
            //   human_label: 'Anonymous Alice'
            //   auth_method_password_algorithm: { type: 'ARGON2ID', memlimit_kb: 3, opslimit: 65536, parallelism: 1, salt: 0x706570706572, }
            //   auth_method_hmac_key: 0x2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1
            //   auth_method_id: ext(2, 0x9aae259f748045cc9fe7146eab0b132e)
            //   vault_key_access: 0x7661756c745f6b65795f616363657373
            hex!(
                "87a3636d64b66163636f756e745f6372656174655f70726f63656564b076616c696461"
                "74696f6e5f746f6b656ec410672bc6ba9c43455da28344e975dc72b7ab68756d616e5f"
                "6c6162656caf416e6f6e796d6f757320416c696365be617574685f6d6574686f645f70"
                "617373776f72645f616c676f726974686d85a474797065a84152474f4e324944ab6d65"
                "6d6c696d69745f6b6203a86f70736c696d6974ce00010000ab706172616c6c656c6973"
                "6d01a473616c74c406706570706572b4617574685f6d6574686f645f686d61635f6b65"
                "79c4202ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
                "ae617574685f6d6574686f645f6964d8029aae259f748045cc9fe7146eab0b132eb076"
                "61756c745f6b65795f616363657373c4107661756c745f6b65795f616363657373"
            )
            .as_ref(),
            anonymous_account_cmds::account_create_proceed::Req {
                validation_token: EmailValidationToken::from_hex(
                    "672bc6ba9c43455da28344e975dc72b7",
                )
                .unwrap(),
                human_label: "Anonymous Alice".to_string(),
                vault_key_access: Bytes::from("vault_key_access"),
                auth_method_id: AccountAuthMethodID::from_hex("9aae259f748045cc9fe7146eab0b132e")
                    .unwrap(),
                auth_method_hmac_key: SecretKey::from(hex!(
                    "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
                )),
                auth_method_password_algorithm: Some(PasswordAlgorithm::Argon2id {
                    memlimit_kb: 3,
                    opslimit: 65536,
                    parallelism: 1,
                    salt: b"pepper".to_vec(),
                }),
            },
        ),
        (
            // Generated from Parsec 3.4.0-a.7+dev
            // Content:
            //   cmd: 'account_create_proceed'
            //   validation_token: 0x672bc6ba9c43455da28344e975dc72b7
            //   human_label: 'Anonymous Alice'
            //   auth_method_password_algorithm: None
            //   auth_method_hmac_key: 0x2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1
            //   auth_method_id: ext(2, 0x9aae259f748045cc9fe7146eab0b132e)
            //   vault_key_access: 0x7661756c745f6b65795f616363657373
            hex!(
                "87a3636d64b66163636f756e745f6372656174655f70726f63656564b076616c696461"
                "74696f6e5f746f6b656ec410672bc6ba9c43455da28344e975dc72b7ab68756d616e5f"
                "6c6162656caf416e6f6e796d6f757320416c696365be617574685f6d6574686f645f70"
                "617373776f72645f616c676f726974686dc0b4617574685f6d6574686f645f686d6163"
                "5f6b6579c4202ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cb"
                "c2e3f1ae617574685f6d6574686f645f6964d8029aae259f748045cc9fe7146eab0b13"
                "2eb07661756c745f6b65795f616363657373c4107661756c745f6b65795f6163636573"
                "73"
            )
            .as_ref(),
            anonymous_account_cmds::account_create_proceed::Req {
                validation_token: EmailValidationToken::from_hex(
                    "672bc6ba9c43455da28344e975dc72b7",
                )
                .unwrap(),
                human_label: "Anonymous Alice".to_string(),
                vault_key_access: Bytes::from("vault_key_access"),
                auth_method_id: AccountAuthMethodID::from_hex("9aae259f748045cc9fe7146eab0b132e")
                    .unwrap(),
                auth_method_hmac_key: SecretKey::from(hex!(
                    "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
                )),
                auth_method_password_algorithm: None,
            },
        ),
    ];

    for (raw, req) in raw_req {
        let expected = anonymous_account_cmds::AnyCmdReq::AccountCreateProceed(req.clone());
        println!("***expected: {:?}", req.dump().unwrap());

        let data = anonymous_account_cmds::AnyCmdReq::load(raw).unwrap();
        p_assert_eq!(data, expected);

        // Also test serialization round trip
        let anonymous_account_cmds::AnyCmdReq::AccountCreateProceed(req2) = data else {
            unreachable!()
        };

        let raw2 = req2.dump().unwrap();

        let data2 = anonymous_account_cmds::AnyCmdReq::load(&raw2).unwrap();

        p_assert_eq!(data2, expected);
    }
}

// Responses

pub fn rep_ok() {
    // Generated from Parsec 3.4.0-a.7+dev
    // Content:
    //   status: 'ok'
    let raw: &[u8] = hex!("81a6737461747573a26f6b").as_ref();
    let expected = anonymous_account_cmds::account_create_proceed::Rep::Ok {};
    println!("***expected: {:?}", expected.dump().unwrap());

    let data = anonymous_account_cmds::account_create_proceed::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_account_cmds::account_create_proceed::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_invalid_validation_token() {
    // Generated from Parsec 3.4.0-a.7+dev
    // Content:
    //   status: 'invalid_validation_token'
    let raw: &[u8] =
        hex!("81a6737461747573b8696e76616c69645f76616c69646174696f6e5f746f6b656e").as_ref();

    let expected = anonymous_account_cmds::account_create_proceed::Rep::InvalidValidationToken {};
    println!("***expected: {:?}", expected.dump().unwrap());

    let data = anonymous_account_cmds::account_create_proceed::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_account_cmds::account_create_proceed::Rep::load(&raw2).unwrap();

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
        anonymous_account_cmds::account_create_proceed::Rep::AuthMethodIdAlreadyExists {};
    println!("***expected: {:?}", expected.dump().unwrap());

    let data = anonymous_account_cmds::account_create_proceed::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_account_cmds::account_create_proceed::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
