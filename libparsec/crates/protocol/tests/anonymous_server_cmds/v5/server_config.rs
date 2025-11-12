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
            //   client_agent: 'NATIVE_ONLY'
            //   openbao: { type: 'DISABLED', }
            //   organization_bootstrap: 'WITH_BOOTSTRAP_TOKEN'
            hex!(
                "85a6737461747573a26f6ba76163636f756e74a844495341424c4544ac636c69656e74"
                "5f6167656e74ab4e41544956455f4f4e4c59a76f70656e62616f81a474797065a84449"
                "5341424c4544b66f7267616e697a6174696f6e5f626f6f747374726170b4574954485f"
                "424f4f5453545241505f544f4b454e"
            ).as_ref(),
            anonymous_server_cmds::server_config::Rep::Ok {
                client_agent: anonymous_server_cmds::server_config::ClientAgentConfig::NativeOnly,
                account: anonymous_server_cmds::server_config::AccountConfig::Disabled,
                organization_bootstrap: anonymous_server_cmds::server_config::OrganizationBootstrapConfig::WithBootstrapToken,
                openbao: anonymous_server_cmds::server_config::OpenBaoConfig::Disabled,
            }
        ),
        (
            // Generated from Parsec 3.5.3-a.0+dev
            // Content:
            //   status: 'ok'
            //   account: 'ENABLED_WITHOUT_VAULT'
            //   client_agent: 'NATIVE_ONLY'
            //   openbao: { type: 'DISABLED', }
            //   organization_bootstrap: 'WITH_BOOTSTRAP_TOKEN'
            hex!(
                "85a6737461747573a26f6ba76163636f756e74b5454e41424c45445f574954484f5554"
                "5f5641554c54ac636c69656e745f6167656e74ab4e41544956455f4f4e4c59a76f7065"
                "6e62616f81a474797065a844495341424c4544b66f7267616e697a6174696f6e5f626f"
                "6f747374726170b4574954485f424f4f5453545241505f544f4b454e"
            ).as_ref(),
            anonymous_server_cmds::server_config::Rep::Ok {
                client_agent: anonymous_server_cmds::server_config::ClientAgentConfig::NativeOnly,
                account: anonymous_server_cmds::server_config::AccountConfig::EnabledWithoutVault,
                organization_bootstrap: anonymous_server_cmds::server_config::OrganizationBootstrapConfig::WithBootstrapToken,
                openbao: anonymous_server_cmds::server_config::OpenBaoConfig::Disabled,
            }
        ),
        (
            // Generated from Parsec 3.5.3-a.0+dev
            // Content:
            //   status: 'ok'
            //   account: 'ENABLED_WITH_VAULT'
            //   client_agent: 'NATIVE_OR_WEB'
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
                "85a6737461747573a26f6ba76163636f756e74b2454e41424c45445f574954485f5641"
                "554c54ac636c69656e745f6167656e74ad4e41544956455f4f525f574542a76f70656e"
                "62616f84a474797065a7454e41424c4544a561757468739282a26964a848455841474f"
                "4e45aa6d6f756e745f70617468ad617574682f68657861676f6e6582a26964ab50524f"
                "5f434f4e4e454354aa6d6f756e745f70617468b0617574682f70726f5f636f6e6e6563"
                "74a673656372657482a474797065a34b5632aa6d6f756e745f70617468a77365637265"
                "7473aa7365727665725f75726cbe68747470733a2f2f6f70656e62616f2e7061727365"
                "632e696e76616c6964b66f7267616e697a6174696f6e5f626f6f747374726170ab5350"
                "4f4e54414e454f5553"
            ).as_ref(),
            anonymous_server_cmds::server_config::Rep::Ok {
                client_agent: anonymous_server_cmds::server_config::ClientAgentConfig::NativeOrWeb,
                account: anonymous_server_cmds::server_config::AccountConfig::EnabledWithVault,
                organization_bootstrap: anonymous_server_cmds::server_config::OrganizationBootstrapConfig::Spontaneous,
                openbao: anonymous_server_cmds::server_config::OpenBaoConfig::Enabled {
                    server_url: "https://openbao.parsec.invalid".to_string(),
                    secret: anonymous_server_cmds::server_config::OpenBaoSecretConfig::KV2 {
                        mount_path: "secrets".to_string(),
                    },
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
