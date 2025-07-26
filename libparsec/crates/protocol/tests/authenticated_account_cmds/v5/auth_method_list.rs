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
    //   cmd: 'auth_method_list'
    let raw: &[u8] = hex!("81a3636d64b0617574685f6d6574686f645f6c697374").as_ref();

    let req = authenticated_account_cmds::auth_method_list::Req {};

    let expected = authenticated_account_cmds::AnyCmdReq::AuthMethodList(req.clone());
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
    // Generated from Parsec 3.4.1-a.0+dev
    // Content:
    //   status: 'ok'
    //   items: [
    //     {
    //       auth_method_id: ext(2, 0x1fe91588816a4df98400a9dc2d0bdfa7),
    //       created_by_ip: '127.0.0.2',
    //       created_by_user_agent: 'Parsec-Client/3.4.2 Windows',
    //       created_on: ext(1, 946771200000000) i.e. 2000-01-02T01:00:00Z,
    //       password_algorithm: { type: 'ARGON2ID', memlimit_kb: 131072, opslimit: 3, parallelism: 1, },
    //       vault_key_access: 0x3c7661756c745f6b65795f61636365737320323e,
    //     },
    //     {
    //       auth_method_id: ext(2, 0x09e489cc8ba844acbec359e27790fd68),
    //       created_by_ip: '',
    //       created_by_user_agent: 'Parsec-Client/3.4.1 Windows',
    //       created_on: ext(1, 946857600000000) i.e. 2000-01-03T01:00:00Z,
    //       password_algorithm: None,
    //       vault_key_access: 0x3c7661756c745f6b65795f61636365737320313e,
    //     },
    //   ]
    let raw: &[u8] = hex!(
        "82a6737461747573a26f6ba56974656d739286ae617574685f6d6574686f645f6964d8"
        "021fe91588816a4df98400a9dc2d0bdfa7ad637265617465645f62795f6970a9313237"
        "2e302e302e32b5637265617465645f62795f757365725f6167656e74bb506172736563"
        "2d436c69656e742f332e342e322057696e646f7773aa637265617465645f6f6ed70100"
        "035d15590f4000b270617373776f72645f616c676f726974686d84a474797065a84152"
        "474f4e324944ab6d656d6c696d69745f6b62ce00020000a86f70736c696d697403ab70"
        "6172616c6c656c69736d01b07661756c745f6b65795f616363657373c4143c7661756c"
        "745f6b65795f61636365737320323e86ae617574685f6d6574686f645f6964d80209e4"
        "89cc8ba844acbec359e27790fd68ad637265617465645f62795f6970a0b56372656174"
        "65645f62795f757365725f6167656e74bb5061727365632d436c69656e742f332e342e"
        "312057696e646f7773aa637265617465645f6f6ed70100035d2976e6a000b270617373"
        "776f72645f616c676f726974686dc0b07661756c745f6b65795f616363657373c4143c"
        "7661756c745f6b65795f61636365737320313e"
    )
    .as_ref();

    let expected = authenticated_account_cmds::auth_method_list::Rep::Ok {
        items: vec![
            authenticated_account_cmds::auth_method_list::AuthMethod {
                auth_method_id: AccountAuthMethodID::from(hex!("1fe91588816a4df98400a9dc2d0bdfa7")),
                created_on: "2000-01-02T00:00:00Z".parse().unwrap(),
                created_by_ip: "127.0.0.2".to_string(),
                created_by_user_agent: "Parsec-Client/3.4.2 Windows".to_string(),
                vault_key_access: b"<vault_key_access 2>".as_ref().into(),
                password_algorithm: Some(UntrustedPasswordAlgorithm::Argon2id {
                    opslimit: 3,
                    memlimit_kb: 128 * 1024,
                    parallelism: 1,
                }),
            },
            authenticated_account_cmds::auth_method_list::AuthMethod {
                auth_method_id: AccountAuthMethodID::from(hex!("09e489cc8ba844acbec359e27790fd68")),
                created_on: "2000-01-03T00:00:00Z".parse().unwrap(),
                created_by_ip: "".to_string(),
                created_by_user_agent: "Parsec-Client/3.4.1 Windows".to_string(),
                vault_key_access: b"<vault_key_access 1>".as_ref().into(),
                password_algorithm: None,
            },
        ],
    };
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = authenticated_account_cmds::auth_method_list::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_account_cmds::auth_method_list::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
