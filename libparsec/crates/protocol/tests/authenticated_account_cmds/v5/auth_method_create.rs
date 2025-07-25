// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

use super::authenticated_account_cmds;

// Request
pub fn req() {
    let raw_req = [
        (
            // Generated from Parsec 3.4.1-a.0+dev
            // Content:
            //   cmd: 'auth_method_create'
            //   auth_method_password_algorithm: { type: 'ARGON2ID', memlimit_kb: 131072, opslimit: 3, parallelism: 1, }
            //   auth_method_mac_key: 0x2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1
            //   auth_method_id: ext(2, 0x9aae259f748045cc9fe7146eab0b132e)
            //   vault_key_access: 0x7661756c745f6b65795f616363657373
            hex!(
                "85a3636d64b2617574685f6d6574686f645f637265617465be617574685f6d6574686f"
                "645f70617373776f72645f616c676f726974686d84a474797065a84152474f4e324944"
                "ab6d656d6c696d69745f6b62ce00020000a86f70736c696d697403ab706172616c6c65"
                "6c69736d01b3617574685f6d6574686f645f6d61635f6b6579c4202ff13803789977db"
                "4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1ae617574685f6d6574686f"
                "645f6964d8029aae259f748045cc9fe7146eab0b132eb07661756c745f6b65795f6163"
                "63657373c4107661756c745f6b65795f616363657373"
            )
            .as_ref(),
            authenticated_account_cmds::auth_method_create::Req {
                vault_key_access: Bytes::from_static(b"vault_key_access"),
                auth_method_id: AccountAuthMethodID::from_hex("9aae259f748045cc9fe7146eab0b132e")
                    .unwrap(),
                auth_method_mac_key: SecretKey::from(hex!(
                    "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
                )),
                auth_method_password_algorithm: Some(UntrustedPasswordAlgorithm::Argon2id {
                    memlimit_kb: 128 * 1024,
                    opslimit: 3,
                    parallelism: 1,
                }),
            },
        ),
        (
            // Generated from Parsec 3.4.1-a.0+dev
            // Content:
            //   cmd: 'auth_method_create'
            //   auth_method_password_algorithm: None
            //   auth_method_mac_key: 0x2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1
            //   auth_method_id: ext(2, 0x9aae259f748045cc9fe7146eab0b132e)
            //   vault_key_access: 0x7661756c745f6b65795f616363657373
            hex!(
                "85a3636d64b2617574685f6d6574686f645f637265617465be617574685f6d6574686f"
                "645f70617373776f72645f616c676f726974686dc0b3617574685f6d6574686f645f6d"
                "61635f6b6579c4202ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f76"
                "90cbc2e3f1ae617574685f6d6574686f645f6964d8029aae259f748045cc9fe7146eab"
                "0b132eb07661756c745f6b65795f616363657373c4107661756c745f6b65795f616363"
                "657373"
            )
            .as_ref(),
            authenticated_account_cmds::auth_method_create::Req {
                vault_key_access: Bytes::from_static(b"vault_key_access"),
                auth_method_id: AccountAuthMethodID::from_hex("9aae259f748045cc9fe7146eab0b132e")
                    .unwrap(),
                auth_method_mac_key: SecretKey::from(hex!(
                    "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
                )),
                auth_method_password_algorithm: None,
            },
        ),
    ];

    for (raw, req) in raw_req {
        let expected = authenticated_account_cmds::AnyCmdReq::AuthMethodCreate(req.clone());
        println!("***expected: {:?}", req.dump().unwrap());

        let data = authenticated_account_cmds::AnyCmdReq::load(raw).unwrap();
        p_assert_eq!(data, expected);

        // Also test serialization round trip
        let authenticated_account_cmds::AnyCmdReq::AuthMethodCreate(req2) = data else {
            unreachable!()
        };

        let raw2 = req2.dump().unwrap();

        let data2 = authenticated_account_cmds::AnyCmdReq::load(&raw2).unwrap();

        p_assert_eq!(data2, expected);
    }
}

// Responses

pub fn rep_ok() {
    // Generated from Parsec 3.4.1-a.0+dev
    // Content:
    //   status: 'ok'
    let raw: &[u8] = hex!("81a6737461747573a26f6b").as_ref();

    let expected = authenticated_account_cmds::auth_method_create::Rep::Ok {};
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = authenticated_account_cmds::auth_method_create::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();
    let data2 = authenticated_account_cmds::auth_method_create::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_auth_method_id_already_exists() {
    // Generated from Parsec 3.4.1-a.0+dev
    // Content:
    //   status: 'auth_method_id_already_exists'
    let raw: &[u8] = hex!(
        "81a6737461747573bd617574685f6d6574686f645f69645f616c72656164795f657869"
        "737473"
    )
    .as_ref();

    let expected =
        authenticated_account_cmds::auth_method_create::Rep::AuthMethodIdAlreadyExists {};
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = authenticated_account_cmds::auth_method_create::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();
    let data2 = authenticated_account_cmds::auth_method_create::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
