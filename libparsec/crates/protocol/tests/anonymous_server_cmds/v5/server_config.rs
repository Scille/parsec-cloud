// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;

use super::anonymous_server_cmds;

// Request

pub fn req() {
    // Generated from Parsec 3.5.3-a.0+dev
    // Content:
    //   cmd: 'server_config'
    let raw: &[u8] = hex!("81a3636d64ad7365727665725f636f6e666967").as_ref();
    let req = anonymous_server_cmds::server_config::Req;

    println!("***expected: {:?}", req.dump().unwrap());
    let expected = anonymous_server_cmds::AnyCmdReq::ServerConfig(req);
    let data = anonymous_server_cmds::AnyCmdReq::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let anonymous_server_cmds::AnyCmdReq::ServerConfig(req2) = data else {
        unreachable!()
    };
    let raw2 = req2.dump().unwrap();

    let data2 = anonymous_server_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    let raw_expected = [
        (
            // Generated from Parsec 3.5.3-a.0+dev
            // Content:
            //   status: 'ok'
            //   account: 'DISABLED'
            //   openbao: { type: 'DISABLED', }
            //   organization_bootstrap: 'WITH_BOOTSTRAP_TOKEN'
            hex!(
                "84a6737461747573a26f6ba76163636f756e74a844495341424c4544a76f70656e6261"
                "6f81a474797065a844495341424c4544b66f7267616e697a6174696f6e5f626f6f7473"
                "74726170b4574954485f424f4f5453545241505f544f4b454e"
            ).as_ref(),
            anonymous_server_cmds::server_config::Rep::Ok {
                account: anonymous_server_cmds::server_config::AccountConfig::Disabled,
                organization_bootstrap: anonymous_server_cmds::server_config::OrganizationBootstrapConfig::WithBootstrapToken,
                openbao: anonymous_server_cmds::server_config::OpenBaoConfig::Disabled,
            }
        ),
        (
            // Legacy format from Parsec < 3.8, missing the `transit_mount_path` field
            //
            // Generated from Parsec 3.5.3-a.0+dev
            // Content:
            //   status: 'ok'
            //   account: 'ENABLED_WITH_VAULT'
            //   openbao: {
            //     type: 'ENABLED',
            //     auths: [
            //       { id: 'HEXAGONE', mount_path: 'auth/hexagone', },
            //       { id: 'PRO_CONNECT', mount_path: 'auth/pro_connect', },
            //     ],
            //     secret: { type: 'KV2', mount_path: 'secrets', },
            //     server_url: 'https://openbao.parsec.invalid',
            //   }
            //   organization_bootstrap: 'SPONTANEOUS'
            hex!(
                "84a6737461747573a26f6ba76163636f756e74b2454e41424c45445f574954485f5641"
                "554c54a76f70656e62616f84a474797065a7454e41424c4544a561757468739282a269"
                "64a848455841474f4e45aa6d6f756e745f70617468ad617574682f68657861676f6e65"
                "82a26964ab50524f5f434f4e4e454354aa6d6f756e745f70617468b0617574682f7072"
                "6f5f636f6e6e656374a673656372657482a474797065a34b5632aa6d6f756e745f7061"
                "7468a773656372657473aa7365727665725f75726cbe68747470733a2f2f6f70656e62"
                "616f2e7061727365632e696e76616c6964b66f7267616e697a6174696f6e5f626f6f74"
                "7374726170ab53504f4e54414e454f5553"
            ).as_ref(),
            anonymous_server_cmds::server_config::Rep::Ok {
                account: anonymous_server_cmds::server_config::AccountConfig::EnabledWithVault,
                organization_bootstrap: anonymous_server_cmds::server_config::OrganizationBootstrapConfig::Spontaneous,
                openbao: anonymous_server_cmds::server_config::OpenBaoConfig::Enabled {
                    server_url: "https://openbao.parsec.invalid".to_string(),
                    secret: anonymous_server_cmds::server_config::OpenBaoSecretConfig::KV2 {
                        mount_path: "secrets".to_string(),
                    },
                    transit_mount_path: None,
                    auths: vec![
                        anonymous_server_cmds::server_config::OpenBaoAuthConfig {
                            id: "HEXAGONE".to_string(),
                            mount_path: "auth/hexagone".to_string(),
                        },
                        anonymous_server_cmds::server_config::OpenBaoAuthConfig {
                            id: "PRO_CONNECT".to_string(),
                            mount_path: "auth/pro_connect".to_string(),
                        }
                    ]
                },
            }
        ),
        (
            // Generated from Parsec 3.7.2-a.0+dev
            // Content:
            //   status: 'ok'
            //   account: 'ENABLED_WITH_VAULT'
            //   openbao: {
            //     type: 'ENABLED',
            //     auths: [
            //       { id: 'HEXAGONE', mount_path: 'auth/hexagone', },
            //       { id: 'PRO_CONNECT', mount_path: 'auth/pro_connect', },
            //     ],
            //     secret: { type: 'KV2', mount_path: 'secrets', },
            //     server_url: 'https://openbao.parsec.invalid',
            //     transit_mount_path: 'transit',
            //   }
            //   organization_bootstrap: 'SPONTANEOUS'
            hex!(
                "84a6737461747573a26f6ba76163636f756e74b2454e41424c45445f574954485f5641"
                "554c54a76f70656e62616f85a474797065a7454e41424c4544a561757468739282a269"
                "64a848455841474f4e45aa6d6f756e745f70617468ad617574682f68657861676f6e65"
                "82a26964ab50524f5f434f4e4e454354aa6d6f756e745f70617468b0617574682f7072"
                "6f5f636f6e6e656374a673656372657482a474797065a34b5632aa6d6f756e745f7061"
                "7468a773656372657473aa7365727665725f75726cbe68747470733a2f2f6f70656e62"
                "616f2e7061727365632e696e76616c6964b27472616e7369745f6d6f756e745f706174"
                "68a77472616e736974b66f7267616e697a6174696f6e5f626f6f747374726170ab5350"
                "4f4e54414e454f5553"
            ).as_ref(),
            anonymous_server_cmds::server_config::Rep::Ok {
                account: anonymous_server_cmds::server_config::AccountConfig::EnabledWithVault,
                organization_bootstrap: anonymous_server_cmds::server_config::OrganizationBootstrapConfig::Spontaneous,
                openbao: anonymous_server_cmds::server_config::OpenBaoConfig::Enabled {
                    server_url: "https://openbao.parsec.invalid".to_string(),
                    secret: anonymous_server_cmds::server_config::OpenBaoSecretConfig::KV2 {
                        mount_path: "secrets".to_string(),
                    },
                    transit_mount_path: Some("transit".to_string()),
                    auths: vec![
                        anonymous_server_cmds::server_config::OpenBaoAuthConfig {
                            id: "HEXAGONE".to_string(),
                            mount_path: "auth/hexagone".to_string(),
                        },
                        anonymous_server_cmds::server_config::OpenBaoAuthConfig {
                            id: "PRO_CONNECT".to_string(),
                            mount_path: "auth/pro_connect".to_string(),
                        }
                    ]
                },
            }
        )
    ];

    for (raw, expected) in raw_expected {
        println!("***expected: {:?}", expected.dump().unwrap());

        let data = anonymous_server_cmds::server_config::Rep::load(raw).unwrap();
        p_assert_eq!(data, expected);

        // Also test serialization round trip
        let raw2 = data.dump().unwrap();

        let data2 = anonymous_server_cmds::server_config::Rep::load(&raw2).unwrap();

        p_assert_eq!(data2, expected);
    }
}
