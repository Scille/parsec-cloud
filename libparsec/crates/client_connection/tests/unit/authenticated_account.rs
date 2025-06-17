// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use crate::{
    authenticated_account_cmds::AccountAuthMethod, test_register_low_level_send_hook,
    AuthenticatedAccountCmds, Bytes, ConnectionError, HeaderMap, HeaderName, HeaderValue,
    ProxyConfig, ResponseMock, StatusCode,
};
use libparsec_protocol::{
    authenticated_account_cmds::latest as authenticated_account_cmds, API_LATEST_VERSION,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;
use reqwest::header::AUTHORIZATION;

#[parsec_test(testbed = "empty")]
async fn ok_mocked(env: &TestbedEnv) {
    ok(env, true).await
}

// TODO
#[ignore = "no authentication yet"]
#[parsec_test(testbed = "empty", with_server)]
async fn ok_with_server(env: &TestbedEnv) {
    ok(env, false).await
}

async fn ok(env: &TestbedEnv, mocked: bool) {
    // Good request

    let addr = env.server_addr.clone();
    let auth_method = AccountAuthMethod {
        time_provider: TimeProvider::default(),
        id: AccountAuthMethodID::from_hex("9aae259f748045cc9fe7146eab0b132e").unwrap(),
        mac_key: SecretKey::generate(),
    };
    let cmds = AuthenticatedAccountCmds::new(
        &env.discriminant_dir,
        addr,
        ProxyConfig::default(),
        auth_method,
    )
    .unwrap();

    if mocked {
        test_register_low_level_send_hook(&env.discriminant_dir, |request_builder| async {
            let request = request_builder.build().unwrap();
            let headers = request.headers();
            println!("headers: {:?}", headers);
            p_assert_eq!(
                headers.get("Content-Type"),
                Some(&HeaderValue::from_static("application/msgpack"))
            );

            assert!(headers
                .get(AUTHORIZATION)
                .unwrap()
                .as_bytes()
                .starts_with(b"Bearer PARSEC-MAC-BLAKE2B.9aae259f748045cc9fe7146eab0b132e."));

            let body = request.body().unwrap().as_bytes().unwrap();
            let request = authenticated_account_cmds::AnyCmdReq::load(body).unwrap();
            p_assert_matches!(request, authenticated_account_cmds::AnyCmdReq::Ping(authenticated_account_cmds::ping::Req { ping }) if ping == "foo");

            Ok(ResponseMock::Mocked((
                StatusCode::OK,
                HeaderMap::new(),
                authenticated_account_cmds::ping::Rep::Ok {
                    pong: "foo".to_owned(),
                }
                .dump()
                .unwrap()
                .into(),
            )))
        });
    }

    let rep = cmds
        .send(authenticated_account_cmds::ping::Req {
            ping: "foo".to_owned(),
        })
        .await;
    p_assert_eq!(
        rep.unwrap(),
        authenticated_account_cmds::ping::Rep::Ok {
            pong: "foo".to_owned()
        }
    );
}

// TODO
#[ignore = "no authentication yet"]
#[parsec_test(testbed = "empty")]
async fn invalid_token_mocked(env: &TestbedEnv) {
    invalid_token(env, true).await
}

// TODO
#[ignore = "no authentication yet"]
#[parsec_test(testbed = "empty", with_server)]
async fn invalid_token_with_server(env: &TestbedEnv) {
    invalid_token(env, false).await
}

async fn invalid_token(env: &TestbedEnv, mocked: bool) {
    // Bad request: invalid account token

    let addr = env.server_addr.clone();
    let auth_method = AccountAuthMethod {
        id: AccountAuthMethodID::from_hex("9aae259f748045cc9fe7146eab0b132e").unwrap(),
        time_provider: TimeProvider::default(),
        mac_key: SecretKey::generate(),
    };
    let cmds = AuthenticatedAccountCmds::new(
        &env.discriminant_dir,
        addr,
        ProxyConfig::default(),
        auth_method,
    )
    .unwrap();

    if mocked {
        test_register_low_level_send_hook(&env.discriminant_dir, |_request_builder| async {
            Ok(ResponseMock::Mocked((
                StatusCode::FORBIDDEN,
                HeaderMap::new(),
                Bytes::new(),
            )))
        });
    }

    let rep = cmds
        .send(authenticated_account_cmds::ping::Req {
            ping: "foo".to_owned(),
        })
        .await;
    p_assert_matches!(rep, Err(ConnectionError::BadAuthenticationInfo));
}

// Must use a macro to generate the parametrized tests here given `test_register_low_level_send_hook`
// only accept a static closure (i.e. `fn`, not `FnMut`).
macro_rules! register_rpc_http_hook {
    ($test_name: ident, $response_status_code: expr, $assert_err_cb: expr $(, $header_key:literal : $header_value:expr)* $(,)?) => {
        #[parsec_test(testbed = "minimal")]
        async fn $test_name(env: &TestbedEnv) {
            let addr = env.server_addr.clone();
            let auth_method = AccountAuthMethod {
                id: AccountAuthMethodID::from_hex("9aae259f748045cc9fe7146eab0b132e").unwrap(),
                time_provider: TimeProvider::default(),
                mac_key: SecretKey::generate()
            };
            let cmds =
                AuthenticatedAccountCmds::new(
                    &env.discriminant_dir,
                    addr,
                    ProxyConfig::default(),
                    auth_method
                ).unwrap();

            test_register_low_level_send_hook(
                &env.discriminant_dir,
                move |_request_builder| async {
                    let header_map = HeaderMap::from_iter(
                        [
                            $({
                                let header_key: HeaderName = $header_key.try_into().unwrap();
                                let header_value: HeaderValue = $header_value.try_into().unwrap();
                                (header_key, header_value)
                            },)*
                        ]
                    );
                    Ok(ResponseMock::Mocked((
                        $response_status_code,
                        header_map,
                        "".into(),
                    )))
                },
            );

            let err = cmds
                .send(authenticated_account_cmds::ping::Req {
                    ping: "foo".to_owned(),
                })
                .await
                .unwrap_err();
            ($assert_err_cb)(err)
        }
    };
}
register_rpc_http_hook!(
    rpc_missing_authentication_info_http_codes_401,
    StatusCode::from_u16(401).unwrap(),
    |err| {
        p_assert_matches!(err, ConnectionError::MissingAuthenticationInfo);
    }
);
register_rpc_http_hook!(
    rpc_bad_authentication_info_http_codes_403,
    StatusCode::from_u16(403).unwrap(),
    |err| {
        p_assert_matches!(err, ConnectionError::BadAuthenticationInfo);
    }
);

register_rpc_http_hook!(
    rpc_bad_accept_type_http_codes_406,
    StatusCode::from_u16(406).unwrap(),
    |err| {
        p_assert_matches!(err, ConnectionError::BadAcceptType);
    }
);

register_rpc_http_hook!(
    rpc_bad_content_type_http_codes_415,
    StatusCode::from_u16(415).unwrap(),
    |err| {
        p_assert_matches!(err, ConnectionError::BadContent);
    }
);
register_rpc_http_hook!(
    rpc_unsupported_api_version_http_codes_422,
    StatusCode::from_u16(422).unwrap(),
    |err| {
        p_assert_matches!(err, ConnectionError::MissingSupportedApiVersions { api_version } if api_version == *API_LATEST_VERSION );
    }
);
register_rpc_http_hook!(
    rpc_unsupported_api_version_http_codes_422_with_supported_api_versions,
    StatusCode::from_u16(422).unwrap(),
    |err| {
        p_assert_matches!(err, ConnectionError::UnsupportedApiVersion { api_version, supported_api_versions } if api_version == *API_LATEST_VERSION && supported_api_versions == vec![(8,1).into(), (9,0).into()]);
    },
    "Supported-Api-Versions": "8.1;9.0"
);

register_rpc_http_hook!(
    rpc_invalid_http_status_499,
    StatusCode::from_u16(499).unwrap(),
    |err| {
        p_assert_matches!(err, ConnectionError::InvalidResponseStatus(status) if status == 499);
    }
);
register_rpc_http_hook!(
    rpc_no_response_http_codes_502,
    StatusCode::BAD_GATEWAY,
    |err| {
        p_assert_matches!(err, ConnectionError::NoResponse(_));
    }
);
register_rpc_http_hook!(
    rpc_no_response_http_codes_503,
    StatusCode::SERVICE_UNAVAILABLE,
    |err| {
        p_assert_matches!(err, ConnectionError::NoResponse(_));
    }
);
register_rpc_http_hook!(
    rpc_no_response_http_codes_504,
    StatusCode::GATEWAY_TIMEOUT,
    |err| {
        p_assert_matches!(err, ConnectionError::NoResponse(_));
    }
);
