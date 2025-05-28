// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use bytes::Bytes;

use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

use super::anonymous_account_cmds;

// Request

fn step0() {
    let raw_req = [
        (
    // Generated from Parsec 3.4.0-a.7+dev
    // Content:
    //   cmd: 'account_create_proceed'
    //   email: 'alice@invalid.com'
    //   account_create_step: { step: 'NUMBER_0_CHECK_CODE', code: 'COODEE', }
     hex!(
    "83a3636d64b66163636f756e745f6372656174655f70726f63656564a5656d61696cb1"
    "616c69636540696e76616c69642e636f6db36163636f756e745f6372656174655f7374"
    "657082a473746570b34e554d4245525f305f434845434b5f434f4445a4636f6465a643"
    "4f4f444545"
    ).as_ref(),
            anonymous_account_cmds::account_create_proceed::Req {
                account_create_step: anonymous_account_cmds::account_create_proceed::AccountCreateStep::Number0CheckCode {code: "COODEE".to_string()},
                email: EmailAddress::try_from("alice@invalid.com").unwrap()
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

fn step1() {
    let raw_req = [
        (
           // Generated from Parsec 3.4.0-a.7+dev
    // Content:
    //   cmd: 'account_create_proceed'
    //   email: 'alice@invalid.com'
    //   account_create_step: { step: 'NUMBER_1_CREATE', auth_method_hmac_key: 0x2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1, auth_method_id: ext(2, 0x9aae259f748045cc9fe7146eab0b132e), auth_method_password_algorithm: { Argon2id: { salt: 0x706570706572, opslimit: 65536, memlimit_kb: 3, parallelism: 1, }, }, human_label: 'Anonymous Alice', vault_key_access: 0x7661756c745f6b65795f616363657373, }
     hex!(
    "83a3636d64b66163636f756e745f6372656174655f70726f63656564a5656d61696cb1"
    "616c69636540696e76616c69642e636f6db36163636f756e745f6372656174655f7374"
    "657086a473746570af4e554d4245525f315f435245415445b4617574685f6d6574686f"
    "645f686d61635f6b6579c4202ff13803789977db4f8ccabfb6b26f3e70eb4453d396dc"
    "b2315f7690cbc2e3f1ae617574685f6d6574686f645f6964d8029aae259f748045cc9f"
    "e7146eab0b132ebe617574685f6d6574686f645f70617373776f72645f616c676f7269"
    "74686d81a84172676f6e32696484a473616c74c406706570706572a86f70736c696d69"
    "74ce00010000ab6d656d6c696d69745f6b6203ab706172616c6c656c69736d01ab6875"
    "6d616e5f6c6162656caf416e6f6e796d6f757320416c696365b07661756c745f6b6579"
    "5f616363657373c4107661756c745f6b65795f616363657373"
    ).as_ref(),
            anonymous_account_cmds::account_create_proceed::Req {
                email: EmailAddress::try_from("alice@invalid.com").unwrap(),

                account_create_step: anonymous_account_cmds::account_create_proceed::AccountCreateStep::Number1Create {


                human_label: "Anonymous Alice".to_string(),
                vault_key_access: Bytes::from("vault_key_access"),
                auth_method_id: AccountAuthMethodID::from_hex("9aae259f748045cc9fe7146eab0b132e")
                    .unwrap(),
                auth_method_hmac_key: SecretKey::from(hex!(
                    "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
                )),
                auth_method_password_algorithm: Some(UntrustedPasswordAlgorithm::Argon2id {
                    memlimit_kb: 128 * 1024,
                    opslimit: 3,
                    parallelism: 1,
                }),
            }
            },
        ),
        (
              // Generated from Parsec 3.4.0-a.7+dev
    // Content:
    //   cmd: 'account_create_proceed'
    //   email: 'alice@invalid.com'
    //   account_create_step: { step: 'NUMBER_1_CREATE', auth_method_hmac_key: 0x2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1, auth_method_id: ext(2, 0x9aae259f748045cc9fe7146eab0b132e), auth_method_password_algorithm: None, human_label: 'Anonymous Alice', vault_key_access: 0x7661756c745f6b65795f616363657373, }
     hex!(
    "83a3636d64b66163636f756e745f6372656174655f70726f63656564a5656d61696cb1"
    "616c69636540696e76616c69642e636f6db36163636f756e745f6372656174655f7374"
    "657086a473746570af4e554d4245525f315f435245415445b4617574685f6d6574686f"
    "645f686d61635f6b6579c4202ff13803789977db4f8ccabfb6b26f3e70eb4453d396dc"
    "b2315f7690cbc2e3f1ae617574685f6d6574686f645f6964d8029aae259f748045cc9f"
    "e7146eab0b132ebe617574685f6d6574686f645f70617373776f72645f616c676f7269"
    "74686dc0ab68756d616e5f6c6162656caf416e6f6e796d6f757320416c696365b07661"
    "756c745f6b65795f616363657373c4107661756c745f6b65795f616363657373"
    ).as_ref(),
            anonymous_account_cmds::account_create_proceed::Req {
                    email: EmailAddress::try_from("alice@invalid.com").unwrap(),

                account_create_step: anonymous_account_cmds::account_create_proceed::AccountCreateStep::Number1Create {

                human_label: "Anonymous Alice".to_string(),
                vault_key_access: Bytes::from("vault_key_access"),
                auth_method_id: AccountAuthMethodID::from_hex("9aae259f748045cc9fe7146eab0b132e")
                    .unwrap(),
                auth_method_hmac_key: SecretKey::from(hex!(
                    "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
                )),
                auth_method_password_algorithm: None,
            }
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

pub fn req() {
    step0();
    step1();
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
