// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use bytes::Bytes;

use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

use super::anonymous_account_cmds;
use std::str::FromStr;

fn step0() {
    let raw_req = [
        (
    // Generated from Parsec 3.4.1-a.0+dev
    // Content:
    //   cmd: 'account_create_proceed'
    //   account_create_step: { step: 'NUMBER_0_CHECK_CODE', email: 'alice@invalid.com', validation_code: 'C88DEE', }
    hex!(
        "82a3636d64b66163636f756e745f6372656174655f70726f63656564b36163636f756e"
        "745f6372656174655f7374657083a473746570b34e554d4245525f305f434845434b5f"
        "434f4445a5656d61696cb1616c69636540696e76616c69642e636f6daf76616c696461"
        "74696f6e5f636f6465a6433838444545"
    ).as_ref(),
            anonymous_account_cmds::account_create_proceed::Req {
                account_create_step: anonymous_account_cmds::account_create_proceed::AccountCreateStep::Number0CheckCode {validation_code: ValidationCode::from_str("C88DEE").unwrap(), email:EmailAddress::try_from("alice@invalid.com").unwrap() },
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
    // Generated from Parsec 3.4.1-a.0+dev
    // Content:
    //   cmd: 'account_create_proceed'
    //   account_create_step: { step: 'NUMBER_1_CREATE', auth_method_hmac_key: 0x2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1, auth_method_id: ext(2, 0x9aae259f748045cc9fe7146eab0b132e), auth_method_password_algorithm: { type: 'ARGON2ID', memlimit_kb: 131072, opslimit: 3, parallelism: 1, }, human_handle: [ 'alice@invalid.com', 'Anonymous Alice', ], validation_code: 'C88DEE', vault_key_access: 0x7661756c745f6b65795f616363657373, }
    hex!(
        "82a3636d64b66163636f756e745f6372656174655f70726f63656564b36163636f756e"
        "745f6372656174655f7374657087a473746570af4e554d4245525f315f435245415445"
        "b4617574685f6d6574686f645f686d61635f6b6579c4202ff13803789977db4f8ccabf"
        "b6b26f3e70eb4453d396dcb2315f7690cbc2e3f1ae617574685f6d6574686f645f6964"
        "d8029aae259f748045cc9fe7146eab0b132ebe617574685f6d6574686f645f70617373"
        "776f72645f616c676f726974686d84a474797065a84152474f4e324944ab6d656d6c69"
        "6d69745f6b62ce00020000a86f70736c696d697403ab706172616c6c656c69736d01ac"
        "68756d616e5f68616e646c6592b1616c69636540696e76616c69642e636f6daf416e6f"
        "6e796d6f757320416c696365af76616c69646174696f6e5f636f6465a6433838444545"
        "b07661756c745f6b65795f616363657373c4107661756c745f6b65795f616363657373"
    ).as_ref(),
            anonymous_account_cmds::account_create_proceed::Req {

                account_create_step: anonymous_account_cmds::account_create_proceed::AccountCreateStep::Number1Create {

    validation_code: ValidationCode::from_str("C88DEE").unwrap(),
                human_handle: HumanHandle::new(EmailAddress::try_from("alice@invalid.com").unwrap(),"Anonymous Alice").unwrap(),
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
    // Generated from Parsec 3.4.1-a.0+dev
    // Content:
    //   cmd: 'account_create_proceed'
    //   account_create_step: { step: 'NUMBER_1_CREATE', auth_method_hmac_key: 0x2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1, auth_method_id: ext(2, 0x9aae259f748045cc9fe7146eab0b132e), auth_method_password_algorithm: None, human_handle: [ 'alice@invalid.com', 'Anonymous Alice', ], validation_code: 'C88DEE', vault_key_access: 0x7661756c745f6b65795f616363657373, }
    hex!(
        "82a3636d64b66163636f756e745f6372656174655f70726f63656564b36163636f756e"
        "745f6372656174655f7374657087a473746570af4e554d4245525f315f435245415445"
        "b4617574685f6d6574686f645f686d61635f6b6579c4202ff13803789977db4f8ccabf"
        "b6b26f3e70eb4453d396dcb2315f7690cbc2e3f1ae617574685f6d6574686f645f6964"
        "d8029aae259f748045cc9fe7146eab0b132ebe617574685f6d6574686f645f70617373"
        "776f72645f616c676f726974686dc0ac68756d616e5f68616e646c6592b1616c696365"
        "40696e76616c69642e636f6daf416e6f6e796d6f757320416c696365af76616c696461"
        "74696f6e5f636f6465a6433838444545b07661756c745f6b65795f616363657373c410"
        "7661756c745f6b65795f616363657373"
    ).as_ref(),
            anonymous_account_cmds::account_create_proceed::Req {


                account_create_step: anonymous_account_cmds::account_create_proceed::AccountCreateStep::Number1Create {
validation_code: ValidationCode::from_str("C88DEE").unwrap(),
                human_handle: HumanHandle::new(EmailAddress::try_from("alice@invalid.com").unwrap(),"Anonymous Alice").unwrap(),
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

pub fn rep_send_validation_code_required() {
    // Generated from Parsec 3.4.1-a.0+dev
    // Content:
    //   status: 'send_validation_code_required'
    let raw: &[u8] = hex!(
        "81a6737461747573bd73656e645f76616c69646174696f6e5f636f64655f7265717569"
        "726564"
    )
    .as_ref();
    let expected =
        anonymous_account_cmds::account_create_proceed::Rep::SendValidationCodeRequired {};
    println!("***expected: {:?}", expected.dump().unwrap());

    let data = anonymous_account_cmds::account_create_proceed::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_account_cmds::account_create_proceed::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_invalid_validation_code() {
    // Generated from Parsec 3.4.1-a.0+dev
    // Content:
    //   status: 'invalid_validation_code'
    let raw: &[u8] =
        hex!("81a6737461747573b7696e76616c69645f76616c69646174696f6e5f636f6465").as_ref();
    let expected = anonymous_account_cmds::account_create_proceed::Rep::InvalidValidationCode {};
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
