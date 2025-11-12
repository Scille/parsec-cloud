// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

use super::anonymous_server_cmds;

// Request

pub fn req() {
    // Generated from Parsec 3.4.0-a.7+dev
    // Content:
    //   cmd: 'auth_method_password_get_algorithm'
    //   email: 'alice@invalid.com'
    let raw: &[u8] = hex!(
        "82a3636d64d922617574685f6d6574686f645f70617373776f72645f6765745f616c67"
        "6f726974686da5656d61696cb1616c69636540696e76616c69642e636f6d"
    )
    .as_ref();

    let req = anonymous_server_cmds::auth_method_password_get_algorithm::Req {
        email: "alice@invalid.com".parse().unwrap(),
    };

    let expected = anonymous_server_cmds::AnyCmdReq::AuthMethodPasswordGetAlgorithm(req.clone());
    println!("***expected: {:?}", req.dump().unwrap());

    let data = anonymous_server_cmds::AnyCmdReq::load(raw).unwrap();
    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let anonymous_server_cmds::AnyCmdReq::AuthMethodPasswordGetAlgorithm(req2) = data else {
        unreachable!()
    };

    let raw2 = req2.dump().unwrap();

    let data2 = anonymous_server_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    // Generated from Parsec 3.4.1-a.0+dev
    // Content:
    //   status: 'ok'
    //   password_algorithm: { type: 'ARGON2ID', memlimit_kb: 131072, opslimit: 3, parallelism: 1, }
    let raw: &[u8] = hex!(
        "82a6737461747573a26f6bb270617373776f72645f616c676f726974686d84a4747970"
        "65a84152474f4e324944ab6d656d6c696d69745f6b62ce00020000a86f70736c696d69"
        "7403ab706172616c6c656c69736d01"
    )
    .as_ref();

    let expected = anonymous_server_cmds::auth_method_password_get_algorithm::Rep::Ok {
        password_algorithm: UntrustedPasswordAlgorithm::Argon2id {
            opslimit: 3,
            memlimit_kb: 128 * 1024,
            parallelism: 1,
        },
    };
    println!("***expected: {:?}", expected.dump().unwrap());

    let data = anonymous_server_cmds::auth_method_password_get_algorithm::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 =
        anonymous_server_cmds::auth_method_password_get_algorithm::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
