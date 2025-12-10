// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use crate::{
    test_register_low_level_send_hook, AnonymousCmds, ConnectionError, HeaderMap, HeaderName,
    HeaderValue, ProxyConfig, ResponseMock, StatusCode,
};
use libparsec_protocol::{anonymous_cmds::latest as anonymous_cmds, API_LATEST_VERSION};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

#[parsec_test(testbed = "minimal")]
async fn ok_mocked(env: &TestbedEnv) {
    ok(env, true).await
}

#[parsec_test(testbed = "minimal", with_server)]
async fn ok_with_server(env: &TestbedEnv) {
    ok(env, false).await
}

async fn ok(env: &TestbedEnv, mocked: bool) {
    let addr =
        ParsecPkiEnrollmentAddr::new(env.server_addr.clone(), env.organization_id.to_owned());
    let cmds =
        AnonymousCmds::new(&env.discriminant_dir, addr.into(), ProxyConfig::default()).unwrap();

    // Good request

    if mocked {
        test_register_low_level_send_hook(&env.discriminant_dir, |request_builder| async {
            let request = request_builder.build().unwrap();
            let headers = request.headers();
            println!("headers: {headers:?}");
            p_assert_eq!(
                headers.get("Content-Type"),
                Some(&HeaderValue::from_static("application/msgpack"))
            );
            // Cannot check `User-Agent` here given reqwest adds it in a later step
            // assert!(headers.get("User-Agent").unwrap().to_str().unwrap().starts_with("Parsec-Client/"));
            assert!(headers.get("Authorization").is_none());

            let body = request.body().unwrap().as_bytes().unwrap();
            let request = anonymous_cmds::AnyCmdReq::load(body).unwrap();
            p_assert_matches!(request, anonymous_cmds::AnyCmdReq::Ping(anonymous_cmds::ping::Req { ping }) if ping == "foo");

            Ok(ResponseMock::Mocked((
                StatusCode::OK,
                HeaderMap::new(),
                anonymous_cmds::ping::Rep::Ok {
                    pong: "foo".to_owned(),
                }
                .dump()
                .unwrap()
                .into(),
            )))
        });
    }

    let rep = cmds
        .send(anonymous_cmds::ping::Req {
            ping: "foo".to_owned(),
        })
        .await;
    p_assert_eq!(
        rep.unwrap(),
        anonymous_cmds::ping::Rep::Ok {
            pong: "foo".to_owned()
        }
    );
}

// Must use a macro to generate the parametrized tests here given `test_register_low_level_send_hook`
// only accept a static closure (i.e. `fn`, not `FnMut`).
macro_rules! register_rpc_http_hook {
    ($test_name: ident, $response_status_code: expr, $assert_err_cb: expr $(, $header_key:literal : $header_value:expr)* $(,)?) => {
        #[parsec_test(testbed = "minimal")]
        async fn $test_name(env: &TestbedEnv) {
            let addr = ParsecPkiEnrollmentAddr::new(
                env.server_addr.clone(),
                env.organization_id.to_owned(),
            );
            let cmds =
                AnonymousCmds::new(&env.discriminant_dir, addr.into(), ProxyConfig::default()).unwrap();

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
                .send(anonymous_cmds::ping::Req {
                    ping: "foo".to_owned(),
                })
                .await
                .unwrap_err();
            ($assert_err_cb)(err)
        }
    };
}
register_rpc_http_hook!(
    rpc_organization_not_found_http_codes_404,
    StatusCode::from_u16(404).unwrap(),
    |err| {
        p_assert_matches!(err, ConnectionError::OrganizationNotFound);
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
    rpc_expired_organization_http_codes_460,
    StatusCode::from_u16(460).unwrap(),
    |err| {
        p_assert_matches!(err, ConnectionError::ExpiredOrganization);
    }
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
